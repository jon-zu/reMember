use anyhow::Context;

use num_enum::TryFromPrimitive;

use shroom_meta::id::{
    skill_id::{
        CRUSADER_COMBO_ATTACK, DW3_ADVANCED_COMBO, DW3_COMBO_ATTACK, HERO_ADVANCED_COMBO_ATTACK,
    },
    SkillId,
};
use shroom_meta::{buffs::char::ComboCounter, class::warrior::WarriorSkillData};
use shroom_proto95::game::life::summon::BeholderBuffIx;


use crate::life::char::buffs::ApplyBuff;
use crate::life::char::summon::Summon;
use crate::life::char::Character;
use crate::life::mob::buffs::MobApplyDebuff;

use super::{AttackData, ClassContext, UseSkillData};

pub struct WarriorHandler;

impl WarriorHandler {
    pub fn handle_skill(
        &mut self,
        mut ctx: ClassContext,
        req: &UseSkillData,
    ) -> anyhow::Result<()> {
        log::info!("Handling skill: {:?}", req.skill_id);
        //chr.handle_warrior_skill(skill_id, t)
        let look_up_id = match req.skill_id {
            CRUSADER_COMBO_ATTACK if ctx.chr.skills.has_leveled(HERO_ADVANCED_COMBO_ATTACK) => {
                HERO_ADVANCED_COMBO_ATTACK
            }
            DW3_COMBO_ATTACK if ctx.chr.skills.has_leveled(DW3_ADVANCED_COMBO) => {
                DW3_ADVANCED_COMBO
            }
            _ => req.skill_id,
        };

        let skill = ctx.chr.skills.get(look_up_id)?;
        let slvl = skill.level as u8;
        let Ok(skill) = WarriorSkillData::from_skill(skill.meta, skill.level as u8) else {
            log::info!("Unknown warrior skill: {}", req.skill_id);
            return Ok(());
        };

        let buffs = &mut ctx.chr.buffs;
        match skill {
            WarriorSkillData::IronWill(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::IronBody(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::HyperBody(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Rage(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Booster(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::PowerGuard(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::WeaponCharge(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Combo(buff) => buff.apply_buff(req.t, buffs),
            //TODO: how to handle adv combo
            WarriorSkillData::AdvCombo(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Stance(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::DragonBlood(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::MapleWarrior(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Enrage(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Echo(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Recovery(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::NimbleFeet(buff) => buff.apply_buff(req.t, buffs),
            WarriorSkillData::Threaten(buff) => {
                log::info!("Unhandled buff: {look_up_id} - {buff:?}");
            }
            WarriorSkillData::Crash(buff) => {
                log::info!("Unhandled buff: {look_up_id} - {buff:?}");
            }
            WarriorSkillData::Beholder(beholder) => {
                beholder.buff.apply_buff(req.t, buffs);
                let chr = &ctx.chr;
                let summ = Summon {
                    pos: chr.pos,
                    fh: chr.fh,
                    skill_id: look_up_id,
                    skill_level: slvl,
                    char_level: chr.stats.level,
                    char_id: chr.id,
                    move_ability: beholder.summon.move_ability,
                    assist_type: beholder.summon.assist_type,
                    expiration: req.t + beholder.summon.dur,
                };
                ctx.spawn_summon(summ)?;
                log::info!("Applying beholder...");
            }
            WarriorSkillData::BeholderHeal(heal) => {
                ctx.chr.apply_heal(heal.heal)?;
            }
            WarriorSkillData::BeholderBuff(buff) => {
                let ix = req.buff_ix.context("No buff ix")?;
                let buff_ix = BeholderBuffIx::try_from_primitive(ix)?;
                match buff_ix {
                    BeholderBuffIx::ACC => buff.acc.apply_buff(req.t, buffs),
                    BeholderBuffIx::EVA => buff.eva.apply_buff(req.t, buffs),
                    BeholderBuffIx::EMDD => buff.extra_mdd.apply_buff(req.t, buffs),
                    BeholderBuffIx::EPAD => buff.extra_pad.apply_buff(req.t, buffs),
                    BeholderBuffIx::EPDD => buff.extra_pdd.apply_buff(req.t, buffs),
                }
                log::info!("Applied beholder buff: {buff_ix:?}");
            }
            WarriorSkillData::HpRecovery(heal) => {
                ctx.chr.apply_heal(heal)?;
            }
            _ => unimplemented!("buff: {look_up_id}"),
        }

        Ok(())
    }

    pub fn handle_attack(
        &mut self,
        mut ctx: ClassContext,
        atk: &AttackData
    ) -> anyhow::Result<()> {
        if !atk.targets.is_empty() {
            ctx.chr.buffs.update(|combo: &mut ComboCounter| combo.proc());
        }


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

        let skill = chr.skills.get_mut(skill_id)?;

        Ok(Some(
            match WarriorSkillData::from_skill(skill.meta, skill.level as u8)
                .context("not a warrior skill")?
            {
                WarriorSkillData::Shout(d) => Box::new(d.debuff),
                WarriorSkillData::Panic(d) => {
                    chr.buffs.update(|combo: &mut ComboCounter| combo.reset());
                    Box::new(d.attack.debuff)
                }
                WarriorSkillData::Coma(d) => {
                    chr.buffs.update(|combo: &mut ComboCounter| combo.reset());
                    Box::new(d.attack.debuff)
                }
                WarriorSkillData::Threaten(d) => Box::new(d),
                WarriorSkillData::ChargedBlow(d) => Box::new(d.debuff),
                _ => {
                    return Ok(None);
                }
            },
        ))
    }

    /* 
    pub fn handle_attack(
        &mut self,
        chr: &mut Character,
        skill_id: Option<SkillId>,
        targets: &[ObjectId],
        _t: GameTime,
    ) -> anyhow::Result<()> {
        log::info!("Attacking with skill: {skill_id:?}");
        if !targets.is_empty() {
            chr.buffs.update(|combo: &mut ComboCounter| combo.proc());
        }
        Ok(())
    }

    pub fn get_attack_data(
        &self,
        chr: &mut Character,
        skill_id: SkillId,
    ) -> anyhow::Result<Option<Box<dyn MobApplyDebuff>>> {
        if skill_id.0 == 0 {
            return Ok(None);
        }

        let skill = chr.skills.get_mut(skill_id)?;

        Ok(Some(
            match WarriorSkillData::from_skill(skill.meta, skill.level as u8)
                .context("not a warrior skill")?
            {
                WarriorSkillData::Shout(d) => Box::new(d.debuff),
                WarriorSkillData::Panic(d) => {
                    chr.buffs.update(|combo: &mut ComboCounter| combo.reset());
                    Box::new(d.attack.debuff)
                }
                WarriorSkillData::Coma(d) => {
                    chr.buffs.update(|combo: &mut ComboCounter| combo.reset());
                    Box::new(d.attack.debuff)
                }
                WarriorSkillData::Threaten(d) => Box::new(d),
                WarriorSkillData::ChargedBlow(d) => Box::new(d.debuff),
                _ => {
                    return Ok(None);
                }
            },
        ))
    }*/
}
