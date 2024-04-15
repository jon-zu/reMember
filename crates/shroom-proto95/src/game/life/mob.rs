use num_enum::{IntoPrimitive, TryFromPrimitive};
use shroom_meta::{
    id::{BuffId, CharacterId, FootholdId, ItemId, MobId, ObjectId}, twod::{TagPoint, Vec2}
};
use shroom_pkt::{
    mark_shroom_bitflags, mark_shroom_enum, packet_try_wrap, packet_wrap, partial::PartialFlag,
    partial_data, shroom_enum_code, with_opcode, CondOption, HasOpCode, OptionTail,
    ShroomDurationMs16, ShroomDurationMs32, ShroomEncodePacket, ShroomList32, ShroomOption8,
    ShroomPacket, ShroomPacketEnum,
};

use crate::{
    game::field::CrcSeed,
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::movement::{MovePassivePath, MovePath},
};


shroom_enum_code!(
    MobAction,
    u8,
    MOVE = 0x0,
    STAND = 0x1,
    JUMP = 0x2,
    FLY = 0x3,
    ROPE = 0x4,
    REGEN = 0x5,
    BOMB = 0x6,
    HIT1 = 0x7,
    HIT2 = 0x8,
    HITF = 0x9,
    DIE1 = 0xA,
    DIE2 = 0xB,
    DIEF = 0xC,
    ATTACK1 = 0xD, // 13
    ATTACK2 = 0xE,
    ATTACK3 = 0xF,
    ATTACK4 = 0x10,
    ATTACK5 = 0x11,
    ATTACK6 = 0x12,
    ATTACK7 = 0x13,
    ATTACK8 = 0x14,
    ATTACKF = 0x15, // 21
    SKILL1 = 0x16,  // 22
    SKILL2 = 0x17,
    SKILL3 = 0x18,
    SKILL4 = 0x19,
    SKILL5 = 0x1A,
    SKILL6 = 0x1B,
    SKILL7 = 0x1C,
    SKILL8 = 0x1D,
    SKILL9 = 0x1E, // 30
    SKILL10 = 0x1F,
    SKILL11 = 0x20,
    SKILL12 = 0x21,
    SKILL13 = 0x22,
    SKILL14 = 0x23,
    SKILL15 = 0x24,
    SKILL16 = 0x25,
    SKILLF = 0x26, // 38
    CHASE = 0x27,
    MISS = 0x28,
    SAY = 0x29,
    EYE = 0x2A,
    NO = 0x2B,
    NONE = 0x7F
);

impl MobAction {
    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }
    pub fn from_skill(id: u8) -> Self {
        match id {
            0 => Self::SKILL1,
            1 => Self::SKILL2,
            2 => Self::SKILL3,
            3 => Self::SKILL4,
            4 => Self::SKILL5,
            5 => Self::SKILL6,
            6 => Self::SKILL7,
            7 => Self::SKILL8,
            8 => Self::SKILL9,
            9 => Self::SKILL10,
            10 => Self::SKILL11,
            11 => Self::SKILL12,
            12 => Self::SKILL13,
            13 => Self::SKILL14,
            14 => Self::SKILL15,
            15 => Self::SKILL16,
            //TODO handle
            _ => Self::SKILLF,
        }
    }

    pub fn skill_id(&self) -> Option<u8> {
        Some(match self {
            Self::SKILL1 => 0,
            Self::SKILL2 => 1,
            Self::SKILL3 => 2,
            Self::SKILL4 => 3,
            Self::SKILL5 => 4,
            Self::SKILL6 => 5,
            Self::SKILL7 => 6,
            Self::SKILL8 => 7,
            Self::SKILL9 => 8,
            Self::SKILL10 => 9,
            Self::SKILL11 => 10,
            Self::SKILL12 => 11,
            Self::SKILL13 => 12,
            Self::SKILL14 => 13,
            Self::SKILL15 => 14,
            Self::SKILL16 => 15,
            _ => return None,
        })
    }

    pub fn is_skill(&self) -> bool {
        matches!(
            self,
            Self::SKILL1
                | Self::SKILL2
                | Self::SKILL3
                | Self::SKILL4
                | Self::SKILL5
                | Self::SKILL6
                | Self::SKILL7
                | Self::SKILL8
                | Self::SKILL9
                | Self::SKILL10
                | Self::SKILL11
                | Self::SKILL12
                | Self::SKILL13
                | Self::SKILL14
                | Self::SKILL15
                | Self::SKILL16
                | Self::SKILLF
        )
    }

    pub fn from_attack(id: u8) -> Self {
        match id {
            0 => Self::ATTACK1,
            1 => Self::ATTACK2,
            2 => Self::ATTACK3,
            3 => Self::ATTACK4,
            4 => Self::ATTACK5,
            5 => Self::ATTACK6,
            6 => Self::ATTACK7,
            7 => Self::ATTACK8,
            //TODO handle
            _ => Self::ATTACKF,
        }
    }

    pub fn attack_id(&self) -> Option<u8> {
        Some(match self {
            Self::ATTACK1 => 0,
            Self::ATTACK2 => 1,
            Self::ATTACK3 => 2,
            Self::ATTACK4 => 3,
            Self::ATTACK5 => 4,
            Self::ATTACK6 => 5,
            Self::ATTACK7 => 6,
            Self::ATTACK8 => 7,
            _ => return None,
        })
    }

    pub fn is_attack(&self) -> bool {
        matches!(
            self,
            Self::ATTACK1
                | Self::ATTACK2
                | Self::ATTACK3
                | Self::ATTACK4
                | Self::ATTACK5
                | Self::ATTACK6
                | Self::ATTACK7
                | Self::ATTACK8
                | Self::ATTACKF
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ActionDir {
    pub left: bool,
    pub action: MobAction,
}

impl From<ActionDir> for u8 {
    fn from(val: ActionDir) -> Self {
        val.left as u8 | (val.action as u8) << 1
    }
}

impl TryFrom<u8> for ActionDir {
    type Error = shroom_pkt::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let left = value & 1 != 0;
        let action = MobAction::try_from(value >> 1)
            .map_err(|_| shroom_pkt::Error::InvalidEnumPrimitive(value as u32))?;
        Ok(Self { left, action })
    }
}

packet_try_wrap!(ActionDir<>, u8, u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct MobSkillData {
    pub id: u8,
    pub lvl: u8,
    pub delay: u16,
}

impl From<u32> for MobSkillData {
    fn from(v: u32) -> Self {
        Self {
            id: (v & 0xFF) as u8,
            lvl: ((v >> 8) & 0xFF) as u8,
            delay: ((v >> 16) & 0xFFFF) as u16,
        }
    }
}

impl From<MobSkillData> for u32 {
    fn from(v: MobSkillData) -> Self {
        v.id as u32 | ((v.lvl as u32) << 8) | ((v.delay as u32) << 16)
    }
}

packet_wrap!(MobSkillData<>, u32, u32);

#[derive(ShroomPacket, Debug, Clone)]
pub struct TempStatValue {
    pub value: i16,
    pub reason: BuffId,
    pub dur: ShroomDurationMs16,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct BurnedInfo {
    pub char_id: CharacterId,
    pub skill_id: BuffId,
    pub n_dmg: u32,
    pub interval: ShroomDurationMs32,
    pub end: ShroomDurationMs32,
    pub dot_count: u32,
}

partial_data!(
    MobTemporaryStat,
    MobTemporaryStatFlags,
    u128,
    derive(Debug),
    Pad(TempStatValue) => 1 << 0,
    Pdr(TempStatValue) => 1 << 1,
    Mad(TempStatValue) => 1 << 2,
    Mdr(TempStatValue) => 1 << 3,
    Acc(TempStatValue) => 1 << 4,
    Eva(TempStatValue) => 1 << 5,
    Speed(TempStatValue) => 1 << 6,
    Stun(TempStatValue) => 1 << 7,
    Freeze(TempStatValue) => 1 << 8,
    Poison(TempStatValue) => 1 << 9,
    Seal(TempStatValue) => 1 << 10,
    Darkness(TempStatValue) => 1 << 11,
    PowerUp(TempStatValue) => 1 << 12,
    MagicUp(TempStatValue) => 1 << 13,
    PGuardUp(TempStatValue) => 1 << 14,
    MGuardUp(TempStatValue) => 1 << 15,
    Doom(TempStatValue) => 1 << 16,
    Web(TempStatValue) => 1 << 17,
    PImmune(TempStatValue) => 1 << 18,
    MImmune(TempStatValue) => 1 << 19,
    HardSkin(TempStatValue) => 1 << 21,
    Ambush(TempStatValue) => 1 << 22,
    Venom(TempStatValue) => 1 << 24,
    Blind(TempStatValue) => 1 << 25,
    SealSkill(TempStatValue) => 1 << 26,
    Dazzle(TempStatValue) => 1 << 28,
    PCounter(TempStatValue) => 1 << 29,
    MCounter(TempStatValue) => 1 << 30,
    RiseByToss(TempStatValue) => 1 << 32,
    BodyPressure(TempStatValue) => 1 << 33,
    Weakness(TempStatValue) => 1 << 34,
    TimeBomb(TempStatValue) => 1 << 35,
    Showdown(TempStatValue) => 1 << 20,
    MagicCrash(TempStatValue) => 1 << 36,
    DamagedElemAttr(TempStatValue) => 1 << 23,
    HealByDamage(TempStatValue) => 1 << 37,
    Burned(ShroomList32<BurnedInfo>) => 1 << 27,
    //TODO this is wrong, only there for the meta crate to have the flag
    Disable(TempStatValue) => 1 << 31,
);

#[derive(ShroomPacket, Debug)]
pub struct MobTemporaryStatTail {
    // If PCounter is set
    pub w_pcounter: u32,
    // If PCounter is set
    pub w_mcounter: u32,
    // If either counter is set
    pub counter_prob: u32,
    // If disable is set
    pub invincible: bool,
    pub disable: bool,
}

pub type PartialMobTemporaryStat = PartialFlag<(), MobTemporaryStatPartial>;

//TODO figure out what the u32 is, summon id?

#[derive(ShroomPacketEnum, Debug, Clone)]
#[repr(i8)]
pub enum MobSummonType {
    Effect(u32) = 0,
    Normal(()) = -1,
    Regen(()) = -2,
    Revived(u32) = -3,
    Suspended(()) = -4,
    Delay(()) = -5,
}

shroom_enum_code!(CarnivalTeam, u8, None = 0xff, Blue = 0, Red = 1);

#[derive(ShroomPacket, Debug)]
pub struct MobInitData {
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub origin_fh: FootholdId,
    pub summon_type: MobSummonType,
    pub carnival_team: CarnivalTeam,
    pub effect_id: u32,
    pub phase: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct MobEnterFieldResp {
    pub id: ObjectId,
    pub calc_dmg_stat_ix: u8,
    pub tmpl_id: MobId,
    pub stats: PartialMobTemporaryStat,
    pub init_data: MobInitData,
}
with_opcode!(MobEnterFieldResp, SendOpcodes::MobEnterField);

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum MobLeaveType {
    RemainHp(()) = 0,
    Etc(()) = 1,
    SelfDestruct(()) = 2,
    DestructByMiss(()) = 3,
    Swallow(CharacterId) = 4,
    SummonTimeout(()) = 5,
}

#[derive(ShroomPacket, Debug)]
pub struct MobLeaveFieldResp {
    pub id: ObjectId,
    pub leave_type: MobLeaveType,
}
with_opcode!(MobLeaveFieldResp, SendOpcodes::MobLeaveField);

#[derive(ShroomPacket, Debug)]
pub struct LocalMobData {
    pub tmpl_id: MobId,
    pub stats: PartialMobTemporaryStat,
    pub init: OptionTail<MobInitData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MobCtrlTy {
    Passive = 0xff,
    Passive0 = 0xfe,
    Passive1 = 0xfd,
    None = 0,
    ActiveInt = 1,
    ActiveReq = 2,
    ActivePerm0 = 3,
    ActivePerm1 = 4,
}
impl MobCtrlTy {
    pub fn is_none(&self) -> bool {
        *self == Self::None
    }

    pub fn is_not_none(&self) -> bool {
        !self.is_none()
    }
}

mark_shroom_enum!(MobCtrlTy);

#[derive(ShroomPacket, Debug)]
pub struct MobChangeControllerResp {
    pub ty: MobCtrlTy,
    #[pkt(check(field = "ty", cond = "MobCtrlTy::is_none"))]
    pub crc_seed: CondOption<CrcSeed>, // TODO this depends on is_not_none + client socket option 2
    pub id: ObjectId,
    pub calc_damage_index: u8,
    #[pkt(check(field = "ty", cond = "MobCtrlTy::is_not_none"))]
    pub local_mob_data: CondOption<LocalMobData>,
}
with_opcode!(MobChangeControllerResp, SendOpcodes::MobChangeController);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlyTargetPoint(pub Option<TagPoint>);

pub const NONE_FLY_TARGET_POS: TagPoint = TagPoint::new(0xffddcc, 0xffddcc);

impl From<TagPoint> for FlyTargetPoint {
    fn from(v: TagPoint) -> Self {
        if v == NONE_FLY_TARGET_POS {
            Self(None)
        } else {
            Self(Some(v))
        }
    }
}

impl From<FlyTargetPoint> for TagPoint {
    fn from(v: FlyTargetPoint) -> Self {
        v.0.unwrap_or(NONE_FLY_TARGET_POS)
    }
}

packet_wrap!(FlyTargetPoint<>, TagPoint, TagPoint);

#[derive(ShroomPacket, Debug)]
pub struct MobMoveResp {
    pub id: ObjectId,
    pub not_force_landing: bool,
    pub not_change_action: bool,
    pub next_atk_possible: bool,
    pub action_dir: ActionDir,
    pub data: MobSkillData,
    pub multi_target: ShroomList32<TagPoint>,
    pub rand_time: ShroomList32<u32>,
    pub move_path: MovePath,
}
with_opcode!(MobMoveResp, SendOpcodes::MobMove);

bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub struct MobMoveFlags: u8 {
        const CAN_ATTACK = 1 << 0;
        // TODO 1 << 1 (unused?)
        const RUSH_MOVE = 1 << 2;
        const RISE_BY_TOSS = 1 << 3;
        const MOB_CTRL_STATE = 1 << 4;
    }
}

mark_shroom_bitflags!(MobMoveFlags);

#[derive(ShroomPacket, Debug)]
pub struct MobMoveReq {
    pub id: ObjectId,
    pub ctrl_sn: u16,
    pub flag: MobMoveFlags,
    pub action_dir: ActionDir,
    pub data: MobSkillData,
    pub multi_target: ShroomList32<TagPoint>,
    pub rand_time: ShroomList32<u32>,
    pub move_crc_flag: u8,
    pub hacked_code: u32,
    pub fly_target_pos: FlyTargetPoint,
    pub hacked_code_crc: u32,
    pub move_path: MovePassivePath,
    pub chasing: bool,
    pub has_target: bool,
    pub chasing2: bool,
    pub chasing_hack: bool,
    pub chase_duration: u32,
}

impl MobMoveReq {
    pub fn get_skill_data(&self) -> Option<MobSkillData> {
        if self.action_dir.action.is_skill() {
            Some(self.data)
        } else {
            None
        }
    }
}

with_opcode!(MobMoveReq, RecvOpcodes::MobMove);

#[cfg(test)]
mod tests2 {
    use shroom_pkt::{DecodePacket, PacketReader};

    use super::MobMoveReq;

    #[test]
    fn sample_shroom_move() {
        let data = [
            0x19u8, 0x1C, 0x53, 0x00, 0xCB, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xCC, 0xDD, 0xFF,
            0x00, 0xCC, 0xDD, 0xFF, 0x00, 0x0D, 0x8C, 0xB5, 0x9C, 0xAC, 0x00, 0x64, 0x00, 0x98,
            0xFF, 0x45, 0x00, 0x03, 0x00, 0x64, 0x00, 0x94, 0x00, 0x78, 0xFF, 0x5A, 0x00, 0x16,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x1C, 0x02, 0x00, 0x5A, 0x00, 0x9B, 0x00, 0x5D,
            0xFF, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x4A, 0x00, 0x00, 0x1D,
            0x00, 0x9B, 0x00, 0x83, 0xFF, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
            0xD2, 0x01, 0x00, 0x1D, 0x00, 0x64, 0x00, 0xAC, 0x00, 0x9B, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let _move = MobMoveReq::decode_complete(&mut PacketReader::from(&data)).unwrap();
    }
}

#[derive(ShroomPacket, Debug)]
pub struct MobMoveCtrlAckResp {
    pub id: ObjectId,
    pub ctrl_sn: u16,
    pub next_atk_possible: bool,
    pub mp: u16,
    pub skill_id: u8,
    pub slv: u8,
}
with_opcode!(MobMoveCtrlAckResp, SendOpcodes::MobCtrlAck);

#[derive(ShroomPacket, Debug)]
pub struct MobDamagedResp {
    pub id: ObjectId,
    pub ty: u8,
    pub dec_hp: u32,
    // If template->DamagedByMob !=  false
    pub hp: u32,
    pub max_hp: u32,
}
with_opcode!(MobDamagedResp, SendOpcodes::MobDamaged);

/* 
#[derive(ShroomPacket, Debug)]
pub struct MobOnStatSetResp {
    pub id: ObjectId,
    //TODO if (MobStat::IsMovementAffectingStat(uFlag: var_44) != 0 && this->m_bDoomReserved != 0)
    pub stats: PartialMobTemporaryStat,
    pub delay: u16,
    pub calc_dmg_stat_ix: u8,
    pub movement_affected: u8, // Optional
}
with_opcode!(MobOnStatSetResp, SendOpcodes::MobStatSet);*/

#[derive(ShroomPacket, Debug)]
pub struct MobOnStatSetResp<T> {
    pub id: ObjectId,
    //TODO if (MobStat::IsMovementAffectingStat(uFlag: var_44) != 0 && this->m_bDoomReserved != 0)
    pub stats: T,
    pub delay: u16,
    pub calc_dmg_stat_ix: u8,
    pub movement_affected: u8, // Optional
}
impl<T> HasOpCode for MobOnStatSetResp<T> {
    type OpCode = SendOpcodes;

    const OPCODE: Self::OpCode = SendOpcodes::MobStatSet;
}

#[derive(ShroomPacket, Debug)]
pub struct ResetBurnInfo {
    pub char_id: CharacterId,
    pub skill_id: BuffId,
}

#[derive(ShroomEncodePacket, Debug)]
pub struct MobOnStatResetResp {
    pub id: ObjectId,
    pub flags: MobTemporaryStatFlags,
    pub reset_burns: Option<ShroomList32<ResetBurnInfo>>,
    pub calc_dmg_stat_ix: u8,
    pub movement_affected: u8, // Optional
                               //TODO
}
with_opcode!(MobOnStatResetResp, SendOpcodes::MobStatReset);

#[derive(ShroomPacket, Debug)]
pub struct MobOnSuspendReset {
    pub id: ObjectId,
    pub suspend_reset: bool,
}
with_opcode!(MobOnSuspendReset, SendOpcodes::MobSuspendReset);

#[derive(ShroomPacket, Debug)]
pub struct MobAffectedResp {
    pub id: ObjectId,
    pub buff_id: BuffId,
    pub start_delay: ShroomDurationMs16,
}
with_opcode!(MobAffectedResp, SendOpcodes::MobAffected);

#[derive(ShroomPacket, Debug)]
pub struct MobSpecialEffectBySkillResp {
    pub id: ObjectId,
    pub skill_id: u32,
    pub char_id: CharacterId,
    pub start_delay: ShroomDurationMs16,
}
with_opcode!(
    MobSpecialEffectBySkillResp,
    SendOpcodes::MobSpecialEffectBySkill
);

#[derive(ShroomPacket, Debug)]
pub struct MobHPIndicatorResp {
    pub id: ObjectId,
    pub hp_perc: u8,
}
with_opcode!(MobHPIndicatorResp, SendOpcodes::MobHPIndicator);

#[derive(ShroomPacket, Debug)]
pub struct MobCatchEffectResp {
    pub id: ObjectId,
    pub success: bool,
    pub delay: u8, // TODO
}
with_opcode!(MobCatchEffectResp, SendOpcodes::MobCatchEffect);

#[derive(ShroomPacket, Debug)]
pub struct MobEffectByItem {
    pub id: ObjectId,
    pub item: ItemId,
    pub success: bool,
}
with_opcode!(MobEffectByItem, SendOpcodes::MobEffectByItem);

#[derive(ShroomPacket, Debug)]
pub struct MobSpeakingResp {
    pub id: ObjectId,
    pub speak_info: u32,
    pub speech: u32,
}
with_opcode!(MobSpeakingResp, SendOpcodes::MobSpeaking);

#[derive(ShroomPacket, Debug)]
pub struct MobIncChargeCount {
    pub id: ObjectId,
    pub mob_charge_count: u32,
    pub attack_ready: bool,
}
with_opcode!(MobIncChargeCount, SendOpcodes::MobChargeCount);

#[derive(ShroomPacket, Debug)]
pub struct MobSkillDelayResp {
    pub id: ObjectId,
    pub skill_delay: ShroomDurationMs32,
    pub skill_id: u32,
    pub slv: u32,
    pub skill_option: u32,
}
with_opcode!(MobSkillDelayResp, SendOpcodes::MobSkillDelay);

#[derive(ShroomPacket, Debug)]
pub struct MobEscortPathResp {
    pub id: ObjectId,
    pub u1: u32,
    pub u2: u32,
    pub u3: u32,
    pub u4: u32,
    pub u5: u32,
    pub u6: u32,
    //TODO
}
with_opcode!(MobEscortPathResp, SendOpcodes::MobRequestResultEscortInfo);

#[derive(ShroomPacket, Debug)]
pub struct MobEscortStopSayResp {
    pub id: ObjectId,
    pub stop_escort: ShroomDurationMs32,
    pub chat_ballon: u32,
    pub chat_msg: ShroomOption8<String>,
}
with_opcode!(
    MobEscortStopSayResp,
    SendOpcodes::MobEscortStopEndPermmision
);

#[derive(ShroomPacket, Debug)]
pub struct MobEscortReturnBeforeResp {
    pub id: ObjectId,
    pub u: u32,
}
with_opcode!(MobEscortReturnBeforeResp, SendOpcodes::MobEscortStopSay);

#[derive(ShroomPacket, Debug)]
pub struct MobNextAttackResp {
    pub id: ObjectId,
    pub force_atk_id: u32,
}
with_opcode!(MobNextAttackResp, SendOpcodes::MobNextAttack);

#[derive(ShroomPacket, Debug)]
pub struct MobAttackedByMobResp {
    pub id: ObjectId,
    pub mob_atk_id: u8,
    pub dmg: u32,
    //  Only read if
    pub mob_tmpl_id: MobId,
    pub left: bool,
}
with_opcode!(MobAttackedByMobResp, SendOpcodes::MobAttackedByMob);

#[derive(ShroomPacket, Debug)]
pub struct MobDropPickUpReq {
    pub mob_id: ObjectId,
    pub drop_id: ObjectId,
}
with_opcode!(MobDropPickUpReq, RecvOpcodes::MobDropPickUpRequest);

#[derive(ShroomPacket, Debug)]
pub struct MobApplyCtrlReq {
    pub mob_id: ObjectId,
    pub ctrl_prio: u16,
}
with_opcode!(MobApplyCtrlReq, RecvOpcodes::MobApplyCtrl);

#[cfg(test)]
mod tests {
    use shroom_pkt::{DecodePacket, PacketReader};

    use super::{ActionDir, MobMoveReq};

    #[test]
    fn decode_move_req() {
        let data = [
            227, 0, 4, 0, 0, 0, 1, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
            204, 221, 255, 0, 204, 221, 255, 0, 13, 140, 181, 156, 211, 0, 231, 255, 0, 0, 0, 0, 1,
            0, 0, 1, 231, 255, 43, 0, 0, 0, 46, 0, 0, 0, 0, 0, 2, 56, 4, 0, 211, 0, 231, 255, 0, 1,
            231, 255, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let move_data = MobMoveReq::decode_complete(&mut PacketReader::new(&data[2..])).unwrap();

        dbg!(move_data);
    }

    #[test]
    fn action_dir() {
        let a = ActionDir {
            left: true,
            action: super::MobAction::NONE,
        };

        assert_eq!(a, ActionDir::try_from(u8::from(a)).unwrap());

        let a = ActionDir {
            left: false,
            action: super::MobAction::NONE,
        };
        assert_eq!(a, ActionDir::try_from(u8::from(a)).unwrap());
    }
}
