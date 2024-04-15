use shroom_meta::{
    id::{ItemId, JobClass, ObjectId, SkillId},
    twod::Vec2,
};
use shroom_proto95::game::user::AttackTargetInfo;
use shroom_srv::{net::session::NetSessionContext, GameTime, act::Context};

use crate::{
    game::{GameContext, GameSession},
    life::mob::buffs::MobApplyDebuff,
};

use super::{summon::Summon, Character};

pub mod archer;
pub mod mage;
pub mod pirate;
pub mod thief;
pub mod warrior;

pub struct ClassHandler;

pub struct ClassContext<'a, 'b> {
    pub chr: &'a mut Character,
    pub ctx: &'a mut NetSessionContext<'b, GameSession>,
}

impl<'a, 'b> ClassContext<'a, 'b> {
    pub fn new(chr: &'a mut Character, ctx: &'a mut GameContext<'b>) -> ClassContext<'a, 'b> {
        ClassContext { chr, ctx }
    }

    pub fn spawn_summon(&mut self, summon: Summon) -> anyhow::Result<()> {
        self.chr.add_summon(self.ctx, summon)?;
        Ok(())
    }

    pub fn attack_mobs(
        &mut self,
        skill: Option<SkillId>,
        targets: &[AttackTargetInfo],
        debuff: &Option<Box<dyn MobApplyDebuff>>,
    ) -> anyhow::Result<()> {
        let ctx = &mut self.ctx;
        let mut field = crate::field!(ctx);
        let debuff = Box::new(debuff);
        for target in targets {
            let dmg = target.hits.iter().sum::<u32>();
            let attacker: &Character = self.chr;
            field.attack_mob(target.mob_id, dmg, attacker, &debuff, skill.unwrap_or(SkillId(0)))?;
        }

        Ok(())
    }
}

pub struct UseSkillData {
    pub skill_id: SkillId,
    pub pos: Option<Vec2>,
    pub spirit_javelin_item: Option<ItemId>,
    pub buff_ix: Option<u8>,
    pub t: GameTime,
    pub affected_mobs: Vec<ObjectId>,
    /*
       todo affected mobs and items
    */
}

pub struct AttackData {
    pub skill_id: Option<SkillId>,
    pub targets: Vec<AttackTargetInfo>

    /*
       todo affected mobs and items
    */
}

macro_rules! handle_class {
    ($class:ident, $def:expr, $($call:tt)*) => {
        match $class {
            JobClass::Warrior | JobClass::DawnWarrior => warrior::WarriorHandler.$($call)*,
            JobClass::Bowman | JobClass::WindArcher => archer::ArcherHandler.$($call)*,
            JobClass::Thief | JobClass::NightWalker => thief::ThiefHandler.$($call)*,
            JobClass::Magician => mage::MageHandler.$($call)*,
            JobClass::Pirate => pirate::PirateHandler.$($call)*,
            //TODO
            JobClass::Beginner => warrior::WarriorHandler.$($call)*,
            _ => {
                log::info!("Unhandled class: {:?}", $class);
                Ok($def)
            },
        }
    };
}

impl ClassHandler {
    pub fn handle_skill(ctx: ClassContext, req: &UseSkillData) -> anyhow::Result<()> {
        let class = ctx.chr.stats.job.class();
        handle_class!(class, (), handle_skill(ctx, req))
    }

    pub fn handle_attack(
        ctx: ClassContext,
        atk: &AttackData
    ) -> anyhow::Result<()> {
        let class = ctx.chr.stats.job.class();
        handle_class!(class, (), handle_attack(ctx, atk))
    }
}
