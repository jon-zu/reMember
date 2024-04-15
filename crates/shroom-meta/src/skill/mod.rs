pub mod eval;


use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use crate::id::SkillId;
use crate::shared;
use crate::shared::{ElementAttribute, EvalExpr};

pub type SkillLevel = u8;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDotData {
    pub dmg: EvalExpr,
    pub time: EvalExpr,
    pub interval: EvalExpr,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum SkillType {
    Normal = 0, // TODO: does not exist in the files, but we default to it
    Mastery = 1,
    Booster = 2,
    FinalAttack = 3,
    DarkSight = 4, // TODO confirm
}

impl SkillType {
    pub fn is_mastery(&self) -> bool {
        matches!(self, Self::Mastery)
    }

    pub fn is_booster(&self) -> bool {
        matches!(self, Self::Booster)
    }
}

#[derive(Debug)]
pub struct StanceData {
    pub knockback_resistance_prop: i16,
}

macro_rules! skill_eval {
    ($stat:ident) => {
        pub fn $stat(&self, lvl: SkillLevel) -> i16 {
            self.stats.$stat.as_ref().unwrap().eval(lvl as i32) as i16
        }
    };
}

impl Skill {
    pub fn is_sharp_eyes(&self) -> bool {
        self.stats.critical_damage_max.is_some()
    }

    pub fn is_maple_warrior(&self) -> bool {
        //TODO: 1121000 etc
        self.has_affected && self.stats.time.is_some() && self.stats.x.is_some()
    }

    pub fn is_echo_of_hero(&self) -> bool {
        self.id.0 % 1000000 == 1005
    }

    pub fn is_passive(&self) -> bool {
        self.passive.is_some()
    }

    pub fn is_hyperbody(&self) -> bool {
        self.id.0 == 1301007
            || self.id.0 % 10000 == 8003
            || self.id.0 == 9101008
            || self.id.0 == 9001008
    }

    pub fn is_heros_will(&self) -> bool {
        matches!(self.id.0, 1121011 | 1221012 | 1321010)
    }

    skill_eval!(max_hp_ratio);
    skill_eval!(max_mp_ratio);
    skill_eval!(damage);
    skill_eval!(damage_ratio);
    skill_eval!(x);
    skill_eval!(y);
    skill_eval!(z);
    skill_eval!(pad);
    skill_eval!(pdd);
    skill_eval!(mdd);
    //skill_eval!(prop);
    skill_eval!(accuracy);
    skill_eval!(evasion);
    skill_eval!(evasion_ratio);
    skill_eval!(critical_ratio);
    skill_eval!(critical_damage_max);
    skill_eval!(extra_pad);
    skill_eval!(extra_pdd);
    skill_eval!(extra_mdd);
    skill_eval!(mad);
    skill_eval!(hp);
    skill_eval!(mp);
    skill_eval!(speed);
    skill_eval!(jump);

    pub fn prop(&self, lvl: SkillLevel) -> i16 {
        self.stats.prop.as_ref().map(|p| p.eval(lvl as i32)).unwrap_or(100) as i16
    }

    pub fn morph(&self, _lvl: SkillLevel) -> i16 {
        self.stats.morph.unwrap() as i16
    }

    pub fn mob_count(&self, lvl: SkillLevel) -> usize {
        self.stats
            .mob_count
            .as_ref()
            .map(|e| e.eval(lvl as i32))
            .unwrap_or(1)
            as usize
    }

    pub fn attack_count(&self, lvl: SkillLevel) -> usize {
        self.stats
            .attack_count
            .as_ref()
            .map(|e| e.eval(lvl as i32))
            .unwrap_or(1)
            as usize
    }

    pub fn range(&self, _lvl: SkillLevel) -> usize {
        //TODO
        1000
    }

    pub fn time_dur(&self, lvl: SkillLevel) -> Duration {
        Duration::from_secs(self.stats.time.as_ref().unwrap().eval(lvl as i32) as u64)
    }

    pub fn time_dur_min(&self, lvl: SkillLevel) -> Duration {
        Duration::from_secs(self.stats.time.as_ref().unwrap().eval(lvl as i32) as u64)
    }


    pub fn sub_time_dur(&self, lvl: SkillLevel) -> Duration {
        Duration::from_secs(self.stats.sub_time.as_ref().unwrap().eval(lvl as i32) as u64)
    }
}

impl SkillDotData {
    pub fn time_dur(&self, lvl: SkillLevel) -> Duration {
        Duration::from_secs(self.time.eval(lvl as i32) as u64)
    }

    pub fn interval_dur(&self, lvl: SkillLevel) -> Duration {
        Duration::from_secs(self.interval.eval(lvl as i32) as u64)
    }

    pub fn damage(&self, lvl: SkillLevel) -> i16 {
        self.dmg.eval(lvl as i32) as i16
    }
}

impl TryFrom<i64> for SkillType {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Normal,
            1 => Self::Mastery,
            2 => Self::Booster,
            3 => Self::FinalAttack,
            4 => Self::DarkSight,
            _ => anyhow::bail!("Invalid skill type: {}", value),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillCost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub money: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooltime: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<(i64, u32)>,
    pub bullets: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack_count: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_time: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_prop: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evasion: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub morph: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mastery: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad_x: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad_x: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_mob_p_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_max_hp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_max_mp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_pad: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_pdd: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_mdd: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_hp_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_mp_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub money_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_damage_min: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_damage_max: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evasion_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abnormal_status_res: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attr_atk_status_res: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<EvalExpr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassiveSkillData {
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub skills: BTreeSet<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillSummonAttack {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_range: Option<shared::Rect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circular_range: Option<shared::Circ>,
    pub ty: u32,
    pub attack_after: u32,
    pub mob_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillSummonDieAttack {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_range: Option<shared::Rect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circular_range: Option<shared::Circ>,
    pub attack_after: u32,
    pub mob_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillSummonData {
    pub fly: bool,
    pub attack: Option<SkillSummonAttack>,
    pub die_attack: Option<SkillSummonDieAttack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: SkillId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_attr: Option<ElementAttribute>,
    pub invisible: bool,
    pub disable: bool,
    pub has_affected: bool,
    pub skill_type: SkillType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot: Option<SkillDotData>,
    pub cost: SkillCost,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_weapon: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_level: Option<u32>,
    pub max_level: u32,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub req_skills: BTreeMap<u32, u32>,
    pub stats: SkillStats,
    pub combat_orders: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passive: Option<PassiveSkillData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summon: Option<SkillSummonData>,
}
