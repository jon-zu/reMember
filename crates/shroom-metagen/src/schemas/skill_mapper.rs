use std::{collections::BTreeMap, str::FromStr};

use shroom_meta::{
    id::SkillId,
    shared::{opt_map1, opt_map2, EvalExpr},
    skill::{
        eval, PassiveSkillData, Skill, SkillCost, SkillDotData, SkillStats, SkillSummonAttack,
        SkillSummonData, SkillSummonDieAttack,
    },
};

use super::{
    shroom_schemas::{SkillExpr, StrOrNum},
    SchemaCirc, SchemaRect,
};

pub use super::shroom_schemas as skill_schema;

impl From<&i64> for SkillExpr {
    fn from(value: &i64) -> Self {
        Self::Int(*value)
    }
}

impl From<&i64> for StrOrNum {
    fn from(value: &i64) -> Self {
        Self::Int(*value)
    }
}

impl From<&StrOrNum> for SkillExpr {
    fn from(value: &StrOrNum) -> Self {
        Self::Int(value.into())
    }
}

impl From<&SkillExpr> for EvalExpr {
    fn from(value: &SkillExpr) -> Self {
        match value {
            SkillExpr::Int(v) => Self::Num(*v as i32),
            SkillExpr::Expr(v) => Self::Expr(eval::Expr::from_str(v.as_str().trim()).unwrap()),
        }
    }
}

impl TryFrom<&skill_schema::SkillCommonInfo> for Option<SkillDotData> {
    type Error = anyhow::Error;

    fn try_from(value: &skill_schema::SkillCommonInfo) -> Result<Self, Self::Error> {
        Ok(value.dot.as_ref().map(|dot| SkillDotData {
            dmg: dot.into(),
            time: value.dot_time.as_ref().unwrap().into(),
            interval: value.dot_interval.as_ref().unwrap().into(),
        }))
    }
}

impl TryFrom<&skill_schema::SkillCommonInfo> for SkillCost {
    type Error = anyhow::Error;

    fn try_from(value: &skill_schema::SkillCommonInfo) -> Result<Self, Self::Error> {
        let bullets = value
            .bullet_consume
            .as_ref()
            .or(value.bullet_count.as_ref())
            .map(|v| v.into())
            .unwrap_or(0i64);
        let items = value.item_con_no.as_ref().map(|v| v.into()).unwrap_or(1u32);

        Ok(Self {
            hp: opt_map1(&value.hp_con)?,
            mp: opt_map1(&value.mp_con)?,
            money: opt_map1(&value.money_con)?,
            cooltime: opt_map1(&value.cooltime)?,
            item: value
                .item_con
                .as_ref()
                .or(value.item_consume.as_ref())
                .map(|item| (item.into(), items)),
            bullets,
        })
    }
}

impl TryFrom<&skill_schema::SkillCommonInfo> for SkillStats {
    type Error = anyhow::Error;

    fn try_from(value: &skill_schema::SkillCommonInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            attack_count: opt_map1(&value.attack_count)?,
            mob_count: opt_map1(&value.mob_count)?,
            damage: opt_map1(&value.damage)?,
            time: opt_map1(&value.time)?,
            sub_time: opt_map1(&value.sub_time)?,
            prop: opt_map1(&value.prop)?,
            sub_prop: opt_map1(&value.sub_prop)?,

            hp: opt_map1(&value.hp)?,
            mp: opt_map1(&value.mp)?,
            critical_ratio: opt_map1(&value.cr)?,
            pad: opt_map1(&value.pad)?,
            pdd: opt_map1(&value.pdd)?,
            mad: opt_map1(&value.mad)?,
            mdd: opt_map1(&value.mdd)?,
            accuracy: opt_map1(&value.acc)?,
            evasion: opt_map1(&value.eva)?,
            speed: opt_map1(&value.speed)?,
            jump: opt_map1(&value.jump)?,
            morph: value.morph.as_ref().map(|v| v.into()),
            mastery: opt_map1(&value.mastery)?,

            mad_x: opt_map1(&value.mad_x)?,
            pad_x: opt_map1(&value.pad_x)?,

            ignore_mob_p_ratio: opt_map1(&value.ignore_mobpdp_r)?,

            extra_max_hp: opt_map1(&value.emhp)?,
            extra_max_mp: opt_map1(&value.emmp)?,
            extra_pad: opt_map1(&value.epad)?,
            extra_pdd: opt_map1(&value.epdd)?,
            extra_mdd: opt_map1(&value.emdd)?,

            max_hp_ratio: opt_map1(&value.mhp_r)?,
            max_mp_ratio: opt_map1(&value.mmp_r)?,
            pdd_ratio: opt_map1(&value.pdd_r)?,
            mdd_ratio: opt_map1(&value.mdd_r)?,
            damage_ratio: opt_map1(&value.dam_r)?,
            critical_damage_min: opt_map1(&value.criticaldamage_min)?,
            critical_damage_max: opt_map1(&value.criticaldamage_max)?,
            evasion_ratio: opt_map1(&value.er)?,
            abnormal_status_res: opt_map1(&value.asr_r)?,
            attr_atk_status_res: opt_map1(&value.ter_r)?,
            money_ratio: opt_map1(&value.meso_r)?,
            exp_ratio: opt_map1(&value.exp_r)?,

            t: opt_map1(&value.t)?,
            u: opt_map1(&value.u)?,
            v: opt_map1(&value.v)?,
            w: opt_map1(&value.w)?,
            x: opt_map1(&value.x)?,
            y: opt_map1(&value.y)?,
            z: opt_map1(&value.z)?,
        })
    }
}

pub struct SkillWithId<'a>(pub SkillId, pub &'a skill_schema::SkillSkillValue);

impl<'a> TryFrom<SkillWithId<'a>> for Skill {
    type Error = anyhow::Error;
    fn try_from(v: SkillWithId<'a>) -> Result<Self, Self::Error> {
        let SkillWithId(id, value) = v;
        let cmn = value.common.as_ref().unwrap();

        let max_level = cmn
            .max_level
            .as_ref()
            .map(|v| v.into())
            .unwrap_or(value.level.len() as u32);

        let mut master_level = value.master_level.as_ref().map(|v| v.into());
        if id.has_master_level() && master_level.is_none() {
            master_level = Some(max_level);
        }

        let mut req_skills = BTreeMap::new();
        for (id, level) in value.req.iter() {
            req_skills.insert(id.parse()?, level.into());
        }

        let passive = value.psd.as_ref().map(|_| PassiveSkillData {
            skills: value
                .psd_skill
                .keys()
                .map(|id| id.as_str().parse().unwrap())
                .collect(),
        });

        Ok(Self {
            id,
            element_attr: opt_map2(&value.elem_attr)?,
            skill_type: value.skill_type.unwrap_or(0).try_into()?,
            dot: cmn.try_into()?,
            cost: cmn.try_into()?,
            has_affected: !value.affected.is_empty(),
            weapon: value.weapon.as_ref().map(|v| v.into()),
            sub_weapon: value.weapon.as_ref().map(|v| v.into()),
            req_skills,
            master_level: value.master_level.as_ref().map(|v| v.into()),
            max_level,
            invisible: value.invisible.as_ref().map(|v| v.into()).unwrap_or(false),
            disable: value.disable.as_ref().map(|v| v.into()).unwrap_or(false),
            stats: cmn.try_into()?,
            combat_orders: value.combat_orders.as_ref().map(|v| v.into()).unwrap_or(0),
            passive,
            summon: opt_map1(&value.summon)?,
        })
    }
}

impl TryFrom<&skill_schema::SkillSkillValueLevelValue> for skill_schema::SkillCommonInfo {
    type Error = anyhow::Error;
    fn try_from(v: &skill_schema::SkillSkillValueLevelValue) -> Result<Self, Self::Error> {
        Ok(Self {
            acc: opt_map1(&v.acc)?,
            action: None,
            asr_r: None,
            attack_count: opt_map1(&v.attack_count)?,
            bullet_consume: None,
            bullet_count: None,
            cooltime: opt_map1(&v.cooltime)?,
            cr: None,
            criticaldamage_max: opt_map1(&v.criticaldamage_max)?,
            criticaldamage_min: None,
            dam_r: None,
            // TODO fix damage
            damage: opt_map1(&v.damage)?,
            dot: opt_map1(&v.dot)?,
            dot_interval: opt_map1(&v.dot_interval)?,
            dot_time: opt_map1(&v.dot_time)?,
            emdd: None,
            emhp: None,
            emmp: None,
            epad: None,
            epdd: None,
            er: None,
            eva: opt_map1(&v.eva)?,
            exp_r: None,
            hp: None,
            hp_con: opt_map1(&v.hp_con)?,
            ignore_mobpdp_r: None,
            item_con: opt_map1(&v.item_con)?,
            item_con_no: opt_map1(&v.item_con_no)?,
            item_consume: None,
            jump: opt_map1(&v.jump)?,
            lt: opt_map1(&v.lt)?,
            mad: opt_map1(&v.mad)?,
            mad_x: None,
            mastery: opt_map1(&v.mastery)?,
            max_level: None,
            mdd: opt_map1(&v.mdd)?,
            mdd_r: None,
            meso_r: None,
            mhp_r: None,
            mmp_r: None,
            mob_count: opt_map1(&v.mob_count)?,
            money_con: None,
            morph: None,
            mp: None,
            mp_con: opt_map1(&v.mp_con)?,
            pad: opt_map1(&v.pad)?,
            pad_x: None,
            pdd: opt_map1(&v.pdd)?,
            pdd_r: None,
            prop: opt_map1(&v.prop)?,
            range: opt_map1(&v.range)?,
            rb: opt_map1(&v.rb)?,
            self_destruction: None,
            speed: opt_map1(&v.speed)?,
            sub_prop: None,
            sub_time: None,
            t: None,
            ter_r: None,
            time: opt_map1(&v.time)?,
            u: None,
            v: None,
            w: None,
            x: opt_map1(&v.x)?,
            y: opt_map1(&v.y)?,
            z: opt_map1(&v.z)?,
        })
    }
}

impl TryFrom<&skill_schema::SkillSkillValueSummon> for SkillSummonData {
    type Error = anyhow::Error;

    fn try_from(value: &skill_schema::SkillSkillValueSummon) -> Result<Self, Self::Error> {
        Ok(Self {
            fly: !value.fly.is_empty(),
            attack: value
                .attack1
                .as_ref()
                .map(|v| v.info.as_ref().unwrap().try_into().unwrap()),
            die_attack: value
                .die
                .as_ref()
                .and_then(|v| v.info.as_ref())
                .map(|v| v.try_into().unwrap()),
        })
    }
}

impl TryFrom<&skill_schema::SkillSkillValueSummonDieInfo> for SkillSummonDieAttack {
    type Error = anyhow::Error;

    fn try_from(value: &skill_schema::SkillSkillValueSummonDieInfo) -> Result<Self, Self::Error> {
        let (rect_range, circular_range) = if let Some(range) = value.range.as_ref() {
            (
                range
                    .lt
                    .clone()
                    .map(|v| SchemaRect(v, range.rb.clone().unwrap())),
                range
                    .sp
                    .clone()
                    .map(|v| SchemaCirc(v, range.r.unwrap() as u32)),
            )
        } else {
            (None, None)
        };

        Ok(Self {
            rect_range: rect_range.map(|v| v.into()),
            circular_range: circular_range.map(|v| v.into()),
            attack_after: value.attack_after.unwrap_or(0) as u32,
            mob_count: value.mob_count.unwrap_or(0) as u32,
        })
    }
}

impl TryFrom<&skill_schema::SkillSkillValueSummonAttack1Info> for SkillSummonAttack {
    type Error = anyhow::Error;

    fn try_from(
        value: &skill_schema::SkillSkillValueSummonAttack1Info,
    ) -> Result<Self, Self::Error> {
        let (rect_range, circular_range) = if let Some(range) = value.range.as_ref() {
            (
                range
                    .lt
                    .clone()
                    .map(|v| SchemaRect(v, range.rb.clone().unwrap())),
                range
                    .sp
                    .clone()
                    .map(|v| SchemaCirc(v, range.r.unwrap() as u32)),
            )
        } else {
            (None, None)
        };

        Ok(Self {
            rect_range: rect_range.map(|v| v.into()),
            circular_range: circular_range.map(|v| v.into()),
            ty: value.type_.unwrap_or(0) as u32,
            attack_after: value.attack_after.unwrap_or(0) as u32,
            mob_count: value.mob_count.unwrap_or(0) as u32,
        })
    }
}
