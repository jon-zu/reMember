use std::{collections::BTreeMap, str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};

use shroom_meta::{
    buffs::mob::DamagedElemAttr,
    id::{FieldId, ItemId, MobId, SkillId},
    mob::*,
    shared::{ElemAttrList, ElementAttribute},
    MobMoveAbility, MoveAbility,
};

use super::{shroom_schemas::{self as sch, StrOrNum}, IntoBool, IntoNum};

pub type MobStat = i64;



impl TryFrom<&sch::MobInfoBan> for MobBanMap {
    type Error = anyhow::Error;

    fn try_from(value: &sch::MobInfoBan) -> Result<Self, Self::Error> {
        let target = value.ban_map.values().next().unwrap();
        Ok(Self {
            ban_msg: value.ban_msg.clone(),
            msg_ty: value.ban_msg_type,
            ban_type: MobBanType::try_from(value.ban_type.unwrap_or(0) as i8)?,
            target: BanMapTarget {
                field: FieldId(target.field.unwrap() as u32),
                portal: target.portal.clone(),
            },
        })
    }
}

impl TryFrom<&sch::MobInfoLoseItemValue> for MobLoseItem {
    type Error = anyhow::Error;

    fn try_from(value: &sch::MobInfoLoseItemValue) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ItemId(value.id.unwrap() as u32),
            amount: value.x.unwrap_or(1),
            not_drop: value.not_drop.into_bool(),
            lose_msg: value
                .lose_msg
                .as_ref()
                .map(|v| (value.lose_msg_type.unwrap(), v.clone())),
        })
    }
}

impl TryFrom<&sch::MobInfo> for MobDamagedBy {
    type Error = anyhow::Error;

    fn try_from(value: &sch::MobInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            mob: value.damaged_by_mob.into_bool(),
            selected_skill: value
                .damaged_by_selected_skill
                .values()
                .map(|v| SkillId(*v as u32))
                .collect(),
            selected_mob: value
                .damaged_by_selected_mob
                .values()
                .map(|v| MobId(*v as u32))
                .collect(),
        })
    }
}

impl TryFrom<&sch::MobInfoSkillValue> for MobSkillEntry {
    type Error = anyhow::Error;

    fn try_from(value: &sch::MobInfoSkillValue) -> Result<Self, Self::Error> {
        let effect_after = value.effect_after.unwrap_or(0);
        let effect_after = (effect_after != 0).then(|| Duration::from_millis(effect_after as u64));

        Ok(Self {
            action: value.action.unwrap(),
            effect_after,
            level: value.level.into_num(),
            skill: value.skill.into_num(),
            skill_after: value
                .skill_after
                .as_ref()
                .map(|v| Duration::from_millis(*v as u64)),
        })
    }
}

/*
<enum name="__unnamed" type="int" length="0x4" >
    <member name="MOBSPECIES_BEAST" value="0" />
    <member name="MOBSPECIES_DRAGON" value="1" />
    <member name="MOBSPECIES_UNDEAD" value="2" />
    <member name="MOBSPECIES_ETC" value="3" />
    <member name="MOBSPECIES_NO" value="4" />
</enum>

        <enum name="__unnamed" type="int" length="0x4" >
    <member name="MOBAPPEAR_NORMAL" value="-1" />
    <member name="MOBAPPEAR_REGEN" value="-2" />
    <member name="MOBAPPEAR_REVIVED" value="-3" />
    <member name="MOBAPPEAR_SUSPENDED" value="-4" />
    <member name="MOBAPPEAR_EFFECT" value="0" />
</enum>
<enum name="__unnamed" type="int" length="0x4" >
    <member name="MOBBANTYPE_NONE" value="0" />
    <member name="MOBBANTYPE_COLLISION" value="1" />
    <member name="MOBBANTYPE_USERATTACK" value="2" />
    <member name="MOBBANTYPE_MOBSKILL" value="-1" />
</enum>

        <enum name="__unnamed" type="int" length="0x4" >
    <member name="MOB_DAMAGERAND_NORMAL" value="0" />
    <member name="MOB_DAMAGERAND_FAKE" value="1" />
    <member name="MOB_DAMAGERAND_BLIND" value="2" />
    <member name="MOB_DAMAGERAND_BLOCKING" value="3" />
    <member name="MOB_DAMAGERAND_NO" value="4" />
</enum>

        <enum name="__unnamed" type="int" length="0x4" >
    <member name="Mob_AttackElem_None" value="0" />
    <member name="Mob_AttackElem_Ice" value="1" />
    <member name="Mob_AttackElem_Fire" value="2" />
    <member name="Mob_AttackElem_Light" value="3" />
    <member name="Mob_AttackElem_Poison" value="4" />
    <member name="Mob_AttackElem_Holy" value="5" />
    <member name="Mob_AttackElem_Dark" value="6" />
    <member name="Mob_AttackElem_Count" value="7" />
</enum>

<enum name="MobSelfDestruction::__unnamed" type="int" length="0x4" >
    <member name="MSD_NOBOMB" value="0" />
    <member name="MSD_HP" value="1" />
    <member name="MSD_FIRSTATTACK" value="2" />
    <member name="MSD_TIMEATTACK" value="4" />
</enum>
<enum name="MobAttackInfo::__unnamed" type="int" length="0x4" >
    <member name="AT_RANGE" value="0" />
    <member name="AT_SHOOT" value="1" />
    <member name="AT_PIERCE" value="2" />
</enum>


 */

impl TryFrom<&sch::Mob> for Mob {
    type Error = anyhow::Error;

    fn try_from(value: &sch::Mob) -> Result<Self, Self::Error> {
        let i = value.info.as_ref().unwrap();

        let has_fly = !value.fly.is_empty();
        let has_jump = !value.jump.is_empty();
        let has_move = !value.move_.is_empty();

        let move_ability = match (has_fly, has_jump, has_move) {
            (true, _, _) => MobMoveAbility::Fly,
            (_, true, false) => MobMoveAbility::Jump,
            (_, _, true) => MobMoveAbility::Move,
            _ => MobMoveAbility::Stop,
        };

        Ok(Self {
            id: MobId(0),
            revive: i
                .revive
                .values()
                .map(|v| MobId(v.into_num() as u32))
                .collect(),
            move_ability,
            max_hp: i.max_hp.into_num(),
            max_mp: i.max_mp.into_num(),
            acc: i.acc.into_num(),
            eva: i.eva.into_num(),
            hp_recovery: i.hp_recovery.into_num(),
            mp_recovery: i.mp_recovery.into_num(),
            speed: i.speed.into_num(),
            fly_speed: i.fly_speed.into_num(),
            chase_speed: i.chase_speed.into_num(),
            fixed_damage: i.fixed_damage.into_num(),
            link: i.link.as_ref().map(|v| MobId(v.into_num() as u32)),
            hp_tag_color: i.hp_tag_color,
            hp_tag_bg_color: i.hp_tag_bgcolor,

            elem_attr_list: ElemAttrList::from_str(i.elem_attr.as_deref().unwrap_or(""))?,

            damaged_by: MobDamagedBy::try_from(i)?,

            ma_dmg: i.ma_damage.into_num(),
            md_dmg: i.md_damage.into_num(),
            md_rate: i.md_rate.into_num(),
            pa_dmg: i.pa_damage.into_num(),
            pd_dmg: i.pd_damage.into_num(),
            pd_rate: i.pd_rate.into_num(),

            push_min_dmg: i.pushed.into_num(),

            level: i.level.into_num(),
            exp: i.exp.into_num(),
            category: MobCategory::try_from(i.category.into_num() as u8)?,
            summon_type: i.summon_type.into_num(),
            rate_item_drop_level: i.rare_item_drop_level.into_num(),
            point: i.point.into_num(),

            drop_item_period: i.drop_item_period.map(|v| Duration::from_secs(v as u64)),
            buff_reward: i.buff.as_ref().map(|v| SkillId(v.into_num() as u32)),

            ban_map: i.ban.as_ref().map(|v| MobBanMap::try_from(v)).transpose()?,
            lose_items: i
                .lose_item
                .values()
                .map(|v| MobLoseItem::try_from(v))
                .collect::<Result<_, _>>()?,
            skills: i
                .skill
                .iter()
                .map(|(k, v)| (k.parse().unwrap(), MobSkillEntry::try_from(v).unwrap()))
                .collect(),

            anger_gauge: i.anger_gauge,
            fs: i.fs.unwrap_or(1.),
            boss: i.boss.into_bool(),
            body_attack: i.body_attack.into_bool(),
            first_attack: i.first_attack.into_bool(),
            explosive_reward: i.explosive_reward.into_bool(),
            public_reward: i.public_reward.into_bool(),
            passable_by_teleport: !i.cant_pass_by_teleport.into_bool(),
            no_remove: i.do_not_remove.into_bool(),
            cannot_evade: i.cannot_evade.into_bool(),
            not_attack: i.not_attack.into_bool(),
            no_flip: i.no_flip.into_bool(),
            no_doom: i.no_doom.into_bool(),
            invincible: i.invincible.into_bool(),
            undead: i.undead.into_bool(),
            remove_on_miss: i.remove_on_miss.into_bool(),
            remove_quest: i.remove_quest.into_bool(),
            upper_most_layer: i.upper_most_layer.into_bool(),
            hp_gauge_hide: i.h_pgauge_hide.into_bool(),
            ignore_field_out: i.ignore_field_out.into_bool(),
            escort: i.escort.into_bool(),
            can_fly: !value.fly.is_empty(),
            can_jump: !value.jump.is_empty(),
            has_stop: !value.stop.is_empty(),
            has_stand: !value.stand.is_empty(),
        })
    }
}

impl TryFrom<&sch::MobSkillValueLevelValue> for MobSkillLevelData {
    type Error = anyhow::Error;

    fn try_from(value: &sch::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        let summons = [
            value._0.into_num(),
            value._1.into_num(),
            value._2.into_num(),
            value._3.into_num(),
            value._4.into_num(),
            value._5.into_num(),
            value._6.into_num(),
            value._7.into_num(),
            value._8.into_num(),
            value._9.into_num(),
            value._10.into_num(),
            value._11.into_num(),
        ]
        .iter()
        .filter_map(|v| {
            if *v == 0 {
                None
            } else {
                Some(MobId(*v as u32))
            }
        })
        .collect();

        Ok(Self {
            id: SkillId(0),
            count: value.count.as_ref().map(|v| v.into_num()),
            elem_attr: value
                .elem_attr
                .as_ref()
                .map(|v| v.as_str().try_into().unwrap()),
            hp: value.hp.as_ref().map(|v| v.into_num()),
            interval: value
                .interval
                .as_ref()
                .map(|v| Duration::from_secs(v.into_num() as u64)),
            limit: value
                .limit
                .as_ref()
                .map(|v| Duration::from_millis(v.into_num() as u64)),
            time: value
                .time
                .as_ref()
                .map(|v| Duration::from_millis(v.into_num() as u64)),
            mob_count: value.mob_count.as_ref().map(|v| v.into_num()),
            mp_con: value.mp_con.into_num(),
            random_target: value.random_target.into_bool(),
            x: value.x.as_ref().map(|v| v.into_num()),
            y: value.y.as_ref().map(|v| v.into_num()),
            summon_effect: value.summon_effect.as_ref().map(|v| v.into_num()),
            summon_mobs: summons,
        })
    }
}

impl TryFrom<&sch::MobSkillValue> for MobSkillData {
    type Error = anyhow::Error;

    fn try_from(value: &sch::MobSkillValue) -> Result<Self, Self::Error> {
        Ok(Self {
            id: SkillId(0),
            levels: value
                .level
                .iter()
                .map(|(k, v)| (k.parse().unwrap(), MobSkillLevelData::try_from(v).unwrap()))
                .collect(),
        })
    }
}
