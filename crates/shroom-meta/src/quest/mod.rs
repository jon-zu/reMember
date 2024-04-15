use std::{
    collections::{HashMap, HashSet}, ops::RangeInclusive, str::FromStr, time::Duration
};

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{
    id::{job_id::JobId, FieldId, ItemId, MobId, NpcId, QuestId, SkillId}, skill::SkillLevel, CharLevel, Money, PetSkill, PetTameness, Pop, TamingMobLevel, World
};
#[derive(Debug, Deserialize, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum ItemVariation {
    None = 0,
    Better = 1,
    Normal = 2,
    Great = 3,
    Perfect = 4,
    Gachapon = 5,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum QuestRange<T> {
    Range(RangeInclusive<T>),
    Min(T),
    Max(T),
    None,
}

impl<T> QuestRange<T> {
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        match self {
            Self::Range(range) => range.contains(value),
            Self::Min(min) => value >= min,
            Self::Max(max) => value <= max,
            Self::None => true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl FromStr for DayOfWeek {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "mon" => Self::Monday,
            "tue" => Self::Tuesday,
            "wed" => Self::Wednesday,
            "thu" => Self::Thursday,
            "fri" => Self::Friday,
            "sat" => Self::Saturday,
            "sun" => Self::Sunday,
            _ => anyhow::bail!("Invalid day of week: {}", s),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestState(pub u32);

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestItemReq {
    pub items: Vec<(ItemId, usize)>,
    pub equips_all: Vec<ItemId>,
    pub equips_select: Vec<ItemId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestMonsterBookReq {
    pub min_total_cards: Option<usize>,
    pub cards: HashMap<MobId, QuestRange<usize>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestReq {
    pub jobs: HashSet<JobId>,
    pub field_enter: HashSet<FieldId>,
    pub quest_states: HashMap<QuestId, QuestState>,
    pub level_range: QuestRange<CharLevel>,
    pub taming_mob_level_range: QuestRange<TamingMobLevel>,
    pub pet_tameness_range: QuestRange<PetTameness>,
    pub min_pop: Option<Pop>,
    pub items: QuestItemReq,
    // Require skill and a flag whether it has to be leveld
    pub skills: HashMap<SkillId, SkillLevel>,
    pub mobs: Vec<(MobId, usize)>,
    pub mbook: QuestMonsterBookReq,
    pub worlds: QuestRange<World>,
    pub days: HashSet<DayOfWeek>,
    pub repeat_per_day: bool,
    pub end_money: Option<Money>,
    pub normal_auto_start: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestStartReq {
    pub req: QuestReq,
    pub npc: Option<NpcId>,
    pub interval: Option<Duration>,
    pub script: Option<String>,
    // TODO start-end whitelist time
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestEndReq {
    pub req: QuestReq,
    pub npc: Option<NpcId>,
    pub script: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestActSkill {
    pub id: SkillId,
    pub master_level: Option<SkillLevel>,
    pub skill_level: SkillLevel,
    pub jobs: HashSet<JobId>,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestActSkillPoints {
    pub job: HashSet<JobId>,
    pub points: u8
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TakeItemAct {
    pub item_id: ItemId,
    pub count: usize,
}

// Var1 per job?

#[derive(Debug, Deserialize, Serialize)]
pub struct RewardItemAct {
    pub item_id: ItemId,
    pub count: usize,
    pub interval: Option<Duration>,
    pub gender: Option<u8>,
    pub chance: Option<u8>,
    pub job: Option<u32>, // Is this a flag?
}


#[derive(Debug, Deserialize, Serialize)]
pub struct QuestActItem {
    pub take: Vec<TakeItemAct>,
    pub reward: Vec<RewardItemAct>,
    pub selection: Vec<RewardItemAct>,
    pub variance_items_1: Vec<RewardItemAct>,
    pub variance_items_2: Vec<RewardItemAct>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestAct {
    pub inc_exp: Option<u32>,
    pub inc_money: Option<Money>,
    pub inc_pop: Option<Pop>,
    pub inc_pet_tameness: Option<PetTameness>,
    pub inc_pet_speed: Option<u8>,
    pub pet_skill: Option<PetSkill>,
    pub next_quest: Option<QuestId>,
    //TODO lvl range?
    pub buff_item_id: Option<ItemId>,
    pub items: QuestActItem,
    pub skills: Vec<QuestActSkill>,
    pub skill_points: Vec<QuestActSkillPoints>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct QuestInfo {
    pub auto_complete: bool,
    pub auto_pre_complete: bool,
    pub auto_accept: bool,
    pub auto_start: bool,
    pub one_shot: bool,
    pub time_limit: Option<Duration>,
    pub area: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Quest {
    pub name: String,
    pub info: QuestInfo,
    pub start_req: QuestStartReq,
    pub end_req: QuestEndReq,
    pub start_act: QuestAct,
    pub end_act: QuestAct
}
