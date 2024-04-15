use num_enum::{IntoPrimitive, TryFromPrimitive};
use shroom_meta::{
    id::{CharacterId, FootholdId, ObjectId, SkillId},
    twod::Vec2,
};
use shroom_pkt::{
    mark_shroom_enum, shroom_enum_code, time::ClientTime, with_opcode, DecodePacket, OptionTail,
    ShroomEncodePacket, ShroomList8, ShroomOption8, ShroomPacket,
};

use crate::{
    game::user::{ForeActionDir, SingleAttackTargetInfo, ValWithCrc},
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{char::AvatarData, movement::MovePath},
};

pub type SummonId = ObjectId;

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonEnterType {
    Default = 0,
    CreateSummon = 1,
    ReregisterSummon = 2,
}
mark_shroom_enum!(SummonEnterType);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonLeaveType {
    Update = 0,
    Die = 1,
    Mystery = 2,
    Default = 3,
    LeaveField = 4,
    SelfDestruct = 5,
    Gabiota = 6,
    EnterForbidenMap = 7,
    EnterEventField = 8,
    UserDead = 9,
    OnRemove = 10,
    TeslaCoilError = 11,
    NotAbleMultiple = 12,
    NotSelfDestruct = 13,
    SummonCountOver = 14,
}
mark_shroom_enum!(SummonLeaveType);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonAssistType {
    None = 0,
    Attack = 1,
    Heal = 2,
    AttackEx = 3,
    AttackEx2 = 4,
    ManualAttack = 5,
}
mark_shroom_enum!(SummonAssistType);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonMoveAbility {
    NoMove = 0,
    Follow = 1,
    WalkRandom = 2,
    Jump = 3,
    CircleFollow = 4,
    FlyRandom = 5,
    Escort = 6,
}
mark_shroom_enum!(SummonMoveAbility);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonMoveAction {
    Walk = 0,
    Move = 1,
    Stand = 2,
    Jump = 3,
    Alert = 4,
    Prone = 5,
    Fly1 = 6,
    Ladder = 7,
    Rope = 8,
    Dead = 9,
    Sit = 10,
    Stand0 = 11,
    Hungry = 12,
    Rest0 = 13,
    Rest1 = 14,
    Hang = 15,
    Chase = 16,
    Fly2 = 17,
    Fly2Move = 18,
    Dash2 = 19,
    RocketBooster = 20,
    TeslaCoilTriangle = 21,
    NoMove = 22,
}
mark_shroom_enum!(SummonMoveAction);

#[derive(Debug, ShroomPacket)]
pub struct SummonTeslaCoilInitData {
    //TODO first u8 is coil state
    pub state: ShroomOption8<[Vec2; 3]>,
}

#[derive(Debug, ShroomPacket)]
pub struct SummonInitData {
    pub pos: Vec2,
    pub move_action: SummonMoveAction,
    pub cur_fh: FootholdId,
    pub move_ability: SummonMoveAbility,
    pub assist_type: SummonAssistType,
    pub enter_type: SummonEnterType,
    pub avatar: ShroomOption8<AvatarData>,
    // TODO if skill id is tesla coil then there's extra data
}

#[derive(Debug, ShroomPacket)]
pub struct SummonCreateResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub skill_id: SkillId,
    pub char_level: u8,
    pub skill_level: u8,
    pub init: SummonInitData,
}
with_opcode!(SummonCreateResp, SendOpcodes::SummonedEnterField);

#[derive(Debug, ShroomPacket)]
pub struct SummonDeleteResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub leave: SummonLeaveType,
}
with_opcode!(SummonDeleteResp, SendOpcodes::SummonedLeaveField);

#[derive(Debug, ShroomPacket)]
pub struct SummonMoveResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub path: MovePath,
}
with_opcode!(SummonMoveResp, SendOpcodes::SummonedMove);

#[derive(Debug, ShroomPacket)]
pub struct SummonAttackInfo {
    pub target_mob: ObjectId,
    pub hit_action: u8,
    pub dmg: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct SummonAttackResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub fore_action: ForeActionDir,
    pub attack_info: ShroomList8<SummonAttackInfo>,
    pub u: u8, // TODO
}
with_opcode!(SummonAttackResp, SendOpcodes::SummonedAttack);

#[derive(Debug, ShroomPacket)]
pub struct SummonSkillResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub attack_action: bool, // TODO: this is used as bool 0x7F
}
with_opcode!(SummonSkillResp, SendOpcodes::SummonedSkill);

shroom_enum_code!(
    BeholderBuffIx,
    u8,
    EPDD = 0,
    EMDD = 1,
    ACC = 2,
    EVA = 3,
    EPAD = 4
);

#[derive(Debug, ShroomPacket)]
pub struct SummonSkillReq {
    pub summon_id: SummonId,
    pub skill_id: SkillId,
    pub fore_action: ForeActionDir,
    pub buff_ix: OptionTail<BeholderBuffIx>,
}

with_opcode!(SummonSkillReq, RecvOpcodes::SummonedSkill);

#[derive(Debug, ShroomPacket)]
pub struct SummonHitResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub atk_index: i8,
    pub damage: u32,
    // The following is only encdoed if atk_index > -i
    pub mob_tmpl_id: u32,
    pub left: bool,
}
with_opcode!(SummonHitResp, SendOpcodes::SummonedHit);

#[derive(Debug, ShroomEncodePacket)]
pub struct SummonAttackReq {
    pub summon_id: SummonId,
    pub dr0: u32,
    pub dr1: u32,
    pub time: ClientTime,
    pub dr2: u32,
    pub dr3: u32,
    pub action: ForeActionDir,
    pub rand: ValWithCrc,
    pub atk_count: u8, // Always 1 for mob attack
    pub char_pos: Vec2,
    pub summon_pos: Vec2,
    pub u2: u32, // Always 0 for mob attack
    pub targets: Vec<SingleAttackTargetInfo>,
    pub skill_crc: u32,
}

impl<'de> DecodePacket<'de> for SummonAttackReq {
    fn decode(pr: &mut shroom_pkt::PacketReader<'de>) -> shroom_pkt::PacketResult<Self> {
        let summon_id = ObjectId(u32::decode(pr)?);
        let dr0 = u32::decode(pr)?;
        let dr1 = u32::decode(pr)?;
        let time = ClientTime::decode(pr)?;
        let dr2 = u32::decode(pr)?;
        let dr3 = u32::decode(pr)?;
        let action = ForeActionDir::decode(pr)?;
        let rand = ValWithCrc::decode(pr)?;
        let atk_count = u8::decode(pr)?;
        let char_pos = Vec2::decode(pr)?;
        let summon_pos = Vec2::decode(pr)?;
        let u2 = u32::decode(pr)?;
        let targets = SingleAttackTargetInfo::decode_n(pr, atk_count as usize)?;
        let skill_crc = u32::decode(pr)?;
        Ok(Self {
            summon_id,
            dr0,
            dr1,
            time,
            dr2,
            dr3,
            action,
            rand,
            atk_count,
            char_pos,
            summon_pos,
            u2,
            targets,
            skill_crc,
        })
    }
}

with_opcode!(SummonAttackReq, RecvOpcodes::SummonedAttack);

#[cfg(test)]
mod tests {
    use super::*;
    use hexlit::hex;
    use shroom_pkt::{DecodePacket, PacketReader};

    #[test]
    fn summon_attack() {
        let pkt = hex!("0e 00 00 00 ff ff ff ff 7f f8 ff ff 52 77 00 00 4f fb ff ff db 3c f2 85 84 33 3a c5 5c 03 2e e6 fc 01 a5 02 71 04 be 02 d4 03 00 00 00 00 0c 00 00 00 35 f8 7c 00 07 01 00 05 5d 02 6b 04 5d 02 6b 04 64 00 00 00 00 00 f3 e8 c5 6c");
        let mut pr = PacketReader::new(&pkt);
        let _ = SummonAttackReq::decode_complete(&mut pr).unwrap();
    }
}
