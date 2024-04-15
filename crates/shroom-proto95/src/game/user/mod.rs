pub mod char;
pub mod effect;
pub mod pet;
pub mod remote;
pub mod secondary_stats;


use bitflags::bitflags;
use bytes::BufMut;
use shroom_meta::{
    id::{CharacterId, FieldId, ItemId, MobId, ObjectId, QuestId, SkillId},
    twod::{TagPoint, Vec2},
};
use shroom_pkt::{
    mark_shroom_bitflags, packet_wrap, time::Ticks, with_opcode, CondOption, DecodePacket,
    EncodePacket, PacketReader, PacketResult, PacketWriter, ShroomDurationMs16, ShroomDurationMs32,
    ShroomEncodePacket, ShroomExpirationTime, ShroomList16, ShroomList8, ShroomOption8,
    ShroomPacket, ShroomPacketEnum, SizeHint,
};

use crate::{
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::movement::MovePath,
};

#[derive(ShroomPacket, Debug)]
pub struct UserDropMoneyReq {
    pub ticks: Ticks,
    pub money: u32,
}
with_opcode!(UserDropMoneyReq, RecvOpcodes::UserDropMoneyRequest);

#[derive(ShroomPacket, Debug)]
pub struct UserDropPickUpReq {
    pub field_key: u8,
    pub ticks: Ticks,
    pub point: Vec2,
    pub drop_id: ObjectId,
    pub crc: u32,
}
with_opcode!(UserDropPickUpReq, RecvOpcodes::DropPickUpRequest);

#[derive(ShroomPacket, Debug)]
pub struct UserPortalScriptReq {
    pub field_key: u8,
    pub portal: String,
    pub pos: Vec2,
}
with_opcode!(UserPortalScriptReq, RecvOpcodes::UserPortalScriptRequest);

fn is_not_empty(s: &str) -> bool {
    !s.is_empty()
}

#[derive(ShroomPacket, Debug)]
pub struct UserTransferFieldReq {
    pub field_key: u8,
    pub target_field: FieldId,
    pub portal: String,
    #[pkt(check(field = "portal", cond = "is_not_empty"))]
    pub target_pos: CondOption<Vec2>,
    pub unknown: u8,
    pub premium: bool,
    pub chase_target_pos: ShroomOption8<TagPoint>,
}
with_opcode!(UserTransferFieldReq, RecvOpcodes::UserTransferFieldRequest);

#[derive(ShroomPacket, Debug)]
pub struct UserMoveReq {
    // DR 1-4?
    pub u1: u32,
    pub u2: u32,
    pub field_key: u8,
    pub u3: u32,
    pub u4: u32,
    pub field_crc: u32,
    pub rand: u32,
    pub movement_crc: u32,
    pub move_path: MovePath,
}
with_opcode!(UserMoveReq, RecvOpcodes::UserMove);

#[derive(ShroomPacket, Debug)]
pub struct UserStatChangeReq {
    pub ticks: Ticks,
    // Constant 5120
    pub flags: u32,
    pub hp: u16,
    pub mp: u16,
    pub option: u8,
}
with_opcode!(UserStatChangeReq, RecvOpcodes::UserChangeStatRequest);

#[derive(Debug, ShroomPacket)]
pub struct HitReflectInfo {
    pub power_guard: bool,
    pub reflect_mob_id: ObjectId,
    pub reflect_hit_action: u8,
    pub reflect_mob_pos: Vec2,
    pub pos: Vec2,
}

#[derive(Debug, ShroomEncodePacket)]
pub struct HitInfoMob {
    pub magic_elem_attr: u8,
    pub dmg: u32,
    pub mob_tmpl_id: MobId,
    pub mob_id: ObjectId,
    pub left: bool,
    pub reflect: u8,
    pub guard: u8,
    pub knockback: u8,
    pub reflect_info: Option<HitReflectInfo>,
    pub stance: u8, // bStance | (nSkillID_Stance == 33101006 ? 2 : 0)
}

impl<'de> DecodePacket<'de> for HitInfoMob {
    fn decode(pr: &mut PacketReader<'de>) -> PacketResult<Self> {
        let magic_elem_attr = u8::decode(pr)?;
        let dmg = u32::decode(pr)?;
        let mob_tmpl_id = MobId::decode(pr)?;
        let mob_id = ObjectId::decode(pr)?;
        let left = bool::decode(pr)?;
        let reflect = u8::decode(pr)?;
        let guard = u8::decode(pr)?;
        let knockback = u8::decode(pr)?;
        let reflect_info = if knockback > 1 || reflect != 0 {
            Some(HitReflectInfo::decode(pr)?)
        } else {
            None
        };
        let stance = u8::decode(pr)?;
        Ok(Self {
            magic_elem_attr,
            dmg,
            mob_tmpl_id,
            mob_id,
            left,
            reflect,
            guard,
            knockback,
            reflect_info,
            stance,
        })
    }
}

#[derive(ShroomPacket, Debug)]
pub struct HitInfoObstacle {
    pub elem_attr: u8,
    pub dmg: u32,
    pub obstacle_data: u16,
    pub u: u8, // TODO: Always 2?, maybe knockback? or the stance stuff
}

#[derive(ShroomPacket, Debug)]
pub struct HitInfoStat {
    pub elem_attr: u8,
    pub dmg: u32,
    pub disease_data: u16,
    pub disease_type: u8,
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(i8)]
pub enum UserHit {
    MobPhysical(HitInfoMob) = 0,
    MobMagic(HitInfoMob) = -1,
    Counter(HitInfoObstacle) = -2,
    Obstacle(HitInfoObstacle) = -3,
    Stat(HitInfoStat) = -4,
}

#[derive(ShroomPacket, Debug)]
pub struct UserHitReq {
    pub damaged_ticks: Ticks,
    //pub hit: UserHit,
}
with_opcode!(UserHitReq, RecvOpcodes::UserHit);

bitflags! {
    #[derive(Debug)]
    pub struct AttackFlags : u8 {
        const FINAL_ATTACK = 0x01;
        const SHADOW_PARTNER = 0x04;
        const SERIAL_ATTACK = 0x08;
        const SPARK = 0x10;
    }
}

mark_shroom_bitflags!(AttackFlags);

#[derive(Debug, ShroomPacket)]
pub struct DrCheckData {
    data: [u8; 8],
}

#[derive(Debug, Clone)]
pub struct HitTargetCount {
    pub hits: u8,
    pub targets: u8,
}

impl From<u8> for HitTargetCount {
    fn from(v: u8) -> Self {
        Self {
            targets: v >> 4,
            hits: v & 0xF,
        }
    }
}

impl From<HitTargetCount> for u8 {
    fn from(v: HitTargetCount) -> Self {
        (v.targets << 4) | (v.hits & 0xF)
    }
}

packet_wrap!(HitTargetCount<>, u8, u8);

#[derive(Debug, Clone)]
pub struct ActionDir {
    pub left: bool,
    pub action: u16,
}

impl From<u16> for ActionDir {
    fn from(v: u16) -> Self {
        Self {
            left: v >> 15 == 1,
            action: v & 0x7FFF,
        }
    }
}

impl From<ActionDir> for u16 {
    fn from(v: ActionDir) -> Self {
        (v.left as Self) << 15 | (v.action & 0x7FFF)
    }
}

packet_wrap!(ActionDir<>, u16, u16);

#[derive(Debug, Clone)]
pub struct ForeActionDir {
    pub left: bool,
    pub action: u8,
}

impl From<u8> for ForeActionDir {
    fn from(v: u8) -> Self {
        Self {
            left: v >> 7 == 1,
            action: v & 0x7F,
        }
    }
}

impl From<ForeActionDir> for u8 {
    fn from(v: ForeActionDir) -> Self {
        (v.left as Self) << 7 | (v.action & 0x7F)
    }
}

packet_wrap!(ForeActionDir<>, u8, u8);

/*#[derive(PackedStruct, Debug, Clone, Copy)]
#[packed_struct(bit_numbering = "msb0")]
pub struct HitTargetCount {
    target_count: Integer<u8, packed_bits::Bits<4>>,
    hit_count: Integer<u8, packed_bits::Bits<4>>,
}*/

#[derive(Debug, ShroomPacket)]
pub struct SingleAttackTargetInfo {
    pub mob_id: ObjectId,
    pub mob_tmpl_id: MobId,
    pub hit_action: u8,
    pub fore_action: ForeActionDir,
    pub frame_id: u8,
    pub calc_damage_stat_ix: u8,
    pub pos: Vec2,
    pub pos_prev: Vec2,
    pub delay: u16,
    pub hit: u32,
}

const MAX_HITS: usize = 0xF;

#[derive(Debug)]
pub struct Hits(pub smallvec::SmallVec<[u32; MAX_HITS]>);

impl EncodePacket for Hits {
    const SIZE_HINT: SizeHint = SizeHint::NONE;

    fn encode<B: BufMut>(&self, pw: &mut PacketWriter<B>) -> PacketResult<()> {
        for hit in self.0.iter() {
            hit.encode(pw)?;
        }
        Ok(())
    }

    fn encode_len(&self) -> usize {
        self.0.len() * 4
    }
}

impl Hits {
    pub fn single(hit: u32) -> Self {
        Self(smallvec::smallvec![hit])
    }

    pub fn sum(&self) -> u32 {
        self.0.iter().sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = &u32> {
        self.0.iter()
    }

    pub fn decode(pr: &mut PacketReader, n: usize) -> PacketResult<Self> {
        let mut hits = smallvec::SmallVec::with_capacity(n);
        for _ in 0..n {
            hits.push(u32::decode(pr).unwrap());
        }
        Ok(Self(hits))
    }
}

#[derive(Debug, ShroomEncodePacket)]
pub struct AttackTargetInfo {
    pub mob_id: ObjectId,
    pub hit_action: u8,
    pub fore_action: ForeActionDir,
    pub frame_id: u8,
    pub calc_damage_stat_ix: u8,
    pub pos: Vec2,
    pub pos_prev: Vec2,
    pub delay: u16,
    pub hits: Hits,
    pub mob_crc: u32,
}

impl AttackTargetInfo {
    pub fn decode(
        pr: &mut PacketReader<'_>,
        targets: usize,
        hits: usize,
    ) -> Result<Vec<Self>, shroom_pkt::Error> {
        (0..targets)
            .map(|_| {
                Ok(Self {
                    mob_id: ObjectId::decode(pr)?,
                    hit_action: u8::decode(pr)?,
                    fore_action: ForeActionDir::decode(pr)?,
                    frame_id: u8::decode(pr)?,
                    calc_damage_stat_ix: u8::decode(pr)?,
                    pos: Vec2::decode(pr)?,
                    pos_prev: Vec2::decode(pr)?,
                    delay: u16::decode(pr)?,
                    hits: Hits::decode(pr, hits)?,
                    mob_crc: u32::decode(pr)?,
                })
            })
            .collect()
    }
}

pub trait AttackInfo {
    fn targets(&self) -> usize;
    fn hits(&self) -> usize;
}

impl AttackInfo for MeleeAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

impl<'de, Info, Extra> DecodePacket<'de> for AttackReq<Info, Extra>
where
    Info: AttackInfo + DecodePacket<'de> + std::fmt::Debug,
    Extra: DecodePacket<'de>,
{
    fn decode(pr: &mut PacketReader<'de>) -> PacketResult<Self> {
        let info = Info::decode(pr)?;
        let targets = AttackTargetInfo::decode(pr, info.targets(), info.hits())?;
        let extra = Extra::decode(pr)?;
        Ok(Self {
            info,
            targets,
            extra,
        })
    }
}

#[derive(Debug, ShroomEncodePacket)]
pub struct AttackReq<Info: AttackInfo, Extra> {
    pub info: Info,
    pub targets: Vec<AttackTargetInfo>,
    pub extra: Extra,
}

/// Workaround hack for invalid extra byte for Reactor attacks
/// Essentially for ignoring the extra byte when user attacks a single target reactor
/// TODO toggle this via  feature flag
#[derive(Debug)]
pub struct ReactorFlag(pub bool);

impl EncodePacket for ReactorFlag {
    const SIZE_HINT: SizeHint = SizeHint::NONE;

    fn encode<B: BufMut>(&self, pw: &mut PacketWriter<B>) -> PacketResult<()> {
        if self.0 {
            self.0.encode(pw)?;
        }

        Ok(())
    }

    fn encode_len(&self) -> usize {
        if self.0 {
            1
        } else {
            0
        }
    }
}

impl<'de> DecodePacket<'de> for ReactorFlag {
    fn decode(pr: &mut PacketReader<'de>) -> PacketResult<Self> {
        let n = pr.remaining_slice().len();
        if n == 60 {
            let _ = pr.read_u8()?;
            Ok(Self(true))
        } else {
            Ok(Self(false))
        }
    }
}

#[derive(Debug, ShroomPacket)]
pub struct MeleeAttackInfo {
    pub portal: u8, // Field key
    pub flag: ReactorFlag,
    pub hit_target_count: DrHitTargetCount,
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rnd: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    #[pkt(check(field = "skill_id", cond = "SkillId::is_charge_skill"))]
    pub key_down_dur: CondOption<ShroomDurationMs32>,
    pub attack_flags: AttackFlags,
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32, // update time
    //Special bmage handling
    pub affected_area_id: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct MeleeAttackTail {
    pub pos: Vec2,
    // TODO: If skillid == 14111006
    //pub grenade_pos: ShroomOption8<Vec2>
}

#[derive(Debug, ShroomPacket, Clone)]
pub struct DrCtx {
    pub dr0: u32,
    pub dr1: u32,
    pub dr2: u32,
    pub dr3: u32,
}

#[derive(Debug, Clone)]
pub struct DrHitTargetCount {
    pub dr: DrCtx,
    pub hit_target_count: HitTargetCount,
}

type DrHitTargetCountTuple = (u32, u32, HitTargetCount, u32, u32);

impl From<DrHitTargetCountTuple> for DrHitTargetCount {
    fn from(v: DrHitTargetCountTuple) -> Self {
        Self {
            dr: DrCtx {
                dr0: v.0,
                dr1: v.1,
                dr2: v.3,
                dr3: v.4,
            },
            hit_target_count: v.2,
        }
    }
}

impl From<DrHitTargetCount> for DrHitTargetCountTuple {
    fn from(v: DrHitTargetCount) -> Self {
        (
            v.dr.dr0,
            v.dr.dr1,
            v.hit_target_count.clone(),
            v.dr.dr2,
            v.dr.dr3,
        )
    }
}

packet_wrap!(
    DrHitTargetCount<>,
    DrHitTargetCountTuple,
    DrHitTargetCountTuple
);

pub type UserMeleeAttackReq = AttackReq<MeleeAttackInfo, MeleeAttackTail>;
with_opcode!(UserMeleeAttackReq, RecvOpcodes::UserMeleeAttack);

#[derive(ShroomPacket, Debug)]
pub struct SkillInfoCrc {
    pub crc1: u32,
    pub crc2: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct ValWithCrc {
    pub val: u32,
    pub crc: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct MagicAttackInfo {
    pub portal: u8, // Field key
    pub hit_target_count: DrHitTargetCount,
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rng: ValWithCrc,
    pub dr_ctx: DrCtx,
    pub init: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    #[pkt(check(field = "skill_id", cond = "SkillId::is_keydown"))]
    pub key_down_dur: CondOption<ShroomDurationMs32>,
    pub attack_flags: u8,
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32,
    pub phase: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct MagicAttackTail {
    pub pos: Vec2,
    pub dragon_pos: ShroomOption8<Vec2>,
}

impl AttackInfo for MagicAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

pub type UserMagicAttackReq = AttackReq<MagicAttackInfo, MagicAttackTail>;
with_opcode!(UserMagicAttackReq, RecvOpcodes::UserMagicAttack);

#[derive(Debug, ShroomPacket)]
pub struct BodyAttackInfo {
    pub portal: u8, // Field key
    pub hit_target_count: DrHitTargetCount,
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rnd: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    pub attack_flags: u8, //always 0
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32,
    pub id: u32, // dwid?
}

#[derive(Debug, ShroomPacket)]
pub struct BodyAttackTail {
    pub pos: Vec2,
}

impl AttackInfo for BodyAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

pub type UserBodyAttackReq = AttackReq<BodyAttackInfo, BodyAttackTail>;
with_opcode!(UserBodyAttackReq, RecvOpcodes::UserBodyAttack);

bitflags! {
    #[derive(Debug)]
    pub struct ShotAttackFlags : u8 {
        const SOUL_ARROW = 0x02;
        const MORTAL_BLOW = 0x04;
        const SHADOW_PARTNER = 0x08;
        const SERIAL_ATTACK = 0x20;
        const SPIRIT_JAVELIN = 0x40;
        const SPARK = 0x80;
    }
}
mark_shroom_bitflags!(ShotAttackFlags);

#[derive(Debug, ShroomPacket)]
pub struct ShootAttackHeader {
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rnd: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    #[pkt(check(field = "skill_id", cond = "SkillId::is_keydown"))]
    pub key_down_dur: CondOption<ShroomDurationMs32>,
    pub attack_flags: ShotAttackFlags,
}

impl ShootAttackHeader {
    pub fn has_javelin_bullet_id(&self) -> bool {
        self.attack_flags.contains(ShotAttackFlags::SPIRIT_JAVELIN)
            && !self.skill_id.is_not_consuming_bullet()
    }
}

#[derive(Debug, ShroomPacket)]
pub struct ShotAttackInfo {
    pub portal: u8, // Field key
    pub hit_target_count: DrHitTargetCount,
    pub hdr: ShootAttackHeader,
    pub jablin: bool, // v291->m_bNextShootExJablin && CUserLocal::CheckApplyExJablin(v291, pSkill, nAttackAction)
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32,
    pub id: u32, // dwid?
    pub bullet_slot: u16,
    pub cash_bullet_slot: u16,
    pub shot_range: u8,
    #[pkt(check(field = "hdr", cond = "ShootAttackHeader::has_javelin_bullet_id"))]
    pub spirit_javelin_bullet_id: CondOption<ItemId>,
}

#[derive(Debug, ShroomPacket)]
pub struct ShootAttackTail {
    pub pos: Vec2,
    // If wildhunter
    // body_rel_y_move: u16
    pub atk_pos: Vec2,
    // If skill_id == 15111006 (spark) -> reserve_spark_delay: u32
}

impl AttackInfo for ShotAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

pub type UserShotAttackReq = AttackReq<ShotAttackInfo, ShootAttackTail>;
with_opcode!(UserShotAttackReq, RecvOpcodes::UserShootAttack);

#[derive(ShroomPacket, Debug)]
pub struct UserSkillUpReq {
    pub ticks: Ticks,
    pub skill_id: SkillId,
}
with_opcode!(UserSkillUpReq, RecvOpcodes::UserSkillUpRequest);

#[derive(Debug, ShroomEncodePacket)]
pub struct AffectedMembers(Option<u8>);

impl AffectedMembers {
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        let val = self.0.unwrap_or(0x80);
        (0..6).filter(move |v| val & (1 << v) != 0)
    }
}

impl<'de> DecodePacket<'de> for AffectedMembers {
    fn decode(pr: &mut PacketReader<'de>) -> PacketResult<Self> {
        let rem = pr.remaining();
        Ok(Self(match rem {
            // Either without Dispel(3) or with Dispel(+2) the remaining + the delay at the end
            _ if rem.checked_sub(3 + 1).map_or(false, |n| n % 4 == 0)
                || rem.checked_sub(5 + 1).map_or(false, |n| n % 4 == 0) =>
            {
                Some(u8::decode(pr)?)
            }
            _ => None,
        }))
    }
}

#[derive(Debug, ShroomEncodePacket)]
pub struct AffectedMobs(Option<ShroomList8<ObjectId>>);

impl AffectedMobs {
    pub fn iter(&self) -> impl Iterator<Item = &ObjectId> {
        self.0.iter().flat_map(|v| v.iter())
    }
}

impl<'de> DecodePacket<'de> for AffectedMobs {
    fn decode(pr: &mut PacketReader<'de>) -> PacketResult<Self> {
        // Remaining must be 2 + 1(n) + 4*n
        let rem = pr.remaining().saturating_sub(3);
        Ok(Self(if rem != 0 && rem % 4 == 0 {
            Some(ShroomList8::decode(pr)?)
        } else {
            None
        }))
    }
}

#[derive(ShroomPacket, Debug)]
pub struct UserSkillUseReq {
    pub ticks: Ticks,
    pub skill_id: SkillId,
    pub skill_level: u8,
    #[pkt(check(field = "skill_id", cond = "SkillId::is_anti_repeat_buff_skill"))]
    pub pos: CondOption<Vec2>,
    #[pkt(check(field = "skill_id", cond = "SkillId::is_spirit_javelin"))]
    pub spirit_javelin_item: CondOption<ItemId>,
    pub affected: AffectedMembers, // TODO use affected check later
    #[pkt(check(field = "skill_id", cond = "SkillId::is_dispel"))]
    pub dispel_delay: CondOption<ShroomDurationMs16>,
    pub affected_mobs: AffectedMobs,
    pub delay: ShroomDurationMs16,
}
with_opcode!(UserSkillUseReq, RecvOpcodes::UserSkillUseRequest);

#[derive(ShroomPacket, Debug)]
pub struct UserSkillCancelReq {
    pub skill_id: SkillId,
}
with_opcode!(UserSkillCancelReq, RecvOpcodes::UserSkillCancelRequest);

#[derive(ShroomPacket, Debug)]
pub struct UpdatedSkillRecord {
    pub id: SkillId,
    pub level: u32,
    pub master_level: u32,
    pub expiration: ShroomExpirationTime,
}

#[derive(ShroomPacket, Debug)]
pub struct ChangeSkillRecordResp {
    pub reset_excl: bool,
    pub skill_records: ShroomList16<UpdatedSkillRecord>,
    pub updated_secondary_stat: bool,
}
with_opcode!(ChangeSkillRecordResp, SendOpcodes::ChangeSkillRecordResult);

#[derive(Debug, ShroomPacket)]
pub struct SkillUseResultResp {
    // Unused, the client reset excl regardless
    pub unknown: u8,
}
with_opcode!(SkillUseResultResp, SendOpcodes::SkillUseResult);

#[derive(ShroomPacket, Debug)]
pub struct SkillCooldownSetResp {
    pub skill_id: SkillId,
    pub cooldown_s: u16, //TODO ShroomDurationSec16
}
with_opcode!(SkillCooldownSetResp, SendOpcodes::SkillCooltimeSet);

#[derive(ShroomPacket, Debug)]
pub struct PopularityResult {
    pub name: String,
    pub inc: bool,
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum GivePopularityResp {
    Success(PopularityResult, u32) = 0,
    InvalidCharacter(()) = 1,
    LevelTooLow(()) = 2,
    DailyLimit(()) = 3,
    TargetLimit(()) = 4,
    Notify(PopularityResult) = 5,
}

with_opcode!(GivePopularityResp, SendOpcodes::GivePopularityResult);

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum DropPickUpMsg {
    Item((ItemId, u32)) = 0, // Item, quantity
    Mesos((u8, ItemId, u16)) = 1,
    Equip(ItemId) = 2,
}

fn is_true(b: &bool) -> bool {
    *b
}

#[derive(Debug, ShroomPacket)]
pub struct IncExpMsg {
    pub last_hit: bool,
    pub gain: u32,
    pub chat: bool,
    pub event_bonus: u32,
    pub unknown1: bool,
    pub unknown2: u8,
    pub wedding_bonus: u32,
    #[pkt(check(field = "unknown1", cond = "is_true"))]
    pub unknown_bonus: CondOption<u8>,
    #[pkt(check(field = "chat", cond = "is_true"))]
    pub chat_bonus: CondOption<u8>,
    pub unknown3: u8, // TODO: BONUX type ?
    pub party_bonus: u32,
    pub equip_bonus: u32,
    pub internet_cafe_bonus: u32,
    pub rainbow_week_bonus: u32,
    pub unknown1_bonus: u32,
    pub unknown2_bonus: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct QuestRecordEx {
    pub quest_id: QuestId,
    pub raw_str: String,
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum MessageResp {
    DropPickUp(DropPickUpMsg) = 0,
    // Quest Record = 1
    CashItemExpire(ItemId) = 2,
    IncExp(IncExpMsg) = 3,
    IncSp((u16, u8)) = 4,
    IncPop(CharacterId) = 5,
    IncMoney(u32) = 6,
    IncGp(u32) = 7,
    GiveBuff(u32) = 8,
    ItemExpire(ShroomList8<ItemId>) = 9,
    System(String) = 10,
    QueryRecordEx(QuestRecordEx) = 11,
    ItemProtectExpire(ShroomList8<ItemId>) = 12,
    ItemExpireReplace(ShroomList8<String>) = 13,
    SkillExpire(ShroomList8<SkillId>) = 14,
}

with_opcode!(MessageResp, SendOpcodes::Message);

#[cfg(test)]
mod tests {
    use shroom_pkt::{DecodePacket, PacketReader};

    use crate::game::user::UserMagicAttackReq;

    use super::UserHitReq;

    #[test]
    fn user_hit_req() {
        let data = [
            52, 0, 232, 211, 221, 3, 255, 0, 1, 0, 0, 0, 160, 134, 1, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let hit = UserHitReq::decode_complete(&mut PacketReader::new(&data[2..])).unwrap();
        dbg!(hit);
    }

    #[test]
    fn user_melee_atk() {
        let data = [
            47, 0, 0, 55, 29, 230, 255, 31, 127, 55, 139, 17, 59, 132, 173, 136, 215, 117, 129,
            160, 0, 0, 0, 0, 0, 117, 25, 20, 0, 125, 21, 153, 165, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
            128, 194, 165, 88, 168, 1, 4, 228, 215, 221, 3, 0, 0, 0, 0, 18, 0, 0, 0, 7, 128, 7, 5,
            187, 2, 139, 1, 187, 2, 139, 1, 137, 1, 10, 0, 0, 0, 225, 199, 157, 247, 241, 2, 139,
            1,
        ];
        let atk = UserMagicAttackReq::decode_complete(&mut PacketReader::new(&data[2..])).unwrap();
        dbg!(atk);
    }

    #[test]
    fn user_magic_atk() {
        // No target
        let data = [
            49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 1, 43, 26, 230, 255, 27, 25, 230, 255,
            232, 3, 0, 0, 0, 70, 110, 165, 0, 160, 86, 130, 241, 255, 255, 255, 255, 35, 21, 76,
            174, 11, 150, 35, 251, 71, 25, 230, 255, 211, 225, 182, 41, 80, 154, 220, 44, 238, 211,
            198, 119, 238, 211, 198, 119, 0, 37, 0, 194, 165, 88, 168, 1, 6, 45, 220, 157, 2, 0, 0,
            0, 0, 58, 1, 18, 1, 0,
        ];
        let atk = UserMagicAttackReq::decode_complete(&mut PacketReader::new(&data[2..])).unwrap();
        dbg!(atk);

        // 1 targets
        let data = [
            49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 17, 43, 26, 230, 255, 27, 25, 230, 255,
            232, 3, 0, 0, 0, 194, 254, 171, 2, 234, 36, 91, 175, 255, 255, 255, 255, 35, 21, 76,
            174, 11, 150, 35, 251, 71, 25, 230, 255, 140, 87, 162, 88, 61, 93, 199, 49, 238, 211,
            198, 119, 238, 211, 198, 119, 0, 37, 128, 194, 165, 88, 168, 1, 6, 139, 155, 158, 2, 0,
            0, 0, 0, 14, 0, 0, 0, 7, 128, 6, 5, 187, 2, 139, 1, 187, 2, 139, 1, 8, 2, 10, 0, 0, 0,
            225, 199, 157, 247, 29, 3, 139, 1, 0,
        ];
        let atk = UserMagicAttackReq::decode_complete(&mut PacketReader::new(&data[2..])).unwrap();
        dbg!(atk);
    }
    /*
    Unhandled packet: [49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 1, 43, 26, 230, 255, 27, 25, 230, 255, 232, 3, 0, 0, 0, 70, 110, 165, 0, 160, 86, 130, 241, 255, 255, 255, 255, 35, 21, 76, 174, 11, 150, 35, 251, 71, 25, 230, 255, 211, 225, 182, 41, 80, 154, 220, 44, 238, 211, 198, 119, 238, 211, 198, 119, 0, 37, 0, 194, 165, 88, 168, 1, 6, 45, 220, 157, 2, 0, 0, 0, 0, 58, 1, 18, 1, 0]
    [49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 17, 43, 26, 230, 255, 27, 25, 230, 255, 232, 3, 0, 0, 0, 194, 254, 171, 2, 234, 36, 91, 175, 255, 255, 255, 255, 35, 21, 76, 174, 11, 150, 35, 251, 71, 25, 230, 255, 140, 87, 162, 88, 61, 93, 199, 49, 238, 211, 198, 119, 238, 211, 198, 119, 0, 37, 128, 194, 165, 88, 168, 1, 6, 139, 155, 158, 2, 0, 0, 0, 0, 14, 0, 0, 0, 7, 128, 6, 5, 187, 2, 139, 1, 187, 2, 139, 1, 8, 2, 10, 0, 0, 0, 225, 199, 157, 247, 29, 3, 139, 1, 0]
     */

    // use skill: [103, 0, 67, 16, 159, 2, 234, 3, 0, 0, 1, 0, 0]
}
