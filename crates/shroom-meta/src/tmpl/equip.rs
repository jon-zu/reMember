use enum_map::{Enum, EnumMap};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{
    id::{job_id::JobId, ItemId, Money, SkillId},
    item::{EquipStat, EquipStats, ItemStat, ItemStatRange, ItemStatRatio, JobFlag},
    shared::ElementAttribute,
    skill::SkillLevel,
    CharLevel, Pop, ProcChance,
};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct SetId(pub u8);

#[derive(Debug, Deserialize, Serialize)]
pub struct EquipLevelValue {
    pub exp: u32,
    pub inc: EnumMap<EquipStat, ItemStatRange>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EquipReq {
    pub level: CharLevel,
    //TODO Which level?
    pub mob_level: u8,
    pub pop: Pop,
    pub str: ItemStat,
    pub dex: ItemStat,
    pub int: ItemStat,
    pub luk: ItemStat,
    pub job: JobFlag,
}


#[derive(Debug, Enum, Clone, Deserialize, Serialize)]
pub enum EquipIncreaseMagicElem {
    Fire,
    Poison,
    Ice,
    Lightning,
}
pub type EquipIncreaseMagicElems = EnumMap<EquipIncreaseMagicElem, ItemStatRatio>;

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
    pub struct EquipItemFlags: u16 {
        const CASH = 1 << 0;
        const ACCOUNT_SHARABLE = 1 << 1;
        const CAN_DROP = 1 << 2;
        const ON_EQUIP_TRADE_BLOCK = 1 << 3;
        const ON_LOGOUT_EXPIRE = 1 << 4;
        const HIDE = 1 << 5;
        const NAME_TAG_ABLE = 1 << 6;
        const SELL_ABLE = 1 << 7;
        const UNIQUE = 1 << 8;
        const QUEST_ITEM = 1 << 9;
        const CAN_TRADE = 1 << 10;
        const EXTEND_ABLE = 1 << 11;
        const IS_TIME_LIMITED = 1 << 12;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttackAction(u8);

impl TryFrom<u8> for AttackAction {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

#[derive(Debug, Deserialize, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum EnchantCategory {
    None = 0,
    Normal = 1,
    Visitor = 2,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive, Deserialize, Serialize)]
#[repr(u8)]
pub enum AttackSpeedDegree {
    FASTEST = 0,
    FASTEST_ = 1,
    FASTER = 2,
    FASTER_ = 3,
    FAST = 4,
    FAST_ = 5,
    NORMAL = 6,
    NORMAL_ = 7,
    SLOW = 8,
    SLOW_ = 9,
    SLOWER = 10,
    SLOWER_ = 11,
    SLOWEST = 12,
}

// TODO additions for equips
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EquipAdditionCond {
    pub job: Option<JobId>,
    pub level: CharLevel,
    pub craft: ItemStat,
    pub str: ItemStat,
    pub dex: ItemStat,
    pub int: ItemStat,
    pub luk: ItemStat,
    pub item_quality: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionCrit {
    pub prob: ProcChance,
    pub damage: u8,
}

//"elemVol": "S50"
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionElemBoost {
    pub elem: ElementAttribute,
    pub boost: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionBoss {
    pub prob: ProcChance,
    pub dmg: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionHpMpChange {
    pub hp_change: ItemStat,
    pub mp_change: ItemStat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionMobCategory {
    pub category: u8,
    pub dmg: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobDieIncStat {
    pub flat: ItemStat,
    pub flat_chance: ProcChance,
    pub ratio: ItemStatRatio,
    pub ratio_chance: ProcChance,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionMobDie {
    pub hp_inc: MobDieIncStat,
    pub mp_inc: MobDieIncStat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionSkill {
    pub skill_id: SkillId,
    pub level: SkillLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Addition<T> {
    pub cond: EquipAdditionCond,
    pub value: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EquipAdditions {
    pub crit: Option<Addition<AdditionCrit>>,
    pub elem_boost: Option<Addition<AdditionElemBoost>>,
    pub boss: Option<Addition<AdditionBoss>>,
    pub hp_mp_change: Option<Addition<AdditionHpMpChange>>,
    pub mob_category: Option<Addition<AdditionMobCategory>>,
    pub mob_die: Option<Addition<AdditionMobDie>>,
    pub skill: Option<Addition<AdditionSkill>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EquipItemTmpl {
    pub id: ItemId,
    pub flags: EquipItemFlags,
    pub req: EquipReq,
    pub stats: EquipStats,
    pub price: Money,
    pub chat_balloon_id: Option<u8>,
    pub upgrade_slots: u8,
    pub max_enhancements: Option<u8>,
    pub enchant_category: EnchantCategory,
    pub set_id: Option<SetId>,
    pub additions: Option<EquipAdditions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeaponItemTmpl {
    pub equip: EquipItemTmpl,
    pub atk_speed_degree: AttackSpeedDegree,
    pub attack_action: AttackAction,
    pub equip_increase_magic_elem: EquipIncreaseMagicElems,
}
