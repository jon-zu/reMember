use std::{marker::PhantomData, net::IpAddr, ops::RangeInclusive, time::Duration};
use crate::{
    net::system::{NetSystem, NetSystemHandler}, rpc::{RpcListener, RpcService}, util::supervised_task::{SupervisedTask, SupervisedTaskHandle}
};

pub struct RuntimeConfig {
    pub bind_addr: IpAddr,
    pub login_port: u16,
    pub game_ports: RangeInclusive<u16>,
}

pub trait RuntimeHandler: Send + 'static {
    type Ctx: Send + Sync + 'static;
    type NetHandler: NetSystemHandler;
    type LoginService: RpcService<Ctx = Self::Ctx, Codec = <Self::NetHandler as NetSystemHandler>::Codec>
        + Send
        + 'static;
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

/*
pub struct ChannelTask<H: NetSystemHandler> {
    acceptor: ServerAcceptor<H>,
    bind_addr: IpAddr,
    port: u16,
}
impl<H: NetSystemHandler> SupervisedTask for ChannelTask<H>
{
    type Context = ();

    async fn run(&mut self, _ctx: &mut Self::Context) -> anyhow::Result<()> {
        self.acceptor.run_tcp((self.bind_addr, self.port)).await?;
        Ok(())
    }
}*/

pub struct ServerRuntime<H: RuntimeHandler> {
    _handler: PhantomData<H>,
    net_sys: NetSystem<H::NetHandler>,
    login_task: LoginTask<H>,
    addr: IpAddr,
    game_ports: RangeInclusive<u16>,
}

impl<H: RuntimeHandler> ServerRuntime<H> {
    pub fn new(
        cfg: &RuntimeConfig,
        net_sys: NetSystem<H::NetHandler>,
        cdc: <H::NetHandler as NetSystemHandler>::Codec,
        ctx: H::Ctx,
    ) -> Self {
        //let acceptor = ServerAcceptor::new(net, cdc.clone(), sys.handle());
        Self {
            _handler: PhantomData,
            login_task: LoginTask {
                login: RpcListener::new(cdc, ctx),
                bind_addr: cfg.bind_addr,
                port: cfg.login_port,
            },
            net_sys,
            addr: cfg.bind_addr,
            game_ports: cfg.game_ports.clone(),
            /*channel_task: ChannelTask {
                acceptor,
                bind_addr: cfg.bind_addr,
                port: *cfg.game_ports.start(),
            },*/
            //system: sys,
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        let _login = SupervisedTaskHandle::spawn(self.login_task, (), Duration::from_secs(1));
        self.net_sys.spawn_acceptors(self.addr, self.game_ports);
        self.net_sys.run().await.unwrap();
        Ok(())
    }
}
