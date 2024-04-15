pub mod buffs;
pub mod pool;
pub mod skill;

use std::{collections::HashMap, num::Wrapping, time::Duration};

use itertools::Itertools;
use rand::Rng;
use shroom_meta::{
    buffs::mob::Seal, drops::QuestDropFlags, id::{CharacterId, FootholdId, MobId, MobSkillId, ObjectId}, mob::MobSkill, twod::Vec2, Meta, MetaService, MobMeta, MobMoveAbility, MoveActionType
};

use shroom_proto95::game::life::mob::{
    CarnivalTeam, LocalMobData, MobChangeControllerResp, MobCtrlTy,
    MobEnterFieldResp, MobInitData, MobLeaveFieldResp, MobLeaveType, MobOnStatResetResp,
    MobOnStatSetResp, MobSummonType, PartialMobTemporaryStat, ResetBurnInfo,
};

use shroom_srv::{
    game::pool::{CtrlPoolItem, PoolCtx, PoolItem},
    time::interval::Interval,
    GameTime,
};

use crate::{life::mob::buffs::MobBuffPacket, life::char::stats::ClampedStat};

use super::Obj;

use self::buffs::MobBuffs;

#[derive(Debug)]
pub struct MobSkillEntry {
    // Lazy load this
    pub meta: Meta<MobSkill>,
    pub skill_meta: Meta<shroom_meta::mob::MobSkillEntry>,
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

#[derive(Debug)]
pub struct Mob {
    pub meta: MobMeta,
    pub tmpl_id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub origin_fh: Option<FootholdId>,
    pub hp: ClampedStat<u32>,
    pub mp: ClampedStat<u32>,
    pub spawn_ix: Option<usize>,
    pub buffs: MobBuffs,
    pub skills: Vec<MobSkillEntry>,
    pub summon_ty: MobSummonType,
    pub quest_drop_flags: QuestDropFlags,
    move_action: MoveActionType,
    flip: bool,
    last_recovery: Interval,
    skill_interval: Interval,
    calc_damage_index: Wrapping<u8>,
    parent_link: Option<(ObjectId, usize)>,
    attackers: HashMap<CharacterId, u32>,
}

impl Mob {
    pub fn new_at(
        svc: &'static MetaService,
        tmpl_id: MobId,
        pos: Vec2,
        fh: FootholdId,
        summon_ty: Option<MobSummonType>,
    ) -> Self {
        // TODO don't reload that on respawn all the time
        let mob_meta = svc.get_mob_data(tmpl_id).unwrap();
        let skills = mob_meta
            .skills
            .iter()
            .filter_map(|(ix, skill)| {
                let ix = *ix as u8;
                let id = MobSkillId::try_from(skill.skill as u8).unwrap();
                let level = skill.level as u8;
                log::info!("Loading mob skill: {:?} - {skill:?}", id);
                let Some(meta) = svc.get_mob_skill_data(id, level) else {
                    log::info!("Missing mob skill: {id:?} {level}");
                    return None;
                };
                Some(MobSkillEntry {
                    meta,
                    skill_meta: skill,
                    ix,
                    id,
                    level: skill.level as u8,
                    interval: None,
                    count: 0,
                })
            })
            .collect_vec();

        let move_action = match mob_meta.move_ability {
            MobMoveAbility::Fly => MoveActionType::Fly1,
            MobMoveAbility::Stop => MoveActionType::Stand,
            _ => MoveActionType::Walk,
        };

        Self {
            meta: mob_meta,
            tmpl_id,
            pos,
            fh,
            origin_fh: Some(fh),
            hp: ClampedStat::maxed(mob_meta.max_hp as u32),
            mp: ClampedStat::maxed(mob_meta.max_mp as u32),
            spawn_ix: None,
            buffs: Default::default(),
            calc_damage_index: Wrapping(1),
            summon_ty: summon_ty.unwrap_or(MobSummonType::Normal(())),
            last_recovery: Interval::from_dur_next(Duration::from_secs(5)),
            skill_interval: Interval::from_dur_next(Duration::from_secs(3)),
            quest_drop_flags: Default::default(),
            skills,
            parent_link: None,
            move_action,
            flip: false,
            attackers: Default::default(),
        }
    }

    pub fn spawn(meta: &'static MetaService, sp: &SpawnPoint, spawn_ix: usize) -> Self {
        let mut mob = Self::new_at(meta, sp.tmpl_id, sp.pos, sp.fh, None);
        mob.spawn_ix = Some(spawn_ix);
        mob.flip = sp.flip;
        mob
    }

    pub fn damage(&mut self, dmg: u32, attacker: CharacterId) {
        *self.attackers.entry(attacker)
            .or_default() += dmg;
            
        self.hp.add_signed(-(dmg as i32));
    }

    pub fn is_dead(&self) -> bool {
        self.hp.value == 0
    }

    pub fn get_next_skill_ix(&mut self, t: GameTime) -> Option<usize> {
        // Check for seal
        if self.buffs.get::<Seal>().is_some() {
            return None;
        }

        if !self.skill_interval.try_tick(t) {
            return None;
        }

        // No skills
        if self.skills.is_empty() {
            return None;
        }

        // random start point
        let mut rng = rand::thread_rng();
        let n = self.skills.len();
        let off = rng.gen_range(0..n);
        for i in 0..self.skills.len() {
            let ix = (i + off) % n;
            if self.skills[ix].try_cast(t, &self.hp, &mut self.mp) {
                return Some(ix);
            }
        }
        None
    }

    pub fn update(&mut self, ctx: &mut impl PoolCtx, id: ObjectId) -> anyhow::Result<()> {
        let t = ctx.t();
        if let Some(removals) = self.buffs.update_expirations(t) {
            let reset_burns = removals.removed_burn.map(|b| {
                std::iter::once(ResetBurnInfo {
                    char_id: b.0,
                    skill_id: b.1,
                })
                .collect()
            });
            ctx.tx().broadcast_encode(MobOnStatResetResp {
                id,
                flags: removals.flags,
                calc_dmg_stat_ix: self.calc_damage_index.0,
                reset_burns,
                movement_affected: 0,
            })?;
        }

        let updates = self.buffs.take_updated();
        if !updates.is_empty() {
            log::info!("Doing set for mob: {} with updates: {:?}", id, updates);
            let stats = MobBuffPacket {
                buffs: &self.buffs,
                flags: updates,
                t,
            };
            let update = MobOnStatSetResp {
                id,
                stats,
                delay: 0,
                calc_dmg_stat_ix: self.calc_damage_index.0,
                movement_affected: 0,
            };

            ctx.tx().broadcast_encode(update)?;
        }

        if self.last_recovery.try_tick(t) {
            self.hp.add_signed(self.meta.hp_recovery as i32);
            self.mp.add_signed(self.meta.mp_recovery as i32);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct SpawnPoint {
    pub meta: MobMeta,
    pub tmpl_id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub origin_fh: Option<FootholdId>,
    pub flip: bool,
    pub delay: Option<Duration>
}

impl Mob {
    fn get_init_data(&self) -> MobInitData {
        MobInitData {
            pos: self.pos,
            move_action: self.flip as u8  | (2 * self.move_action as u8),
            fh: self.fh,
            origin_fh: self.origin_fh.unwrap_or(FootholdId(0)),
            summon_type: self.summon_ty.clone(),
            carnival_team: CarnivalTeam::None,
            effect_id: 0,
            phase: 0,
        }
    }

    fn get_mob_stats(&self) -> PartialMobTemporaryStat {
        PartialMobTemporaryStat {
            hdr: (),
            data: Default::default(),
        }
    }
}

impl PoolItem for Mob {
    type Id = ObjectId;

    type EnterMsg = MobEnterFieldResp;
    type LeaveMsg = MobLeaveFieldResp;
    type LeaveParam = MobLeaveType;

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        MobEnterFieldResp {
            id,
            calc_dmg_stat_ix: self.calc_damage_index.0,
            tmpl_id: self.tmpl_id,
            stats: self.get_mob_stats(),
            init_data: self.get_init_data(),
        }
    }

    fn leave_msg(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeaveMsg {
        MobLeaveFieldResp {
            id,
            leave_type: param,
        }
    }
}

impl CtrlPoolItem for Obj<Mob> {
    type ControllerId = CharacterId;
    type AssignControllerMsg = MobChangeControllerResp;

    fn enter_assign_ctrl_msg(
        &self,
        id: Self::Id,
        _ctrl: Self::ControllerId,
    ) -> Self::AssignControllerMsg {
        MobChangeControllerResp {
            ty: MobCtrlTy::ActiveInt,
            crc_seed: None.into(),
            id,
            calc_damage_index: self.calc_damage_index.0,
            local_mob_data: Some(LocalMobData {
                tmpl_id: self.tmpl_id,
                stats: self.get_mob_stats(),
                init: Some(self.get_init_data()).into(),
            })
            .into(),
        }
    }

    fn assign_ctrl_msg(
        &self,
        id: Self::Id,
        _ctrl: Self::ControllerId,
    ) -> Self::AssignControllerMsg {
        MobChangeControllerResp {
            ty: MobCtrlTy::ActiveInt,
            crc_seed: None.into(),
            id,
            calc_damage_index: self.calc_damage_index.0,
            local_mob_data: Some(LocalMobData {
                tmpl_id: self.tmpl_id,
                stats: self.get_mob_stats(),
                init: None.into(),
            })
            .into(),
        }
    }

    fn unassign_ctrl_msg(&self, id: Self::Id) -> Self::AssignControllerMsg {
        MobChangeControllerResp {
            ty: MobCtrlTy::None,
            crc_seed: None.into(),
            id,
            calc_damage_index: self.calc_damage_index.0,
            local_mob_data: None.into(),
        }
    }
}
