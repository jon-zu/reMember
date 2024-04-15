pub mod chat;
pub mod drop;
pub mod field;
pub mod friend;
pub mod key_map;
pub mod life;
pub mod macros;
pub mod party;
pub mod script;
pub mod shop;
pub mod user;
pub mod quest;

use shroom_meta::{id::{job_id::JobId, CharacterId, NpcId}, twod::Vec2};
use shroom_pkt::{with_opcode, time::Ticks, ShroomList32, ShroomPacket, ShroomPacketEnum};

use crate::{
    login::MachineId,
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{ Gender, ServerSocketAddr},
};


use super::login::ClientKey;

#[derive(ShroomPacket, Debug)]
pub struct AntiMacroResultResp {
    pub u1: u8, //6
    pub u2: u8, // 2,
    pub u3: u8, // 1
    pub data: ShroomList32<u8>,
}
with_opcode!(AntiMacroResultResp, SendOpcodes::AntiMacroResult);

#[derive(ShroomPacket, Debug)]
pub struct CharacterInfoReq {
    pub ticks: Ticks,
    pub char_id: CharacterId,
    pub pet_info: bool,
}
with_opcode!(CharacterInfoReq, RecvOpcodes::UserCharacterInfoRequest);

#[derive(ShroomPacket, Debug)]
pub struct CharacterInfoResp {
    pub char_id: CharacterId,
    pub level: u8,
    pub job: JobId,
}
with_opcode!(CharacterInfoResp, SendOpcodes::CharacterInfo);

#[derive(ShroomPacket, Debug)]
pub struct MigrateInGameReq {
    pub char_id: CharacterId,
    pub machine_id: MachineId,
    pub is_gm: bool,
    pub unknown: bool,
    pub client_key: ClientKey,
}
with_opcode!(MigrateInGameReq, RecvOpcodes::MigrateIn);


#[derive(ShroomPacket, Debug)]
pub struct MigrateInGameTokenReq {
    pub char_id: CharacterId,
    pub machine_id: MachineId,
    pub is_gm: bool,
    pub unknown: bool,
    pub client_key: ClientKey,
    pub token: [u8; 32],
}

#[derive(ShroomPacket, Debug)]
pub struct TransferChannelReq {
    pub channel_id: u8,
    pub ticks: Ticks,
}
with_opcode!(TransferChannelReq, RecvOpcodes::UserTransferChannelRequest);

#[derive(ShroomPacket, Debug)]
pub struct MigrateCommandResp {
    pub unknown: bool, //always true?
    pub addr: ServerSocketAddr,
}
with_opcode!(MigrateCommandResp, SendOpcodes::MigrateCommand);

#[derive(ShroomPacket, Debug)]
pub struct UpdateGMBoardReq {
    board_id: u32,
}
with_opcode!(UpdateGMBoardReq, RecvOpcodes::UpdateGMBoard);

#[derive(ShroomPacket, Debug)]
pub struct UserPortalScriptReq {
    field_key: u8,
    portal_name: String,
    pos: Vec2,
}
with_opcode!(UserPortalScriptReq, RecvOpcodes::UserPortalScriptRequest);

#[derive(ShroomPacket, Debug)]
pub struct ResetNLCPQ;
//TODO opcode name??
with_opcode!(ResetNLCPQ, RecvOpcodes::RequireFieldObstacleStatus);

#[derive(ShroomPacket, Debug)]
pub struct CtxSetGenderResp {
    pub gender: Gender,
}
with_opcode!(CtxSetGenderResp, SendOpcodes::SetGender);

#[derive(ShroomPacket, Debug)]
pub struct ClaimSvrStatusChangedResp {
    pub connected: bool,
}
with_opcode!(
    ClaimSvrStatusChangedResp,
    SendOpcodes::ClaimSvrStatusChanged
);

#[derive(ShroomPacket, Debug)]
pub struct ServerMessage {
    pub flag: bool,
    pub msg: String,
}

#[derive(ShroomPacket, Debug)]
pub struct NoticeMessage {
    pub msg: String,
    pub unknown: i32,
}

#[derive(ShroomPacket, Debug)]
pub struct UtilDlgExMessage {
    pub msg: String,
    pub npc: NpcId,
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum BroadcastMessageResp {
    Notice(String) = 0,
    Alert(String) = 1,
    Channel(String) = 2,
    World(String) = 3,
    ServerMessage(ServerMessage) = 4,
    PinkMessage(String) = 5,
    NoticeWithoutPrefix(NoticeMessage) = 6,
    UtilDlgEx(UtilDlgExMessage) = 7,
    ItemSpeaker(()) = 8,
    SpeakerBridge(()) = 9,
    ArtSpeakerWorld(()) = 10,
    BlowWeather(()) = 11,
    GachaponAnnounce(()) = 12,
    GachaponAnnounceOpen(()) = 13,
    GachaponAnnounceClose(()) = 14,
    UListClip(()) = 15,
    FreeMarketClip(()) = 16,
    DestroyShop(()) = 17,
    CashShopAd(()) = 18,
    HeartSpeaker(()) = 19,
    SkillSpeaker(()) = 20,
}
with_opcode!(BroadcastMessageResp, SendOpcodes::BroadcastMsg);
