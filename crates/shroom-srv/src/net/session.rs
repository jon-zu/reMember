use std::ops::{Deref, DerefMut};

use shroom_pkt::{pkt::{EncodeMessage, Message}, util::encode_buf::EncodeBuf};
use tokio::sync::mpsc;

use crate::{act::{
    room::RoomActor, session::SessionActor, Context, RoomSessionContext, TickActor, MESSAGES_PER_TICK
}, Id, Instant};

use super::socket::{PktMsg, ServerSocketHandle};

pub trait NetMsg: From<PktMsg> + TryInto<PktMsg> {
    fn into_pkg_msg(self) -> Result<PktMsg, Self>;
}
impl NetMsg for PktMsg {
    fn into_pkg_msg(self) -> Result<PktMsg, Self> {
        Ok(self)
    }
}

pub struct NetSocket {
    socket: ServerSocketHandle,
    encode_buffer: EncodeBuf
}

impl Deref for NetSocket {
    type Target = ServerSocketHandle;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl DerefMut for NetSocket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.socket
    }
}

impl NetSocket {
    #[must_use]
    pub fn new(socket: ServerSocketHandle) -> Self {
        Self {
            socket,
            encode_buffer: EncodeBuf::new()
        }
    }

    pub fn send_pkt(&mut self, msg: Message) {
        self.socket.send(msg).expect("tx full");
    }

    pub fn reply<M: EncodeMessage>(&mut self, m: M) -> Result<(), shroom_pkt::Error> {
        let msg = self.encode_buffer.encode_onto(m)?;
        self.send_pkt(msg);
        Ok(())
    }

}

pub struct NetSession<H: Handler> {
    pub handler: H,
    pub socket: NetSocket
}

impl<H: Handler> NetSession<H> {
    pub fn new(handler: H, socket: ServerSocketHandle) -> Self {
        Self {
            handler,
            socket: NetSocket::new(socket)
        }
    }

    pub fn handler(&self) -> &H {
        &self.handler
    }

    pub fn handler_mut(&mut self) -> &mut H {
        &mut self.handler
    }
}

pub struct NetSessionContext<'ctx, H: Handler> {
    pub room: &'ctx mut RoomSessionContext<H::Room, NetSession<H>>,
    pub socket: &'ctx mut NetSocket,
}

impl<'ctx, H: Handler> Context for NetSessionContext<'ctx, H> {
    fn time(&self) -> Instant {
        self.room.time()
    }
}

pub trait Handler: Sized + Send + 'static {
    type Id: Id;
    type RoomId: Id;

    type Error: std::fmt::Debug + Send + Sync + From<shroom_pkt::Error> + From<std::io::Error>;
    type Msg: NetMsg + Send + 'static;
    type Room: RoomActor<Session = NetSession<Self>, Error = Self::Error>;

    fn id(&self) -> Self::Id;
    fn room_id(&self) -> Self::RoomId;

    fn on_net_msg(
        &mut self,
        ctx: &mut NetSessionContext<Self>,
        msg: Message,
    ) -> Result<(), Self::Error>;
    fn on_msg(
        &mut self,
        ctx: &mut NetSessionContext<Self>,
        msg: Self::Msg,
    ) -> Result<(), Self::Error>;
    fn on_tick(&mut self, ctx: &mut NetSessionContext<Self>) -> Result<(), Self::Error>;

    fn on_enter_room(&mut self, ctx: &mut NetSessionContext<Self>) -> Result<(), Self::Error>;
    fn on_leave_room(&mut self, ctx: &mut NetSessionContext<Self>) -> Result<(), Self::Error>;
}

impl<H: Handler + Send + 'static> TickActor for NetSession<H> {
    type Msg = H::Msg;
    type Error = H::Error;
    type Context = RoomSessionContext<H::Room, Self>;

    fn on_tick(&mut self, ctx: &mut Self::Context) -> Result<(), Self::Error> {
        let mut ctx = NetSessionContext {
            room: ctx,
            socket: &mut self.socket,
        };


        for _ in 0..MESSAGES_PER_TICK {
            match ctx.socket.socket.try_recv() {
                Ok(msg) => self.handler.on_net_msg(
                    &mut ctx,
                    msg,
                )?,
                Err(mpsc::error::TryRecvError::Empty) => break,
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    return Err(Self::Error::from(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "rx disconnected",
                    )))
                }
            }
        }

        self.handler.on_tick(&mut ctx)
    }

    fn on_msg(&mut self, ctx: &mut Self::Context, msg: H::Msg) -> Result<(), Self::Error> {
        match msg.into_pkg_msg() {
            Ok(pkg_msg) => {
                self.socket.send_pkt_msg(pkg_msg).expect("msg tx full");
                Ok(())
            }
            Err(msg) => self.handler.on_msg(
                &mut NetSessionContext {
                    room: ctx,
                    socket: &mut self.socket,
                },
                msg,
            ),
        }
    }
}

impl<H: Handler> SessionActor<H::Room> for NetSession<H> {
    type Id = H::Id;
    type RoomId = H::RoomId;

    fn id(&self) -> Self::Id {
        self.handler.id()
    }

    fn room_id(&self) -> Self::RoomId {
        self.handler.room_id()
    }

    fn on_enter_room(&mut self, ctx: &mut Self::Context) -> Result<(), Self::Error> {
        self.handler.on_enter_room(&mut NetSessionContext {
            room: ctx,
            socket: &mut self.socket,
        })
    }

    fn on_leave_room(&mut self, ctx: &mut Self::Context) -> Result<(), Self::Error> {
        self.handler.on_leave_room(&mut NetSessionContext {
            room: ctx,
            socket: &mut self.socket,
        })
    }
}
