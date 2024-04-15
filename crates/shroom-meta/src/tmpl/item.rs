use bitflags::bitflags;

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use std::time::Duration;

use crate::id::item_id::EquipType;
use crate::id::{FieldId, ItemId, ItemOptionId, MobId, Money, QuestId, SkillId};
use crate::item::{EquipBaseStats, EquipStat, ItemStat, ItemStatRatio, ScrollChance};
use crate::skill::SkillLevel;
use crate::{CharLevel, ProcChance};

use super::equip::EnchantCategory;

pub static EQ_TY: [&str; 13] = [
    "Accessory",
    "Cap",
    "Cape",
    "Coat",
    "Face",
    "Glove",
    "Hair",
    "Longcoat",
    "Mechanic",
    "Pants",
    //"PetEquip",
    "Ring",
    "Shield",
    "Shoes",
];

pub static ITEM_TY: [&str; 4] = ["Consume", "Etc", "Install", "Cash"];

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
    pub struct ItemFlags: u16 {
        const CASH = 1 << 0;
        const ACCOUNT_SHARABLE = 1 << 1;
        const EXPIRE_ON_LOGOUT = 1 << 2;
        const SELL_ABLE = 1 << 3;
        const UNIQUE = 1 << 4;
        const QUEST_ITEM = 1 << 5;
        const PARTY_QUEST_ITEM = 1 << 6;
        const CAN_TRADE = 1 << 7;
        const EXTEND_ABLE = 1 << 8;
        const IS_TIME_LIMITED = 1 << 9;
        const BIG_SIZE = 1 << 10;
        const NO_CANCEL_MOUSE = 1 << 11;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceItemInfo {
    pub id: ItemId,
    pub period: Duration,
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemReqInfo {
    pub level: Option<CharLevel>,
    pub fields: Vec<FieldId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInfo {
    pub id: ItemId,
    pub flag: ItemFlags,
    pub req: ItemReqInfo,
    pub pad: Option<ItemStat>,
    pub applyable_karma: Option<u8>,
    pub slot_max: u16,
    pub max: u16,
    pub price: Money,
    pub req_quest_on_progress: Option<QuestId>,
    pub unit_price: Option<f32>,
}

pub type RandStatRange = RangeInclusive<i16>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ScrollItem {
    pub inc_stats: EquipBaseStats,
    /// Durations in secs
    pub inc_period: Option<Duration>,
    pub prevent_slip: bool,
    pub warm_support: bool,
    pub recover_slots: usize,
    // TODO incLEV (?)
    pub item_enchant_category: EnchantCategory,
    /// Chaos scroll
    pub rand_stats: Option<RandStatRange>,
    pub success: ScrollChance,
    pub destroy: ScrollChance,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MasteryBookItem {
    pub skills: Vec<SkillId>,
    pub master_level: SkillLevel,
    pub chance: ProcChance,
    pub required_skill_level: SkillLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecoveryItem {
    pub hp: ItemStat,
    pub mp: ItemStat,
    pub interval: Option<Duration>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonsterBookItem {
    pub mob: MobId,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
    pub struct CureFlags: u16 {
        const SEAL = 1 << 0;
        const CURSE = 1 << 1;
        const POISON = 1 << 2;
        const WEAKNESS = 1 << 3;
        const DARKNESS = 1 << 4;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumableItem {
    pub hp: ItemStat,
    pub mp: ItemStat,
    pub hp_ratio: ItemStatRatio,
    pub mp_ratio: ItemStatRatio,
    pub exp: u32,
    pub max_hp_ratio: ItemStatRatio,
    pub max_mp_ratio: ItemStatRatio,
    pub acc: ItemStat,
    pub acc_ratio: ItemStatRatio,
    pub eva: ItemStat,
    pub eva_ratio: ItemStatRatio,
    pub speed: ItemStat,
    pub jump: ItemStat,
    pub speed_ratio: ItemStatRatio,
    pub move_to: Option<FieldId>,
    pub ignore_continent: bool,
    pub cp: Option<u32>,
    pub cp_skill: Option<SkillId>,
    pub cure: CureFlags,
    pub attack_ix: Option<u8>,
    pub barrier: bool,
    pub berserk: Option<u32>,
    pub bf_skill: Option<u8>,
    pub dojang_shield: Option<u8>,
    pub meso_up_by_item: bool,
    pub item_up_by_item: u8,
    pub exp_buff: ItemStatRatio,
    pub script: Option<String>,
    pub time: Option<Duration>,
    pub morph: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StateChangeItem {
    pub hp: ItemStat,
    pub max_hp_ratio: ItemStatRatio,
    pub mp: ItemStat,
    pub max_mp_ratio: ItemStatRatio,
    pub pad: ItemStat,
    pub pdd: ItemStat,
    pub mad: ItemStat,
    pub mdd: ItemStat,
    pub acc: ItemStat,
    pub acc_ratio: ItemStatRatio,
    pub eva: ItemStat,
    pub eva_ratio: ItemStatRatio,
    pub craft: ItemStat,
    pub speed: ItemStat,
    pub speed_ratio: ItemStatRatio, // What's this
    pub jump: ItemStat,
    pub thaw: ItemStat,
    pub morph: Option<u8>,
    pub meso_up_by_item: bool,
    pub tamed_mob_fatigue: Option<i16>,
    pub booster: Option<i16>,
    pub cure: CureFlags,
    pub party: bool,
    pub time: Option<Duration>,
    pub cp: Option<u32>,
    pub cp_skill: Option<SkillId>,
    //TODO verify
    pub exp: u32,
    pub move_to: Option<FieldId>,
    pub ignore_continent: bool,
    pub attack_ix: Option<u8>,
    pub barrier: bool,
    pub berserk: Option<u32>,
    pub bf_skill: Option<u8>,
    pub dojang_shield: Option<u8>,
    pub item_up_by_item: u8,
    pub exp_buff: ItemStatRatio,
    pub script: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PetFoodItem {
    pub repleteness: i32,
    pub pets: Vec<ItemId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectOnDieItem {
    pub recovery_rate: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestDeliveryItem {
    pub ty: u8,
    pub effect: String,
    pub disallow_complete: Vec<QuestId>,
    pub disallow_accept: Vec<QuestId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BundleItemValue {
    MonsterBook(MonsterBookItem),
    Scroll(ScrollItem),
    MasteryBook(MasteryBookItem),
    Bullet,
    SummonSack,
    Consumable(ConsumableItem),
    Etc,
    Install,
    Cash,
    StateChange(StateChangeItem),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BundleItemTmpl {
    pub id: ItemId,
    pub info: ItemInfo,
    pub value: BundleItemValue,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemOptionLevel {
    pub attack_type: Option<u8>,
    pub level: u8,
    pub time: Option<Duration>,
    pub prop: Option<ProcChance>,
    pub boss: bool,
    pub face: Option<String>,
    pub stats: EnumMap<EquipStat, ItemStat>,
    pub ratio_stats: EnumMap<EquipStat, ItemStatRatio>,
    pub dam_reflect: ItemStat,
    pub ignore_dam_ratio: ItemStatRatio,
    pub ignore_dam: ItemStat,
    pub ignore_target_def: ItemStat,
    pub inc_all_skills: ItemStat,
    pub meso_prop_ratio: ItemStatRatio,
    pub dam_ratio: ItemStatRatio,
    pub mp: ItemStat,
    pub mp_restore: ItemStat,
    pub mpcon_reduce: ItemStat,
    pub recovery_hp: ItemStat,
    pub recovery_mp: ItemStat,
    pub recovery_up: ItemStat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemOptionType(pub u8);

impl ItemOptionType {
    pub fn matches_equip(&self, eq: EquipType) -> bool {
        match self.0 {
            10 => eq == EquipType::Weapon,
            11 => eq != EquipType::Weapon,
            20 => !eq.is_accessory() && eq != EquipType::Weapon,
            40 => eq.is_accessory(),
            51 => eq == EquipType::Cap,
            52 => matches!(eq, EquipType::Shirt | EquipType::Coat),
            53 => matches!(eq, EquipType::Pants | EquipType::Coat),
            54 => eq == EquipType::Gloves,
            55 => eq == EquipType::Shoes,
            _ => true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemOption {
    pub id: ItemOptionId,
    pub req_level: CharLevel,
    pub ty: u8,
    pub levels: Vec<ItemOptionLevel>,
}

/*
<enum name="ITEMVARIATIONOPTION" type="int" length="0x4" >
    <member name="ITEMVARIATION_NONE" value="0" />
    <member name="ITEMVARIATION_BETTER" value="1" />
    <member name="ITEMVARIATION_NORMAL" value="2" />
    <member name="ITEMVARIATION_GREAT" value="3" />
    <member name="ITEMVARIATION_PERPECT" value="4" />
    <member name="ITEMVARIATION_GACHAPON" value="5" />
</enum>



<enum name="&lt;unnamed_02f0&gt;" type="int" length="0x4" >
    <member name="ITEMQUALITY_COLOR_BAD" value="819" />
    <member name="ITEMQUALITY_COLOR_GOOD" value="879" />
    <member name="ITEMQUALITY_COLOR_VERYGOOD" value="3183" />
    <member name="ITEMQUALITY_COLOR_PREMIUM" value="4032" />
    <member name="ITEMQUALITY_COLOR_EXCELLENT" value="1008" />
    <member name="ITEMQUALITY_COLOR_SPECIAL" value="3848" />
</enum>

<enum name="&lt;unnamed_023c&gt;" type="int" length="0x4" >
    <member name="RECOVERYHP" value="10151" />
    <member name="RECOVERYMP" value="10156" />
    <member name="RECOVERYHPMP_STATE_SIT" value="20181" />
    <member name="HP_STATE_KILL" value="20401" />
    <member name="MP_STATE_KILL" value="20406" />
    <member name="INVINCIBLE_INC1" value="20366" />
    <member name="INVINCIBLE_INC2" value="30366" />
    <member name="STATUS_TIME" value="20369" />
    <member name="INVINCIBLE" value="30371" />
    <member name="LEARN_SKILL_HASTE" value="31001" />
    <member name="LEARN_SKILL_MYSTIC_DOOR" value="31002" />
    <member name="LEARN_SKILL_SHARP_EYES" value="31003" />
    <member name="LEARN_SKILL_HYPER_BODY" value="31004" />
    <member name="AUTOSTEAL1" value="30701" />
    <member name="AUTOSTEAL2" value="30702" />
</enum>
<enum name="&lt;unnamed_023d&gt;" type="int" length="0x4" >
    <member name="MONSTER_BOMB_START" value="30" />
    <member name="MONSTER_BOMB_START_HEIGHT" value="160" />
    <member name="MONSTER_BOMB_START_DELAY" value="450" />
    <member name="MONSTER_BOMB_LIMITED_TIME" value="500" />
</enum>


<enum name="ENCHANT_SCROLL_CATEGORY" type="int" length="0x4" >
    <member name="ENCHANT_SCROLL_CATEGORY_NORMAL" value="1" />
    <member name="ENCHANT_SCROLL_CATEGORY_VISITOR" value="2" />
</enum>


<enum name="&lt;unnamed_02bb&gt;" type="int" length="0x4" >
    <member name="ITEM_CATEGORY_STRENGTHEN_GEM" value="425" />
    <member name="ITEM_CATEGORY_MONSTER_CRYSTAL" value="426" />
    <member name="ITEM_CATEGORY_HIDDEN" value="999" />
    <member name="ITEM_CATEGORY_EQUIP_DISASSEMBLE" value="998" />
    <member name="ITEM_CATEGORY_CONSUME" value="200" />
    <member name="ITEM_CATEGORY_INSTALL" value="300" />
    <member name="ITEM_CATEGORY_ETC" value="400" />
    <member name="ITEM_CATEGORY_CATALYST" value="4130" />
</enum>
*/
