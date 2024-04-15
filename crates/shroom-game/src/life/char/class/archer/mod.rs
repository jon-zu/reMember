use anyhow::Context;
use shroom_meta::{
    class::archer::ArcherSkillData,
    id::SkillId,
};


use crate::{
    life::{char::summon::Summon, mob::buffs::MobApplyDebuff},
    life::char::{buffs::ApplyBuff, Character},
};

use super::{AttackData, ClassContext, UseSkillData};

pub struct ArcherHandler;

impl ArcherHandler {
    pub fn handle_skill(&mut self, mut ctx: ClassContext, req: &UseSkillData) -> anyhow::Result<()> {
        let skill_id = req.skill_id;
        log::info!("Handling skill: {skill_id:?}");

        let skill = ctx.chr.skills.get_leveled(skill_id)?;
        let slvl = skill.level as u8;
        let Ok(skill) = ArcherSkillData::from_skill(skill.meta, skill.level as u8) else {
            log::info!("Unknown archer skill: {}", skill_id);
            return Ok(());
        };

        let buffs = &mut ctx.chr.buffs;
        match skill {
            ArcherSkillData::SharpEyes(d) => d.apply_buff(req.t, buffs),
            ArcherSkillData::Focus(d) => d.apply_buff(req.t, buffs),
            ArcherSkillData::Booster(d) => d.apply_buff(req.t, buffs),
            ArcherSkillData::SoulArrow(d) => d.apply_buff(req.t, buffs),
            ArcherSkillData::Concentrate(d) => d.apply_buff(req.t, buffs),
            ArcherSkillData::Inferno(d) => {
                log::info!("Inferno skill: {skill_id:?} {d:?}");
            }
            ArcherSkillData::Blizzard(d) => {
                log::info!("Blizzard skill: {skill_id:?} {d:?}");
            }
            ArcherSkillData::Hurricane(d) => {
                log::info!("Hurricane skill: {skill_id:?} {d:?}");
            }
            ArcherSkillData::FinalAttack(d) => {
                log::info!("FinalAttack skill: {skill_id:?} {d:?}");
            }
            ArcherSkillData::MapleWarrior(d) => d.apply_buff(req.t, buffs),
            ArcherSkillData::SilverHawk(d)
            | ArcherSkillData::GoleanEagle(d)
            | ArcherSkillData::Phoenix(d)
            | ArcherSkillData::Frostprey(d) => {
                log::info!("hawk skill: {skill_id:?} {d:?}");

                d.buff.apply_buff(req.t, buffs);
                let chr = &ctx.chr;
                
                ctx.spawn_summon(
                    Summon {
                        pos: chr.pos,
                        fh: chr.fh,
                        skill_id,
                        skill_level: slvl,
                        char_level: chr.stats.level,
                        char_id: chr.id,
                        move_ability: d.summon.move_ability,
                        assist_type: d.summon.assist_type,
                        expiration: req.t + d.summon.dur,
                    },
                )?;
            }
        }

        Ok(())
    }

    pub fn handle_attack(&mut self, mut ctx: ClassContext, atk: &AttackData) -> anyhow::Result<()> {
        let debuff = match atk.skill_id {
            Some(skill_id) => Self::get_atk_debuff(ctx.chr, skill_id)?,
            None => None,
        };
        ctx.attack_mobs(atk.skill_id, &atk.targets, &debuff)?;
        Ok(())
    }

    pub fn get_atk_debuff(
        chr: &mut Character,
        skill_id: SkillId,
    ) -> anyhow::Result<Option<Box<dyn MobApplyDebuff>>> {
        if skill_id.0 == 0 {
            return Ok(None);
        }

        let skill = chr.skills.get_leveled(skill_id)?;

        Ok(Some(
            match ArcherSkillData::from_skill(skill.meta, skill.level as u8)
                .context("not an archer skill")?
            {
                ArcherSkillData::Inferno(d) => Box::new(d.debuff),
                ArcherSkillData::Blizzard(d) => Box::new(d.debuff),
                ArcherSkillData::SilverHawk(d)
                | ArcherSkillData::GoleanEagle(d)
                | ArcherSkillData::Phoenix(d)
                | ArcherSkillData::Frostprey(d) => Box::new(d.stun),
                _ => {
                    return Ok(None);
                }
            },
        ))
    }
}
