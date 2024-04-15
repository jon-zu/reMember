use std::{collections::HashMap, time::Duration};

use shroom_meta::{
    id::{job_id::JobId, FieldId, ItemId, MobId, NpcId, QuestId, SkillId},
    quest::{
        Quest, QuestAct, QuestActItem, QuestActSkill, QuestActSkillPoints, QuestEndReq, QuestInfo,
        QuestItemReq, QuestMonsterBookReq, QuestRange, QuestReq, QuestStartReq, QuestState,
        RewardItemAct, TakeItemAct,
    },
    skill::SkillLevel, Money, Pop,
};

use super::{shroom_schemas::{self as sch, QuestActValueValue, QuestCheckValueValue}, IntoBool, IntoNum};

//TODO use indexmap for the json/sort by key

pub struct SchQuest<'a> {
    pub id: QuestId,
    pub act_0: &'a QuestActValueValue,
    pub act_1: &'a QuestActValueValue,
    pub check_0: &'a QuestCheckValueValue,
    pub check_1: &'a QuestCheckValueValue,
    pub info: &'a sch::QuestInfoValue,
}


fn map_range<T: From<u8>>(min: Option<i64>, max: Option<i64>) -> QuestRange<T> {
    match (min, max) {
        (Some(min), Some(max)) => QuestRange::Range(T::from(min as u8)..=T::from(max as u8)),
        (Some(min), None) => QuestRange::Min(T::from(min as u8)),
        (None, Some(max)) => QuestRange::Max(T::from(max as u8)),
        (None, None) => QuestRange::None,
    }
}

fn map_range_len(min: Option<i64>, max: Option<i64>) -> QuestRange<usize> {
    match (min, max) {
        (Some(min), Some(max)) => QuestRange::Range(min as usize..=max as usize),
        (Some(min), None) => QuestRange::Min(min as usize),
        (None, Some(max)) => QuestRange::Max(max as usize),
        (None, None) => QuestRange::None,
    }
}

fn map_interval(v: i64) -> Option<Duration> {
    match v {
        0 => None,
        v => Some(Duration::from_secs(v as u64 * 60)),
    }
}

pub fn map_to_list<T: Clone>(v: &HashMap<String, T>) -> Vec<(usize, T)> {
    let mut v: Vec<(usize, T)> = v
        .iter()
        .map(|(k, v)| (k.parse().unwrap(), v.clone()))
        .collect();
    v.sort_by_key(|(k, _)| *k);
    v
}

/*
fn map_quest_job_class(v: i64) -> Option<JobClass> {
    Some(match v {
        0 => {
            return None;
        }
        3279875 => JobClass::Warrior,
        1197073 => JobClass::Thief,
        5379077 => JobClass::Magician,
        1188873 => JobClass::Bowman,
        1213473 => JobClass::Pirate,
        _ => todo!(),
    })
}*/

impl TryFrom<&sch::QuestCheckValueValue> for QuestReq {
    type Error = anyhow::Error;

    fn try_from(value: &sch::QuestCheckValueValue) -> Result<Self, Self::Error> {
        let item_req = QuestItemReq {
            items: value
                .item
                .values()
                .map(|v| (ItemId(v.id.unwrap() as u32), v.count.unwrap_or(1) as usize))
                .collect(),
            equips_all: value
                .equip_all_need
                .values()
                .map(|v| ItemId(*v as u32))
                .collect(),
            equips_select: value
                .equip_select_need
                .values()
                .map(|v| ItemId(*v as u32))
                .collect(),
        };

        let mbook_req = QuestMonsterBookReq {
            min_total_cards: value.mbmin.map(|v| v as usize),
            cards: value
                .mbcard
                .values()
                .map(|v| (MobId(v.id.unwrap() as u32), map_range_len(v.min, None)))
                .collect(),
        };

        Ok(Self {
            jobs: value
                .job
                .values()
                .map(|v| JobId::try_from(*v as u16).unwrap())
                .collect(),
            field_enter: value
                .field_enter
                .values()
                .map(|v| FieldId(*v as u32))
                .collect(),
            level_range: map_range(value.lvmin.clone(), value.lvmax.clone()),
            taming_mob_level_range: map_range(
                value.tamingmoblevelmin.clone(),
                None, // TODO add max
            ),
            pet_tameness_range: map_range(
                value.pettamenessmin.clone(),
                None, // TODO add max
            ),
            quest_states: value
                .quest
                .values()
                .map(|v| {
                    (
                        QuestId(v.id.into_num() as u16),
                        QuestState(v.state.into_num() as u32),
                    )
                })
                .collect(),
            min_pop: value.pop.map(|v| Pop(v as i16)),
            items: item_req,
            skills: value
                .skill
                .values()
                .map(|v| {
                    (
                        SkillId(v.id.unwrap() as u32),
                        v.acquire.into_bool() as SkillLevel,
                    )
                })
                .collect(),
            mobs: map_to_list(&value.mob)
                .iter()
                .map(|(_, v)| (MobId(v.id.unwrap() as u32), v.count.unwrap() as usize))
                .collect(),
            mbook: mbook_req,
            worlds: map_range(
                value.worldmin.as_ref().map(|v| v.into_num()),
                value.worldmax.as_ref().map(|v| v.into_num()),
            ),
            days: value
                .day_of_week
                .keys()
                .map(|v| v.parse().unwrap())
                .collect(),
            repeat_per_day: value.day_by_day.into_bool(),
            end_money: value.endmeso.map(|v| Money(v as i32)),
            normal_auto_start: value.normal_auto_start.into_bool(),
        })
    }
}

impl TryFrom<&sch::QuestCheckValueValue> for QuestStartReq {
    type Error = anyhow::Error;

    fn try_from(value: &sch::QuestCheckValueValue) -> Result<Self, Self::Error> {
        Ok(Self {
            req: value.try_into()?,
            npc: value.npc.map(|v| NpcId(v as u32)),
            interval: map_interval(value.interval.into_num()),
            script: value.startscript.clone(),
        })
    }
}

impl TryFrom<&sch::QuestCheckValueValue> for QuestEndReq {
    type Error = anyhow::Error;

    fn try_from(value: &sch::QuestCheckValueValue) -> Result<Self, Self::Error> {
        Ok(Self {
            req: value.try_into()?,
            npc: value.npc.map(|v| NpcId(v as u32)),
            script: value.endscript.clone(),
        })
    }
}

impl TryFrom<&sch::QuestActValueValue> for QuestAct {
    type Error = anyhow::Error;

    fn try_from(value: &sch::QuestActValueValue) -> Result<Self, Self::Error> {
        let mut items = QuestActItem {
            take: Vec::new(),
            reward: Vec::new(),
            selection: Vec::new(),
            variance_items_1: Vec::new(),
            variance_items_2: Vec::new(),
        };

        for item in value.item.values() {
            let count = item.count.unwrap_or(1);
            let prop = item.prop.unwrap_or(100);
            let var = item.var.unwrap_or(0);
            let id = ItemId(item.id.unwrap() as u32);

            let gender = item.gender.map(|v| v as u8);
            let job = item.job.map(|v| v as u32);
            let interval = map_interval(item.period.into_num());

            match (count, prop, var) {
                (count, _, _) if count < 0 => {
                    items.take.push(TakeItemAct {
                        item_id: id,
                        count: count.unsigned_abs() as usize,
                    });
                }
                (_, prop, _) if prop < 0 => {
                    items.selection.push(RewardItemAct {
                        item_id: id,
                        count: count as usize,
                        interval,
                        gender: item.gender.map(|v| v as u8),
                        chance: None,
                        job,
                    });
                }
                (_, _, 1) => {
                    items.variance_items_1.push(RewardItemAct {
                        item_id: id,
                        count: count as usize,
                        interval,
                        gender,
                        chance: None,
                        job,
                    });
                }
                (_, _, 2) => {
                    items.variance_items_2.push(RewardItemAct {
                        item_id: id,
                        count: count as usize,
                        interval,
                        gender,
                        chance: None,
                        job,
                    });
                }
                _ => {
                    items.reward.push(RewardItemAct {
                        item_id: id,
                        count: count as usize,
                        interval,
                        gender,
                        chance: Some(prop as u8),
                        job,
                    });
                }
            }
        }

        Ok(Self {
            inc_exp: value.exp.map(|v| v as u32),
            inc_money: value.money.map(|v| Money(v as i32)),
            inc_pop: value.pop.map(|v| Pop(v as i16)),
            inc_pet_tameness: value.pettameness.map(|v| v as u8),
            inc_pet_speed: value.petspeed.map(|v| v as u8),
            pet_skill: value.petskill.map(|v| v as u8),
            next_quest: value.next_quest.map(|v| QuestId(v as u16)),
            buff_item_id: value.buff_item_id.map(|v| ItemId(v as u32)),
            skill_points: value
                .sp
                .values()
                .map(|v| QuestActSkillPoints {
                    job: v
                        .job
                        .values()
                        .map(|v| JobId::try_from(*v as u16).unwrap())
                        .collect(),
                    points: v.sp_value.into_num() as u8,
                })
                .collect(),
            skills: value
                .skill
                .values()
                .map(|v| QuestActSkill {
                    id: SkillId(v.id.unwrap() as u32),
                    master_level: v.master_level.map(|v| v as SkillLevel),
                    skill_level: v.skill_level.into_num() as SkillLevel,
                    jobs: v
                        .job
                        .values()
                        .map(|v| JobId::try_from(*v as u16).unwrap())
                        .collect(),
                })
                .collect(),
            items,
        })
    }
}

impl TryFrom<&sch::QuestInfoValue> for QuestInfo {
    type Error = anyhow::Error;

    fn try_from(value: &sch::QuestInfoValue) -> Result<Self, Self::Error> {
        Ok(Self {
            auto_complete: value.auto_complete.into_bool(),
            auto_pre_complete: value.auto_pre_complete.into_bool(),
            auto_accept: value.auto_accept.into_bool(),
            auto_start: value.auto_start.into_bool(),
            one_shot: value.one_shot.into_bool(),
            time_limit: value.time_limit.map(|v| Duration::from_secs(v as u64)),
            area: value.area.map(|v| v as u32),
        })
    }
}

impl<'a> TryFrom<SchQuest<'a>> for Quest {
    type Error = anyhow::Error;

    fn try_from(value: SchQuest<'a>) -> Result<Self, Self::Error> {
        let info = value.info;
        let name = info.name.as_ref().unwrap().clone();

        Ok(Quest {
            name,
            info: info.try_into()?,
            start_req: value.check_0.try_into()?,
            end_req: value.check_1.try_into()?,
            start_act: value.act_0.try_into()?,
            end_act: value.act_1.try_into()?,
        })
    }
}
