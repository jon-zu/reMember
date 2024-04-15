use std::{collections::HashMap, sync::Arc};

use futures::Future;
use tokio::sync::oneshot;

use crate::{
    act::{Receiver, Sender},
    net::session::NetSocket,
    ClockHandle,
};

use super::{
    room::{NetRoomHandle, NetRoomRunner, RoomHandler},
    session::{NetSessionRunner, SessionHandler},
};

pub trait SystemHandler {
    type Error: std::fmt::Debug
        + From<std::io::Error>
        + From<shroom_pkt::Error>
        + From<super::Error>
        + Send
        + Sync
        + 'static;

    type Context: Send + 'static;
    type Session: SessionHandler<Error = Self::Error, Room = Self::Room, System = Self>;
    type Room: RoomHandler<Error = Self::Error, Session = Self::Session>;

    fn create_session(ctx: &Self::Context, sck: &mut NetSocket) -> impl Future<Output = Result<Self::Session, Self::Error>> + Send;

    fn create_room(
        &mut self,
        room_id: <Self::Room as RoomHandler>::Id,
    ) -> (Self::Room, <Self::Room as RoomHandler>::State);
}

pub struct NetSystemHandle<H: SystemHandler> {
    tx: Sender<SystemControlMessage<H>>,
    ctx: Arc<H::Context>,
}

impl<H: SystemHandler> Clone for NetSystemHandle<H> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            ctx: self.ctx.clone(),
        }
    }

}

impl<H: SystemHandler> NetSystemHandle<H> {
    pub async fn create_session(
        &self,
        sck: &mut NetSocket,
    ) -> Result<H::Session, H::Error> {
        Ok(H::create_session(&self.ctx, sck).await?)
    }


    pub async fn add_session(
        &self,
        session: <H::Room as RoomHandler>::Session,
        sck: NetSocket,
    ) -> Result<(), H::Error> {
        self.tx
            .send(SystemControlMessage::AddSession(session, sck))
            .await
            .map_err(|_| super::Error::SendTx)?;
        Ok(())
    }

    pub async fn remove_session(
        &self,
        id: <H::Session as SessionHandler>::Id,
    ) -> Result<(), H::Error> {
        self.tx
            .send(SystemControlMessage::RemoveSession(id))
            .await
            .map_err(|_| super::Error::SendTx)?;
        Ok(())
    }

    pub async fn get_room(
        &self,
        room_id: <H::Room as RoomHandler>::Id,
    ) -> Result<NetRoomHandle<H::Room>, H::Error> {
        let (rply_tx, rply_rx) = oneshot::channel();
        self.tx
            .send(SystemControlMessage::GetRoom(room_id, rply_tx))
            .await
            .map_err(|_| super::Error::SendTx)?;
        Ok(rply_rx.await.map_err(|_| super::Error::Recv)?)
    }

    pub async fn shutdown_room(
        &self,
        room_id: <H::Room as RoomHandler>::Id,
    ) -> Result<(), H::Error> {
        Ok(self
            .tx
            .send(SystemControlMessage::ShutdownRoom(room_id))
            .await
            .map_err(|_| super::Error::SendTx)?)
    }
}

pub struct SystemConfig {
    pub room_msg_cap: usize,
    pub sys_msg_cap: usize,
    pub session_msg_cap: usize,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            room_msg_cap: 100,
            sys_msg_cap: 100,
            session_msg_cap: 100,
        }
    }
}

pub enum SystemControlMessage<H: SystemHandler> {
    AddSession(<H::Room as RoomHandler>::Session, NetSocket),
    RemoveSession(<H::Session as SessionHandler>::Id),
    GetRoom(
        <H::Room as RoomHandler>::Id,
        oneshot::Sender<NetRoomHandle<H::Room>>,
    ),
    ShutdownRoom(<H::Room as RoomHandler>::Id),
}

pub struct SessionSet<H: RoomHandler>(
    HashMap<<H::Session as SessionHandler>::Id, NetSessionRunner<H::Session>>,
);

pub struct NetSystem<H: SystemHandler> {
    handler: H,
    rooms: HashMap<<H::Room as RoomHandler>::Id, NetRoomRunner<H::Room>>,
    sessions: SessionSet<H::Room>,
    cfg: SystemConfig,
    tx: Sender<SystemControlMessage<H>>,
    rx: Receiver<SystemControlMessage<H>>,
    clock: ClockHandle,
    ctx: Arc<H::Context>
}

impl<H: SystemHandler> NetSystem<H> {
    pub fn new(
        handler: H,
        cfg: SystemConfig,
        clock: ClockHandle,
        ctx: H::Context,
    ) -> Self {
        let (tx, rx) = crate::act::channel(cfg.sys_msg_cap);
        Self {
            handler,
            rooms: HashMap::new(),
            sessions: SessionSet(HashMap::new()),
            cfg,
            tx,
            rx,
            clock,
            ctx: Arc::new(ctx),
        }
    }

    pub fn handle(&self) -> NetSystemHandle<H> {
        NetSystemHandle {
            tx: self.tx.clone(),
            ctx: self.ctx.clone(),
        }
    }

    fn get_room(
        &mut self,
        room_id: <H::Room as RoomHandler>::Id,
    ) -> Result<NetRoomHandle<H::Room>, H::Error> {
        match self.rooms.get(&room_id) {
            Some(runner) => Ok(runner.handle()),
            None => {
                let (room, state) = self.handler.create_room(room_id);
                let runner =
                    NetRoomRunner::spawn(room, state, self.clock.clone(), self.cfg.room_msg_cap);
                let handle = runner.handle();
                self.rooms.insert(room_id, runner);
                Ok(handle)
            }
        }
    }

    pub fn add_session(
        &mut self,
        session: <H::Room as RoomHandler>::Session,
        sck: NetSocket,
    ) -> Result<(), H::Error> {
        let id = session.id();
        let sys = NetSystemHandle {
            tx: self.tx.clone(),
            ctx: self.ctx.clone(),
        };
        let sess = NetSessionRunner::spawn(
            session,
            sck,
            self.clock.clone(),
            sys,
            self.cfg.session_msg_cap,
        );
        self.sessions.0.insert(id, sess);
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), H::Error> {
        loop {
            match self.rx.recv().await.expect("sys message rx") {
                SystemControlMessage::AddSession(session, socket) => {
                    self.add_session(session, socket)?;
                }
                SystemControlMessage::RemoveSession(id) => {
                    self.sessions.0.remove(&id);
                }
                SystemControlMessage::GetRoom(room_id, rply) => {
                    let _ = rply.send(self.get_room(room_id)?);
                }
                SystemControlMessage::ShutdownRoom(room_id) => {
                    if let Some(_) = self.rooms.remove(&room_id) {
                        // TODO
                    }
                }
            }
            //TODO handle room creation
        }
    }
}
