use std::sync::Arc;

use futures::{Stream, StreamExt};
use shroom_net::codec::ShroomCodec;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_stream::wrappers::TcpListenerStream;

use crate::act::system::SystemHandle;

use super::{socket::ServerSocketHandle, system::NetSystemHandler};

pub struct ServerAcceptor<H: NetSystemHandler> {
    handler: Arc<H>,
    codec: Arc<H::Codec>,
    sys: SystemHandle<H::System>,
}

impl<H: NetSystemHandler> ServerAcceptor<H> {
    pub fn new(handler: Arc<H>, codec: Arc<H::Codec>, sys: SystemHandle<H::System>) -> Self {
        Self {
            handler,
            codec,
            sys,
        }
    }

    pub async fn run<S>(&mut self, mut io_stream: S) -> anyhow::Result<()>
    where
        S: Stream<Item = Result<<H::Codec as ShroomCodec>::Transport, std::io::Error>> + Unpin,
    {
        while let Some(io) = io_stream.next().await {
            let io = io?;
            let sys = self.sys.clone();
            let cdc = self.codec.clone();
            let ctx = self.handler.clone();

            tokio::spawn(async move {
                let socket_handle = ServerSocketHandle::new_server(&*cdc, io).await.unwrap();
                let session = H::create_session(&ctx, socket_handle).await.unwrap();
                sys.add_session(session).unwrap();
            });
        }

        Ok(())
    }

    pub async fn run_tcp(&mut self, addr: impl ToSocketAddrs) -> anyhow::Result<()>
    where
        H::Codec: ShroomCodec<Transport = TcpStream>,
    {
        let listener = TcpListener::bind(addr).await?;
        let listener_stream = TcpListenerStream::new(listener);
        self.run(listener_stream).await
    }

    /*
    #[cfg(test)]
    pub async fn run_turmoil_tcp(
        &mut self,
        listener: turmoil::net::TcpListener,
    ) -> anyhow::Result<()>
    where
        C: ShroomCodec<
            Transport = shroom_net::codec::LocalShroomTransport<turmoil::net::TcpStream>,
        >,
        H::Ctx: Sync,
    {
        use futures::stream;
        use shroom_net::codec::LocalShroomTransport;

        let listener_stream = stream::unfold(listener, |listener| async move {
            let res = listener
                .accept()
                .await
                .map(|(s, _)| LocalShroomTransport(s));
            Some((res, listener))
        });
        let listener_stream = std::pin::pin!(listener_stream);
        self.run(listener_stream).await
    }*/
}
