use std::marker::PhantomData;

use futures::Future;

use tokio::sync::mpsc;

use crate::{
    act::{
        channel, Context, Interval, Receiver, Sender, TickActor,
        MESSAGES_PER_TICK,
    }, time::clock::Ticks, Id
};

use super::RoomSessionContext;

pub trait SessionActor<R>: TickActor<Context = RoomSessionContext<R, Self>> {
    type Id: Id;
    type RoomId: Id;

    fn id(&self) -> Self::Id;
    fn room_id(&self) -> Self::RoomId;

    fn on_enter_room(&mut self, _ctx: &mut Self::Context) -> Result<(), Self::Error> {
        Ok(())
    }
    fn on_leave_room(&mut self, _ctx: &mut Self::Context) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct SessionHandle<R, A: SessionActor<R>> {
    id: A::Id,
    tx: Sender<A::Msg>,
}

impl<R, A: SessionActor<R>> SessionHandle<R, A> {
    pub fn send(
        &self,
        msg: A::Msg,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<A::Msg>>> + '_ {
        self.tx.send(msg)
    }

    pub fn try_send(&self, msg: A::Msg) -> Result<(), mpsc::error::TrySendError<A::Msg>> {
        self.tx.try_send(msg)
    }

    pub fn id(&self) -> A::Id {
        self.id
    }
}

pub struct SessionCell<R, A: SessionActor<R>> {
    pub(crate) session: A,
    rx: Receiver<A::Msg>,
    tx: Sender<A::Msg>,
    update_interval: Interval,
    _r: PhantomData<R>,
}

impl<R, A: SessionActor<R>> std::fmt::Debug for SessionCell<R, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionCell")
            .field("update_interval", &self.update_interval)
            .field("_r", &self._r)
            .finish_non_exhaustive()
    }
}

impl<R, A: SessionActor<R>> SessionCell<R, A> {
    pub fn new(session: A, msg_cap: usize) -> Self {
        let (tx, rx) = channel(msg_cap);
        Self {
            session,
            tx,
            rx,
            update_interval: Interval::new_next(Ticks(1)),
            _r: PhantomData,
        }
    }

    pub fn inner(&self) -> &A {
        &self.session
    }

    pub fn inner_mut(&mut self) -> &mut A {
        &mut self.session
    }

    pub fn handle(&self) -> SessionHandle<R, A> {
        SessionHandle {
            id: self.id(),
            tx: self.tx.clone(),
        }
    }

    pub fn id(&self) -> A::Id {
        self.session.id()
    }

    pub fn tx(&self) -> Sender<A::Msg> {
        self.tx.clone()
    }

    pub fn run_once(&mut self, ctx: &mut A::Context) -> Result<(), A::Error> {
        if self.update_interval.try_tick(ctx.time()) {
            for msg in self.rx.try_recv_many(MESSAGES_PER_TICK) {
                self.session.on_msg(ctx, msg.unwrap())?;
            }
            self.session.on_tick(ctx)?;
        }

        Ok(())
    }
}
