pub mod config;
pub mod login_state;

use config::LoginConfig;
use login_state::LoginState;
use shroom_data::services::account::AccountServiceError;
use shroom_data::services::character::{CharWithEquips, CharacterCreateDTO, ItemStarterSet};
use shroom_game::services::shared::SharedServices;
use shroom_game::session::shroom_session_backend::{
    AccountAuth, ShroomSessionData, ShroomSessionError,
};
use shroom_game::session::ShroomMigrationKey;
use shroom_meta::id::job_id::JobGroup;
use shroom_meta::id::{FaceId, HairId, ItemId, Skin};
use shroom_net::codec::ShroomCodec;
use shroom_pkt::pkt::Message;
use shroom_proto95::login::{
    BlockedIp, ChannelId, CheckPasswordReq, CheckPasswordResp, CheckPinReq, CheckPinResp,
    ConfirmEULAReq, ConfirmEULAResp, LoginAccountData, LoginInfo, LogoutWorldReq,
    RecommendWorldMessageResp, SelectWorldReq, SetGenderReq, SuccessResult, UpdatePinReq,
    UpdatePinResp, WorldCheckUserLimitReq, WorldCheckUserLimitResp, WorldId, WorldInfoReq,
    WorldInfoResp, WorldReq,
};
use shroom_proto95::recv_opcodes::RecvOpcodes;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use shroom_proto95::shared::char::AvatarEquips;
use shroom_proto95::shared::PongReq;
use shroom_proto95::{
    login::{
        char::{
            CharRankInfo, CheckDuplicateIDReq, CheckDuplicateIDResp, CheckDuplicateIDResult,
            CreateCharReq, CreateCharResp, DeleteCharReq, DeleteCharResp, DeleteCharResult,
            MigrateStageInfo, SelectCharReq, SelectCharResp, SelectCharResult, SelectWorldCharList,
            SelectWorldResp, ViewChar, ViewCharWithRank,
        },
        CreateSecurityHandleReq, LoginOpt, LoginResultHeader,
    },
    shared::{
        char::{AvatarData, CharStat, PetIds},
        UpdateScreenSettingReq,
    },
};

use shroom_pkt::{ShroomList8, ShroomTime};
use shroom_srv::rpc::{RpcCtx, RpcResponse, RpcService};
use shroom_srv::session::Error;


pub struct LoginService<C> {
    services: SharedServices,
    cfg: Arc<LoginConfig>,
    login_state: Box<LoginState>,
    migrating: bool,
    _c: PhantomData<C>
}

impl<C: ShroomCodec> LoginService<C> {
    pub fn new(services: SharedServices, cfg: Arc<LoginConfig>) -> Self {
        Self {
            migrating: false,
            services,
            cfg,
            login_state: Box::new(LoginState::new()),
            _c: PhantomData,
        }
    }
}


impl<C: ShroomCodec> RpcService for LoginService<C> {
    type Ctx = SharedServices;
    type Codec = C;
    type PingPacket = PongReq;

    fn create(ctx: &Self::Ctx) -> anyhow::Result<Self> {
        Ok(Self::new(
            ctx.clone(),
            Arc::new(LoginConfig::default()),
        ))
    }

    fn ping_packet(&self) -> Self::PingPacket {
        PongReq
    }

    async fn on_packet(&mut self, pkt: Message, ctx: &mut RpcCtx<C>) -> anyhow::Result<RpcResponse> {
        let op = pkt.opcode_value();
        log::info!("Got packet: {op:?}");
        let res = self.handle_packet(pkt, ctx).await;
        if let Err(err) = &res {
            log::error!("Error handling login packet: {:?} - op: {:?}", err, op);
        }

        res
    }

    async fn finish(self) -> anyhow::Result<()> {
        //TODO check migrate
        if self.migrating {
            tokio::time::sleep(std::time::Duration::from_secs(7)).await;
        }

        Ok(())
    }
}

pub type LoginResponse = anyhow::Result<RpcResponse>;

impl<C: ShroomCodec> LoginService<C> {
    pub async fn handle_packet(
        &mut self,
        msg: Message,
        ctx: &mut RpcCtx<C>,
    ) -> anyhow::Result<RpcResponse> {
        macro_rules! handler {
            ($ctx:ident, $msg:ident, $this:ident, $default:ident, $($req:ty => $handler:ident),*) => {
                match $msg.opcode().unwrap() {
                    $(
                        <$req as shroom_pkt::HasOpCode>::OPCODE => $this.$handler($ctx, $msg.decode::<$req>()?).await,
                    )*
                    _ => $this.$default($ctx, $msg).await
                }
            };
        }

        let op: RecvOpcodes = msg.opcode()?;
        if op == RecvOpcodes::CheckPassword && !self.login_state.is_unauthorized_stage() {
            log::info!("Skipping duplicate login packet");
            return Ok(RpcResponse::Ok);
        }

        log::info!("Handling login packet: {op:?}");

        //ExceptionLogReq => handle_exception_log

        handler!(
            ctx,
            msg,
            self,
            handle_default,
            PongReq => handle_pong,
            CreateSecurityHandleReq => handle_create_security_handle,
            UpdateScreenSettingReq => handle_update_screen_setting,
            CheckPasswordReq => handle_check_password,
            SetGenderReq => handle_set_gender,
            CheckPinReq => handle_check_pin,
            UpdatePinReq => handle_register_pin,
            ConfirmEULAReq => handle_accept_tos,
            WorldInfoReq => handle_world_information,
            LogoutWorldReq => handle_world_logout,
            WorldReq => handle_world_request,
            WorldCheckUserLimitReq => handle_world_check_user_limit,
            SelectWorldReq => handle_select_world,
            CheckDuplicateIDReq => handle_check_duplicate_id,
            CreateCharReq => handle_create_char,
            DeleteCharReq => handle_delete_character,
            SelectCharReq => handle_select_char
        )
    }

    pub async fn handle_default(&mut self, _ctx: &mut RpcCtx<C>, msg: Message) -> LoginResponse {
        log::info!("Unhandled packet: {:?}", msg.opcode::<RecvOpcodes>().unwrap());
        Ok(RpcResponse::Ok)
    }

    async fn handle_pong(&mut self, _ctx: &mut RpcCtx<C>, _req: PongReq) -> LoginResponse {
        Ok(RpcResponse::Pong)
    }

    /*async fn handle_exception_log(
        &mut self,
        _ctx: &mut RpcCtx<C>,
        _req: ExceptionLogReq,
    ) -> LoginResponse {
        Ok(RpcResponse::Ok)
    }*/

    async fn handle_create_security_handle(
        &mut self,
        _ctx: &mut RpcCtx<C>,
        _req: CreateSecurityHandleReq,
    ) -> LoginResponse {
        Ok(RpcResponse::Ok)
    }

    async fn handle_update_screen_setting(
        &mut self,
        _ctx: &mut RpcCtx<C>,
        _req: UpdateScreenSettingReq,
    ) -> LoginResponse {
        Ok(RpcResponse::Ok)
    }

    async fn handle_accept_tos(&mut self, ctx: &mut RpcCtx<C>, req: ConfirmEULAReq) -> LoginResponse {
        self.login_state.get_accept_tos()?;

        if !req.accepted {
            anyhow::bail!("Should accept the TOS");
        }

        let svc = self.services.clone();

        self.login_state
            .update_account(|acc| svc.game.data.account.accept_tos(acc))
            .await?;
        self.login_state.reset();

        ctx.send(ConfirmEULAResp { success: true }).await?;
        Ok(RpcResponse::Ok)
    }

    async fn handle_check_pin(&mut self, ctx: &mut RpcCtx<C>, req: CheckPinReq) -> LoginResponse {
        let acc = self.login_state.get_pin()?;

        ctx.send(if self.cfg.enable_pin {
            match req.pin.opt {
                Some(pin) => {
                    if self.services.game.data.account.check_pin(acc, &pin.pin)? {
                        CheckPinResp::Accepted
                    } else {
                        CheckPinResp::InvalidPin
                    }
                }
                _ => CheckPinResp::EnterPin,
            }
        } else {
            CheckPinResp::Accepted
        })
        .await?;

        Ok(RpcResponse::Ok)
    }

    async fn handle_register_pin(&mut self, ctx: &mut RpcCtx<C>, req: UpdatePinReq) -> LoginResponse {
        self.login_state.get_pin()?;

        let Some(pin) = req.pin.opt else {
            //TODO handle a login reset here not a dc
            anyhow::bail!("Pin registration cancelled");
        };

        let svc = self.services.clone();

        self.login_state
            .update_account(|acc| svc.game.data.account.set_pin(acc, pin))
            .await?;

        ctx.send(UpdatePinResp { success: true }).await?;

        Ok(RpcResponse::Ok)
    }

    async fn handle_set_gender(&mut self, _ctx: &mut RpcCtx<C>, req: SetGenderReq) -> LoginResponse {
        let _ = self.login_state.get_set_gender()?;

        let gender = req
            .gender
            .opt
            .ok_or_else(|| anyhow::format_err!("Gender not set"))?;

        let svc = self.services.clone();

        self.login_state
            .update_account(|acc| svc.game.data.account.set_gender(acc, gender.into()))
            .await?;
        /*
        self.login_state.transition_login().unwrap();

        //TODO this doesn't set the client key, maybe make it dc?
        Ok(SetGenderResp {
            gender,
            success: true,
        })
        */

        todo!("Set gender");
    }

    async fn handle_world_logout(&mut self, _ctx: &mut RpcCtx<C>, _req: LogoutWorldReq) -> LoginResponse {
        self.login_state.get_char_select()?;
        self.login_state.transition_server_select()?;

        Ok(RpcResponse::Ok)
    }

    async fn handle_world_check_user_limit(
        &mut self,
        ctx: &mut RpcCtx<C>,
        _req: WorldCheckUserLimitReq,
    ) -> LoginResponse {
        let _acc = self.login_state.get_server_selection()?;

        ctx.send(WorldCheckUserLimitResp {
            over_user_limit: false,
            populate_level: 0,
        })
        .await?;

        Ok(RpcResponse::Ok)
    }

    fn get_world_info(&self) -> Vec<WorldInfoResp> {
        self.services
            .game
            .server_info
            .get_world_info_packets()
            .into_iter()
            .collect()
    }

    async fn handle_world_information(
        &mut self,
        ctx: &mut RpcCtx<C>,
        _req: WorldInfoReq,
    ) -> LoginResponse {
        ctx.send_all(self.get_world_info().into_iter()).await?;
        tokio::time::sleep(Duration::from_millis(500)).await;
        ctx.send(RecommendWorldMessageResp {
            messages: ShroomList8::default(),
        })
        .await?;

        Ok(RpcResponse::Ok)
    }

    async fn handle_world_request(&mut self, ctx: &mut RpcCtx<C>, _req: WorldReq) -> LoginResponse {
        ctx.send_all(self.get_world_info().into_iter()).await?;
        tokio::time::sleep(Duration::from_millis(500)).await;
        ctx.send(RecommendWorldMessageResp {
            messages: ShroomList8::default(),
        })
        .await?;

        Ok(RpcResponse::Ok)
    }

    pub async fn handle_check_password(
        &mut self,
        ctx: &mut RpcCtx<C>,
        req: CheckPasswordReq,
    ) -> LoginResponse {
        let login_result = self
            .services
            .session_manager
            .create_claimed_session(AccountAuth::UsernamePassword(req.id, req.pw))
            .await;
        let hdr = LoginResultHeader::default();

        let resp = match login_result {
            Err(Error::Backend(ShroomSessionError::Account(acc))) => match acc {
                AccountServiceError::UsernameNotFound => CheckPasswordResp::InvalidUserName(hdr),
                AccountServiceError::PasswordMismatch => CheckPasswordResp::InvalidPassword(hdr),
                AccountServiceError::AccountBanned => CheckPasswordResp::BlockedIp(BlockedIp {
                    hdr,
                    reason: 0,
                    ban_time: ShroomTime::now(), // TODO
                }),
                AccountServiceError::AccountAlreadyLoggedIn => {
                    CheckPasswordResp::AlreadyLoggedIn(hdr)
                }

                _ => todo!("Unhandled Account Service Login Result: {:?}", acc),
            },

            Ok(login_session) => {
                // TODO, add a try_map function to owned session
                let login_session = login_session.map(|sess| match sess.as_mut() {
                    ShroomSessionData::Ingame(_) => unreachable!("Session is not a login session"),
                    ShroomSessionData::Login(l) => l,
                });

                self.login_state
                    .transition_login_with_session(login_session)?;
                let client_key = self
                    .login_state
                    .get_client_key()
                    .expect("Must have client key after login");

                let login_info = (!self.login_state.is_set_gender_stage())
                    .then_some(LoginInfo {
                        skip_pin: false,
                        login_opt: shroom_proto95::login::LoginOpt::EnableSecondPassword,
                        client_key,
                    })
                    .into();

                if self.login_state.is_accept_tos_stage() {
                    CheckPasswordResp::TOS(hdr)
                } else {
                    CheckPasswordResp::Success(SuccessResult {
                        hdr,
                        account: LoginAccountData {
                            account_info: self.login_state.get_account_info()?,
                            login_info,
                        },
                    })
                }
            }
            _ => todo!("Unhandled Account Service Login Result: {:?}", login_result),
        };

        ctx.send(resp).await?;

        Ok(RpcResponse::Ok)
    }

    async fn handle_select_world(&mut self, ctx: &mut RpcCtx<C>, req: SelectWorldReq) -> LoginResponse {
        let acc = self.login_state.get_server_selection()?;
        let char_list = self
            .services
            .game
            .data
            .char()
            .get_characters_with_equips(acc.id)
            .await?;

        let select_char_list = SelectWorldCharList {
            characters: char_list.iter().map(map_char_with_rank).collect(),
            //TODO pic handling
            login_opt: LoginOpt::NoSecondPassword1,
            slot_count: acc.character_slots as u32,
            //TODO get buy count
            buy_char_count: 3,
        };

        self.login_state.transition_char_select(
            req.world_id as WorldId,
            req.channel_id as ChannelId,
            char_list,
        )?;

        ctx.send(SelectWorldResp::Success(select_char_list)).await?;
        Ok(RpcResponse::Ok)
    }

    async fn handle_check_duplicate_id(
        &mut self,
        ctx: &mut RpcCtx<C>,
        req: CheckDuplicateIDReq,
    ) -> LoginResponse {
        let _ = self.login_state.get_char_select()?;
        let name_used = !self.services.game.data.char().check_name(&req.name).await?;

        let resp = if name_used {
            CheckDuplicateIDResp {
                name: String::new(),
                result: CheckDuplicateIDResult::Error1,
            }
        } else {
            CheckDuplicateIDResp {
                name: req.name,
                result: CheckDuplicateIDResult::Success,
            }
        };

        ctx.send(resp).await?;

        Ok(RpcResponse::Ok)
    }

    async fn handle_create_char(&mut self, ctx: &mut RpcCtx<C>, req: CreateCharReq) -> LoginResponse {
        let (acc, _, _, _) = self.login_state.get_char_select()?;

        let starter_set = ItemStarterSet {
            shoes: req.starter_set.shoes,
            bottom: req.starter_set.bottom,
            weapon: req.starter_set.weapon,
            top: req.starter_set.top,
            guide: req.job.get_guide_item(),
        };

        let char_id = Box::pin(self.services.game.data.char().create_character(
            acc.id,
            CharacterCreateDTO {
                name: req.name,
                job: either::Either::Left(JobGroup::Legend),
                face: req.starter_set.face,
                skin: (req.starter_set.skin_color as u8).try_into()?,
                hair: req.starter_set.hair,
                //TODO hair color
                starter_set,
                gender: req.gender,
                max_skills: false,
                level: None
            },
            &self.services.game.data.item,
        ))
        .await?;

        let char = self.services.game.data.char().get(char_id).await?.unwrap();
        // TODO add actual eqs
        let empty_eq = CharWithEquips {
            char,
            equips: Default::default(),
        };
        ctx.send(CreateCharResp::Success(Box::new(map_char(&empty_eq))))
            .await?;
        Ok(RpcResponse::Ok)
    }

    async fn handle_delete_character(
        &mut self,
        ctx: &mut RpcCtx<C>,
        req: DeleteCharReq,
    ) -> LoginResponse {
        let (acc, _, _, _) = self.login_state.get_char_select()?;
        let status = self
            .services
            .game
            .data
            .char()
            .delete_character(acc, req.char_id, &req.pic)
            .await?;

        let result = match status {
            DeleteCharResult::Success => DeleteCharResult::Success,
            //TODO add more
            _ => DeleteCharResult::UnknownErr,
        };

        ctx.send(DeleteCharResp {
            char_id: req.char_id,
            result,
        })
        .await?;

        Ok(RpcResponse::Ok)
    }

    async fn handle_select_char(&mut self, ctx: &mut RpcCtx<C>, req: SelectCharReq) -> LoginResponse {
        let (char, client_key, world, channel) =
            self.login_state.transition_game(req.char_id)?;

        let session = self.login_state.claim_session()?.unmap();
        Box::pin(self.services.session_manager.transition_migrate_session(
            ShroomMigrationKey::new(client_key, ctx.peer_addr()),
            session,
            char,
        ))
        .await?;

    log::info!("Handling selecting...");

        let addr = self
            .services
            .game
            .server_info
            .get_channel_addr(world, channel)?;
        let migrate = MigrateStageInfo {
            socket_addr: addr.try_into()?,
            char_id: req.char_id,
            premium: false,
            premium_arg: 0,
        };
        log::info!("Migration...");

        let pkt = SelectCharResp {
            error_code: 0,
            result: SelectCharResult::Success(migrate),
        };

        log::info!("About to migrate: {pkt:?}");
        ctx.send(pkt).await?;
        self.migrating = true;
        Ok(RpcResponse::Migrate)
    }
}

pub fn map_char_to_avatar(char: &CharWithEquips) -> AvatarData {
    let eq = &char.equips;
    let char = &char.char;
    AvatarData {
        gender: (&char.gender).into(),
        skin: Skin::try_from(char.skin as u8).unwrap(),
        mega: true,
        face: FaceId(char.face as u32),
        hair: HairId(char.hair as u32),
        equips: AvatarEquips {
            equips: eq
                .equipped
                .iter()
                .map(|(slot, item)| (*slot as u8, *item))
                .collect(),
            masked_equips: eq
                .masked
                .iter()
                .map(|(slot, item)| (*slot as u8, *item))
                .collect(),
            weapon_sticker_id: ItemId(0),
        },
        pets: PetIds::default(),
    }
}

pub fn map_rank_info(_char: &CharWithEquips) -> CharRankInfo {
    CharRankInfo {
        world_rank: 0,
        rank_move: 0,
        job_rank: 0,
        job_rank_mode: 0,
    }
}

pub fn map_char(char: &CharWithEquips) -> ViewChar {
    let stats: CharStat = (&char.char).into();
    let avatar_data = map_char_to_avatar(char);

    ViewChar { stats, avatar_data }
}

fn map_char_with_rank(char: &CharWithEquips) -> ViewCharWithRank {
    ViewCharWithRank {
        view_char: map_char(char),
        u1: 0,
        rank_info: Some(map_rank_info(char)).into(),
    }
}
