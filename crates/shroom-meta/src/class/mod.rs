use std::time::Duration;

use crate::{
    buffs::{
        char::{CharBuff, CharBuffStat}, mob::{self, BurnData, MobBuffStat}, SkillChance, SkillPerc
    }, shared::ElementAttribute, skill::SkillLevel, SkillMeta

};

pub mod archer;
pub mod mage;
pub mod warrior;
pub mod thief;
pub mod pirate;

#[derive(Debug)]
pub struct MobDebuff<T: MobBuffStat> {
    pub stat: T,
    pub dur: Duration,
    pub proc: SkillChance,
    _t: std::marker::PhantomData<T>,
}

impl MobDebuff<mob::Burned> {
    pub fn from_dot_skill(skill: SkillMeta, lvl: SkillLevel) -> Self {
        let dot = skill.dot.as_ref().expect("No dot data");
        let time = dot.time_dur(lvl);
        let iv = dot.interval_dur(lvl);
        let count = (time.as_millis() / iv.as_millis()) as u32;
        Self::new(
            dot.time_dur(lvl),
            BurnData {
                n_dmg: dot.damage(lvl) as u32,
                interval: dot.interval_dur(lvl),
                dot_count: count,
            },
            SkillChance::always(),
        )
    }
}

impl<T: MobBuffStat> MobDebuff<T> {
    pub fn new<U: Into<T>>(dur: Duration, stat: U, proc: SkillChance) -> Self {
        Self {
            dur,
            stat: stat.into(),
            proc,
            _t: std::marker::PhantomData,
        }
    }

    pub fn from_skill(skill: SkillMeta, lvl: SkillLevel, proc: SkillChance, stat: T) -> Self {
        Self::new(skill.time_dur(lvl), stat, proc)
    }
}

impl<T: MobBuffStat> MobDebuff<T>
where
    T: From<i16> + Into<i16>,
{
    pub fn from_skill_1(skill: SkillMeta, lvl: SkillLevel) -> Self {
        Self {
            dur: skill.time_dur(lvl),
            stat: 1.into(),
            proc: SkillChance(skill.prop(lvl)),
            _t: std::marker::PhantomData,
        }
    }

    pub fn from_skill_x(skill: SkillMeta, lvl: SkillLevel) -> Self {
        Self {
            dur: skill.time_dur(lvl),
            stat: skill.x(lvl).into(),
            proc: SkillChance(skill.prop(lvl)),
            _t: std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct AttackDebuffSkill<T: MobBuffStat> {
    pub attack_skill: AttackSkill,
    pub debuff: MobDebuff<T>,
}

#[derive(Debug)]
pub struct AttackBuffSkill<T: CharBuffStat> {
    pub attack_skill: AttackSkill,
    pub buff: CharBuff<T>,
}

#[derive(Debug)]
pub struct AttackSkill {
    pub mob_count: usize,
    pub damage_ratio: i16,
    pub range: usize,
    pub attack_count: usize,
}

impl AttackSkill {
    pub fn from_skill(skill: SkillMeta, lvl: SkillLevel) -> Self {
        Self {
            mob_count: skill.mob_count(lvl),
            damage_ratio: skill.damage(lvl),
            range: skill.range(lvl),
            attack_count: skill.attack_count(lvl),
        }
    }
}

#[derive(Debug)]
pub struct ElemAttackSkill {
    pub elem: ElementAttribute,
    pub skill: AttackSkill,
}

#[derive(Debug)]
pub struct ElemDebuffAttackSkill<T: MobBuffStat> {
    pub elem: ElementAttribute,
    pub skill: AttackSkill,
    pub debuff: MobDebuff<T>,
}

#[derive(Debug, Clone)]
pub enum SummonMoveAbility {
    Fly,
    Walk,
    Follow,
    CircleFollow,
    Escort,
    Jump,
    None
}


#[derive(Debug, Clone)]
pub enum SummonAssistType {
    None,
    Attack,
    Heal,
    AttackExtra1,
    AttackExtra2,
    ManualAttack
}

#[derive(Debug)]
pub struct SummonSkill {
    pub move_ability: SummonMoveAbility,
    pub assist_type: SummonAssistType,
    pub dur: Duration,
}

#[derive(Debug)]
pub enum HealBuff {
    Flat(i16),
    Ratio(SkillPerc),
}