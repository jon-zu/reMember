use std::{net::IpAddr, ops::RangeInclusive, sync::Arc};

use futures::Future;
use shroom_net::codec::ShroomCodec;
use tokio::net::TcpStream;

use crate::act::system::{System, SystemHandler};

use super::{acceptor::ServerAcceptor, socket::ServerSocketHandle};

pub trait NetSystemHandler: Send + Sync + 'static {
    type Error: std::fmt::Debug + Send + Sync;
    type Codec: ShroomCodec<Transport = TcpStream> + Send + Sync + 'static;
    type System: SystemHandler<Error = Self::Error>;

    fn create_session(
        &self,
        socket: ServerSocketHandle,
    ) -> impl Future<Output = Result<<Self::System as SystemHandler>::Session, Self::Error>> + Send;
}

pub struct NetSystem<H: NetSystemHandler> {
    codec: Arc<H::Codec>,
    sys: System<H::System>,
    handler: Arc<H>,
    acceptor_tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl<H: NetSystemHandler> NetSystem<H> {
    pub fn new(codec: H::Codec, handler: H, sys: System<H::System>) -> Self {
        Self {
            sys,
            handler: Arc::new(handler),
            codec: Arc::new(codec),
            acceptor_tasks: Vec::new(),
        }
    }

    pub fn spawn_acceptors(&mut self, addr: IpAddr, ports: RangeInclusive<u16>) {
        for port in ports {
            let addr = (addr, port);
            let handler = self.handler.clone();
            let cdc = self.codec.clone();
            let sys = self.sys.handle();
            let task = tokio::spawn(async move {
                let mut acceptor = ServerAcceptor::<H>::new(handler, cdc, sys);
                acceptor.run_tcp(addr).await.unwrap();
            });
            self.acceptor_tasks.push(task);
        }
    }

    pub async fn run(self) -> Result<(), H::Error> {
        self.sys.run().await?;
        Ok(())
    }
}