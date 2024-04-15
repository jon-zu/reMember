/*

    TODO:
    * The shutdown is not working properly, because sending errors in the system are not handled

*/

use futures::Future;
use tick::TickHandler;
use tokio::sync::mpsc;

use crate::{time::interval::Interval, Instant};

pub mod broadcast;
pub mod room;
pub mod session;
pub mod system;
pub mod tick;

pub use broadcast::{BroadcastSet, RoomSessionContext, SessionSet};

pub const MESSAGES_PER_TICK: usize = 100;

#[derive(Debug)]
pub struct Sender<T>(mpsc::Sender<T>);

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Sender<T> {
    pub fn send(&self, msg: T) -> impl Future<Output = Result<(), mpsc::error::SendError<T>>> + '_ {
        self.0.send(msg)
    }

    pub fn try_send(&self, msg: T) -> Result<(), mpsc::error::TrySendError<T>> {
        self.0.try_send(msg)
    }
}

#[derive(Debug)]
pub struct Receiver<T>(mpsc::Receiver<T>);

impl<T> Receiver<T> {
    pub fn close(&mut self) {
        self.0.close();
    }

    pub fn try_recv(&mut self) -> Result<T, mpsc::error::TryRecvError> {
        self.0.try_recv()
    }

    pub fn try_recv_many(
        &mut self,
        limit: usize,
    ) -> impl Iterator<Item = Result<T, mpsc::error::TryRecvError>> + '_ {
        (0..limit)
            .map(move |_| self.try_recv())
            .take_while(|r| !matches!(r, Err(mpsc::error::TryRecvError::Empty)))
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.0.recv().await
    }
}

#[must_use]
pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel(capacity);
    (Sender(tx), Receiver(rx))
}

pub trait State: Send + 'static {}
impl<T: Send + 'static> State for T {}

pub trait Context {
    fn time(&self) -> Instant;
}

pub trait TickActor: Sized + Send + 'static {
    type Msg: State;
    type Context: Context + Send;
    type Error: std::fmt::Debug + Sync + State;

    fn on_tick(&mut self, ctx: &mut Self::Context) -> Result<(), Self::Error>;
    fn on_msg(&mut self, ctx: &mut Self::Context, msg: Self::Msg) -> Result<(), Self::Error>;
}

pub struct TickActorCell<A: TickActor> {
    actor: A,
    ctx: A::Context,
    rx: Receiver<A::Msg>,
    update_interval: Interval,
}

impl<A: TickActor> TickHandler for TickActorCell<A> {
    type Error = A::Error;

    fn on_tick(&mut self, t: Instant) -> Result<(), Self::Error> {
        if self.update_interval.try_tick(t) {
            for msg in self.rx.try_recv_many(MESSAGES_PER_TICK) {
                self.actor.on_msg(&mut self.ctx, msg.unwrap())?;
            }
            self.actor.on_tick(&mut self.ctx)?;
        }

        Ok(())
    }
}
