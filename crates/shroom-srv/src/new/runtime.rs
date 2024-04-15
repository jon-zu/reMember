use std::{marker::PhantomData, net::IpAddr, ops::RangeInclusive, sync::Arc, time::Duration};

use shroom_net::codec::ShroomCodec;
use tokio::net::TcpStream;

use crate::{
    rpc::{RpcListener, RpcService},
    util::{SupervisedTask, SupervisedTaskHandle}, Clock,
};

use super::{
    acceptor::ServerAcceptor,
    sys::{NetSystem, SystemHandler},
};

pub struct RuntimeConfig {
    pub bind_addr: IpAddr,
    pub login_port: u16,
    pub game_ports: RangeInclusive<u16>,
}

pub trait RuntimeHandler: Send + 'static {
    type Ctx: Send + Sync + 'static;
    type Codec: ShroomCodec<Transport = TcpStream> + Send + Sync + 'static;
    type System: SystemHandler;
    type LoginService: RpcService<Ctx = Self::Ctx, Codec = Self::Codec> + Send + 'static;
}

pub struct LoginTask<H: RuntimeHandler> {
    login: RpcListener<H::LoginService>,
    bind_addr: IpAddr,
    port: u16,
}
impl<H: RuntimeHandler> SupervisedTask for LoginTask<H> {
    type Context = ();

    async fn run(&mut self, _ctx: &mut Self::Context) -> anyhow::Result<()> {
        self.login.run_tcp((self.bind_addr, self.port)).await?;
        Ok(())
    }
}

pub struct ChannelTask<H: RuntimeHandler> {
    acceptor: ServerAcceptor<H::System, H::Codec>,
    bind_addr: IpAddr,
    port: u16,
}
impl<H: RuntimeHandler> SupervisedTask for ChannelTask<H> {
    type Context = ();

    async fn run(&mut self, _ctx: &mut Self::Context) -> anyhow::Result<()> {
        self.acceptor.run_tcp((self.bind_addr, self.port)).await?;
        Ok(())
    }
}

pub struct ServerRuntime<H: RuntimeHandler> {
    _handler: PhantomData<H>,
    net_sys: NetSystem<H::System>,
    login_task: LoginTask<H>,
    channel_task: Vec<ChannelTask<H>>,
    clck: Clock
}

impl<H: RuntimeHandler> ServerRuntime<H> {
    pub fn new(
        cfg: &RuntimeConfig,
        net_sys: NetSystem<H::System>,
        cdc: Arc<H::Codec>,
        ctx: H::Ctx,
        clck: Clock
    ) -> Self {
        Self {
            _handler: PhantomData,
            login_task: LoginTask {
                login: RpcListener::new(cdc.clone(), ctx),
                bind_addr: cfg.bind_addr,
                port: cfg.login_port,
            },
            channel_task: cfg
                .game_ports
                .clone()
                .map(|port| ChannelTask {
                    acceptor: ServerAcceptor::new(net_sys.handle(), cdc.clone()),
                    bind_addr: cfg.bind_addr,
                    port,
                })
                .collect(),
            net_sys,
            clck
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        let _login = SupervisedTaskHandle::spawn(self.login_task, (), Duration::from_secs(1));
        let _channels = self
            .channel_task
            .into_iter()
            .map(|task| SupervisedTaskHandle::spawn(task, (), Duration::from_secs(1)))
            .collect::<Vec<_>>();

        tokio::spawn(async move {
            loop {
                self.clck.tick().await;
            }
        });
        //TODO
        self.net_sys.run().await.unwrap();
        Ok(())
    }
}
