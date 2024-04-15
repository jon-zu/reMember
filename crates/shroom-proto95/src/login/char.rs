use shroom_pkt::{
    shroom_enum_code, with_opcode, ShroomList8, ShroomOption8, ShroomPacket, ShroomPacketEnum,
};

use shroom_meta::id::{
    job_id::{JobGroup, SubJob}, CharacterId, FaceId, HairId, ItemId
};

use crate::{
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{
        char::{AvatarData, CharStat},
        Gender, ServerSocketAddr,
    },
};

use super::{
    AccountId, HardwareInfo, LoginOpt, MachineId, StartMode, StartModeInfo, WorldId,
};

#[derive(ShroomPacket, Debug)]
pub struct ViewAllCharFlagSet {
    pub set: bool,
}
with_opcode!(ViewAllCharFlagSet, RecvOpcodes::VACFlagSet);

#[derive(ShroomPacket, Debug)]
pub struct MigrateStageInfo {
    pub socket_addr: ServerSocketAddr,
    pub char_id: CharacterId,
    pub premium: bool,
    pub premium_arg: u32,
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum SelectCharResult {
    Success(MigrateStageInfo) = 0, //TODO add the rest
}

#[derive(ShroomPacket, Debug)]
pub struct SelectCharResp {
    //TODO: use enums
    pub error_code: u8,
    //TODO add all options
    pub result: SelectCharResult,
}
with_opcode!(SelectCharResp, SendOpcodes::SelectCharacterResult);

//TODO how does this work? must use prestored world i guess
#[derive(ShroomPacket, Debug)]
pub struct ViewAllCharReq {
    start_mode: StartModeInfo,
}
with_opcode!(ViewAllCharReq, RecvOpcodes::ViewAllChar);

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum ViewAllCharResp {
    Success(ViewAllCharList) = 0,
    Prepare(ViewAllCharPrepare) = 1,
    Reset(()) = 2,
    Error3(ViewAllCharCustomError) = 3,
    Error4(()) = 4,
    Error5(()) = 5,
    Error6(ViewAllCharCustomError) = 6,
    Error7(ViewAllCharCustomError) = 7,
}
with_opcode!(ViewAllCharResp, SendOpcodes::ViewAllCharResult);

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum SelectWorldResp {
    Success(SelectWorldCharList) = 0,
    Err(()) = 1, //TODO add more errors
}
with_opcode!(SelectWorldResp, SendOpcodes::SelectWorldResult);

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum CreateCharResp {
    Success(Box<ViewChar>) = 0,
    Timeout(()) = 0xa,
    SystemError(()) = 0x1a,
    InvalidCharName(()) = 0x1e, //TODO more errors?
}
with_opcode!(CreateCharResp, SendOpcodes::CreateNewCharacterResult);

shroom_enum_code!(
    SelectCharResultCode,
    u8,
    Success = 0,
    DBFail = 6,
    UnknownErr = 9,
    Timeout = 0xA,
    InvalidBirthday = 0x12,
    InvalidPic = 0x14,
    ErrGuildMaster = 0x16,
    ErrPendingWedding = 0x18,
    ErrPendingWorldTransfer = 0x1A,
    ErrHasFamily = 0x1D
);

shroom_enum_code!(
    DeleteCharResult,
    u8,
    Success = 0,
    DBFail = 6,
    UnknownErr = 9,
    Timeout = 0xA,
    InvalidBirthday = 0x12,
    InvalidPic = 0x14,
    ErrGuildMaster = 0x16,
    ErrPendingWedding = 0x18,
    ErrPendingWorldTransfer = 0x1A,
    ErrHasFamily = 0x1D
);

#[derive(ShroomPacket, Debug)]
pub struct DeleteCharResp {
    pub char_id: CharacterId,
    pub result: DeleteCharResult,
}
with_opcode!(DeleteCharResp, SendOpcodes::DeleteCharacterResult);

#[derive(ShroomPacket, Debug)]
pub struct DeleteCharReq {
    pub pic: String,
    pub char_id: CharacterId,
}
with_opcode!(DeleteCharReq, RecvOpcodes::DeleteCharacter);

#[derive(ShroomPacket, Debug)]
pub struct EnableSecondPasswordResp {
    pub success: bool,
    // TODO <= 0x17, some error code like others
    pub result: u8,
}
with_opcode!(EnableSecondPasswordResp, SendOpcodes::EnableSPWResult);

#[derive(ShroomPacket, Debug)]
pub struct CheckSecondPasswordResp {
    pub u1: u8, // Todo: Unused code??
}
with_opcode!(CheckSecondPasswordResp, SendOpcodes::CheckSPWResult);

#[derive(Debug, ShroomPacket)]
pub struct ExtraCharInfoResp {
    pub acc_id: AccountId,
    pub no_extra_char: bool,
}
with_opcode!(ExtraCharInfoResp, SendOpcodes::CheckExtraCharInfoResult);

#[derive(ShroomPacket, Debug)]
pub struct ViewChar {
    pub stats: CharStat,
    pub avatar_data: AvatarData,
}

#[derive(ShroomPacket, Debug)]
pub struct CharRankInfo {
    pub world_rank: u32,
    pub rank_move: u32, /* gap */
    pub job_rank: u32,
    pub job_rank_mode: u32, /* gap */
}

#[derive(ShroomPacket, Debug)]
pub struct ViewCharWithRank {
    pub view_char: ViewChar,
    pub u1: u8, //VAC?
    pub rank_info: ShroomOption8<CharRankInfo>,
}

#[derive(ShroomPacket, Debug)]
pub struct SelectWorldCharList {
    pub characters: ShroomList8<ViewCharWithRank>,
    pub login_opt: LoginOpt,
    pub slot_count: u32,
    pub buy_char_count: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct ViewAllCharList {
    pub world_id: u8,
    pub characters: ShroomList8<ViewChar>,
    pub login_opt: LoginOpt,
}

#[derive(ShroomPacket, Debug)]
pub struct ViewAllCharCustomError {
    pub msg: ShroomOption8<String>,
}

#[derive(ShroomPacket, Debug)]
pub struct ViewAllCharPrepare {
    pub count_related_servers: u32,
    pub count_chars: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct CharacterRankData {
    pub world_rank: u32,
    pub world_rank_gap: u32,
    pub job_rank: u32,
    pub job_rank_gap: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct ViewExtraInfo {
    pub hardware_id: String,
    pub machine_id: MachineId,
    pub game_room_client: u32,
    pub start_mode: StartMode,
}

#[derive(ShroomPacket, Debug)]
pub struct ViewAllCharRequest {
    extra_info: ShroomOption8<ViewExtraInfo>,
}

#[derive(ShroomPacket, Debug)]
pub struct SelectCharEnablePicReq {
    pub unknown1: u8, //Always 1 ?
    pub char_id: CharacterId,
    pub hw_info: HardwareInfo,
    pub pic: String,
}
with_opcode!(SelectCharEnablePicReq, RecvOpcodes::EnableSPWRequest);

#[derive(ShroomPacket, Debug)]
pub struct SelectCharCheckPicReq {
    pub pic: String,
    pub char_id: CharacterId,
    pub hw_info: HardwareInfo,
}
with_opcode!(SelectCharCheckPicReq, RecvOpcodes::CheckSPWRequest);

#[derive(ShroomPacket, Debug)]
pub struct SelectCharReq {
    pub char_id: CharacterId,
    pub hw_info: HardwareInfo,
}
with_opcode!(SelectCharReq, RecvOpcodes::SelectCharacter);

// Login Opt 0  == Enable Second Password
#[derive(ShroomPacket, Debug)]
pub struct SelectCharEnablePicVac {
    pub unknown1: u8, //Always 1 ?
    pub char_id: CharacterId,
    pub world_id: WorldId,
    pub hw_info: HardwareInfo,
    pub pic: String,
}
with_opcode!(SelectCharEnablePicVac, RecvOpcodes::EnableSPWRequestByVAC);

// Login Opt 1  == Check Second Password
#[derive(ShroomPacket, Debug)]
pub struct SelectCharCheckPicVac {
    pub pic: String,
    pub char_id: CharacterId,
    pub world_id: WorldId,
    pub hw_info: HardwareInfo,
}
with_opcode!(SelectCharCheckPicVac, RecvOpcodes::CheckSPWRequestByVAC);

// Login Opt 2/3
#[derive(ShroomPacket, Debug)]
pub struct SelectCharReqVac {
    char_id: CharacterId,
    world_id: WorldId,
    hw_info: HardwareInfo,
}
with_opcode!(SelectCharReqVac, RecvOpcodes::SelectCharacterByVAC);

#[derive(ShroomPacket, Debug)]
pub struct CharStarterSet {
    pub face: FaceId,
    pub hair: HairId,
    pub hair_color: u32,
    pub skin_color: u32,
    pub top: ItemId,
    pub bottom: ItemId,
    pub shoes: ItemId,
    pub weapon: ItemId,
}

#[derive(ShroomPacket, Debug)]
pub struct CreateCharReq {
    pub name: String,
    pub job: JobGroup,
    pub sub_job: SubJob,
    pub starter_set: CharStarterSet,
    pub gender: Gender,
}
with_opcode!(CreateCharReq, RecvOpcodes::CreateNewCharacter);

#[derive(ShroomPacket, Debug)]
pub struct CreateCharSale {
    pub name: String,
    pub job: JobGroup,
    pub sale_job: u32,
    pub starter_set: CharStarterSet,
}
with_opcode!(CreateCharSale, RecvOpcodes::CreateNewCharacterInCS);

#[derive(ShroomPacket, Debug)]
pub struct CheckDuplicateIDReq {
    pub name: String,
}
with_opcode!(CheckDuplicateIDReq, RecvOpcodes::CheckDuplicatedID);

shroom_enum_code!(
    CheckDuplicateIDResult,
    u8,
    Success = 0,
    // TODO: mapped to 5
    Error1 = 1,
    // map to 10
    Error2 = 2,
    // map to 18 or well every code aside from 0,1,2
    Error3 = 3
);

#[derive(ShroomPacket, Debug)]
pub struct CheckDuplicateIDResp {
    pub name: String,
    pub result: CheckDuplicateIDResult,
}
with_opcode!(CheckDuplicateIDResp, SendOpcodes::CheckDuplicatedIDResult);
