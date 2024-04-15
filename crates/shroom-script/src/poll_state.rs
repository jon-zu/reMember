use std::{
    cell::UnsafeCell,
    future::Future,
    pin::Pin,
    ptr,
    sync::{
        atomic::{AtomicBool, AtomicPtr, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

use futures::task::noop_waker;
use pin_project_lite::pin_project;

// TODO in theory Shared could be placed into a `UnsafeCell`
// because it's only accessed when the future is polled

pub struct Shared<Ctx, Input> {
    input: Mutex<Option<Input>>,
    ctx: AtomicPtr<Ctx>,
    waiting: AtomicBool,
}

impl<Ctx, Input> Default for Shared<Ctx, Input> {
    fn default() -> Self {
        Self {
            input: Mutex::new(None),
            waiting: AtomicBool::new(false),
            ctx: AtomicPtr::default(),
        }
    }
}

impl<Ctx, Input> Shared<Ctx, Input> {
    fn set_waiting(&self, val: bool) {
        self.waiting.store(val, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn is_waiting(&self) -> bool {
        self.waiting.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn is_empty_ctx(&self) -> bool {
        self.ctx.load(Ordering::SeqCst).is_null()
    }

    fn set_ctx(&self, ctx: &UnsafeCell<Ctx>) {
        self.ctx.store(ctx.get(), Ordering::SeqCst);
    }

    fn clr_ctx(&self) {
        self.ctx.store(ptr::null_mut(), Ordering::SeqCst);
    }

    async fn next_input(&self) -> anyhow::Result<Input> {
        self.set_waiting(true);
        let input = StateInputFuture(self).await;
        self.set_waiting(false);
        Ok(input)
    }

    fn with<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&Ctx) -> U,
    {
        unsafe { f(self.ctx.load(Ordering::SeqCst).as_ref().unwrap()) }
    }

    unsafe fn with_mut<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&mut Ctx) -> U,
    {
        unsafe { f(self.ctx.load(Ordering::SeqCst).as_mut().unwrap()) }
    }
}

struct StateInputFuture<'a, Ctx, Input>(&'a Shared<Ctx, Input>);
impl<'a, Ctx, Input> Future for StateInputFuture<'a, Ctx, Input> {
    type Output = Input;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Some(input) = this.0.input.lock().unwrap().take() {
            Poll::Ready(input)
        } else {
            Poll::Pending
        }
    }
}

pin_project! {
    pub struct StateHandle<Fut, Ctx, Input> {
        #[pin]
        future: Fut,
        finished: bool,
        state: Arc<Shared<Ctx, Input>>,
        waker: Waker,
    }
}

pub struct StateRef<Ctx, Input>(Arc<Shared<Ctx, Input>>);

impl<Ctx, Input> StateRef<Ctx, Input> {
    pub fn with<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&Ctx) -> U,
    {
        self.0.with(f)
    }

    pub fn with_mut<F, U>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut Ctx) -> U,
    {
        unsafe { self.0.with_mut(f) }
    }

    pub async fn next_input(&mut self) -> anyhow::Result<Input> {
        self.0.as_ref().next_input().await
    }

    pub async fn wait_for_input(&mut self, input: Input) -> anyhow::Result<()> 
        where Input: PartialEq + std::fmt::Debug {
            let inp = self.next_input().await?;
        if inp != input {
            anyhow::bail!("Unexpected input: {inp:?}");
        }

        Ok(())
    }
}

impl<Fut, Ctx, Input> StateHandle<Fut, Ctx, Input>
where
    Fut: Future<Output = anyhow::Result<()>>,
{
    fn new_with_state(fut: Fut, state: Arc<Shared<Ctx, Input>>) -> Self {
        Self {
            future: fut,
            state,
            finished: false,
            waker: noop_waker(),
        }
    }


    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnOnce(StateRef<Ctx, Input>) -> Fut,
    {
        let ctx = Arc::new(Shared::default());
        let fut = f(StateRef(ctx.clone()));
        Self::new_with_state(fut, ctx)
    }

    pub fn from_fut(fut: Fut) -> Self {
        let ctx = Arc::new(Shared::default());
        Self::new_with_state(fut, ctx)
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn poll(self: Pin<&mut Self>, ctx: &mut Ctx) -> Option<anyhow::Result<()>> {
        // Check for finish
        if self.finished {
            return Some(Err(anyhow::anyhow!("Already finished")));
        }

        // Ensure the context is empty
        // In theory not required, but this ensures 
        // the context got properly cleared
        if !self.state.is_empty_ctx() {
            return Some(Err(anyhow::anyhow!("Context not empty")));
        }

        let this = self.project();

        // TODO wait for stabilization
        pub fn from_mut<T>(value: &mut T) -> &mut UnsafeCell<T> {
            // SAFETY: `UnsafeCell<T>` has the same memory layout as `T` due to #[repr(transparent)].
            unsafe { &mut *(value as *mut T as *mut UnsafeCell<T>) }
        }

        // Set the context for the state
        this.state.set_ctx(from_mut(ctx));
        // Build the future context
        let mut cx = futures::task::Context::from_waker(this.waker);
        let res = this.future.poll(&mut cx);
        this.state.clr_ctx();

        match res {
            Poll::Ready(res) => {
                *this.finished = true;
                Some(res)
            }
            Poll::Pending if this.state.is_waiting() => Some(Ok(())),
            Poll::Pending => Some(Err(anyhow::format_err!(
                "State poll future not supposed to await non yields"
            ))),
        }
    }

    pub fn transition(
        self: Pin<&mut Self>,
        input: Option<Input>,
        ctx: &mut Ctx,
    ) -> Option<anyhow::Result<()>> {
        let state = self.state.as_ref();
        *state.input.lock().unwrap() = input;
        self.poll(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Input {
        Start,
        Next,
        //Prev,
        End,
    }

    #[test]
    fn poller() {
        type State = String;
        let mut handle = StateHandle::from_fn(|mut n: StateRef<State, Input>| {
            async move {
                n.wait_for_input(Input::Start).await?;
                n.wait_for_input(Input::Next).await?;
                n.wait_for_input(Input::End).await?;
                n.with_mut(|s| s.push_str("Hello"));
                Ok(())
            }
        });
        let mut handle = std::pin::pin!(handle);
        let mut s = String::new();
        handle.as_mut().transition(Some(Input::Start), &mut s);
        handle.as_mut().transition(Some(Input::Next), &mut s);
        s.push_str("abc_");
        handle.as_mut().transition(Some(Input::End), &mut s);
        assert_eq!(s, "abc_Hello");
        assert!(handle.as_mut().is_finished());
    }
}
