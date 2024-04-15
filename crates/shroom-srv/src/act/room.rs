use std::{
    collections::HashMap,
    panic::AssertUnwindSafe,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
};

use crate::{
    act::{
        session::{SessionActor, SessionCell},
        Context, Instant, Interval, Receiver, Sender, MESSAGES_PER_TICK,
    }, time::clock::Ticks, ClockHandle, GameTime
};

use super::{RoomSessionContext, SessionSet, State};

#[derive(Debug, Clone)]
pub struct RoomConfig {
    pub room_channel_cap: usize,
    pub shutdown_after_ticks: usize,
}

impl Default for RoomConfig {
    fn default() -> Self {
        Self {
            room_channel_cap: 256,
            shutdown_after_ticks: 100,
        }
    }
}

pub type RoomId<R> = <<R as RoomActor>::Session as SessionActor<R>>::RoomId;
pub type RoomSessionId<R> = <<R as RoomActor>::Session as SessionActor<R>>::Id;

pub enum ControlMessage<R: RoomActor> {
    AddSession(SessionCell<R, R::Session>),
    RemoveSession(RoomSessionId<R>),
}

impl<R: RoomActor> ControlMessage<R> {
    pub fn into_session(self) -> Option<SessionCell<R, R::Session>> {
        match self {
            Self::AddSession(sess) => Some(sess),
            Self::RemoveSession(_) => None,
        }
    }
}

pub trait RoomController<R: RoomActor> {
    fn remove_room(&mut self);
    fn change_session_to_room(&mut self, session: SessionCell<R, R::Session>, room_id: RoomId<R>);
}

pub struct NoopRoomController;
impl<R: RoomActor> RoomController<R> for NoopRoomController {
    fn remove_room(&mut self) {}
    fn change_session_to_room(
        &mut self,
        _session: SessionCell<R, R::Session>,
        _room_id: RoomId<R>,
    ) {
    }
}

pub trait RoomActor: Sized + Send + 'static {
    type Error: std::fmt::Debug + Sync + State;
    type Session: SessionActor<Self, Error = Self::Error>;
    type Controller: RoomController<Self> + Send + 'static;

    fn id(&self) -> RoomId<Self>;

    fn on_tick(ctx: &mut SessionSet<Self>) -> Result<(), Self::Error>;
    fn on_msg(ctx: &mut SessionSet<Self>, msg: ControlMessage<Self>) -> Result<(), Self::Error>;

    fn on_enter_session(
        _ctx: &mut SessionSet<Self>,
        _session: &mut Self::Session,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_leave_session(
        _ctx: &mut SessionSet<Self>,
        _session: &mut Self::Session,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Shared {
    sessions: AtomicUsize,
    shutdown: AtomicBool,
}

pub struct RoomHandle<R: RoomActor> {
    tx: Sender<ControlMessage<R>>,
    shared: Arc<Shared>,
    _handle: tokio::task::JoinHandle<()>,
}


pub enum AddSessionError<R: RoomActor> {
    RoomShutdown(SessionCell<R, R::Session>),
    Closed,
}

impl<R: RoomActor> std::fmt::Debug for AddSessionError<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoomShutdown(_) => write!(f, "Room shutdown"),
            Self::Closed => write!(f, "Room closed"),
        }
    }
}

impl<R: RoomActor> RoomHandle<R> {
    pub async fn add_session(
        &self,
        actor: SessionCell<R, R::Session>,
    ) -> Result<(), AddSessionError<R>> {
        self.tx
            .send(ControlMessage::AddSession(actor))
            .await
            .map_err(|err| {
                if self
                    .shared
                    .shutdown
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    AddSessionError::RoomShutdown(err.0.into_session().unwrap())
                } else {
                    AddSessionError::Closed
                }
            })?;

        self.shared
            .sessions
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Ok(())
    }

    pub async fn remove_session(&self, id: RoomSessionId<R>) {
        self.tx
            .send(ControlMessage::RemoveSession(id))
            .await
            .expect("Room remove session");
    }
}

pub struct RoomActorRunner<R: RoomActor> {
    sessions: SessionSet<R>,
    ctrl: R::Controller,
    idle_ticks: usize,
    update_interval: Interval,
    rx: Receiver<ControlMessage<R>>,
    cfg: RoomConfig,
    change_room: HashMap<RoomSessionId<R>, RoomId<R>>,
    shared: Arc<Shared>,
}

impl<R: RoomActor> RoomActorRunner<R> {
    pub fn spawn(
        room: R,
        ctrl: R::Controller,
        cfg: RoomConfig,
        clock_handle: ClockHandle,
    ) -> RoomHandle<R> {
        let shared = Arc::new(Shared::default());
        let (runner, tx) = Self::new(room, ctrl, shared.clone(), cfg);
        let handle = tokio::spawn(runner.run(clock_handle));

        RoomHandle {
            tx,
            shared,
            _handle: handle,
        }
    }

    pub fn new(
        room: R,
        ctrl: R::Controller,
        shared: Arc<Shared>,
        cfg: RoomConfig,
    ) -> (Self, Sender<ControlMessage<R>>) {
        let (tx, rx) = crate::act::channel(cfg.room_channel_cap);
        (
            Self {
                rx,
                update_interval: Interval::new_next(Ticks(1)),
                sessions: SessionSet::new(RoomSessionContext::new(room, GameTime::default())),
                change_room: HashMap::new(),
                ctrl,
                cfg,
                idle_ticks: 0,
                shared,
            },
            tx,
        )
    }

    fn remove_session(
        id: RoomSessionId<R>,
        shared: &Shared,
        sessions: &mut SessionSet<R>,
    ) -> Result<Option<SessionCell<R, R::Session>>, R::Error> {
        if let Some(mut sess) = sessions.remove_session(id) {
            shared
                .sessions
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            R::on_leave_session(sessions, &mut sess.session)?;
            // Process pending messages
            // TODO ensure all messages were processed and handle error/panic
            sess.run_once(&mut sessions.ctx)?;


            sess.session.on_leave_room(&mut sessions.ctx)?;


            return Ok(Some(sess));
        }

        Ok(None)
    }

    fn handle_control_msg(&mut self, msg: ControlMessage<R>) -> Result<(), R::Error> {
        match msg {
            ControlMessage::AddSession(actor) => {
                self.add_session(actor)?;
            }
            ControlMessage::RemoveSession(id) => {
                Self::remove_session(id, &self.shared, &mut self.sessions)?;
            }
        }
        Ok(())
    }

    fn add_session(&mut self, mut sess: SessionCell<R, R::Session>) -> Result<(), R::Error> {
        self.sessions.ctx.tx.add(sess.id(), sess.tx());
        R::on_enter_session(&mut self.sessions, &mut sess.session)?;
        sess.session.on_enter_room(&mut self.sessions.ctx)?;
        self.sessions.actors.push(sess);

        Ok(())
    }

    fn handle_errors(&mut self) {
        while let Some(err_id) = self.sessions.ctx.tx.drain_error() {
            if let Err(err) = Self::remove_session(err_id, &self.shared, &mut self.sessions) {
                log::error!("Error closing session: {:?}", err);
            }
        }
    }

    fn handle_transfers(&mut self) -> Result<(), R::Error> {
        for (id, room_id) in self.change_room.drain() {
            if let Some(session) = Self::remove_session(id, &self.shared, &mut self.sessions)? {
                self.ctrl.change_session_to_room(session, room_id);
            }
        }

        Ok(())
    }

    fn check_shutdown(&mut self) -> bool {
        if !self.sessions.actors.is_empty() {
            self.idle_ticks = 0;
        }

        self.idle_ticks += 1;
        self.idle_ticks >= self.cfg.shutdown_after_ticks
    }

    async fn run_inner(&mut self, clock_handle: &mut ClockHandle) -> Result<(), R::Error> {
        loop {
            tokio::select! {
                    () = clock_handle.tick() => {
                        // TODO handle error
                        self.run_once(clock_handle.time())?;

                        if self.check_shutdown() {
                            return Ok(());
                        }

                    },
                    msg = self.rx.recv() => {
                        if let Some(msg) = msg {
                            self.handle_control_msg(msg)?;
                        }
                    }
            }
        }
    }

    async fn run(mut self, mut clock_handle: ClockHandle) {
        while let Err(err) = self.run_inner(&mut clock_handle).await {
            log::error!("Room error: {:?}", err);
        }

        log::info!("Shutting down room {:?}", self.sessions.ctx.room.id());

        // Shutdown
        self.ctrl.remove_room();
        self.shared
            .shutdown
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.rx.close();

        // Forward all pending sessions back to the controller
        while let Ok(msg) = self.rx.try_recv() {
            // Only handle add messages
            if let ControlMessage::AddSession(sess) = msg {
                self.ctrl
                    .change_session_to_room(sess, self.sessions.ctx.room.id());
            }
        }
    }

    pub fn run_once(&mut self, t: Instant) -> Result<(), R::Error> {
        self.sessions.ctx.t = t;
        if self.update_interval.try_tick(self.sessions.time()) {
            for msg in self.rx.try_recv_many(MESSAGES_PER_TICK) {
                R::on_msg(&mut self.sessions, msg.expect("room rx"))?;
            }

            R::on_tick(&mut self.sessions)?;
        }

        for actor in &mut self.sessions.actors {
            match std::panic::catch_unwind(AssertUnwindSafe(|| actor.run_once(&mut self.sessions.ctx))) {
                Ok(Err(err)) => {
                    log::error!("Handler err: {:?}", err);
                    self.sessions.ctx.tx.add_error(actor.id());
                    continue;
                }
                Err(err) => {
                    log::error!("Handler panic: {:?}", err);
                    self.sessions.ctx.tx.add_error(actor.id());
                    continue;
                }
                Ok(Ok(())) => {
                    if let Some(room_id) = self.sessions.ctx.change_to.take() {
                        self.change_room.insert(actor.id(), room_id);
                    }
                }
            }
        }

        self.handle_errors();
        self.handle_transfers()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use crate::{act::TickActor, Clock};

    use super::*;

    pub struct Room(u32);

    impl RoomActor for Room {
        type Session = OpActor;
        type Controller = NoopRoomController;
        type Error = anyhow::Error;

        fn on_tick(_ctx: &mut SessionSet<Self>) -> Result<(), Self::Error> {
            Ok(())
        }

        fn on_msg(_ctx: &mut SessionSet<Self>, _msg: ControlMessage<Self>) -> Result<(), Self::Error> {
            unreachable!("RoomActor should not receive messages")
        }

        fn id(&self) -> RoomId<Self> {
            1
        }
    }

    #[derive(Debug)]
    pub enum Op {
        Add,
        Sub,
    }

    pub struct OpActor(Arc<AtomicUsize>);

    impl TickActor for OpActor {
        type Msg = Op;
        type Context = RoomSessionContext<Room, Self>;
        type Error = anyhow::Error;

        fn on_tick(&mut self, ctx: &mut Self::Context) -> Result<(), Self::Error> {
            f(ctx);
            ctx.set_ten();
            Ok(())
        }

        fn on_msg(&mut self, _ctx: &mut Self::Context, msg: Self::Msg) -> Result<(), Self::Error> {
            match msg {
                Op::Add => {
                    self.0.fetch_add(1, Ordering::SeqCst);
                }
                Op::Sub => {
                    self.0.fetch_sub(1, Ordering::SeqCst);
                }
            }

            Ok(())
        }
    }

    impl SessionActor<Room> for OpActor {
        type Id = usize;
        type RoomId = usize;

        fn id(&self) -> usize {
            1
        }

        fn room_id(&self) -> usize {
            1
        }
    }

    #[tokio::test]
    async fn room() {
        let mut clock = Clock::default();
        let shutdown = Arc::new(Shared::default());
        let (mut runner, _tx) =
            RoomActorRunner::new(Room(0), NoopRoomController, shutdown, RoomConfig::default());

        // Add a session
        let shared = Arc::new(AtomicUsize::new(0));
        let session = SessionCell::new(OpActor(shared.clone()), 10);
        let session_handle = session.handle();
        runner.add_session(session).unwrap();

        loop {
            clock.tick().await;
            runner.run_once(clock.time()).unwrap();

            let v = shared.load(Ordering::SeqCst);
            if v == 10 {
                break;
            }

            session_handle.send(Op::Add).await.unwrap();
            session_handle.send(Op::Add).await.unwrap();
            session_handle.send(Op::Sub).await.unwrap();
        }
    }

    fn f(c: &mut RoomSessionContext<Room, OpActor>) {
        c.set_ten();
    }

    trait RoomSessionExt<'ctx> {
        fn ctx(&self) -> &RoomSessionContext<Room, OpActor>;
        fn ctx_mut(&mut self) -> &mut RoomSessionContext<Room, OpActor>;

        fn get_half(&self) -> u32 {
            let ctx = self.ctx();
            ctx.room.0 / 2
        }

        fn set_ten(&mut self) {
            let ctx = self.ctx_mut();
            ctx.room.0 = 10;
        }
    }

    impl<'ctx> RoomSessionExt<'ctx> for RoomSessionContext<Room, OpActor> {
        fn ctx(&self) -> &RoomSessionContext<Room, OpActor> {
            self
        }

        fn ctx_mut(&mut self) -> &mut RoomSessionContext<Room, OpActor> {
            self
        }
    }
}
