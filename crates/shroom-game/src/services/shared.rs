use std::{
    fs::File,
    ops::Deref,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossbeam::atomic::AtomicCell;
use scripts_lib::ScriptService;
use shroom_data::services::{
    server_service::{ServerInfo, ServerService},
    DataProvider,
};

use shroom_meta::{id::CharacterId, MetaService};
use shroom_pkt::{error::EOFErrorData, PacketReader};
use shroom_proto95::recv_opcodes::RecvOpcodes;
use shroom_srv::GameTime;

use crate::session::{ShroomSessionBackend, ShroomSessionManager};

pub type SharedServices = Arc<Services>;
pub type SharedGameServices = Arc<GameServices>;

#[derive(Debug)]
pub struct PacketEOFHandler {
    pub eof_file: Mutex<File>,
}

impl PacketEOFHandler {
    pub fn new(eof_file: File) -> Self {
        Self {
            eof_file: Mutex::new(eof_file),
        }
    }

    pub fn create(eof_file: File) -> Arc<Self> {
        Arc::new(Self::new(eof_file))
    }

    pub fn handle_eof(
        &self,
        char_id: CharacterId,
        packet: &[u8],
        eof: &EOFErrorData,
    ) -> anyhow::Result<()> {
        use std::io::Write;
        let mut f = self.eof_file.lock().unwrap();

        let op = PacketReader::from(&packet)
            .read_opcode::<RecvOpcodes>()
            .ok();
        let analytics = eof.analytics(packet);

        writeln!(f, "----------")?;
        writeln!(f, "Character ID: {char_id} - Packet: {op:?}")?;
        writeln!(f, "{analytics}")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct GameServices {
    pub data: DataProvider,
    pub server_info: ServerService,
    pub meta: &'static MetaService,
    pub eof_handler: Option<PacketEOFHandler>,
    pub scripts: ScriptService,
    pub current_time: AtomicCell<GameTime>,
}

impl Deref for GameServices {
    type Target = DataProvider;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug)]
pub struct Services {
    pub game: Arc<GameServices>,
    pub session_manager: ShroomSessionManager<ShroomSessionBackend>,
}

impl Deref for Services {
    type Target = GameServices;

    fn deref(&self) -> &Self::Target {
        &self.game
    }
}

impl Services {
    pub fn new(
        data: DataProvider,
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
    ) -> Self {
        let game = Arc::new(GameServices {
            data,
            server_info: ServerService::new(servers),
            meta,
            eof_handler: None,
            scripts: ScriptService::default(),
            current_time: AtomicCell::new(GameTime::default())
        });

        let session_backend = ShroomSessionBackend::new(game.clone());

        Self {
            game,
            session_manager: ShroomSessionManager::new(session_backend, Duration::from_secs(30)),
        }
    }

    pub fn new_with_eof(
        data: DataProvider,
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
        eof_handler: PacketEOFHandler,
    ) -> Self {
        let game = Arc::new(GameServices {
            data,
            server_info: ServerService::new(servers),
            meta,
            eof_handler: Some(eof_handler),
            scripts: ScriptService::default(),
            current_time: AtomicCell::new(GameTime::default())
        });

        let session_backend = ShroomSessionBackend::new(game.clone());

        Self {
            game,
            session_manager: ShroomSessionManager::new(session_backend, Duration::from_secs(30)),
        }
    }
}
