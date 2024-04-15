use std::time::Duration;

use shroom_meta::{id::MobSkillId, mob::MobSkill, Meta};
use shroom_srv::{time::interval::Interval, GameTime};

use crate::life::char::stats::ClampedStat;

#[derive(Debug)]
pub struct MobSkillEntry {
    // Lazy load this
    pub meta: Meta<MobSkill>,
    pub skill_meta: Meta<shroom_meta::mob::MobSkill>,
    pub ix: u8,
    pub id: MobSkillId,
    pub level: u8,
    pub interval: Option<Interval>,
    /// Counter used for summons
    pub count: usize,
}

impl MobSkillEntry {
    pub fn try_cast(
        &mut self,
        t: GameTime,
        _hp: &ClampedStat<u32>,
        mp: &mut ClampedStat<u32>,
    ) -> bool {
        let info = self.meta.info();
        // Check hp threshold
        /*if info
            .hp_threshold
            .map(|threshold| threshold < hp)
            .unwrap_or(false)
        {
            return false;
        }*/

        // Check for mana
        if info.mp_cost as u32 > mp.value {
            return false;
        }

        if let Some(_dur) = info.interval.as_ref() {
            if !self
                .interval
                .get_or_insert_with(|| Interval::from_dur_next(Duration::from_secs(5)))
                .try_tick(t)
            {
                return false;
            }
        }

        // Check summon limit
        if let MobSkill::Summon(summ) = &self.meta {
            if summ.limit <= self.count {
                return false;
            }
        }

        mp.try_add(-(info.mp_cost as i32)).unwrap();

        true
    }
}
