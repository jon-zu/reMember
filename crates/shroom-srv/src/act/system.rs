use std::collections::{hash_map::Entry, HashMap};

use tokio::sync::mpsc;

use crate::{
    act::{
        room::{RoomActor, RoomActorRunner, RoomConfig, RoomController, RoomHandle},
        session::{SessionActor, SessionCell, SessionHandle},
    }, Clock, GameTime, Id
};

use super::room::AddSessionError;

#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub session_message_cap: usize,
    pub room: RoomConfig,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            session_message_cap: 256,
            room: RoomConfig::default(),
        }
    }
}

#[derive(Debug)]
pub enum Message<H: SystemHandler> {
    AddSession(H::Session),
    RemoveSession(H::SessionId),
    ChangeRoom(H::RoomId, SessionCell<H::Room, H::Session>),
    RemoveRoom(H::RoomId, usize),
}

pub trait SystemHandler: Sized + Send + 'static {
    type Error: std::fmt::Debug + Send + 'static;

    type SessionId: Id;
    type RoomId: Id;

    type Session: SessionActor<
        Self::Room,
        Id = Self::SessionId,
        RoomId = Self::RoomId,
        Error = Self::Error,
    >;
    type Room: RoomActor<
        Session = Self::Session,
        Controller = SystemRoomController<Self>,
        Error = Self::Error,
    >;

    fn on_tick(&mut self, t: GameTime) -> Result<(), Self::Error>;
    fn create_room(&mut self, id: Self::RoomId) -> Result<Self::Room, Self::Error>;
}

pub struct SystemHandle<H: SystemHandler> {
    tx: mpsc::UnboundedSender<Message<H>>,
}

impl<H: SystemHandler> Clone for SystemHandle<H> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<H: SystemHandler> SystemHandle<H> {
    pub fn add_session(&self, session: H::Session) -> Result<(), H::Error> {
        self.tx
            .send(Message::AddSession(session))
            .expect("Send add session");
        Ok(())
    }

    pub fn remove_session(&self, session_id: H::SessionId) -> Result<(), H::Error> {
        self.tx
            .send(Message::RemoveSession(session_id))
            .expect("Send remove session");
        Ok(())
    }

    pub fn change_room(
        &self,
        room_id: H::RoomId,
        session: SessionCell<H::Room, H::Session>,
    ) -> Result<(), H::Error> {
        self.tx
            .send(Message::ChangeRoom(room_id, session))
            .expect("Send change room");
        Ok(())
    }

    pub fn remove_room(&self, room_id: H::RoomId, epoch: usize) -> Result<(), H::Error> {
        self.tx
            .send(Message::RemoveRoom(room_id, epoch))
            .expect("Send remove room");
        Ok(())
    }
}

pub struct System<H: SystemHandler> {
    clock: Clock,
    rx: mpsc::UnboundedReceiver<Message<H>>,
    tx: mpsc::UnboundedSender<Message<H>>,
    handler: H,
    rooms: HashMap<H::RoomId, (usize, RoomHandle<H::Room>)>,
    sessions: HashMap<H::SessionId, SessionHandle<H::Room, H::Session>>,
    cfg: SystemConfig,
    epoch: usize,
}

pub struct SystemRoomController<H: SystemHandler> {
    sys: SystemHandle<H>,
    id: H::RoomId,
    epoch: usize,
}

impl<H: SystemHandler> RoomController<H::Room> for SystemRoomController<H> {
    fn remove_room(&mut self) {
        self.sys
            .remove_room(self.id, self.epoch)
            .expect("remove room");
    }

    fn change_session_to_room(
        &mut self,
        session: SessionCell<H::Room, H::Session>,
        room_id: H::RoomId,
    ) {
        self.sys.change_room(room_id, session).expect("Change room");
    }
}

impl<H: SystemHandler> System<H> {
    pub fn new(handler: H, cfg: SystemConfig) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            clock: Clock::default(),
            rx,
            tx,
            handler,
            rooms: HashMap::new(),
            sessions: HashMap::new(),
            cfg,
            epoch: 0,
        }
    }

    pub fn handle(&self) -> SystemHandle<H> {
        SystemHandle {
            tx: self.tx.clone(),
        }
    }

    fn create_room(&mut self, id: H::RoomId) -> Result<&mut RoomHandle<H::Room>, H::Error> {
        let room = self.handler.create_room(id)?;
        let ctrl = SystemRoomController {
            sys: self.handle(),
            id,
            epoch: self.epoch,
        };
        let actor = RoomActorRunner::spawn(room, ctrl, self.cfg.room.clone(), self.clock.handle());
        self.rooms.insert(id, (self.epoch, actor));
        self.epoch += 1;
        Ok(&mut self.rooms.get_mut(&id).expect("new room").1)
    }

    async fn add_session_to_room(
        &mut self,
        room_id: H::RoomId,
        mut session: SessionCell<H::Room, H::Session>,
    ) -> Result<(), H::Error> {
        // Try to get the room
        if let Some((epoch, room)) = self.rooms.get_mut(&room_id) {
            // Attempt to add the session
            match room.add_session(session).await {
                // We managed to add the session
                Ok(()) => {
                    return Ok(());
                }
                // We failed to add the session, this means the room is down
                Err(AddSessionError::RoomShutdown(sess)) => {
                    let epoch = *epoch;
                    // TODO: check if the room crashed or actually shutted down
                    self.remove_room(room_id, epoch)?;
                    session = sess;
                }
                Err(AddSessionError::Closed) => {
                    todo!("Room closed");
                }
            }
        }

        // No room exists no for the id so we create one
        let room = self.create_room(room_id)?;
        room.add_session(session).await.expect("new room tx");
        Ok(())
    }

    async fn add_session(
        &mut self,
        room_id: H::RoomId,
        session: H::Session,
    ) -> Result<(), H::Error> {
        let session = SessionCell::new(session, self.cfg.session_message_cap);
        let handle = session.handle();
        let id = session.id();
        self.add_session_to_room(room_id, session).await?;
        self.sessions.insert(id, handle);
        Ok(())
    }

    fn remove_room(&mut self, room_id: H::RoomId, epoch: usize) -> Result<(), H::Error> {
        match self.rooms.entry(room_id) {
            Entry::Occupied(entry) => {
                if entry.get().0 == epoch {
                    entry.remove();
                }
            }
            Entry::Vacant(_) => {}
        }

        Ok(())
    }

    fn remove_session(&mut self, session_id: H::SessionId) -> Result<(), H::Error> {
        self.sessions.remove(&session_id);
        Ok(())
    }

    pub async fn run(mut self) -> Result<(), H::Error> {
        loop {
            tokio::select! {
                () = self.clock.tick() => {
                    self.handler.on_tick(self.clock.time())?;
                }
                msg = self.rx.recv() => {
                    match msg.expect("Room message rx") {
                        Message::AddSession(session) => {
                            self.add_session(session.room_id(), session).await?;
                        },
                        Message::ChangeRoom(room_id, session) => {
                            self.add_session_to_room(room_id, session).await?;
                        },
                        Message::RemoveSession(session_id) => {
                            self.remove_session(session_id)?;
                        },
                        Message::RemoveRoom(room_id, epoch) => {
                            self.remove_room(room_id, epoch)?;
                        }
                    }
                }
            }
        }
    }
}
