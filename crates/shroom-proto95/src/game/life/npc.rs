use shroom_meta::{id::{FootholdId, NpcId, ObjectId}, twod::{Range2, Vec2}};
use shroom_pkt::{
    OptionTail, with_opcode, ShroomList8, ShroomPacket
};

use crate::{
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{char::AvatarData, movement::MovePath},
};


#[derive(ShroomPacket, Debug)]
pub struct NpcPoolPacket<T> {
    pub id: ObjectId,
    pub data: T,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcData {
    pub template_id: NpcId,
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub range_horz: Range2,
    pub enabled: bool,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcEnterFieldResp {
    pub id: ObjectId,
    pub npc: NpcData
}
with_opcode!(NpcEnterFieldResp, SendOpcodes::NpcEnterField);

#[derive(ShroomPacket, Debug)]
pub struct NpcLeaveFieldResp {
    pub id: ObjectId,
}
with_opcode!(NpcLeaveFieldResp, SendOpcodes::NpcLeaveField);

#[derive(ShroomPacket, Debug)]
pub struct NpcImitateData {
    pub tmpl_id: NpcId,
    pub name: String,
    pub avatar_look: AvatarData,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcImitateDataResp {
    pub data: ShroomList8<NpcImitateData>,
}
with_opcode!(NpcImitateDataResp, SendOpcodes::ImitatedNPCData);

#[derive(ShroomPacket, Debug)]
pub struct NpcUpdateLimitedDisableInfoResp {
    pub data: ShroomList8<ObjectId>,
}
with_opcode!(
    NpcUpdateLimitedDisableInfoResp,
    SendOpcodes::LimitedNPCDisableInfo
);


#[derive(ShroomPacket, Debug)]
pub struct NpcChangeControllerResp {
    pub local: bool,
    pub id: ObjectId,
    // TODO: only required if local is true and npc does not exist
    // yet in the controllers pool
    pub npc: OptionTail<NpcData>
}
with_opcode!(NpcChangeControllerResp, SendOpcodes::NpcChangeController);

#[derive(ShroomPacket, Debug)]
pub struct ScriptInfo {
    pub script: String,
    pub start_date: u32,
    pub end_date: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct ModScript {
    pub template_id: u32,
    pub script: ScriptInfo,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcSetScriptResp {
    pub scripts: ShroomList8<ModScript>,
}
with_opcode!(NpcSetScriptResp, SendOpcodes::NpcSetScript);

#[derive(ShroomPacket, Debug)]
pub struct NpcMove {
    pub action: u8,
    pub chat: u8, //TODO correct?
    pub move_path: OptionTail<MovePath>,
}
pub type NpcMoveResp = NpcPoolPacket<NpcMove>;
with_opcode!(NpcMoveResp, SendOpcodes::NpcMove);

#[derive(ShroomPacket, Debug)]
pub struct NpcMoveReq {
    pub id: ObjectId,
    pub action: u8,
    pub chat_idx: u8,
    pub move_path: OptionTail<MovePath>,
}
with_opcode!(NpcMoveReq, RecvOpcodes::NpcMove);

#[derive(ShroomPacket, Debug)]
pub struct NpcUpdateLimitedInfo {
    pub enabled: bool,
}
pub type NpcUpdateLimitedInfoResp = NpcPoolPacket<NpcUpdateLimitedInfo>;
with_opcode!(NpcUpdateLimitedInfoResp, SendOpcodes::NpcUpdateLimitedInfo);

#[derive(ShroomPacket, Debug)]
pub struct NpcSetSpecialAction {
    pub action: String,
}
pub type NpcSetSpecialActionResp = NpcPoolPacket<NpcSetSpecialAction>;
with_opcode!(NpcSetSpecialActionResp, SendOpcodes::NpcSpecialAction);

#[derive(ShroomPacket, Debug)]
pub struct UserSelectNpcReq {
    pub id: NpcId,
    pub pos: Vec2,
}
with_opcode!(UserSelectNpcReq, RecvOpcodes::UserSelectNpc);
