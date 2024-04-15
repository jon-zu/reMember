use std::sync::Arc;

use dashmap::DashMap;
use futures::Future;
use shroom_pkt::pkt::{EncodeMessage, Message};
use tokio::{
    sync::{broadcast, oneshot},
    task::JoinHandle,
};

use crate::{
    act::{Receiver, Sender},
    net::socket::PktMsg,
    util::EncodeBuf,
    ClockHandle, Id,
};

use super::session::{NetSessionHandle, SessionHandler};

pub enum RoomBroadcastMsg<H: SessionHandler> {
    Filter(<H as SessionHandler>::Id, <H as SessionHandler>::Message),
    Msg(<H as SessionHandler>::Message),
}

impl<H> Clone for RoomBroadcastMsg<H>
where
    H: SessionHandler,
    H::Message: Clone,
{
    fn clone(&self) -> Self {
        match self {
            RoomBroadcastMsg::Filter(id, msg) => RoomBroadcastMsg::Filter(*id, msg.clone()),
            RoomBroadcastMsg::Msg(msg) => RoomBroadcastMsg::Msg(msg.clone()),
        }
    }
}

pub trait RoomHandler: Sized + Send + 'static {
    type Id: Id;
    type Error: std::fmt::Debug
        + From<std::io::Error>
        + From<shroom_pkt::Error>
        + From<super::Error>
        + Send
        + Sync
        + 'static;

    type Session: SessionHandler<Room = Self, Error = Self::Error>;
    type State: Send + 'static;

    fn on_tick(
        &mut self,
        ctx: &RoomContext<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub struct RoomContext<H: RoomHandler> {
    sessions: DashMap<<H::Session as SessionHandler>::Id, NetSessionHandle<H::Session>>,
    broadcast_tx: broadcast::Sender<RoomBroadcastMsg<H::Session>>,
    buf: std::sync::Mutex<EncodeBuf>,
    pub state: H::State,
}

impl<H: RoomHandler> RoomContext<H> {
    fn encode(&self, msg: impl EncodeMessage) -> Result<Message, H::Error> {
        self.buf
            .lock()
            .expect("lock")
            .encode_onto(msg)
            .map_err(|err| err.into())
    }

    pub fn broadcast(
        &self,
        filter_id: Option<<H::Session as SessionHandler>::Id>,
        msg: <H::Session as SessionHandler>::Message,
    ) {
        let msg = match filter_id {
            Some(id) => RoomBroadcastMsg::Filter(id, msg),
            None => RoomBroadcastMsg::Msg(msg),
        };

        self.broadcast_tx
            .send(msg)
            .map_err(|_| ())
            .expect("broadcast");
    }

    pub fn broadcast_encode(
        &self,
        filter_id: Option<<H::Session as SessionHandler>::Id>,
        msg: impl EncodeMessage,
    ) -> Result<(), H::Error> {
        let msg = self.encode(msg)?;
        self.broadcast(filter_id, PktMsg::Packet(msg).into());
        Ok(())
    }

    pub async fn send_to(
        &self,
        id: <H::Session as SessionHandler>::Id,
        msg: <H::Session as SessionHandler>::Message,
    ) -> Result<(), H::Error> {
        Ok(self
            .sessions
            .get(&id)
            .ok_or_else(|| super::Error::SessionNotFound)?
            .value()
            .tx
            .send(msg)
            .await
            .map_err(|_| super::Error::SendTx)?)
    }

    pub async fn send_to_encode(
        &self,
        id: <H::Session as SessionHandler>::Id,
        msg: impl EncodeMessage,
    ) -> Result<(), H::Error> {
        let msg = self.encode(msg)?;
        self.send_to(id, PktMsg::Packet(msg).into()).await
    }
}

pub enum RoomControlMessage<H: RoomHandler> {
    Join(
        NetSessionHandle<H::Session>,
        oneshot::Sender<RoomJoinHandle<H::Session>>,
    ),
    Leave(<H::Session as SessionHandler>::Id, oneshot::Sender<()>),
}

pub struct NetRoom<H: RoomHandler> {
    ctx: Arc<RoomContext<H>>,
    rx: Receiver<RoomControlMessage<H>>,
    clock_handle: ClockHandle,
    idle_counter: usize,
    handler: H,
}

pub struct RoomJoinHandle<H: SessionHandler> {
    pub rx: broadcast::Receiver<RoomBroadcastMsg<H>>,
    pub state: Arc<RoomContext<H::Room>>,
}

impl<H: RoomHandler> NetRoom<H> {
    pub fn new(
        handler: H,
        ctx: Arc<RoomContext<H>>,
        rx: Receiver<RoomControlMessage<H>>,
        clock_handle: ClockHandle,
    ) -> Self {
        Self {
            ctx,
            rx,
            clock_handle,
            idle_counter: 0,
            handler,
        }
    }

    async fn handle_message(&mut self, msg: RoomControlMessage<H>) -> Result<(), H::Error> {
        match msg {
            RoomControlMessage::Join(session, rply) => {
                // Handle enter here
                let rx = self.ctx.broadcast_tx.subscribe();
                self.ctx.sessions.insert(session.id, session);
                rply.send(RoomJoinHandle { rx, state: self.ctx.clone() })
                    .map_err(|_| super::Error::SendTx)?;

                self.idle_counter = 0;
            }
            RoomControlMessage::Leave(id, rx) => {
                self.ctx.sessions.remove(&id);
                rx.send(()).map_err(|_| super::Error::SendTx)?;
            }
        }

        Ok(())
    }

    async fn on_tick(&mut self) -> Result<(), H::Error> {
        if self.ctx.sessions.is_empty() {
            self.handler.on_tick(&self.ctx).await?;
            self.idle_counter += 1;
            if self.idle_counter > 30 {
                //TODO shutdown
            }
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), H::Error> {
        loop {
            tokio::select! {
                msg = self.rx.recv() => {
                    self.handle_message(msg.expect("room control message")).await?;
                },
                _ = self.clock_handle.tick() => {
                    self.on_tick().await?;
                }
            }
        }
    }
}

pub struct NetRoomRunner<H: RoomHandler> {
    tx: Sender<RoomControlMessage<H>>,
    ctx: Arc<RoomContext<H>>,
    _task: JoinHandle<()>,
}

impl<H: RoomHandler> NetRoomRunner<H> {
    pub fn spawn(handler: H, state: H::State, clock: ClockHandle, msg_cap: usize) -> Self {
        let (tx, rx) = crate::act::channel(msg_cap);
        let ctx = Arc::new(RoomContext {
            broadcast_tx: broadcast::channel(msg_cap).0,
            buf: std::sync::Mutex::new(EncodeBuf::new()),
            sessions: DashMap::new(),
            state,
        });
        let mut room = NetRoom::new(handler, ctx.clone(), rx, clock);
        let task = tokio::spawn(async move {
            loop {
                if let Err(err) = room.run().await {
                    log::error!("room error: {:?}", err);
                }
            }
        });

        Self {
            tx,
            ctx,
            _task: task,
        }
    }

    pub fn handle(&self) -> NetRoomHandle<H> {
        NetRoomHandle {
            tx: self.tx.clone(),
            ctx: self.ctx.clone(),
        }
    }
}

pub struct NetRoomHandle<H: RoomHandler> {
    tx: Sender<RoomControlMessage<H>>,
    ctx: Arc<RoomContext<H>>,
}

impl<H: RoomHandler> NetRoomHandle<H> {
    pub async fn join(
        &self,
        session: NetSessionHandle<H::Session>,
    ) -> Result<RoomJoinHandle<H::Session>, H::Error> {
        let (tx, rx) = oneshot::channel();
        //TODO errors
        self.tx
            .send(RoomControlMessage::Join(session, tx))
            .await
            .map_err(|_| super::Error::SendTx)?;
        Ok(rx.await.map_err(|_| super::Error::Recv)?)
    }

    pub async fn leave(&self, id: <H::Session as SessionHandler>::Id) -> Result<(), H::Error> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(RoomControlMessage::Leave(id, tx))
            .await
            .map_err(|_| super::Error::SendTx)?;
        Ok(rx.await.map_err(|_| super::Error::Recv)?)
    }
}
