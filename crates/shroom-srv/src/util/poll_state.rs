use std::{
    cell::UnsafeCell,
    future::Future,
    pin::Pin,
    ptr,
    sync::{
        atomic::{AtomicBool, AtomicPtr, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

struct StateData<Ctx, Input> {
    input: Mutex<Option<Input>>,
    ctx: AtomicPtr<Ctx>,
    waiting: Arc<AtomicBool>,
}

impl<Ctx, Input> StateData<Ctx, Input> {
    pub fn new(waiting: Arc<AtomicBool>) -> Self {
        Self {
            input: Mutex::new(None),
            waiting,
            ctx: AtomicPtr::default(),
        }
    }

    fn set_waiting(&self, val: bool) {
        self.waiting.store(val, std::sync::atomic::Ordering::SeqCst);
    }

    async fn next_input(&self) -> anyhow::Result<Input> {
        self.set_waiting(true);
        let input = StateInputFuture(self).await;
        self.set_waiting(false);
        Ok(input)
    }

    fn set_ctx(&self, ctx: &UnsafeCell<Ctx>) {
        self.ctx.store(ctx.get(), Ordering::SeqCst);
    }

    fn clr_ctx(&self) {
        self.ctx.store(ptr::null_mut(), Ordering::SeqCst);
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

pub enum StatePoll<T> {
    Ready(T),
    WaitingOnInput,
}

pin_project! {
    pub struct StateFuture<Fut> {
        #[pin]
        fut: Fut,
        waiting: Arc<AtomicBool>,
    }
}

impl<Fut, T> Future for StateFuture<Fut>
where
    Fut: Future<Output = T> + Unpin,
{
    type Output = StatePoll<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.fut.poll(cx) {
            Poll::Ready(v) => Poll::Ready(StatePoll::Ready(v)),
            Poll::Pending if this.waiting.load(Ordering::SeqCst) => {
                Poll::Ready(StatePoll::WaitingOnInput)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

struct StateInputFuture<'a, Ctx, Input>(&'a StateData<Ctx, Input>);
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

pub struct StateHandle<Fut, Ctx, Input> {
    future: StateFuture<Fut>,
    finished: bool,
    state: Arc<StateData<Ctx, Input>>,
}

pub struct StateRef<Ctx, Input>(Arc<StateData<Ctx, Input>>);

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
}

// Taken from UnsafeSafe::from_mut ( nightly )
// TODO: use stable version, as soon that one is available
pub fn unsafe_cell_from_mut<T>(value: &mut T) -> &mut UnsafeCell<T> {
    // SAFETY: `UnsafeCell<T>` has the same memory layout as `T` due to #[repr(transparent)].
    unsafe {
        &mut *(value as *mut T).cast::<UnsafeCell<T>>()
    }
}

impl<Fut, Ctx, Input> StateHandle<Fut, Ctx, Input>
where
    Fut: Future<Output = anyhow::Result<()>> + Unpin,
{
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnOnce(StateRef<Ctx, Input>) -> Fut,
    {
        let waiting = Arc::new(AtomicBool::new(false));
        let ctx = Arc::new(StateData::new(waiting.clone()));
        let fut = f(StateRef(ctx.clone()));
        Self {
            future: StateFuture { fut, waiting },
            state: ctx,
            finished: false,
        }
    }

    pub fn from_fut(fut: Fut) -> Self {
        let waiting = Arc::new(AtomicBool::new(false));
        let ctx = Arc::new(StateData::new(waiting.clone()));
        Self {
            future: StateFuture { fut, waiting },
            state: ctx,
            finished: false,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub async fn run_once(&mut self, ctx: &mut Ctx) -> Option<anyhow::Result<()>> {
        if self.finished {
            return Some(Err(anyhow::anyhow!("Already finished")));
        }
        let state = self.state.as_ref();

        state.set_ctx(unsafe_cell_from_mut(ctx));

        let fut = &mut self.future;
        let res = fut.await;

        state.clr_ctx();
        match res {
            StatePoll::WaitingOnInput => None,
            StatePoll::Ready(res) => {
                self.finished = true;
                Some(res)
            }
        }
    }

    pub async fn transition(&mut self, input: Input, ctx: &mut Ctx) -> Option<anyhow::Result<()>> {
        let state = self.state.as_ref();
        *state.input.lock().unwrap() = Some(input);

        self.run_once(ctx).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub enum Input {
        Start,
        Next,
        //Prev,
        End,
    }

    #[tokio::test]
    async fn poller() {
        type State = String;
        let mut handle = StateHandle::from_fn(|mut n: StateRef<State, Input>| {
            Box::pin(async move {
                let _input = n.next_input().await?;
                let _input = n.next_input().await?;
                let _input = n.next_input().await?;
                n.with_mut(|s| s.push_str("Hello"));
                Ok(())
            })
        });

        let mut s = String::new();

        handle.transition(Input::Start, &mut s).await;
        handle.transition(Input::Next, &mut s).await;
        s.push_str("abc_");
        handle.transition(Input::End, &mut s).await;
        assert_eq!(s, "abc_Hello");
        assert!(handle.is_finished());
    }
}
