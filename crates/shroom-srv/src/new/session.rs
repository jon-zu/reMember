use futures::Future;
use tokio::task::JoinHandle;

use crate::{
    act::{Context, Receiver, Sender},
    net::session::{NetMsg, NetSocket},
    new::room::RoomBroadcastMsg,
    ClockHandle, GameTime, Id,
};

use super::{
    room::{RoomHandler, RoomJoinHandle},
    sys::{NetSystemHandle, SystemHandler},
};

pub struct NetSessionHandle<H: SessionHandler> {
    pub id: H::Id,
    pub tx: Sender<H::Message>,
}

pub struct SessionContext<H: SessionHandler> {
    pub socket: NetSocket,
    pub t: GameTime,
    pub room: RoomJoinHandle<H>,
}

impl<H: SessionHandler> Context for SessionContext<H> {
    fn time(&self) -> GameTime {
        self.t
    }
}

pub trait SessionHandler: Sized + Send + 'static {
    type Id: Id;
    type Room: RoomHandler<Session = Self, Error = Self::Error>;
    type System: SystemHandler<Room = Self::Room, Error = Self::Error>;

    type Error: std::fmt::Debug
        + From<std::io::Error>
        + From<shroom_pkt::Error>
        + From<super::Error>
        + Send
        + Sync
        + 'static;
    type Message: NetMsg + Clone + Send + 'static;

    fn id(&self) -> Self::Id;
    fn room_id(&self) -> <Self::Room as RoomHandler>::Id;

    fn on_net_msg(
        &mut self,
        ctx: &mut SessionContext<Self>,
        msg: shroom_pkt::pkt::Message,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn on_msg(
        &mut self,
        ctx: &mut SessionContext<Self>,
        msg: Self::Message,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn on_tick(
        &mut self,
        ctx: &mut SessionContext<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub struct NetSession<H: SessionHandler> {
    handler: H,
    tx: Sender<H::Message>,
    rx: Receiver<H::Message>,
    clock: ClockHandle,
    sys: NetSystemHandle<H::System>,
    sck: Option<NetSocket>,
}

impl<H: SessionHandler> NetSession<H> {
    pub fn new(
        handler: H,
        rx: Receiver<H::Message>,
        tx: Sender<H::Message>,
        socket: NetSocket,
        clock: ClockHandle,
        sys: NetSystemHandle<H::System>,
    ) -> Self {
        Self {
            handler,
            sck: Some(socket),
            rx,
            tx,
            clock,
            sys,
        }
    }

    async fn join_room(
        &mut self,
        room_id: <H::Room as RoomHandler>::Id,
    ) -> Result<RoomJoinHandle<H>, H::Error> {
        let room = self.sys.get_room(room_id).await?;
        let handle = room
            .join(NetSessionHandle {
                id: self.handler.id(),
                tx: self.tx.clone(),
            })
            .await?;
        Ok(handle)
    }

    pub async fn run(mut self) -> Result<(), H::Error> {
        // Build the context
        let mut ctx = SessionContext {
            socket: self.sck.take().expect("socket"),
            t: self.clock.time(),
            room: self.join_room(self.handler.room_id()).await?,
        };

        loop {
            tokio::select! {
                net_msg = ctx.socket.recv() => {
                    let msg = net_msg.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "socket closed"))?;
                    self.handler.on_net_msg(&mut ctx, msg).await?;
                },
                msg = self.rx.recv() => {
                    self.handler.on_msg(&mut ctx, msg.expect("session msg")).await?;
                },
                field_msg = ctx.room.rx.recv() => {
                    match field_msg.map_err(|err| super::Error::RoomBroadcast(err))?  {
                        RoomBroadcastMsg::Filter(id, msg) if id != self.handler.id() => {
                            self.handler.on_msg(&mut ctx, msg).await?;
                        },
                        RoomBroadcastMsg::Msg(msg) => {
                            self.handler.on_msg(&mut ctx, msg).await?;
                        },
                        _ => {}
                    }
                },
                _ = self.clock.tick() => {
                    ctx.t = self.clock.time();
                    self.handler.on_tick(&mut ctx).await?;
                }
            }
        }
    }
}

pub struct NetSessionRunner<H: SessionHandler> {
    tx: Sender<H::Message>,
    _task: JoinHandle<()>,
}

impl<H: SessionHandler> NetSessionRunner<H> {
    pub fn spawn(
        handler: H,
        sck: NetSocket,
        clock: ClockHandle,
        sys: NetSystemHandle<H::System>,
        msg_cap: usize,
    ) -> Self {
        let (tx, rx) = crate::act::channel(msg_cap);
        let session = NetSession::new(handler, rx, tx.clone(), sck, clock, sys);
        let task = tokio::spawn(async move {
            //TODO handle errors properly
            if let Err(err) = session.run().await {
                log::error!("session error: {:?}", err);
            }
        });

        Self { tx, _task: task }
    }
}
