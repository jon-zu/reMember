use num_enum::{IntoPrimitive, TryFromPrimitive};
use shroom_meta::{id::{CharacterId, ObjectId, SkillId}, twod::Rect32};
use shroom_pkt::{mark_shroom_enum, with_opcode, ShroomPacket};

use crate::send_opcodes::SendOpcodes;

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum AffectedAreaType {
    MobSkill = 0,
    UserSkill = 1,
    Smoke = 2,
    Buff = 3,
    BlessedMist = 4,
}
mark_shroom_enum!(AffectedAreaType);

#[derive(ShroomPacket, Debug)]
pub struct AffectedAreaCreateResp {
    pub id: ObjectId,
    pub ty: AffectedAreaType,
    pub owner_id: CharacterId,
    pub skill_id: SkillId,
    pub skill_level: u8,
    pub start_delay: u16, // TODO this is weird, it's multiplied by 100 resulting in ms
    pub area: Rect32,
    pub elem_attr: u32,
    pub phase: u32,
}
with_opcode!(AffectedAreaCreateResp, SendOpcodes::AffectedAreaCreated);

#[derive(ShroomPacket, Debug)]
pub struct AffectedAreaRemoveResp {
    pub id: ObjectId,
}
with_opcode!(AffectedAreaRemoveResp, SendOpcodes::AffectedAreaRemoved);
