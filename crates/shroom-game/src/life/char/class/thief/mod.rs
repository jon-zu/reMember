use anyhow::Context;
use shroom_meta::{
    class::thief::{DisorderData, HasteData, ThiefSkillData},
    id::{BuffId, CharacterId, SkillId},
};
use shroom_srv::GameTime;

use crate::life::{
    char::{
        buffs::{ApplyBuff, CharBuffs},
        Character,
    },
    mob::buffs::MobApplyDebuff,
};

use super::{AttackData, ClassContext, UseSkillData};

impl MobApplyDebuff for DisorderData {
    fn apply_debuff(
        &self,
        buffs: &mut crate::life::mob::buffs::MobBuffs,
        t: GameTime,
        id: BuffId,
        src: CharacterId,
    ) {
        self.att_debuff.apply_debuff(buffs, t, id, src);
        self.def_debuff.apply_debuff(buffs, t, id, src);
    }
}

impl ApplyBuff for HasteData {
    fn apply_buff(&self, t: GameTime, buffs: &mut CharBuffs) {
        self.speed.apply_buff(t, buffs);
        self.jmp.apply_buff(t, buffs);
    }
}

pub struct ThiefHandler;

impl ThiefHandler {
    pub fn handle_skill(&mut self, ctx: ClassContext, req: &UseSkillData) -> anyhow::Result<()> {
        let skill_id = req.skill_id;
        let skill = ctx.chr.skills.get_leveled(skill_id)?;
        let Ok(skill) = ThiefSkillData::from_skill(skill.meta, skill.level as u8) else {
            log::info!("Unknown warrior skill: {}", skill_id);
            return Ok(());
        };

        let buffs = &mut ctx.chr.buffs;
        match skill {
            ThiefSkillData::Booster(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::Haste(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::DarkSight(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::MesoUp(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::PickPocket(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::MesoGuard(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::ShadowPartner(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::MapleWarrior(d) => d.apply_buff(req.t, buffs),
            ThiefSkillData::ShadowStars(mut d) => {
                d.data.0 = req.spirit_javelin_item.context("No javelin item")?;
                d.apply_buff(req.t, buffs)
            }
            s => {
                log::info!("Unhandled skill: {:?}", s);
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
            match ThiefSkillData::from_skill(skill.meta, skill.level as u8)
                .context("not a thief skill")?
            {
                ThiefSkillData::Disorder(d) => Box::new(d),
                ThiefSkillData::Steal(d) => Box::new(d.debuff),
                ThiefSkillData::ShadowWeb(d) => Box::new(d),
                ThiefSkillData::Assaulter(d) => Box::new(d.debuff),
                ThiefSkillData::Taunt(d) => Box::new(d),
                _ => {
                    return Ok(None);
                }
            },
        ))
    }
}
