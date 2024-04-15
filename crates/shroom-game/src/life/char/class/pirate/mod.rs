use shroom_meta::{
    buffs::char::EnergyCharged,
    class::pirate::{BattleshipData, DashData, PirateSkillData},
    id::{skill_id, SkillId},
};
use shroom_srv::{act::Context, GameTime};

use crate::life::{self, char::{buffs::ApplyBuff, Character}, mob::buffs::MobApplyDebuff};

use super::{AttackData, ClassContext, UseSkillData};

impl ApplyBuff for DashData {
    fn apply_buff(&self, t: GameTime, buffs: &mut life::char::buffs::CharBuffs) {
        self.speed.apply_buff(t, buffs);
        self.jump.apply_buff(t, buffs);
    }
}

impl ApplyBuff for BattleshipData {
    fn apply_buff(&self, t: GameTime, buffs: &mut life::char::buffs::CharBuffs) {
        self.extra_pad.apply_buff(t, buffs);
        self.extra_hp.apply_buff(t, buffs);
        self.extra_mp.apply_buff(t, buffs);
        self.extra_pdd.apply_buff(t, buffs);
        self.extra_mdd.apply_buff(t, buffs);
        self.ship_ride.apply_buff(t, buffs);
    }
}

pub struct PirateHandler;

impl PirateHandler {
    pub fn handle_skill(&mut self, ctx: ClassContext, req: &UseSkillData) -> anyhow::Result<()> {
        let skill_id = req.skill_id;
        log::info!("Handling skill: {skill_id:?}");

        let skill = ctx.chr.skills.get_leveled(skill_id)?;
        let Ok(skill) = PirateSkillData::from_skill(skill.meta, skill.level as u8) else {
            log::info!("Unknown pirate skill: {}", skill_id);
            return Ok(());
        };

        let buffs = &mut ctx.chr.buffs;
        match skill {
            PirateSkillData::Dash(d) => d.apply_buff(req.t, buffs),
            PirateSkillData::Booster(d) => d.apply_buff(req.t, buffs),
            PirateSkillData::OakBarrel(d) => d.apply_buff(req.t, buffs),
            //TODO PirateSkillData::HomingBeacon(d) => d.buff.apply_buff(req.t, buffs),
            PirateSkillData::RollDice(d) => d.apply_buff(req.t, buffs),
            PirateSkillData::MapleWarrior(d) => d.apply_buff(req.t, buffs),
            PirateSkillData::Battleship(d) => d.apply_buff(req.t, buffs),
            PirateSkillData::SpeedInfusion(d) => d.apply_buff(req.t, buffs),
            PirateSkillData::EnergyCharged(d) => d.apply_buff(req.t, buffs),
            _ => log::info!("Unhandled pirate skill: {:?}", skill_id),
        }

        Ok(())
    }

    pub fn handle_attack(&mut self, mut ctx: ClassContext, atk: &AttackData) -> anyhow::Result<()> {
        let t = ctx.ctx.time();
        if let Some(skill_id) = atk.skill_id {
            // Try to get homing beacon
            if let Ok(skill) = ctx.chr.skills.get_leveled(skill_id) {
                if let Ok(PirateSkillData::HomingBeacon(mut d)) =
                    PirateSkillData::from_skill(skill.meta, skill.level as u8)
                {
                    log::info!("Applying homing beacon");
                    d.buff.data.mob_id = atk.targets.first().ok_or_else(|| anyhow::format_err!("No mob for homing beacon"))?.mob_id;
                    d.buff.apply_buff(t, &mut ctx.chr.buffs);
                }
            }
        }

        if let Ok(energy_skill) = ctx.chr.skills.get_leveled(skill_id::MARAUDER_ENERGY_CHARGE) {
            if ctx.chr.buffs.get::<EnergyCharged>().is_none() {
                if let Ok(PirateSkillData::EnergyCharged(d)) =
                    PirateSkillData::from_skill(energy_skill.meta, energy_skill.level as u8)
                {
                    d.apply_buff(t, &mut ctx.chr.buffs);
                }
            } else {
                ctx.chr.buffs.update_extend::<EnergyCharged>(t, |b| b.proc());
            }
            log::info!("Updating energy");
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

        let skill = chr.skills.get_leveled(skill_id)?;

        Ok(Some(
            match PirateSkillData::from_skill(skill.meta, skill.level as u8)
                .map_err(|_| anyhow::format_err!("not a pirate skill"))?
            {
                PirateSkillData::Grenade(d) => Box::new(d.debuff),
                PirateSkillData::BlankShot(d) => Box::new(d.debuff),
                PirateSkillData::FlameThrower(d) => Box::new(d.debuff),
                PirateSkillData::IceSplitter(d) => Box::new(d.debuff),
                PirateSkillData::Hypnotize(d) => Box::new(d),
                _ => {
                    log::info!("Unhandled pirate skill: {:?}", skill_id);
                    return Ok(None);
                }
            },
        ))
    }
}
