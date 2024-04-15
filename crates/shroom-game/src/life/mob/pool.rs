use std::time::Duration;

use rand::thread_rng;
use shroom_meta::{
    buffs::{char::CharBuffStat, mob::MobBuffStat, SkillChance},
    field::FieldMob,
    id::{BuffId, CharacterId, MobId, ObjectId, SkillId},
    mob::{MobCharBuffSkill, MobHealSkill, MobSkill, MobSkillRange},
    twod::{Box2, Vec2},
    MetaService, MobMeta,
};
use shroom_pkt::util::packet_buf::PacketBuf;
use shroom_proto95::game::life::mob::{
    LocalMobData, MobAffectedResp, MobChangeControllerResp, MobCtrlTy, MobDamagedResp,
    MobHPIndicatorResp, MobLeaveType, MobMoveCtrlAckResp, MobMoveFlags, MobMoveReq, MobMoveResp,
    MobSkillDelayResp, MobSummonType,
};
use shroom_srv::{
    act::room::RoomSessionId,
    game::pool::{CtrlPool, PoolCtx},
    util::DelayQueue,
    GameTime,
};

use crate::{
    field::{AttackerContext, CharSetRef, FieldHandler},
    game::GameMessage,
};

use super::Obj;

use super::{buffs::MobApplyDebuff, Mob, SpawnPoint};

#[derive(Debug)]
pub struct MobPool {
    meta: &'static MetaService,
    pool: CtrlPool<Obj<Mob>>,
    spawn_points: Vec<SpawnPoint>,
    respawn_queue: DelayQueue<usize>,
    mob_skill_queue: DelayQueue<(ObjectId, usize)>,
    soft_cap: usize,
}

impl MobPool {
    pub fn from_spawns(
        meta: &'static MetaService,
        t: GameTime,
        spawns: impl Iterator<Item = (MobId, MobMeta, &'static FieldMob)>,
    ) -> Self {
        let mobs = CtrlPool::default();
        let mut spawn_points = Vec::new();
        for (id, meta, mob) in spawns {
            spawn_points.push(SpawnPoint {
                meta,
                tmpl_id: id,
                pos: mob.pos,
                fh: mob.fh,
                origin_fh: Some(mob.fh),
                flip: mob.flip,
                delay: mob.respawn_time,
            });
        }

        let mut respawn_queue = DelayQueue::default();
        for sp in spawn_points.iter().enumerate() {
            respawn_queue.push(sp.0, t + sp.1.delay.unwrap_or(Duration::ZERO));
        }

        let soft_cap = ((spawn_points.len() * 75) / 100).max(1);

        Self {
            meta,
            pool: mobs,
            spawn_points,
            respawn_queue,
            mob_skill_queue: DelayQueue::default(),
            soft_cap,
        }
    }

    pub fn respawn(&mut self, ctx: &mut impl PoolCtx<Id = CharacterId>) -> anyhow::Result<()> {
        if self.respawn_queue.is_empty() {
            return Ok(());
        }

        // TODO use a buffer
        let mut n = self.pool.pool.0.len();
        while n < self.soft_cap {
            let Some(ix) = self.respawn_queue.pop(ctx.t()) else {
                break;
            };

            let spawn = &self.spawn_points[ix];
            log::info!("Respawned mob {}", spawn.tmpl_id);
            self.spawn(ctx, Mob::spawn(self.meta, spawn, ix))?;
            n += 1;
        }

        Ok(())
    }

    fn iter_mut_range(&mut self, range: Option<Box2>) -> impl Iterator<Item = &mut Obj<Mob>> {
        self.pool.pool.0.values_mut().filter(move |mob| {
            range
                .map(|r| r.contains(mob.pos.to_point()))
                .unwrap_or(true)
        })
    }

    fn apply_partizan_heal(
        &mut self,
        pos: Vec2,
        range: Option<&MobSkillRange>,
        buff_id: BuffId,
        heal: &MobHealSkill,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
    ) -> anyhow::Result<()> {
        let heal = heal.heal_amount(thread_rng());
        self.iter_mut_range(range.map(|r| r.0.translate(pos)))
            .for_each(|mob| {
                mob.hp.add_signed(heal as i32);
                ctx.tx()
                    .broadcast_encode(MobAffectedResp {
                        id: mob.id,
                        buff_id,
                        start_delay: Duration::ZERO.into(),
                    })
                    .unwrap();
                // TODO broadcast hp
            });
        Ok(())
    }

    fn apply_partizan_buff<T: MobBuffStat + Clone>(
        &mut self,
        pos: Vec2,
        range: Option<&MobSkillRange>,
        buff_id: BuffId,
        b: &shroom_meta::buffs::mob::MobBuff<T>,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
    ) -> anyhow::Result<()> {
        self.iter_mut_range(range.map(|r| r.0.translate(pos)))
            .for_each(|mob| {
                mob.buffs.set_if_not_exists(
                    ctx.t(),
                    shroom_meta::buffs::mob::MobBuff::new(
                        buff_id,
                        b.data.clone(),
                        b.dur,
                        CharacterId(0),
                    ),
                );
            });
        Ok(())
    }

    fn apply_char_buff<T: CharBuffStat + Clone>(
        chars: &mut CharSetRef,
        mob: &Mob,
        d: &MobCharBuffSkill<T>,
        buff_id: BuffId,
        t: GameTime,
    ) -> anyhow::Result<()> {
        let rect = d.range.as_ref().map(|r| r.0.translate(mob.pos));
        for char in chars.iter_mut_rect(rect) {
            if let Some(chance) = d.skill_chance {
                if !chance.proc() {
                    continue;
                }
            }

            let b = &d.stat;
            char.buffs.set(
                t,
                shroom_meta::buffs::char::CharBuff::new(buff_id, b.data.clone(), b.dur),
            );
        }
        Ok(())
    }

    fn apply_char_dispel(
        chars: &mut CharSetRef,
        pos: Vec2,
        range: Option<&MobSkillRange>,
        chance: Option<&SkillChance>,
    ) -> anyhow::Result<()> {
        let rect = range.map(|r| r.0.translate(pos));
        for char in chars.iter_mut_rect(rect) {
            if let Some(chance) = chance {
                if !chance.proc() {
                    continue;
                }
            }

            char.buffs.cancel_all_skills();
        }
        Ok(())
    }

    pub fn on_tick(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
        chars: &mut CharSetRef,
    ) -> anyhow::Result<()> {
        self.respawn(ctx)?;

        while let Some((mob_id, skill_ix)) = self.mob_skill_queue.pop(ctx.t()) {
            // Check if mob still exists
            let Some(mob) = self.pool.get(&mob_id) else {
                continue;
            };

            let skill = &mob.skills[skill_ix];
            let meta = &skill.meta;
            let sid = skill.id;
            let lvl = skill.level;
            let pos = mob.pos;
            let fh = mob.fh;

            let buff_id = BuffId::MobSkill(sid, lvl);

            match meta {
                MobSkill::Buff(b) => {
                    let mob = self.pool.get_mut(&mob_id).unwrap();
                    b.apply_debuff(&mut mob.buffs, ctx.t(), buff_id, CharacterId(0))
                }
                MobSkill::PartizanBuff(b) => match b {
                    //TODO filter the mob itself
                    shroom_meta::mob::MobPartizanBuffSkillData::PowerUpM(b) => {
                        self.apply_partizan_buff(pos, b.range.as_ref(), buff_id, &b.stat, ctx)?;
                    }
                    shroom_meta::mob::MobPartizanBuffSkillData::MagicUpM(b) => {
                        self.apply_partizan_buff(pos, b.range.as_ref(), buff_id, &b.stat, ctx)?;
                    }
                    shroom_meta::mob::MobPartizanBuffSkillData::PGuardUpM(b) => {
                        self.apply_partizan_buff(pos, b.range.as_ref(), buff_id, &b.stat, ctx)?;
                    }
                    shroom_meta::mob::MobPartizanBuffSkillData::MGuardUpM(b) => {
                        self.apply_partizan_buff(pos, b.range.as_ref(), buff_id, &b.stat, ctx)?;
                    }
                    shroom_meta::mob::MobPartizanBuffSkillData::HasteM(b) => {
                        self.apply_partizan_buff(pos, b.range.as_ref(), buff_id, &b.stat, ctx)?;
                    }
                    shroom_meta::mob::MobPartizanBuffSkillData::HealM(b) => {
                        self.apply_partizan_heal(pos, None, buff_id, b, ctx)?;
                    }
                },
                MobSkill::CharBuff(b) => {
                    match b {
                        shroom_meta::mob::MobCharBuffSkillData::Seal(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Darkness(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Weakness(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Stun(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Curse(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Poison(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Slow(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Dispel(_d) => {
                            Self::apply_char_dispel(chars, pos, None, None)?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Attract(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::BanMap(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::ReverseInput(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Fear(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                        shroom_meta::mob::MobCharBuffSkillData::Frozen(d) => {
                            Self::apply_char_buff(chars, mob, d, buff_id, ctx.t())?;
                        }
                    }
                    //TODO
                }
                MobSkill::Summon(summon) => {
                    log::info!("Handling mob summon: {}", summon.mobs.len());
                    for summ_id in &summon.mobs {
                        let mut mob = Mob::new_at(
                            self.meta,
                            *summ_id,
                            pos,
                            fh,
                            Some(MobSummonType::Effect(summon.summon_effect as u32)),
                        );
                        mob.parent_link = Some((mob_id, skill_ix));
                        let id = self.pool.insert(ctx, Obj::next(mob))?;
                        self.pool.get_mut(&id).unwrap().summon_ty = MobSummonType::Regen(());
                    }

                    self.pool.must_get_mut(&mob_id).unwrap().skills[skill_ix].count +=
                        summon.mobs.len();
                }
                _ => {
                    log::info!("Unknown mob skill: {:?}", meta);
                }
            }
        }

        for (&id, mob) in self.pool.pool.0.iter_mut() {
            mob.update(ctx, id)?;
        }

        Ok(())
    }

    pub fn remove(
        &mut self,
        ctx: &mut impl PoolCtx,
        id: ObjectId,
        leave_type: MobLeaveType,
    ) -> Option<Mob> {
        self.pool
            .remove(ctx, &id, leave_type)
            .unwrap()
            .map(|m| m.item)
    }

    pub fn spawn(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
        mob: Mob,
    ) -> anyhow::Result<()> {
        let id = self.pool.insert(ctx, Obj::next(mob))?;
        self.pool.get_mut(&id).unwrap().summon_ty = MobSummonType::Regen(());
        Ok(())
    }

    pub fn set_mob_aggro(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
        id: ObjectId,
        ctrl: CharacterId,
    ) -> anyhow::Result<()> {
        let Ok(mob) = self.pool.must_get_mut(&id) else {
            return Ok(());
        };
        mob.calc_damage_index += 1;

        ctx.tx().send_to_encode(
            ctrl,
            MobChangeControllerResp {
                ty: MobCtrlTy::ActiveReq,
                crc_seed: None.into(),
                id,
                calc_damage_index: mob.calc_damage_index.0,
                local_mob_data: Some(LocalMobData {
                    tmpl_id: mob.tmpl_id,
                    stats: mob.get_mob_stats(),
                    init: None.into(),
                })
                .into(),
            },
        )?;

        Ok(())
    }

    pub fn handle_move(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
        id: ObjectId,
        req: MobMoveReq,
        controller: RoomSessionId<FieldHandler>,
    ) -> anyhow::Result<()> {
        self.pool.check_controller(&controller)?;
        let Some(mob) = self.pool.get_mut(&id) else {
            return Ok(());
        };

        let last_pos_fh = req.move_path.path.get_last_pos_fh();
        if let Some((pos, fh)) = last_pos_fh {
            //TODO post mob state to msg state here
            mob.pos = pos;
            mob.fh = fh.unwrap_or(mob.fh);
        }

        let skill_data = req.get_skill_data().unwrap_or_default();
        let mut ack_skill_id = 0;
        let mut ack_skill_lvl = 0;

        let next_atk_possible = req.flag.contains(MobMoveFlags::CAN_ATTACK);
        if next_atk_possible && req.action_dir.action.is_attack() {
            if let Some(next_skill_ix) = mob.get_next_skill_ix(ctx.t()) {
                let next_skill = &mob.skills[next_skill_ix];
                // Schedule skill
                if let Some(effect_after) = next_skill.skill_meta.effect_after {
                    ctx.tx().send_to_encode(
                        controller,
                        MobSkillDelayResp {
                            id,
                            skill_delay: effect_after.into(),
                            skill_id: next_skill.skill_meta.skill as u32,
                            slv: next_skill.level as u32,
                            skill_option: next_skill.skill_meta.action as u32,
                        },
                    )?;
                }

                self.mob_skill_queue.push(
                    (id, next_skill_ix),
                    ctx.t() + next_skill.skill_meta.effect_after.unwrap_or(Duration::ZERO),
                );

                ack_skill_id = next_skill.id as u8;
                ack_skill_lvl = next_skill.level;
            }
        }

        ctx.tx().broadcast_filter_encode(
            MobMoveResp {
                id,
                not_force_landing: false,
                not_change_action: false,
                next_atk_possible,
                action_dir: req.action_dir,
                data: skill_data,
                multi_target: req.multi_target,
                rand_time: req.rand_time,
                move_path: req.move_path.path,
            },
            controller,
        )?;
        ctx.tx().send_to_encode(
            controller,
            MobMoveCtrlAckResp {
                id,
                ctrl_sn: req.ctrl_sn,
                next_atk_possible,
                mp: mob.mp.value as u16,
                skill_id: ack_skill_id,
                slv: ack_skill_lvl,
            },
        )?;
        Ok(())
    }

    pub fn attack(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId, Msg = GameMessage>,
        attacker: &impl AttackerContext,
        id: ObjectId,
        dmg: u32,
    ) -> anyhow::Result<Option<Mob>> {
        //TODO
        let mob = self.pool.get_mut(&id).unwrap();
        mob.damage(dmg, attacker.attacker());
        if let Some(flags) = attacker.get_mob_quest_flag(mob.tmpl_id) {
            mob.quest_drop_flags.union(flags);
        }

        let pkt = MobDamagedResp {
            id,
            ty: 0,
            dec_hp: dmg,
            hp: mob.hp.value,
            max_hp: mob.meta.max_hp as u32,
        };

        ctx.tx().broadcast_filter_encode(pkt, attacker.attacker())?;
        ctx.tx().send_to_encode(
            attacker.attacker(),
            MobHPIndicatorResp {
                id,
                hp_perc: mob.hp.ratio100(),
            },
        )?;

        if mob.is_dead() {
            Ok(Some(self.kill(ctx, id)?))
        } else {
            Ok(None)
        }
    }

    pub fn kill(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId, Msg = GameMessage>,
        id: ObjectId,
    ) -> anyhow::Result<Mob> {
        let mob = self.remove(ctx, id, MobLeaveType::Etc(())).unwrap();
        if let Some(ix) = mob.spawn_ix {
            let spawn = &self.spawn_points[ix];
            if let Some(delay) = spawn.delay {
                self.respawn_queue.push(ix, ctx.t() + delay);
            }
        }

        if let Some((parent, skill_ix)) = mob.parent_link {
            let parent = self.pool.get_mut(&parent).unwrap();
            parent.skills[skill_ix].count -= 1;
        }

        let exp = mob.meta.exp as u32;
        // TODO do something with the damage
        for (atk, _) in mob.attackers.iter() {
            ctx.tx()
                .send_to(*atk, GameMessage::MobExp(mob.meta.id, exp, 100))
        }

        Ok(mob)
    }

    pub fn debuff(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
        id: ObjectId,
        debuff: &dyn MobApplyDebuff,
        attacker: CharacterId,
        src: SkillId,
    ) -> anyhow::Result<()> {
        let mob = self.pool.get_mut(&id).unwrap();
        debuff.apply_debuff(&mut mob.buffs, ctx.t(), src.into(), attacker);
        Ok(())
    }

    pub fn on_enter(
        &mut self,
        id: CharacterId,
        buf: &mut PacketBuf,
        t: GameTime,
    ) -> anyhow::Result<()> {
        self.pool.on_enter(id, buf, t)
    }

    pub fn update_controller(
        &mut self,
        ctx: &mut impl PoolCtx<Id = CharacterId>,
        old_ctrl: Option<CharacterId>,
        ctrl: Option<CharacterId>,
        update: bool,
    ) -> anyhow::Result<()> {
        self.pool.update_controller(ctx, old_ctrl, ctrl, update)
    }
}
