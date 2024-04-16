use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use chrono::Utc;
use sea_orm::prelude::DateTimeUtc;
use shroom_data::services::character::{
    ActiveQuest, FinishedQuest, MobQuestRecord, QuestData, QuestRecord, QuestSet,
};
use shroom_meta::{
    drops::QuestDropFlags,
    id::{FieldId, MobId, QuestId, ReactorId},
    quest::Quest,
    Meta, MetaService, QuestDataId,
};
use shroom_script::SessionCtx;

use super::{inv::CharInventory, stats::CharStats, Character};

#[derive(Debug, Default)]
pub struct CharQuestFlags<Id>(HashMap<Id, QuestDropFlags>);

impl<Id: Eq + Hash + Copy> CharQuestFlags<Id> {
    pub fn add(&mut self, ids: impl Iterator<Item = Id>, qid: QuestId) {
        for id in ids {
            self.0.entry(id).or_default().insert(qid);
        }
    }

    pub fn remove(&mut self, ids: impl Iterator<Item = Id>, qid: QuestId) {
        for id in ids {
            let f = self.0.entry(id).or_default();
            f.0.remove(&qid);
            if f.0.is_empty() {
                self.0.remove(&id);
            }
        }
    }

    pub fn get(&self, id: &Id) -> Option<&QuestDropFlags> {
        self.0.get(id)
    }
}

#[derive(Debug, Default)]
pub struct CharQuests {
    completed: HashMap<QuestId, DateTimeUtc>,
    active: HashMap<QuestId, QuestRecord>,
    updated_quests: HashSet<QuestId>,
    pub quest_data: HashMap<QuestDataId, Vec<u8>>,
    pub mob_quest_drop_flags: CharQuestFlags<MobId>,
    pub reactor_quest_drop_flags: CharQuestFlags<ReactorId>,
}

#[derive(Debug)]
pub enum QuestCheckError {
    PreQuest,
    Job,
    Field,
    Level,
    Inventory,
}

impl CharQuests {
    pub fn new() -> Self {
        Self {
            completed: HashMap::new(),
            active: HashMap::new(),
            quest_data: HashMap::new(),
            updated_quests: HashSet::new(),
            mob_quest_drop_flags: CharQuestFlags::default(),
            reactor_quest_drop_flags: CharQuestFlags::default(),
        }
    }

    fn add_active_quest(&mut self, qid: QuestId, qr: QuestRecord, meta: &'static MetaService) {
        if let Some(flags) = meta.get_quest_mob_drop_flags(qid) {
            self.mob_quest_drop_flags.add(flags.iter().cloned(), qid);
        }

        if let Some(flags) = meta.get_quest_reactor_drop_flags(qid) {
            self.reactor_quest_drop_flags
                .add(flags.iter().cloned(), qid);
        }

        self.active.insert(qid, qr);
    }

    fn remove_active_quest(&mut self, qid: QuestId, meta: &'static MetaService) {
        self.active.remove(&qid);

        if let Some(flags) = meta.get_quest_mob_drop_flags(qid) {
            self.mob_quest_drop_flags.remove(flags.iter().cloned(), qid);
        }

        if let Some(flags) = meta.get_quest_reactor_drop_flags(qid) {
            self.reactor_quest_drop_flags
                .remove(flags.iter().cloned(), qid);
        }
    }

    pub fn from_data(qs: QuestSet, meta: &'static MetaService) -> Self {
        let mut q = Self::new();
        let QuestSet {
            active,
            finished,
            data,
        } = qs;
        for cq in finished {
            q.completed.insert(cq.quest, cq.finished_at);
        }

        for aq in active {
            let _mq = meta.get_quest(aq.quest).unwrap();
            q.add_active_quest(aq.quest, aq.data, meta);
        }

        for d in data {
            q.quest_data.insert(d.quest, d.data);
        }

        q
    }

    pub fn to_data(&self) -> QuestSet {
        let active = self
            .active
            .iter()
            .map(|(qid, q)| ActiveQuest {
                quest: *qid,
                data: q.clone(),
                started_at: Utc::now(),
            })
            .collect();

        let finished = self
            .completed
            .iter()
            .map(|(qid, dt)| FinishedQuest {
                quest: *qid,
                finished_at: *dt,
                started_at: Utc::now(),
            })
            .collect();

        QuestSet {
            active,
            finished,
            data: self
                .quest_data
                .iter()
                .map(|(qid, data)| QuestData {
                    quest: *qid,
                    data: data.clone(),
                })
                .collect(),
        }
    }

    pub fn active_quest_records(&self) -> impl Iterator<Item = (QuestId, String)> + '_ {
        self.active
            .iter()
            .map(|(qid, qr)| (*qid, qr.to_qr_string()))
    }

    pub fn completed_records(&self) -> impl Iterator<Item = (QuestId, DateTimeUtc)> + '_ {
        self.completed.iter().map(|(qid, dt)| (*qid, *dt))
    }

    pub fn updates_states(&mut self) -> impl Iterator<Item = (QuestId, String)> + '_ {
        self.updated_quests
            .drain()
            .filter_map(|qid| self.active.get(&qid).map(|qr| (qid, qr.to_qr_string())))
    }

    pub fn try_start_quest(
        &mut self,
        id: QuestId,
        meta: &'static MetaService,
        field: &FieldId,
        stats: &CharStats,
        inv: &mut CharInventory,
    ) -> Result<(), QuestCheckError> {
        let quest = meta.get_quest(id).unwrap();
        let pre = &quest.start_req.req;
        let pre_act = &quest.start_act;

        // Check previous quests
        if pre
            .quest_states
            .iter()
            .any(|(qid, _)| !self.completed.contains_key(qid))
        {
            return Err(QuestCheckError::PreQuest);
        }

        // Check job
        if !pre.jobs.is_empty() && !pre.jobs.contains(stats.job()) {
            return Err(QuestCheckError::Job);
        }

        // Check field
        if !pre.field_enter.is_empty() && !pre.field_enter.contains(field) {
            return Err(QuestCheckError::Field);
        }

        // Check popularity
        if pre
            .min_pop
            .map(|v| (stats.fame as i16) < v.0)
            .unwrap_or(false)
        {
            return Err(QuestCheckError::Field);
        }

        // Check level range
        if !pre
            .level_range
            .contains(&shroom_meta::CharLevel(*stats.level()))
        {
            return Err(QuestCheckError::Level);
        }

        for (item, _count) in pre.items.items.iter() {
            // TODO: do count checks
            if !inv.contains_id(item).unwrap() {
                return Err(QuestCheckError::Inventory);
            }
        }

        let mob_req = quest
            .end_req
            .req
            .mobs
            .iter()
            .map(|(mob, count)| MobQuestRecord {
                mob_id: *mob,
                cur: 0,
                target: *count as u16,
            })
            .collect();

        for it in pre_act.items.reward.iter() {
            //TODO make this properly
            inv.try_add_stack_item(it.item_id, it.count, it.item_id.get_inv_type().unwrap())
                .unwrap();
        }

        self.add_active_quest(id, QuestRecord::Mob(mob_req), meta);
        self.updated_quests.insert(id);

        Ok(())
    }

    pub fn check_complete_quest(
        &self,
        _field: &FieldId,
        _stats: &CharStats,
        _inv: &mut CharInventory,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn reward_quest(
        _quest: QuestId,
        meta: Meta<Quest>,
        chr: &mut Character,
    ) -> anyhow::Result<()> {
        //TODO check check if quest can be completes

        // Give out rewards
        let reward = &meta.end_act;

        reward.inc_exp.inspect(|exp| chr.add_exp(*exp));
        if let Some(m) = reward.inc_money {
            chr.update_money(m.0);
        }

        if let Some(m) = reward.inc_pop {
            chr.stats.fame += m.0 as u16;
        }

        for reward in reward.items.reward.iter() {
            chr.add_items(reward.item_id, Some(reward.count))?;
        }

        Ok(())
    }

    pub fn complete_quest(
        &mut self,
        qid: QuestId,
        meta: &'static MetaService,
    ) -> anyhow::Result<()> {
        // Mark as completed
        self.remove_active_quest(qid, meta);
        self.completed.insert(qid, Utc::now());

        Ok(())
    }

    pub fn on_mob_killed(&mut self, mob: MobId, n: usize) {
        for active in self.active.iter_mut() {
            if active.1.update_mobs(mob, n) {
                self.updated_quests.insert(*active.0);
            }
        }
    }

    pub fn is_completed(&self, qid: QuestId) -> bool {
        self.completed.contains_key(&qid)
    }

    pub fn is_active(&self, qid: QuestId) -> bool {
        self.active.contains_key(&qid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_map() {
        let mut flags = CharQuestFlags::<u32>::default();

        flags.add(vec![1, 2, 3].into_iter(), QuestId(1));
        assert_eq!(flags.get(&1).unwrap().0.contains(&QuestId(1)), true);

        flags.remove(vec![1, 2].into_iter(), QuestId(1));
        assert!(flags.get(&1).is_none());
        assert!(flags.get(&2).is_none());
        assert_eq!(flags.get(&3).unwrap().0.contains(&QuestId(1)), true);
    }
}
