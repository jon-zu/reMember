use shroom_meta::twod::Vec2;
use shroom_pkt::{
    mark_shroom_bitflags, shroom_enum_code, with_opcode, CondOption, ShroomList16, ShroomList8,
    ShroomOption8, ShroomPacket, ShroomPacketEnum, ShroomTime,
};

use crate::{
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{Gender, OptionGender},
};

pub mod char;

#[derive(Debug, ShroomPacket, Default)]
pub struct MachineId(pub [u8; 0x10]);
pub type ClientKey = [u8; 8];

#[derive(ShroomPacket, Debug)]
pub struct CreateSecurityHandleReq;
with_opcode!(CreateSecurityHandleReq, RecvOpcodes::CreateSecurityHandle);

shroom_enum_code!(StartMode, u8, WebStart = 0, Unknown1 = 1, GameLaunching = 2);

impl StartMode {
    pub fn has_system_info(&self) -> bool {
        self == &Self::Unknown1
    }
}

#[derive(ShroomPacket, Debug)]
pub struct StartModeInfo {
    pub start_mode: StartMode,
    #[pkt(check(field = "start_mode", cond = "StartMode::has_system_info"))]
    pub system_info: CondOption<SystemInfo>,
}

#[derive(ShroomPacket, Debug)]
pub struct SystemInfo {
    // SupportID?
    unknown: String,
    machine_id: MachineId,
    game_room_client: u32,
    start_mode: u8,
}

shroom_enum_code!(
    RegStateId,
    u8,
    // Both work `Registered` fine as success codes
    default(Registered0 = 0),
    Registered1 = 1,
    // Opens a verify code urlin the browser
    Verify2 = 2,
    Verify3 = 3,
);

#[derive(Debug, ShroomPacket, Default)]
pub struct LoginResultHeader {
    pub reg: RegStateId,
    // Unused variable
    pub unknown: u32,
}

shroom_enum_code!(
    LoginOpt,
    u8,
    EnableSecondPassword = 0,
    CheckSecondPassword = 1,
    NoSecondPassword1 = 2,
    NoSecondPassword2 = 3
);

/*
63, c7 => blocked for typing

*/
pub type BanReason = u8;

#[derive(Debug, ShroomPacket)]
pub struct HardwareInfo {
    mac: String,
    hdd_serial_no: String,
}

#[derive(Debug, ShroomPacket)]
pub struct SSOErrorLog {
    unknown1: u8,
    auth_reply_code: u32,
}

shroom_enum_code!(
    CheckPinResp,
    u8,
    Accepted = 0,
    RegisterNewPin = 1,
    InvalidPin = 2,
    SystemError = 3,
    EnterPin = 4,
    //TODO valid?
    ResetLogin = 7
);
with_opcode!(CheckPinResp, SendOpcodes::CheckPinCodeResult);

#[derive(ShroomPacket, Debug)]
pub struct UpdatePinResp {
    pub success: bool,
}
with_opcode!(UpdatePinResp, SendOpcodes::UpdatePinCodeResult);

#[derive(Debug, ShroomPacket)]
pub struct CheckPinData {
    //TODO: set to true in CheckPasswordResult and OnSelectWorldResult why?
    /// Somehow set to one for CLogin::OnSelectWorldResult, elsewise 0
    pub is_on_select_world_result_request: bool,
    pub pin: String,
}

#[derive(Debug, ShroomPacket)]
pub struct CheckPinReq {
    pub pin: ShroomOption8<CheckPinData>,
}
with_opcode!(CheckPinReq, RecvOpcodes::CheckPinCode);

#[derive(Debug, ShroomPacket)]
pub struct UpdatePinReq {
    pub pin: ShroomOption8<String>,
}
with_opcode!(UpdatePinReq, RecvOpcodes::UpdatePinCode);

pub type AccountId = u32;

#[derive(ShroomPacket, Debug)]
pub struct CheckPasswordReq {
    pub id: String,
    pub pw: String,
    pub machine_id: MachineId,
    pub game_room_client: u32,
    pub start_mode: StartMode,
    // TODO: Always 0?
    pub u1: u8,
    pub u2: u8,
    pub partner_code: u32,
}
with_opcode!(CheckPasswordReq, RecvOpcodes::CheckPassword);

#[derive(ShroomPacket, Debug)]
pub struct BlockedIp {
    pub hdr: LoginResultHeader,
    pub reason: BanReason,
    pub ban_time: ShroomTime,
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct GradeCode: u8 {
        const NORMAL = 0;
        const ADMIN = 1 << 0;
        const FAKE_ADMIN = 1 << 5;
    }
}
mark_shroom_bitflags!(GradeCode);
    

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct SubGradeCode: u16 {
        const NORMAL = 0;
        const PRIMARY_TRACE = 1 << 0;
        const SECONDARY_TRACE = 1 << 1;
        const ADMIN = 1 << 2;
        const MOB_MOVEMENT_OBSERVER = 1 << 3;
        const MANAGER = 1 << 4;
        const SUPER_GM = 1 << 5;
        const GM = 1 << 6;
        const USER_GM = 1 << 7;
        const TESTER = 1 << 8;
    }
}
mark_shroom_bitflags!(SubGradeCode);

#[derive(ShroomPacket, Debug)]
pub struct AccountGrade {
    pub code: GradeCode,
    pub sub_code: SubGradeCode,
}

#[derive(ShroomPacket, Debug)]
pub struct AccountInfo {
    pub id: u32,
    pub gender: OptionGender,
    pub grade: AccountGrade,
    pub country_id: u8,
    pub name: String,
    pub purchase_exp: u8,
    pub chat_block_reason: u8,
    pub chat_block_date: ShroomTime,
    pub registration_date: ShroomTime,
    pub num_chars: u32,
}

impl AccountInfo {
    pub fn has_login_info(&self) -> bool {
        self.gender.is_set()
    }
}

#[derive(ShroomPacket, Debug)]
pub struct LoginInfo {
    pub skip_pin: bool,
    pub login_opt: LoginOpt,
    pub client_key: ClientKey,
}

#[derive(ShroomPacket, Debug)]
pub struct LoginAccountData {
    pub account_info: AccountInfo,
    #[pkt(check(field = "account_info", cond = "AccountInfo::has_login_info"))]
    pub login_info: CondOption<LoginInfo>,
}
#[derive(ShroomPacket, Debug)]
pub struct GuestAccountInfo {
    account_id: u32,
    gender: OptionGender,
    grade_code: u8,
    sub_grade_code: u8,
    is_test_acc: bool,
    country_id: u8,
    name: String,
    purchase_exp: u8,
    chat_block_reason: u8,
    chat_block_date: ShroomTime,
    registration_date: ShroomTime,
    num_chars: u32,
    guest_id_url: String,
}

#[derive(ShroomPacket, Debug)]
pub struct SuccessResult {
    //TODO reg has to be either 0/1 for having an acc
    // 2/3 is some yes/no dialog
    pub hdr: LoginResultHeader,
    pub account: LoginAccountData,
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum CheckPasswordResp {
    Success(SuccessResult) = 0,
    BlockedIp(BlockedIp) = 2,
    IdDeleted(LoginResultHeader) = 3,
    InvalidPassword(LoginResultHeader) = 4,
    InvalidUserName(LoginResultHeader) = 5,
    SystemError(LoginResultHeader) = 6,
    AlreadyLoggedIn(LoginResultHeader) = 7,
    UnableToLoginWithIp(LoginResultHeader) = 13,
    TOS(LoginResultHeader) = 23,
    Unknown(LoginResultHeader) = 255,
}
with_opcode!(CheckPasswordResp, SendOpcodes::CheckPasswordResult);

#[derive(Debug, ShroomPacket)]
pub struct SetGenderReq {
    pub gender: ShroomOption8<Gender>,
}
with_opcode!(SetGenderReq, RecvOpcodes::SetGender);

impl SetGenderReq {
    pub fn set(gender: Gender) -> Self {
        Self {
            gender: Some(gender).into(),
        }
    }

    pub fn cancel() -> Self {
        Self {
            gender: None.into(),
        }
    }
}

#[derive(Debug, ShroomPacket)]
pub struct SetGenderResp {
    pub gender: Gender,
    pub success: bool,
}
with_opcode!(SetGenderResp, SendOpcodes::SetAccountResult);

#[derive(ShroomPacket, Debug)]
pub struct ConfirmEULAReq {
    pub accepted: bool,
}
with_opcode!(ConfirmEULAReq, RecvOpcodes::ConfirmEULA);

#[derive(Debug, ShroomPacket)]
pub struct ConfirmEULAResp {
    pub success: bool,
}
with_opcode!(ConfirmEULAResp, SendOpcodes::ConfirmEULAResult);

pub type WorldId = u32;
pub type WorldId16 = u16;
pub type ChannelId = u16;

#[derive(ShroomPacket, Debug)]
pub struct LogoutWorldReq;
with_opcode!(LogoutWorldReq, RecvOpcodes::LogoutWorld);

#[derive(Debug, ShroomPacket)]
pub struct WorldInfoReq;
with_opcode!(WorldInfoReq, RecvOpcodes::WorldInfoRequest);

#[derive(Debug, ShroomPacket)]
pub struct WorldReq;
with_opcode!(WorldReq, RecvOpcodes::WorldRequest);

#[derive(Debug, ShroomPacket)]
pub struct WorldCheckUserLimitReq {
    pub world: WorldId16,
}
with_opcode!(WorldCheckUserLimitReq, RecvOpcodes::CheckUserLimit);

#[derive(Debug, ShroomPacket)]
pub struct WorldCheckUserLimitResp {
    pub over_user_limit: bool,
    //TODO seems like a bool
    pub populate_level: u8,
}
with_opcode!(WorldCheckUserLimitResp, SendOpcodes::CheckUserLimitResult);

#[derive(Debug, ShroomPacket)]
pub struct RecommendWorldMessage {
    pub world_id: WorldId,
    pub message: String,
}

#[derive(Debug, ShroomPacket)]
pub struct RecommendWorldMessageResp {
    pub messages: ShroomList8<RecommendWorldMessage>,
}
with_opcode!(
    RecommendWorldMessageResp,
    SendOpcodes::RecommendWorldMessage
);

#[derive(Debug, ShroomPacket)]
pub struct LastConnectedWorldResp {
    last_world: WorldId,
}
with_opcode!(LastConnectedWorldResp, SendOpcodes::LatestConnectedWorld);

#[derive(Debug, ShroomPacket)]
pub struct ChannelItem {
    pub name: String,
    pub user_number: u32,
    pub world_id: u8,
    pub id: u8,
    pub adult_channel: bool,
}

#[derive(Debug, ShroomPacket)]
pub struct WorldBalloon {
    pub pos: Vec2,
    pub message: String,
}

shroom_enum_code!(WorldState, u8, Normal = 0, Hot = 1, New = 2);

#[derive(Debug, ShroomPacket)]
pub struct WorldItem {
    pub name: String,
    pub state: WorldState,
    pub event_desc: String,
    pub event_exp: u16,
    pub event_drop_rate: u16,
    pub block_char_creation: bool,
    pub channels: ShroomList8<ChannelItem>,
    pub balloons: ShroomList16<WorldBalloon>,
}

fn has_world_info(world_id: &u8) -> bool {
    *world_id != 0xff
}

#[derive(Debug, ShroomPacket)]
pub struct WorldInfoResp {
    pub world_id: u8,
    #[pkt(check(field = "world_id", cond = "has_world_info"))]
    pub world: CondOption<WorldItem>,
}
with_opcode!(WorldInfoResp, SendOpcodes::WorldInformation);

impl WorldInfoResp {
    pub fn end() -> Self {
        Self {
            world_id: 0xff,
            world: CondOption(None),
        }
    }

    pub fn world(id: u8, world: WorldItem) -> Self {
        Self {
            world_id: id,
            world: CondOption(Some(world)),
        }
    }
}

#[derive(ShroomPacket, Debug)]
pub struct SelectWorldReq {
    pub start_mode: StartModeInfo,
    pub world_id: u8,
    pub channel_id: u8,
    // TODO: 2-5 of sa_data
    pub sa_data: u32,
}
with_opcode!(SelectWorldReq, RecvOpcodes::SelectWorld);
