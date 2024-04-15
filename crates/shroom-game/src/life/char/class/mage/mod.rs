use anyhow::Context;
use shroom_meta::{
    class::mage::{BlessData, MageSkillData},
    id::SkillId,
};
use shroom_srv::GameTime;

use crate::life::{self, char::{buffs::ApplyBuff, summon::Summon, Character}, mob::buffs::MobApplyDebuff};

use super::{AttackData, ClassContext, UseSkillData};

impl ApplyBuff for BlessData {
    fn apply_buff(&self, t: GameTime, buffs: &mut life::char::buffs::CharBuffs) {
        self.mad.apply_buff(t, buffs);
        self.mdd.apply_buff(t, buffs);
        self.pad.apply_buff(t, buffs);
        self.pdd.apply_buff(t, buffs);
        self.acc.apply_buff(t, buffs);
        self.evasion.apply_buff(t, buffs);
    }
}

pub struct MageHandler;

impl MageHandler {
    pub fn handle_skill(&mut self, mut ctx: ClassContext, req: &UseSkillData) -> anyhow::Result<()> {
        let skill_id = req.skill_id;
        log::info!("Handling skill: {skill_id:?}");

        let skill = ctx.chr.skills.get_leveled(skill_id)?;
        let slvl = skill.level as u8;
        let Ok(skill) = MageSkillData::from_skill(skill.meta, skill.level as u8) else {
            log::info!("Unknown mage skill: {}", skill_id);
            return Ok(());
        };

        let buffs = &mut ctx.chr.buffs;
        match skill {
            MageSkillData::MapleWarrior(d) => d.apply_buff(req.t, buffs),
            MageSkillData::MagicGuard(d) => d.apply_buff(req.t, buffs),
            MageSkillData::MagicArmor(d) => d.apply_buff(req.t, buffs),
            MageSkillData::Meditation(d) => d.apply_buff(req.t, buffs),
            MageSkillData::Invincible(d) => d.apply_buff(req.t, buffs),
            MageSkillData::Bless(d) => d.apply_buff(req.t, buffs),
            MageSkillData::SpellBooster(d) => d.apply_buff(req.t, buffs),
            MageSkillData::TeleportMastery(d) => d.apply_buff(req.t, buffs),
            MageSkillData::HolySymbol(d) => d.apply_buff(req.t, buffs),
            //TODO: MageSkillData::Dispel(d) => d.apply_buff(req.t, buffs),
            MageSkillData::Infinity(d) => d.apply_buff(req.t, buffs),
            MageSkillData::ManaReflection(d) => d.apply_buff(req.t, buffs),
            MageSkillData::HolyShield(d) => d.apply_buff(req.t, buffs),
            MageSkillData::Dragon(d)
            | MageSkillData::Bahamut(d)
            | MageSkillData::Elquines(d)
            | MageSkillData::Ifrit(d) => {
                d.buff.apply_buff(req.t, buffs);
                let chr = &ctx.chr;
                let summ = Summon {
                    pos: chr.pos,
                    fh: chr.fh,
                    skill_id,
                    skill_level: slvl,
                    char_level: chr.stats.level,
                    char_id: chr.id,
                    move_ability: d.summon.move_ability,
                    assist_type: d.summon.assist_type,
                    expiration: req.t + d.summon.dur,
                };
                ctx.spawn_summon(
                    summ
                )?;
            }
            MageSkillData::MysticDoor(d) => {
                log::info!("Mystic door: {:?}", d);
                //chr.do_mystic_door = Some(FieldId::HENESYS);
            }
            _ => {
                log::info!("Unhandled mage skill: {skill_id:?} - {skill:?}");
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
            match MageSkillData::from_skill(skill.meta, skill.level as u8)
                .context("not an mage skill")?
            {
                MageSkillData::Seal(d) => Box::new(d),
                MageSkillData::PoisonBreath(d) => Box::new(d.debuff),
                MageSkillData::ColdBeam(d) => Box::new(d.debuff),
                MageSkillData::ElementCompositionPoison(d) => Box::new(d.debuff),
                MageSkillData::ElementCompositionFreeze(d) => Box::new(d.debuff),
                //TODO: MageSkillData::PoisonMist(_) => todo!(),
                MageSkillData::IceStrike(d) => Box::new(d.debuff),
                MageSkillData::ThunderSpear(d) => Box::new(d.debuff),
                MageSkillData::Doom(d) => Box::new(d),
                MageSkillData::ShiningRay(d) => Box::new(d.debuff),
                MageSkillData::Slow(d) => Box::new(d),
                _ => {
                    log::info!("Unhandled debuff mage skill: {skill_id:?} - {skill:?}");
                    return Ok(None);
                }
            },
        ))
    }
}
