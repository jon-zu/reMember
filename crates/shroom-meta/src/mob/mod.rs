use std::{collections::BTreeMap, time::Duration};

use num_enum::TryFromPrimitive;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    buffs::{
        char::{self, CharBuff},
        mob::{self, MobBuff},
        SkillChance,
    },
    id::{FieldId, ItemId, MobId, MobSkillId, SkillId},
    shared::{ElemAttrList, ElementAttribute},
    twod::Box2,
    MobMoveAbility,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct MobSkillRange(pub Box2);

#[derive(Debug, Deserialize, Serialize)]
pub struct MobHealSkill {
    pub info: MobSkillInfo,
    pub amount: u16,
    pub variance: u16,
}

impl MobHealSkill {
    pub fn heal_amount(&self, mut rng: impl Rng) -> u16 {
        let variance = rng.gen_range(0..=self.variance);
        self.amount + variance
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MobSkillInfo {
    pub hp_threshold: Option<u8>,
    pub mp_cost: u16,
    pub interval: Option<Duration>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobPartizanBuffSkill<T> {
    pub info: MobSkillInfo,
    pub stat: MobBuff<T>,
    pub range: Option<MobSkillRange>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobBuffSkill<T> {
    pub info: MobSkillInfo,
    pub stat: MobBuff<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobCharBuffSkill<T> {
    pub info: MobSkillInfo,
    pub stat: CharBuff<T>,
    pub skill_chance: Option<SkillChance>,
    pub range: Option<MobSkillRange>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobSummonSkill {
    pub info: MobSkillInfo,
    pub mobs: Vec<MobId>,
    pub limit: usize,
    pub summon_effect: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobAffectedAreaSkill {
    pub info: MobSkillInfo,
    pub range: MobSkillRange,
    pub elem_attr: Option<ElementAttribute>,
    pub dur: Duration,
    pub value: i16, // Seems to be unused
    pub prop: SkillChance,
    pub count: usize, // TODO fire/poison
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobSpreadFromUserSkill {
    pub info: MobSkillInfo,
    pub count: usize,
    pub random_target: bool,
    pub range: MobSkillRange,
    pub spread_skill: MobSkillId,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MobBuffSkillData {
    PowerUp(MobBuffSkill<mob::PowerUp>),
    MagicUp(MobBuffSkill<mob::MagicUp>),
    PGuardUp(MobBuffSkill<mob::PGuardUp>),
    MGuardUp(MobBuffSkill<mob::MGuardUp>),
    Haste(MobBuffSkill<mob::Speed>),
    PhysicalImmune(MobBuffSkill<mob::PImmune>),
    MagicImmune(MobBuffSkill<mob::MImmune>),
    HardSkin(MobBuffSkill<mob::HardSkin>),
    PhyicalCounter(MobBuffSkill<mob::PCounter>),
    MagicCounter(MobBuffSkill<mob::MCounter>),
    PMCounter(MobBuffSkill<mob::PCounter>, MobBuffSkill<mob::MCounter>),
    Pad(MobBuffSkill<mob::Pad>),
    Mad(MobBuffSkill<mob::Mad>),
    Pdr(MobBuffSkill<mob::Pdr>),
    Mdr(MobBuffSkill<mob::Mdr>),
    Acc(MobBuffSkill<mob::Acc>),
    Eva(MobBuffSkill<mob::Eva>),
    Speed(MobBuffSkill<mob::Speed>),
}
impl MobBuffSkillData {
    pub fn info(&self) -> &MobSkillInfo {
        match self {
            Self::PowerUp(v) => &v.info,
            Self::MagicUp(v) => &v.info,
            Self::PGuardUp(v) => &v.info,
            Self::MGuardUp(v) => &v.info,
            Self::Haste(v) => &v.info,
            Self::PhysicalImmune(v) => &v.info,
            Self::MagicImmune(v) => &v.info,
            Self::HardSkin(v) => &v.info,
            Self::PhyicalCounter(v) => &v.info,
            Self::MagicCounter(v) => &v.info,
            Self::PMCounter(v, _) => &v.info,
            Self::Pad(v) => &v.info,
            Self::Mad(v) => &v.info,
            Self::Pdr(v) => &v.info,
            Self::Mdr(v) => &v.info,
            Self::Acc(v) => &v.info,
            Self::Eva(v) => &v.info,
            Self::Speed(v) => &v.info,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MobPartizanBuffSkillData {
    PowerUpM(MobPartizanBuffSkill<mob::PowerUp>),
    MagicUpM(MobPartizanBuffSkill<mob::MagicUp>),
    PGuardUpM(MobPartizanBuffSkill<mob::PGuardUp>),
    MGuardUpM(MobPartizanBuffSkill<mob::MGuardUp>),
    HasteM(MobPartizanBuffSkill<mob::Speed>),
    HealM(MobHealSkill),
}

impl MobPartizanBuffSkillData {
    pub fn info(&self) -> &MobSkillInfo {
        match self {
            Self::PowerUpM(v) => &v.info,
            Self::MagicUpM(v) => &v.info,
            Self::PGuardUpM(v) => &v.info,
            Self::MGuardUpM(v) => &v.info,
            Self::HasteM(v) => &v.info,
            Self::HealM(v) => &v.info,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MobCharBuffSkillData {
    Seal(MobCharBuffSkill<char::Seal>),
    Darkness(MobCharBuffSkill<char::Darkness>),
    Weakness(MobCharBuffSkill<char::Weakness>),
    Stun(MobCharBuffSkill<char::Stun>),
    Curse(MobCharBuffSkill<char::Curse>),
    Poison(MobCharBuffSkill<char::Poison>),
    Slow(MobCharBuffSkill<char::Slow>),
    Dispel(MobSkillInfo), // TODO
    Attract(MobCharBuffSkill<char::Attract>),
    BanMap(MobCharBuffSkill<char::BanMap>),
    ReverseInput(MobCharBuffSkill<char::ReverseInput>),
    Fear(MobCharBuffSkill<char::Fear>),
    Frozen(MobCharBuffSkill<char::Frozen>),
}

impl MobCharBuffSkillData {
    pub fn info(&self) -> &MobSkillInfo {
        match self {
            Self::Seal(v) => &v.info,
            Self::Darkness(v) => &v.info,
            Self::Weakness(v) => &v.info,
            Self::Stun(v) => &v.info,
            Self::Curse(v) => &v.info,
            Self::Poison(v) => &v.info,
            Self::Slow(v) => &v.info,
            Self::Dispel(v) => v,
            Self::Attract(v) => &v.info,
            Self::BanMap(v) => &v.info,
            Self::ReverseInput(v) => &v.info,
            Self::Fear(v) => &v.info,
            Self::Frozen(v) => &v.info,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MobSkill {
    Buff(MobBuffSkillData),
    PartizanBuff(MobPartizanBuffSkillData),
    CharBuff(MobCharBuffSkillData),

    AreaPoison(MobAffectedAreaSkill),
    AreaFire(MobAffectedAreaSkill),

    Summon(MobSummonSkill),
    SummonCube(MobSummonSkill),

    MobSpreadFromUserSkill(MobSpreadFromUserSkill),
    Undead,      // TODO
    StopPortion, // TODO
    StopMotion,  // TODO
    SealSkill,   // TODO
    HealByDamage,
    BalrogCounter,
    Bind,
}

impl MobSkill {
    pub fn info(&self) -> &MobSkillInfo {
        match self {
            Self::Buff(v) => v.info(),
            Self::PartizanBuff(v) => v.info(),
            Self::CharBuff(v) => v.info(),
            Self::AreaPoison(v) => &v.info,
            Self::AreaFire(v) => &v.info,
            Self::Summon(v) => &v.info,
            Self::SummonCube(v) => &v.info,
            Self::MobSpreadFromUserSkill(v) => &v.info,
            Self::Undead => unimplemented!(),
            Self::StopPortion => unimplemented!(),
            Self::StopMotion => unimplemented!(),
            Self::SealSkill => unimplemented!(),
            Self::HealByDamage => unimplemented!(),
            Self::BalrogCounter => unimplemented!(),
            Self::Bind => unimplemented!(),
        }
    }

    pub fn as_buff(&self) -> Option<&MobBuffSkillData> {
        match self {
            Self::Buff(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobSkills(pub BTreeMap<MobSkillId, BTreeMap<u8, MobSkill>>);

#[derive(Debug, Deserialize, Serialize)]
pub struct BanMapTarget {
    pub field: FieldId,
    pub portal: Option<String>,
}


#[derive(Debug, Deserialize, Serialize, TryFromPrimitive)]
#[repr(i8)]
pub enum MobBanType {
    None = 0,
    Collision = 1,
    UserAttack = 2,
    MobSkill = -1,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobBanMap {
    pub ban_msg: Option<String>,
    pub msg_ty: Option<i64>,
    pub ban_type: MobBanType,
    pub target: BanMapTarget,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobLoseItem {
    pub id: ItemId,
    pub amount: i64,
    pub not_drop: bool,
    pub lose_msg: Option<(i64, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobDamagedBy {
    pub mob: bool,
    pub selected_skill: Vec<SkillId>,
    pub selected_mob: Vec<MobId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobSkillEntry {
    pub action: i64,
    pub effect_after: Option<Duration>,
    pub level: i64,
    pub skill: i64,
    pub skill_after: Option<Duration>,
}



#[derive(Debug, Deserialize, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum MobCategory {
    None = 0,
    Animal = 1,
    Plant = 2,
    Fish = 3,
    Reptilia = 4,
    Demon = 5,
    Spirit = 6,
    Immortal = 7,
    Etc = 8,
    Count = 9,
}

#[derive(Debug, Deserialize, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum MobSpecies {
    Beast = 0,
    Dragon = 1,
    Undead = 2,
    Etc = 3,
    No = 4,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mob {
    pub id: MobId,
    pub max_hp: i64,
    pub max_mp: i64,
    pub acc: i64,
    pub eva: i64,
    pub hp_recovery: i64,
    pub mp_recovery: i64,
    pub speed: i64,
    pub fly_speed: i64,
    pub chase_speed: i64,
    pub fixed_damage: i64,
    pub link: Option<MobId>,
    pub hp_tag_color: Option<i64>,
    pub hp_tag_bg_color: Option<i64>,
    pub move_ability: MobMoveAbility,
    pub revive: Vec<MobId>,

    pub damaged_by: MobDamagedBy,

    pub drop_item_period: Option<Duration>,

    pub category: MobCategory,
    pub summon_type: i64,
    pub elem_attr_list: ElemAttrList,
    pub level: i64,
    pub exp: i64,
    pub rate_item_drop_level: i64,
    pub point: i64,

    pub ma_dmg: i64,
    pub md_dmg: i64,
    pub md_rate: i64,

    pub pa_dmg: i64,
    pub pd_dmg: i64,
    pub pd_rate: i64,

    /// Damage required to push
    pub push_min_dmg: i64,

    /// Buff rewarded on killing the mob
    pub buff_reward: Option<SkillId>,

    pub anger_gauge: Option<i64>,
    pub fs: f64,

    pub ban_map: Option<MobBanMap>,
    pub lose_items: Vec<MobLoseItem>,
    pub skills: BTreeMap<i64, MobSkillEntry>,

    pub boss: bool,
    pub body_attack: bool,
    pub first_attack: bool,
    pub explosive_reward: bool,
    pub public_reward: bool,
    pub passable_by_teleport: bool,
    pub no_remove: bool,
    pub cannot_evade: bool,
    pub not_attack: bool,
    pub no_flip: bool,
    pub no_doom: bool,
    pub invincible: bool,
    pub undead: bool,
    pub remove_on_miss: bool,
    pub remove_quest: bool,
    pub upper_most_layer: bool,
    pub hp_gauge_hide: bool,
    pub ignore_field_out: bool,
    pub escort: bool,
    pub can_fly: bool,
    pub can_jump: bool,
    pub has_stop: bool,
    pub has_stand: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobSkillLevelData {
    pub id: SkillId,
    pub count: Option<i64>,
    pub elem_attr: Option<ElementAttribute>,
    pub hp: Option<i64>,
    pub interval: Option<Duration>,
    pub limit: Option<Duration>,
    pub time: Option<Duration>,
    pub mob_count: Option<i64>,
    pub mp_con: i64,
    pub random_target: bool,
    pub x: Option<i64>,
    pub y: Option<i64>,
    pub summon_effect: Option<i64>,
    pub summon_mobs: Vec<MobId>,
    //TODO rb, lt, affected
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobSkillData {
    pub id: SkillId,
    pub levels: BTreeMap<u8, MobSkillLevelData>,
}
