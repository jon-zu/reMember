use crate::{
    buffs::{
        char::{
            CharBuffBasicStatUp, CharBuffBooster, CharBuffDarkSight, CharBuffJump,
            CharBuffMesoGuard, CharBuffMesoUp, CharBuffPickPocket, CharBuffShadowPartner,
            CharBuffSpeed, CharBuffSpiritJavelin, SpiritJavelin,
        },
        mob, SkillChance,
    },
    skill::SkillLevel,
    SkillMeta,
    id::skill_id::*,
    id::ItemId
};


use super::{AttackDebuffSkill, AttackSkill, MobDebuff};

#[derive(Debug)]
pub struct DisorderData {
    pub att_debuff: MobDebuff<mob::Pad>,
    pub def_debuff: MobDebuff<mob::Pdr>,
}

#[derive(Debug)]
pub struct HasteData {
    pub speed: CharBuffSpeed,
    pub jmp: CharBuffJump,
}

#[derive(Debug)]
pub enum ThiefSkillData {
    Disorder(DisorderData),
    DarkSight(CharBuffDarkSight),
    DoubleStab(AttackSkill),
    LuckySeven(AttackSkill),
    Booster(CharBuffBooster),
    Haste(HasteData),

    // Assasin
    Drain(AttackSkill), //TODO heal

    // Bandit
    Steal(AttackDebuffSkill<mob::Stun>),
    SavageBlow(AttackSkill),

    // Hermit
    MesoUp(CharBuffMesoUp),
    ShadowWeb(MobDebuff<mob::Web>),
    //TODO Shadow Mesos

    // CB
    MesoGuard(CharBuffMesoGuard),
    PickPocket(CharBuffPickPocket),
    BandOfThieves(AttackSkill),
    Assaulter(AttackDebuffSkill<mob::Stun>),

    ShadowPartner(CharBuffShadowPartner),
    Avenger(AttackSkill),
    FlashJump,
    DarkFlare, //TODO

    // 4
    MapleWarrior(CharBuffBasicStatUp),
    Taunt(MobDebuff<mob::Showdown>),
    NinjaAmbush(AttackSkill),

    // NL
    ShadowStars(CharBuffSpiritJavelin),
    TripleThrow(AttackSkill),

    // Shadower
    Assassinate(AttackSkill),
    BoomerangStep(AttackSkill),
}

impl ThiefSkillData {
    pub fn from_skill(skill: SkillMeta, lvl: SkillLevel) -> anyhow::Result<Self> {
        Ok(match skill.id {
            THIEF_DISORDER => Self::Disorder(DisorderData {
                att_debuff: MobDebuff::new(
                    skill.time_dur(lvl),
                    skill.x(lvl),
                    SkillChance::always(),
                ),
                def_debuff: MobDebuff::new(
                    skill.time_dur(lvl),
                    skill.y(lvl),
                    SkillChance::always(),
                ),
            }),
            THIEF_DARK_SIGHT => Self::DarkSight(CharBuffDarkSight::from_skill_x(skill, lvl)),
            THIEF_LUCKY_SEVEN => Self::LuckySeven(AttackSkill::from_skill(skill, lvl)),
            THIEF_DOUBLE_STAB => Self::DoubleStab(AttackSkill::from_skill(skill, lvl)),
            ASSASSIN_CLAW_BOOSTER | BANDIT_DAGGER_BOOSTER => {
                Self::Booster(CharBuffBooster::from_skill_x(skill, lvl))
            }
            ASSASSIN_HASTE | BANDIT_HASTE => {
                let dur = skill.time_dur(lvl);
                Self::Haste(HasteData {
                    speed: CharBuffSpeed::new(skill.id.into(), skill.speed(lvl).into(), dur),
                    jmp: CharBuffJump::new(skill.id.into(), skill.jump(lvl).into(), dur),
                })
            }
            ASSASSIN_DRAIN => Self::Drain(AttackSkill::from_skill(skill, lvl)),
            BANDIT_STEAL => Self::Steal(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill_1(skill, lvl),
            }),
            BANDIT_SAVAGE_BLOW => Self::SavageBlow(AttackSkill::from_skill(skill, lvl)),

            HERMIT_MESO_UP => Self::MesoUp(CharBuffMesoUp::from_skill_x(skill, lvl)),
            HERMIT_SHADOW_PARTNER | CHIEFBANDIT_SHADOW_PARTNER => {
                Self::ShadowPartner(CharBuffShadowPartner::from_skill_x(skill, lvl))
            }
            HERMIT_SHADOW_WEB => Self::ShadowWeb(MobDebuff::from_skill_1(skill, lvl)),
            HERMIT_AVENGER => Self::Avenger(AttackSkill::from_skill(skill, lvl)),

            CHIEFBANDIT_MESO_GUARD => Self::MesoGuard(CharBuffMesoGuard::from_skill_x(skill, lvl)),
            CHIEFBANDIT_PICKPOCKET => {
                Self::PickPocket(CharBuffPickPocket::from_skill_x(skill, lvl))
            }
            CHIEFBANDIT_ASSAULTER => Self::Assaulter(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill_1(skill, lvl),
            }),
            CHIEFBANDIT_BAND_OF_THIEVES => Self::BandOfThieves(AttackSkill::from_skill(skill, lvl)),
            HERMIT_FLASH_JUMP | CHIEFBANDIT_FLASH_JUMP => Self::FlashJump,
            HERMIT_DARK_FLARE | CHIEFBANDIT_DARK_FLARE => Self::DarkFlare,
            NIGHTLORD_MAPLE_WARRIOR | SHADOWER_MAPLE_WARRIOR => {
                Self::MapleWarrior(CharBuffBasicStatUp::from_skill_x(skill, lvl))
            }
            NIGHTLORD_TAUNT | SHADOWER_TAUNT => Self::Taunt(MobDebuff::from_skill_x(skill, lvl)),
            NIGHTLORD_NINJA_AMBUSH | SHADOWER_NINJA_AMBUSH => {
                Self::NinjaAmbush(AttackSkill::from_skill(skill, lvl))
            }
            NIGHTLORD_SHADOW_STARS => Self::ShadowStars(CharBuffSpiritJavelin::from_skill(
                skill,
                lvl,
                SpiritJavelin(ItemId::SUBI_THROWING_STARS),
            )),
            NIGHTLORD_TRIPLE_THROW => Self::TripleThrow(AttackSkill::from_skill(skill, lvl)),
            SHADOWER_ASSASSINATE => Self::Assassinate(AttackSkill::from_skill(skill, lvl)),
            SHADOWER_BOOMERANG_STEP => Self::BoomerangStep(AttackSkill::from_skill(skill, lvl)),
            _ => anyhow::bail!("Invalid skill id: {}", skill.id.0),
        })
    }
}
