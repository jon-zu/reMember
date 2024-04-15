use std::{num::Wrapping, sync::Arc};

use shroom_meta::id::{CharacterId, FieldId};

use shroom_srv::{
    act::system::SystemHandler,
    net::{session::NetSession, system::NetSystemHandler},
    ClockHandle,
};

use shroom_proto95::game::MigrateInGameReq;
use tokio::net::TcpStream;

use crate::{
    field::{FieldHandler, SharedFieldState},
    game::GameSession,
    repl::GameRepl,
    services::shared::Services,
    session::{
        shroom_session_backend::AccountAuth, shroom_session_manager::OwnedShroomGameSession,
        ShroomMigrationKey,
    },
};

pub struct GameCtx {
    pub services: Arc<Services>,
    pub clock: ClockHandle,
}

#[cfg(feature = "websockets")]
pub type GameCodec = shroom_net::codec::websocket::WebSocketCodec<TcpStream>;

#[cfg(not(feature = "websockets"))]
pub type GameCodec = shroom_net::codec::legacy::LegacyCodecShanda<TcpStream>;

pub struct GameSystem {
    pub services: Arc<Services>,
}

impl SystemHandler for GameSystem {
    type Error = anyhow::Error;
    type SessionId = CharacterId;
    type RoomId = FieldId;
    type Session = NetSession<GameSession>;
    type Room = FieldHandler;

    fn create_room(&mut self, id: Self::RoomId) -> Result<Self::Room, Self::Error> {
        log::info!("Creating room: {id}");
        let meta = self.services.game.meta;
        let field_meta = meta.get_field(id).unwrap();
        let field_fh = meta.get_field_fh_data(id).unwrap();
        Ok(FieldHandler::new(
            meta,
            self.services.current_time.load(),
            SharedFieldState {
                field_meta,
                field_fh,
            }
            .into(),
        ))
    }

    fn on_tick(&mut self, t: shroom_srv::GameTime) -> Result<(), Self::Error> {
        self.services.current_time.store(t);
        Ok(())
    }
}

impl NetSystemHandler for GameSystem {
    type Error = anyhow::Error;
    type Codec = GameCodec;
    type System = Self;

    async fn create_session(
        &self,
        mut sck: shroom_srv::net::socket::ServerSocketHandle,
    ) -> Result<NetSession<GameSession>, Self::Error> {
        log::info!("New session: {:?}", sck.peer_addr());
        // Read handshake packet
        let msg = sck
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("Handshake packet not received"))?;
        let (session, client_key) = if msg.opcode_value() == 0x14 {
            log::info!("Normal migrate in...");
            let req: MigrateInGameReq = msg.decode()?;
            let migrate_key = ShroomMigrationKey::new(req.client_key, sck.peer_addr());
            (
                self.services
                    .session_manager
                    .claim_migrating_session(migrate_key)
                    .await?,
                req.client_key,
            )
        } else if msg.opcode_value() == 0x214 {
            let mut pr = msg.reader();
            let char_id = pr.read_i32()?;
            let acc_id = pr.read_i32()?;
            let client_key = pr.read_array::<8>()?;
            let token = pr.read_array::<32>()?;

            log::info!(
                "Logging in with token: {:?} for acc: {acc_id} - char: {char_id}",
                token
            );

            let sess = self
                .services
                .session_manager
                .create_claimed_session(AccountAuth::Token(
                    acc_id,
                    CharacterId(char_id as u32),
                    token,
                ))
                .await?;
            (sess, client_key)
        } else {
            return Err(anyhow::anyhow!("Invalid opcode"));
        };

        // TODO, add a try_map function to owned session
        let session: OwnedShroomGameSession = session.try_map(|sess| sess.as_mut().try_into())?;

        log::info!("Claimed session");

        log::info!(
            "Game session for acc: {} - char: {}",
            session.acc.username,
            session.char.name
        );
        log::info!("Spawning");

        let field_id = session.char.field;

        let sess = GameSession {
            services: self.services.clone(),
            session,
            addr: sck.peer_addr(),
            channel_id: 0,
            world_id: 0,
            client_key,
            current_script: None,
            field_id,
            field_meta: self.services.game.meta.get_field(field_id).unwrap(),
            repl: GameRepl::new(),
            field_key: Wrapping(0),
        };
        Ok(NetSession::new(sess, sck))
    }
}
