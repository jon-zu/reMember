use std::time::Duration;

use crate::{
    buffs::{
        char::{
            CharBuffAcc, CharBuffBasicStatUp, CharBuffBeholder, CharBuffBooster,
            CharBuffConcentration, CharBuffEvasion, CharBuffExtraPad, CharBuffSharpEyes,
            CharBuffSoulArrow, SharpEyes,
        },
        mob, SkillChance,
    },
    skill::SkillLevel,
    SkillMeta,
    id::skill_id::*
};


use super::{AttackDebuffSkill, AttackSkill, MobDebuff, SummonAssistType, SummonMoveAbility, SummonSkill};

#[derive(Debug)]
pub struct Hawk {
    pub summon: SummonSkill,
    pub buff: CharBuffBeholder,
    pub stun: MobDebuff<mob::Stun>,
}

#[derive(Debug)]
pub enum ArcherSkillData {
    SharpEyes(CharBuffSharpEyes),
    Focus((CharBuffAcc, CharBuffEvasion)),
    Booster(CharBuffBooster),
    SoulArrow(CharBuffSoulArrow),
    Concentrate((CharBuffConcentration, CharBuffExtraPad)),
    Inferno(AttackDebuffSkill<mob::Burned>),
    Blizzard(AttackDebuffSkill<mob::Freeze>),
    MapleWarrior(CharBuffBasicStatUp),
    Hurricane(AttackSkill),
    FinalAttack(AttackSkill),
    SilverHawk(Hawk),
    Phoenix(Hawk),
    GoleanEagle(Hawk),
    Frostprey(Hawk),
    //Blizzard(BlizzardData),
    //MortalBlow(CharBuffMo),
    //EvasionBoost(EvasionBoostData),
    //Puppet(PuppetData),
    //Expert(ArcherExpertData),
    //Venegance(VengeanceData),
    //MarksmanShip(MarksmanShipData),
}

impl ArcherSkillData {
    pub fn from_skill(skill: SkillMeta, lvl: SkillLevel) -> anyhow::Result<Self> {
        Ok(match skill.id {
            BOWMAN_FOCUS => Self::Focus((
                CharBuffAcc::from_skill(skill, lvl, skill.accuracy(lvl)),
                CharBuffEvasion::from_skill(skill, lvl, skill.evasion(lvl)),
            )),
            HUNTER_BOW_BOOSTER | CROSSBOWMAN_CROSSBOW_BOOSTER => {
                Self::Booster(CharBuffBooster::from_skill_x(skill, lvl))
            }
            HUNTER_SOUL_ARROW_BOW | CROSSBOWMAN_SOUL_ARROW_CROSSBOW => {
                Self::SoulArrow(CharBuffSoulArrow::from_skill_x(skill, lvl))
            }
            RANGER_INFERNO => Self::Inferno(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_dot_skill(skill, lvl),
            }),
            SNIPER_BLIZZARD => Self::Blizzard(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill(skill, lvl, SkillChance::always(), mob::Freeze(1)),
            }),
            BOWMASTER_SHARP_EYES | MARKSMAN_SHARP_EYES => {
                Self::SharpEyes(CharBuffSharpEyes::from_skill(
                    skill,
                    lvl,
                    SharpEyes {
                        crit_rate: skill.x(lvl) as u8,
                        crit_dmg_max: skill.critical_damage_max(lvl) as u8,
                    },
                ))
            }
            BOWMASTER_CONCENTRATE => Self::Concentrate((
                CharBuffConcentration::from_skill_x(skill, lvl),
                CharBuffExtraPad::from_skill(skill, lvl, skill.extra_pad(lvl)),
            )),
            BOWMASTER_MAPLE_WARRIOR | MARKSMAN_MAPLE_WARRIOR => {
                Self::MapleWarrior(CharBuffBasicStatUp::from_skill_x(skill, lvl))
            }
            BOWMASTER_HURRICANE | WA3_HURRICANE => {
                Self::Hurricane(AttackSkill::from_skill(skill, lvl))
            }
            HUNTER_FINAL_ATTACK_BOW | CROSSBOWMAN_FINAL_ATTACK_CROSSBOW | WA2_FINAL_ATTACK => {
                Self::FinalAttack(AttackSkill::from_skill(skill, lvl))
            }
            RANGER_SILVER_HAWK | SNIPER_GOLDEN_EAGLE | BOWMASTER_PHOENIX | MARKSMAN_FROSTPREY => {
                Self::SilverHawk(Hawk {
                    summon: SummonSkill {
                        move_ability: SummonMoveAbility::CircleFollow,
                        assist_type: SummonAssistType::Attack,
                        dur: skill.time_dur(lvl)
                    },
                    buff: CharBuffBeholder::from_skill(skill, lvl, 1),
                    stun: MobDebuff::new(
                        Duration::from_secs(skill.x(lvl) as u64),
                        1,
                        SkillChance(skill.prop(lvl)),
                    ),
                })
            }
            _ => anyhow::bail!("Invalid skill id: {}", skill.id.0),
        })
    }
}
