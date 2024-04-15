pub mod router;

use std::{net::IpAddr, sync::Arc};

use anyhow::Context;
use futures::{Future, SinkExt, Stream, StreamExt};
use shroom_net::{
    codec::{ShroomCodec, ShroomTransport},
    ShroomStream,
};
use shroom_pkt::{pkt::{EncodeMessage, Message}, util::encode_buf::EncodeBuf};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_stream::wrappers::TcpListenerStream;



pub enum RpcResponse {
    Ok,
    Pong,
    Migrate,
}

pub struct RpcCtx<C: ShroomCodec> {
    conn: ShroomStream<C>,
    buf: EncodeBuf,
    peer_addr: IpAddr,
}

impl<C: ShroomCodec> RpcCtx<C> {
    pub fn new(conn: ShroomStream<C>, peer_addr: IpAddr) -> Self {
        Self {
            conn,
            buf: EncodeBuf::new(),
            peer_addr,
        }
    }

    pub async fn send(&mut self, pkt: impl EncodeMessage) -> anyhow::Result<()> {
        let pkt = self.buf.encode_onto(pkt)?;
        self.conn.send(pkt.as_ref()).await?;
        Ok(())
    }

    pub async fn send_all<T: EncodeMessage>(
        &mut self,
        pkts: impl Iterator<Item = T>,
    ) -> anyhow::Result<()> {
        for pkt in pkts {
            self.send(pkt).await?;
        }
        Ok(())
    }

    pub fn peer_addr(&self) -> IpAddr {
        self.peer_addr
    }
}

pub trait RpcService: Sized {
    type Ctx;
    type Codec: ShroomCodec;
    type PingPacket: EncodeMessage + Send + 'static;

    fn create(ctx: &Self::Ctx) -> anyhow::Result<Self>;
    fn ping_packet(&self) -> Self::PingPacket;
    fn on_packet(
        &mut self,
        pkt: Message,
        ctx: &mut RpcCtx<Self::Codec>,
    ) -> impl Future<Output = anyhow::Result<RpcResponse>> + Send;

    fn finish(self) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub struct RpcListener<S: RpcService> {
    ctx: Arc<S::Ctx>,
    codec: Arc<S::Codec>,
}

impl<S> RpcListener<S>
where
    S: RpcService + Send,
{
    pub fn new(codec: S::Codec, ctx: S::Ctx) -> Self {
        Self {
            ctx: Arc::new(ctx),
            codec: Arc::new(codec),
        }
    }

    async fn run_rpc(service: &mut S, ctx: &mut RpcCtx<S::Codec>) -> anyhow::Result<()> {
        let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(30));
        let mut ping_pending = false;

        ping_interval.tick().await; // Skip first

        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    if ping_pending {
                        anyhow::bail!("Ping timeout");
                    }
                    ctx.send(service.ping_packet()).await?;
                    ping_pending = true;
                }
                pkt = ctx.conn.next() => {
                    let pkt = pkt.context("eof")??;
                    let msg: Message = pkt.try_into()?;
                    let resp = service.on_packet(msg, ctx).await?;
                    match resp {
                        RpcResponse::Ok => {}
                        RpcResponse::Pong => {
                            ping_pending = false;
                        }
                        RpcResponse::Migrate => {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn exec(
        ctx: Arc<S::Ctx>,
        codec: Arc<S::Codec>,
        io: <S::Codec as ShroomCodec>::Transport,
    ) -> anyhow::Result<()> {
        let peer_addr = io.peer_addr()?.ip();
        let sess = codec.create_server(io).await?;
        let mut service = S::create(&ctx)?;
        let mut ctx = RpcCtx::new(sess, peer_addr);
        let resp = Self::run_rpc(&mut service, &mut ctx).await;
        service.finish().await?;
        resp?;

        Ok(())
    }

    pub async fn run<T>(&mut self, mut io_stream: T) -> anyhow::Result<()>
    where
        T: Stream<Item = std::io::Result<<S::Codec as ShroomCodec>::Transport>>
            + Unpin
            + Send
            + 'static,
        S::Codec: Sync + Send + 'static,
        S::Ctx: Sync + Send + 'static,
    {
        loop {
            let io = io_stream
                .next()
                .await
                .ok_or_else(|| anyhow::anyhow!("io_stream closed"))??;

            let codec = self.codec.clone();
            let ctx = self.ctx.clone();
            tokio::spawn(async move {
                if let Err(err) = Self::exec(ctx, codec, io).await {
                    log::error!("Error: {:?}", err);
                }
            });
        }
    }

    pub async fn run_tcp(&mut self, addr: impl ToSocketAddrs) -> anyhow::Result<()>
    where
        S::Codec: ShroomCodec<Transport = tokio::net::TcpStream> + Sync + Send + 'static,
        S::Ctx: Sync + Send + 'static,
    {
        let listener = TcpListener::bind(addr).await?;
        let stream = TcpListenerStream::new(listener);
        self.run(stream).await
    }
}
