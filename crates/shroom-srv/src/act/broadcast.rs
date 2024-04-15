use super::{
    room::{RoomActor, RoomId, RoomSessionId},
    session::{SessionActor, SessionCell},
    Sender, TickActor,
};
use crate::{
    net::{session::NetMsg, socket::PktMsg},
    Id, Instant,
};
use shroom_pkt::{
    pkt::EncodeMessage,
    util::{encode_buf::EncodeBuf, packet_buf::PacketBuf},
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

pub struct BroadcastSet<I: Id, M> {
    tx: HashMap<I, Sender<M>>,
    encode_buf: EncodeBuf,
    err_ids: HashSet<I>,
}

pub type SessionBroadcastSet<R, S> =
    BroadcastSet<<S as SessionActor<R>>::Id, <S as TickActor>::Msg>;

impl<I: Id, M> Default for BroadcastSet<I, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Id, M> BroadcastSet<I, M> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tx: HashMap::new(),
            encode_buf: EncodeBuf::new(),
            err_ids: HashSet::new(),
        }
    }

    pub(crate) fn add(&mut self, id: I, tx: Sender<M>) {
        self.tx.insert(id, tx);
    }

    pub(crate) fn remove(&mut self, id: I) {
        self.tx.remove(&id);
    }

    pub(crate) fn add_error(&mut self, id: I) {
        self.err_ids.insert(id);
    }

    pub(crate) fn drain_error(&mut self) -> Option<I> {
        self.err_ids.drain().next()
    }

    pub fn broadcast(&mut self, msg: M)
    where
        M: Clone,
    {
        for (id, tx) in &self.tx {
            if tx.try_send(msg.clone()).is_err() {
                self.err_ids.insert(*id);
            }
        }
    }

    pub fn broadcast_filter(&mut self, msg: M, filter: impl Fn(&I) -> bool)
    where
        M: Clone,
    {
        for (id, tx) in &self.tx {
            if filter(id) && tx.try_send(msg.clone()).is_err() {
                self.err_ids.insert(*id);
            }
        }
    }

    pub fn broadcast_filter_id(&mut self, msg: M, filter_id: I)
    where
        M: Clone,
    {
        self.broadcast_filter(msg, |id| id != &filter_id);
    }

    pub fn send_to(&mut self, id: I, msg: M) {
        //TODO handle unregistered id
        if let Some(tx) = self.tx.get(&id) {
            if tx.try_send(msg).is_err() {
                self.err_ids.insert(id);
            }
        }
    }
}

impl<I: Id, M> BroadcastSet<I, M>
where
    M: NetMsg + Clone,
{
    pub fn broadcast_encode(&mut self, msg: impl EncodeMessage) -> Result<(), shroom_pkt::Error> {
        let msg = PktMsg::Packet(self.encode_buf.encode_onto(msg)?);
        self.broadcast(msg.into());
        Ok(())
    }

    pub fn broadcast_filter_encode(
        &mut self,
        msg: impl EncodeMessage,
        filter_id: I,
    ) -> Result<(), shroom_pkt::Error> {
        let msg = PktMsg::Packet(self.encode_buf.encode_onto(msg)?);
        self.broadcast_filter_id(msg.into(), filter_id);
        Ok(())
    }

    pub fn send_to_encode(
        &mut self,
        id: I,
        msg: impl EncodeMessage,
    ) -> Result<(), shroom_pkt::Error> {
        let msg = PktMsg::Packet(self.encode_buf.encode_onto(msg)?);
        self.send_to(id, msg.into());
        Ok(())
    }

    pub fn send_all_to_encode<P: EncodeMessage, IM: Iterator<Item = P>>(
        &mut self,
        id: I,
        msg: IM,
    ) -> Result<(), shroom_pkt::Error> {
        let mut buf = PacketBuf::default();
        for msg in msg {
            buf.encode(msg)?;
        }

        self.send_to(id, PktMsg::PacketBuf(Arc::new(buf)).into());
        Ok(())
    }
}

pub struct SessionSet<R: RoomActor> {
    pub ctx: RoomSessionContext<R, R::Session>,
    pub actors: Vec<SessionCell<R, R::Session>>,
}

impl<R: RoomActor> SessionSet<R> {
    pub fn new(ctx: RoomSessionContext<R, R::Session>) -> Self {
        Self {
            ctx,
            actors: Vec::new(),
        }
    }

    pub fn session_slice_mut(&mut self) -> &mut [SessionCell<R, R::Session>] {
        &mut self.actors
    }

    pub fn sessions(&self) -> impl Iterator<Item = &SessionCell<R, R::Session>> {
        self.actors.iter()
    }

    pub fn sessions_mut(&mut self) -> impl Iterator<Item = &mut SessionCell<R, R::Session>> {
        self.actors.iter_mut()
    }

    pub fn remove_session(&mut self, id: RoomSessionId<R>) -> Option<SessionCell<R, R::Session>> {
        let actor_ix = self.actors.iter().position(|sess| sess.id() == id)?;
        self.ctx.tx.remove(id);
        Some(self.actors.remove(actor_ix))
    }

    pub fn add_session(&mut self, session: SessionCell<R, R::Session>) {
        self.ctx.tx.add(session.id(), session.tx());
        self.actors.push(session);
    }
}

impl<R: RoomActor> crate::act::Context for SessionSet<R> {
    fn time(&self) -> Instant {
        self.ctx.t
    }
}

pub struct RoomSessionContext<R, S: SessionActor<R>> {
    pub room: R,
    pub tx: BroadcastSet<S::Id, S::Msg>,
    pub(crate) t: Instant,
    pub(crate) change_to: Option<S::RoomId>,
}

impl<R, S: SessionActor<R>> RoomSessionContext<R, S> {
    pub fn new(room: R, t: Instant) -> Self {
        Self {
            room,
            tx: BroadcastSet::new(),
            t,
            change_to: None,
        }
    }
}

impl<R: RoomActor> RoomSessionContext<R, R::Session> {
    pub fn change_room(&mut self, room_id: RoomId<R>) -> Result<(), R::Error> {
        self.change_to = Some(room_id);
        Ok(())
    }
}

unsafe impl<R: Send, S: SessionActor<R> + Send> Send for RoomSessionContext<R, S> {}

impl<R, S: SessionActor<R>> crate::act::Context for RoomSessionContext<R, S> {
    fn time(&self) -> Instant {
        self.t
    }
}

impl<R, S: SessionActor<R>> RoomSessionContext<R, S> {
    pub fn tx(&mut self) -> &mut BroadcastSet<S::Id, S::Msg> {
        &mut self.tx
    }
}
