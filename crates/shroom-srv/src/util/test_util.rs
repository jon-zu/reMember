use std::{
    future,
    net::{IpAddr, Ipv4Addr},
    sync::atomic::AtomicU32,
    time::Duration,
};

use bytes::BufMut;
use futures::Future;
use shroom_net::codec::{legacy::LegacyCodec, LocalShroomTransport};
use shroom_pkt::{
    pkt::{EncodeMessage, Message},
    DecodePacket, EncodePacket, HasOpCode, PacketReader, PacketResult, PacketWriter,
};

use turmoil::net::{TcpListener, TcpStream};

use crate::{
    srv::{
        room_set::ServerSessionData,
        server_room::{RoomContext, RoomCtx, RoomHandler, RoomSessionHandler},
        server_session::{ServerSession, SessionHandler},
        server_socket::ServerSocketHandle,
        server_system::{ServerSystemTx, SystemHandler},
    },
    Context, MS_PER_TICK,
};

use super::clock::{Clock, ClockHandle};
const PORT: u16 = 12345;
pub async fn bind_to_v4(port: u16) -> std::result::Result<TcpListener, std::io::Error> {
    TcpListener::bind((IpAddr::from(Ipv4Addr::UNSPECIFIED), port)).await
}
pub async fn bind() -> std::result::Result<TcpListener, std::io::Error> {
    bind_to_v4(PORT).await
}

pub async fn connect() -> std::result::Result<TcpStream, std::io::Error> {
    TcpStream::connect(("server", PORT)).await
}

#[derive(Debug)]
pub struct MockMsgReq(pub u32);

impl HasOpCode for MockMsgReq {
    /// OpCode type
    type OpCode = u16;

    /// OpCode value
    const OPCODE: u16 = 2;
}

impl EncodePacket for MockMsgReq {
    fn encode<B: BufMut>(&self, pw: &mut PacketWriter<B>) -> PacketResult<()> {
        pw.write_u32(self.0)?;
        Ok(())
    }

    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::new(4);

    fn encode_len(&self) -> usize {
        4
    }
}

#[derive(Debug, Clone)]
pub enum MockMsg {
    Add(u32),
    Sub(u32),
    Set(u32),
}

impl EncodePacket for MockMsg {
    fn encode<B: BufMut>(&self, pw: &mut PacketWriter<B>) -> PacketResult<()> {
        match self {
            Self::Add(v) => {
                pw.write_u16(0)?;
                pw.write_u32(*v)?;
            }
            Self::Sub(v) => {
                pw.write_u16(1)?;
                pw.write_u32(*v)?;
            }
            Self::Set(v) => {
                pw.write_u16(2)?;
                pw.write_u32(*v)?;
            }
        }

        Ok(())
    }

    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::new(6);

    fn encode_len(&self) -> usize {
        6
    }
}

impl<'de> DecodePacket<'de> for MockMsg {
    fn decode(pr: &mut PacketReader<'de>) -> PacketResult<Self> {
        match pr.read_u16()? {
            0 => Ok(Self::Add(pr.read_u32()?)),
            1 => Ok(Self::Sub(pr.read_u32()?)),
            2 => Ok(Self::Set(pr.read_u32()?)),
            op => Err(shroom_pkt::Error::InvalidEnumDiscriminant(op as usize)),
        }
    }
}

impl HasOpCode for MockMsg {
    /// OpCode type
    type OpCode = u16;

    /// OpCode value
    const OPCODE: u16 = 1;
}

impl MockMsg {
    pub fn apply(&self, acc: &mut u32) {
        match self {
            Self::Add(v) => *acc += v,
            Self::Sub(v) => *acc -= v,
            Self::Set(v) => *acc = *v,
        }
    }

    pub fn apply_all<'a>(acc: u32, msgs: impl Iterator<Item = &'a Self>) -> u32 {
        msgs.fold(acc, |mut acc, msg| {
            msg.apply(&mut acc);
            acc
        })
    }
}

pub struct MockCtx {
    pub clock: ClockHandle,
    last_id: AtomicU32,
}

impl MockCtx {
    pub fn next_id(&self) -> u32 {
        self.last_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

impl Context for MockCtx {
    fn create(clock_ref: ClockHandle) -> Self {
        MockCtx {
            clock: clock_ref,
            last_id: AtomicU32::default(),
        }
    }

    fn time(&self) -> super::clock::Time {
        self.clock.time()
    }

    fn wait_tick(&mut self) -> impl Future<Output = ()> + Send {
        self.clock.tick()
    }
}

impl RoomContext for MockCtx {
    type RoomId = u32;

    fn send_shutdown_req(&mut self, room_id: Self::RoomId) -> anyhow::Result<()> {
        println!("Shutting down: {}", room_id);
        Ok(())
    }
}

pub struct MockSessionHandler {
    pub acc: u32,
    pub session_id: u32,
}

impl SessionHandler for MockSessionHandler {
    type Ctx<'ctx> = MockCtx;
    type Msg = ();
    type SessionId = u32;

    fn session_id(&self) -> Self::SessionId {
        self.session_id
    }

    fn on_update<'sess>(
        &mut self,
        _sck: &mut ServerSocketHandle,
        _ctx: &mut Self::Ctx<'sess>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_socket_msg(
        &mut self,
        sck: &mut ServerSocketHandle,
        _ctx: &mut Self::Ctx<'_>,
        msg: Message,
    ) -> anyhow::Result<()> {
        let msg: MockMsg = msg.decode()?;
        msg.apply(&mut self.acc);

        sck.send(MockMsgReq(self.acc).to_message()?)
            .map_err(|_| anyhow::format_err!("Unable to send"))?;

        Ok(())
    }

    fn on_msg(
        &mut self,
        _sck: &mut ServerSocketHandle,
        _ctx: &mut Self::Ctx<'_>,
        _msg: Self::Msg,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct MockHandler {
    pub session_id: u32,
    pub acc: u32,
}

impl RoomSessionHandler for MockHandler {
    type RoomHandler = MockRoomHandler;
    type Msg = ();
    type SessionId = u32;
    type RoomId = u32;

    fn session_id(&self) -> Self::SessionId {
        self.session_id
    }

    fn on_update(
        &mut self,
        _sck: &mut ServerSocketHandle,
        _ctx: &mut RoomCtx<'_, Self>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_socket_msg(
        &mut self,
        _sck: &mut ServerSocketHandle,
        _ctx: &mut RoomCtx<'_, Self>,
        msg: Message,
    ) -> anyhow::Result<()> {
        let msg: MockMsg = msg.decode()?;
        msg.apply(&mut self.acc);

        _sck.send(MockMsgReq(self.acc).to_message().unwrap())
            .map_err(|_| anyhow::format_err!("Unable to send"))?;

        //ctx.room_sessions.broadcast_encode(MockMsgReq(self.acc))?;

        Ok(())
    }

    fn on_msg(
        &mut self,
        _sck: &mut ServerSocketHandle,
        _ctx: &mut RoomCtx<'_, Self>,
        _msg: Self::Msg,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn on_enter_room<'sess>(
        &mut self,
        _sck: &mut ServerSocketHandle,
        _ctx: &mut RoomCtx<'sess, Self>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn room_id(&self) -> Self::RoomId {
        self.session_id % 2
    }

    fn on_switch_room(
        _ctx: &mut RoomCtx<'_, Self>,
        _session: ServerSessionData<Self>,
        _new_room: Self::RoomId,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct MockRoomHandler {
    room_acc: u32,
    room_id: u32,
}

impl RoomHandler for MockRoomHandler {
    type Ctx = MockCtx;
    type SessionHandler = MockHandler;
    type RoomId = u32;

    fn room_id(&self) -> Self::RoomId {
        self.room_id
    }

    fn on_update(ctx: &mut RoomCtx<Self::SessionHandler>) -> anyhow::Result<()> {
        ctx.room.room_acc += 1;

        Ok(())
    }

    fn on_enter(
        &mut self,
        _ctx: &mut Self::Ctx,
        _session: &mut ServerSessionData<Self::SessionHandler>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_leave(
        _ctx: &mut RoomCtx<'_, Self::SessionHandler>,
        _id: <Self::SessionHandler as RoomSessionHandler>::SessionId,
    ) -> anyhow::Result<()> {
        log::info!("Leaving");
        Ok(())
    }
}

pub struct MockSystemHandler;

impl SystemHandler for MockSystemHandler {
    type Ctx = MockCtx;
    type Msg = ();
    type SessionHandler = MockHandler;
    type RoomHandler = MockRoomHandler;
    type RoomId = u32;

    fn create_room(&mut self, _room_id: Self::RoomId) -> anyhow::Result<Self::RoomHandler> {
        Ok(MockRoomHandler::default())
    }

    fn create_ctx(
        &mut self,
        clock: ClockHandle,
        _tx: ServerSystemTx<Self>,
    ) -> anyhow::Result<Self::Ctx> {
        Ok(MockCtx::create(clock))
    }
    fn on_update(&mut self, _ctx: &mut Self::Ctx) -> anyhow::Result<()> {
        Ok(())
    }

    fn create_session(
        ctx: &Self::Ctx,
        _sck: &mut ServerSocketHandle,
    ) -> impl Future<Output = anyhow::Result<Self::SessionHandler>> + Send {
        future::ready(Ok(MockHandler {
            acc: 0,
            session_id: ctx.next_id(),
        }))
    }
}

pub fn test_clock() -> ClockHandle {
    let mut clock = Clock::default();
    let clock_ref = clock.handle();

    tokio::spawn(async move {
        loop {
            clock.tick().await;
        }
    });

    clock_ref
}

pub type MockCodec = LegacyCodec<LocalShroomTransport<turmoil::net::TcpStream>>;

pub async fn run_mock_client() -> anyhow::Result<()> {
    let conn = connect().await?;
    let cdc = LegacyCodec::default();

    let mut socket = ServerSocketHandle::new_client(&cdc, LocalShroomTransport(conn)).await?;

    let msgs = [
        MockMsg::Set(100),
        MockMsg::Add(1),
        MockMsg::Add(2),
        MockMsg::Sub(1),
    ];
    let mut acc = 0;
    for msg in msgs.iter() {
        socket
            .send(msg.clone().to_message().unwrap())
            .map_err(|_| anyhow::format_err!("Unable to send"))?;
        let p = socket.recv().await.unwrap();
        let mut pr = p.reader();
        acc = pr.read_u32()?;
        tokio::time::sleep(Duration::from_millis(MS_PER_TICK)).await;
    }
    assert_eq!(acc, MockMsg::apply_all(0, msgs.iter()));
    Ok(())
}

pub async fn accept_mock_session<H: SessionHandler>(
    listener: &TcpListener,
    handler: H,
) -> anyhow::Result<ServerSession<H>> {
    let cdc = MockCodec::default();
    let io = listener.accept().await?;
    let socket = ServerSocketHandle::new_server(&cdc, LocalShroomTransport(io.0)).await?;
    Ok(ServerSession::<H>::new(handler, socket)?)
}
