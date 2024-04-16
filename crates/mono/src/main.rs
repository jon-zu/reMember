use std::{fs::File, net::IpAddr, path::PathBuf, sync::Arc, time::Duration};

use dotenv::dotenv;

use shroom_data::services::{server_service::ServerInfo, DataProvider};
use shroom_game::{
    services::shared::{PacketEOFHandler, Services, SharedServices},
    system::{GameCodec, GameSystem},
};
use shroom_login::LoginService;

use shroom_meta::id::job_id::JobId;
use shroom_srv::{act::system::SystemConfig, net::system::NetSystemHandler};

use shroom_srv::runtime::{RuntimeConfig, RuntimeHandler, ServerRuntime};
use tokio::time::interval;

use crate::config::Environment;

mod config;

static BANNER: &str = r#"
                888b     d888                        888                       
                8888b   d8888                        888                       
                88888b.d88888                        888                       
888d888 .d88b.  888Y88888P888  .d88b.  88888b.d88b.  88888b.   .d88b.  888d888 
888P"  d8P  Y8b 888 Y888P 888 d8P  Y8b 888 "888 "88b 888 "88b d8P  Y8b 888P"   
888    88888888 888  Y8P  888 88888888 888  888  888 888  888 88888888 888     
888    Y8b.     888   "   888 Y8b.     888  888  888 888 d88P Y8b.     888     
888     "Y8888  888       888  "Y8888  888  888  888 88888P"   "Y8888  888"#;


pub struct Mono {
    data_dir: PathBuf,
    env: Environment,
    external_ip: IpAddr,
    login_port: u16,
    game_ports: std::ops::RangeInclusive<u16>,
    server_name: String,
}

impl Mono {
    async fn build_services(&self) -> anyhow::Result<Services> {
        let meta = Box::new(shroom_meta::MetaService::load_from_dir(
            self.data_dir.join("shroom-metadata"),
            shroom_meta::MetaOption::Full,
        )?);
        log::info!("Loaded meta data");

        let my_local_ip = local_ip_address::local_ip().unwrap();
        log::info!("Local ip is: {}", my_local_ip);

        let static_meta = Box::leak(meta);

        let servers = [ServerInfo::new(
            self.external_ip,
            self.login_port,
            self.server_name.clone(),
            self.game_ports.clone().count(),
        )];

        let data_services = match self.env {
            Environment::Local => DataProvider::seeded_in_memory(static_meta).await?,
            _ => {
                // Wait for db to start
                tokio::time::sleep(Duration::from_secs(5)).await;
                let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
                log::info!("db url: {db_url}");
                DataProvider::seeded_in_db(static_meta, &db_url).await?
            }
        };
        match self.env {
            Environment::Local => {
                let (acc_id, char_id) = Box::pin(data_services.seed_acc_char()).await?;
                log::info!("Created test account {acc_id} - char: {char_id}");
                data_services
                    .seed_class(
                        "warrior",
                        &[JobId::Hero, JobId::Paladin, JobId::DarkKnight],
                        static_meta.item_sets().get("warrior").unwrap(),
                    )
                    .await?;
                data_services
                    .seed_class(
                        "mage",
                        &[
                            JobId::ArchMageFirePoinson,
                            JobId::ArchMageIceLightning,
                            JobId::Bishop,
                        ],
                        static_meta.item_sets().get("mage").unwrap(),
                    )
                    .await?;
            }
            _ => {
                let (acc_id, char_id) = Box::pin(data_services.seed_acc_char()).await?;
                log::info!("Created test account {acc_id} - char: {char_id}");
            }
        }

        let eof_handler = PacketEOFHandler::new(File::create("packets_eof.log")?);
        Ok(Services::new_with_eof(
            data_services,
            servers,
            static_meta,
            eof_handler,
        ))
    }
}

pub struct MonoRuntime {}

impl RuntimeHandler for MonoRuntime {
    type Ctx = SharedServices;
    type LoginService = LoginService<<GameSystem as NetSystemHandler>::Codec>;
    type NetHandler = GameSystem;
}

#[cfg(feature = "websockets")]
fn build_codec(_v: usize) -> GameCodec {
    shroom_net::codec::websocket::WebSocketCodec::new("ws://127.0.0.1".try_into().unwrap())
}

#[cfg(not(feature = "websockets"))]
fn build_codec(v: usize) -> GameCodec {
    use shroom_net::{
        codec::legacy::{handshake_gen::BasicHandshakeGenerator, LegacyCodec},
        CryptoContext,
    };

    let handshake_gen = match v {
        83 => BasicHandshakeGenerator::v83(),
        95 => BasicHandshakeGenerator::v95(),
        _ => todo!("unexpected client version"),
    };

    let crypto_ctx = Arc::new(CryptoContext::default());
    LegacyCodec::new(crypto_ctx.clone(), handshake_gen.clone())
}

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .thread_stack_size(16 * 1024 * 1024)
        .enable_all()
        .build()?;

    rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();

    log::info!("{BANNER}");

    let data_dir: PathBuf = std::env::var("DATA_DIR")
        .unwrap_or("".to_string())
        .into();

    // Load configuration
    let settings = config::get_configuration(&data_dir).expect("Failed to load configuration");
    log::info!("{0} - Mono - {1}", settings.server_name, settings.version);

    let ext_ip = std::env::var("EXTERNAL_IP")
        .ok()
        .or(settings.external_ip)
        .ok_or_else(|| anyhow::format_err!("No external IP set"))?;

    log::info!("External IP: {0}", ext_ip);

    let server_addr: IpAddr = ext_ip.parse()?;
    let bind_addr: IpAddr = settings.bind_ip.parse()?;

    log::info!("Loaded crypto context");
    // Create login server

    // Meta will be available all the time
    let mono = Mono {
        data_dir,
        env: config::get_environment(),
        external_ip: server_addr,
        login_port: 8484,
        game_ports: 8485..=8485 + (settings.num_channels),
        server_name: settings.server_name.clone(),
    };
    let services = Box::pin(mono.build_services()).await?;
    let services = Arc::new(services);
    let cfg = RuntimeConfig {
        bind_addr,
        login_port: 8484,
        game_ports: 8485..=8486,
    };
    let cdc_sys = build_codec(settings.client_version);
    let cdc_runtime = build_codec(settings.client_version);
    let svc = services.clone();

    tokio::spawn(async move {
        let mut lifecycle = interval(Duration::from_secs(15));

        loop {
            tokio::select! {
                session_key = svc.session_manager.next_dropped_session() => {
                    log::info!("Closing session {session_key}");
                    if let Err(err) = svc.session_manager.close_session_by_key(session_key).await {
                        log::error!("Error during closing session: {err:?}");
                    }
                },
                _ = lifecycle.tick() => {
                    if let Err(err) = svc.session_manager.clean().await {
                        log::error!("Error during cleaning sessions: {err:?}");
                    }
                }
            }
        }
    });

    let svc = services.clone();
    let sys =
        shroom_srv::act::system::System::new(GameSystem { services: svc }, SystemConfig::default());

    let svc = services.clone();
    let net_sys = shroom_srv::net::system::NetSystem::new(cdc_sys, GameSystem { services }, sys);
    let runtime = ServerRuntime::<MonoRuntime>::new(&cfg, net_sys, cdc_runtime, svc);
    log::info!("Spawning system...");
    runtime.run().await?;

    Ok(())
}
