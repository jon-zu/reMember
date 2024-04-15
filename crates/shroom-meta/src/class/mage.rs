use std::time::Duration;

use crate::{
    buffs::{
        char::{
            CharBuffAcc, CharBuffBasicStatUp, CharBuffBeholder, CharBuffBooster, CharBuffEvasion,
            CharBuffHolySymbol, CharBuffHolyshield, CharBuffInfinity, CharBuffInvincible,
            CharBuffMad, CharBuffMagicGuard, CharBuffManaReflection, CharBuffMdd, CharBuffPad,
            CharBuffPdd, CharBuffTeleportMasteryOn,
        },
        mob, SkillChance, SkillPerc,
    },
    id::skill_id::*,
    shared::ElementAttribute,
    skill::SkillLevel,
    SkillMeta,
};

use super::{
    AttackDebuffSkill, AttackSkill, ElemAttackSkill, ElemDebuffAttackSkill, MobDebuff,
    SummonAssistType, SummonMoveAbility, SummonSkill,
};

#[derive(Debug)]
pub struct MysticDoor(pub Duration);

#[derive(Debug)]
pub struct MageSummon {
    pub summon: SummonSkill,
    pub buff: CharBuffBeholder,
    pub damage: i16,
}

#[derive(Debug)]
pub struct MpEaterData {
    pub proc: SkillChance,
    pub mp_absord_perc: SkillPerc,
}

#[derive(Debug)]
pub struct SpellMasteryData {
    pub magic_mastery: i16,
    pub mad: i16,
}

#[derive(Debug)]
pub struct TeleportData {
    pub range: i16,
}

#[derive(Debug)]
pub struct HealData {
    //TODO in theory u(5) is mob count
    // but it also contains mob count
    pub atk: AttackSkill,
    pub heal_ratio: SkillPerc,
}

#[derive(Debug)]
pub struct BlessData {
    pub pad: CharBuffPad,
    pub mad: CharBuffMad,
    pub pdd: CharBuffPdd,
    pub mdd: CharBuffMdd,
    pub acc: CharBuffAcc,
    pub evasion: CharBuffEvasion,
}

#[derive(Debug)]
pub enum MageSkillData {
    EnergyBolt(AttackSkill),
    MagicClaw(AttackSkill),
    MagicGuard(CharBuffMagicGuard),
    MagicArmor((CharBuffPdd, CharBuffMdd)),

    // 2nd
    MpEater(MpEaterData),
    SpellMastery(SpellMasteryData),
    Meditation(CharBuffMad),
    Teleport(TeleportData),
    Slow(MobDebuff<mob::Speed>),

    // FP
    FireArrow(ElemAttackSkill),
    PoisonBreath(ElemDebuffAttackSkill<mob::Poison>),

    // IL
    ColdBeam(ElemDebuffAttackSkill<mob::Freeze>),
    ThunderBolt(ElemAttackSkill),

    // Cleric
    Heal(HealData),
    HolyArrow(ElemAttackSkill),
    Invincible(CharBuffInvincible),
    Bless(BlessData),

    // Mage 2
    ElementCompositionPoison(AttackDebuffSkill<mob::Poison>),
    ElementCompositionFreeze(AttackDebuffSkill<mob::Freeze>),
    Seal(MobDebuff<mob::Seal>),
    SpellBooster(CharBuffBooster),
    TeleportMastery(CharBuffTeleportMasteryOn),
    //TODO Elemental Decreaste

    // Fp 2
    Explosion(ElemAttackSkill),
    PoisonMist(MobDebuff<mob::Poison>), // TODO affected area

    // IL 2
    IceStrike(ElemDebuffAttackSkill<mob::Freeze>),
    ThunderSpear(ElemDebuffAttackSkill<mob::Stun>),

    // Bishop
    //TODO reset
    HolySymbol(CharBuffHolySymbol),
    Doom(MobDebuff<mob::Doom>),
    ShiningRay(ElemDebuffAttackSkill<mob::Stun>),
    Dispel(SkillChance),
    Dragon(MageSummon),
    MysticDoor(MysticDoor),

    // Mage 3
    BigBang(AttackSkill),
    MapleWarrior(CharBuffBasicStatUp),
    //TODO extra mad + recover
    Infinity(CharBuffInfinity),
    //TODO prop proc rate
    ManaReflection(CharBuffManaReflection),

    // Bishop
    HolyShield(CharBuffHolyshield),
    AngelyRay(ElemAttackSkill),
    Genesis(ElemAttackSkill),
    Bahamut(MageSummon),

    // IL
    Blizzard(ElemAttackSkill), // TODO freeze debuff
    ChainLightning(ElemAttackSkill),
    Elquines(MageSummon),

    // FP
    Meteor(ElemAttackSkill),
    Paralyze(AttackSkill),
    Ifrit(MageSummon),
}

impl MageSkillData {
    pub fn from_skill(skill: SkillMeta, lvl: SkillLevel) -> anyhow::Result<Self> {
        Ok(match skill.id {
            MAGE_MAGIC_CLAW => Self::MagicClaw(AttackSkill::from_skill(skill, lvl)),
            MAGE_ENERGY_BOLT => Self::EnergyBolt(AttackSkill::from_skill(skill, lvl)),
            MAGE_MAGIC_GUARD => Self::MagicGuard(CharBuffMagicGuard::from_skill_x(skill, lvl)),
            MAGE_MAGIC_ARMOR => Self::MagicArmor((
                CharBuffPdd::from_skill(skill, lvl, skill.pdd(lvl)),
                CharBuffMdd::from_skill(skill, lvl, skill.mdd(lvl)),
            )),
            IL1_MP_EATER | FP1_MP_EATER | CLERIC_MP_EATER => Self::MpEater(MpEaterData {
                proc: SkillChance(skill.prop(lvl)),
                mp_absord_perc: SkillPerc(skill.x(lvl)),
            }),
            IL1_SPELL_MASTERY | FP1_SPELL_MASTERY | CLERIC_SPELL_MASTERY => {
                Self::SpellMastery(SpellMasteryData {
                    magic_mastery: skill.x(lvl),
                    mad: skill.mad(lvl),
                })
            }
            IL1_MEDITATION | FP1_MEDITATION => {
                Self::Meditation(CharBuffMad::from_skill(skill, lvl, skill.mad(lvl)))
            }
            IL1_TELEPORT | FP1_TELEPORT | CLERIC_TELEPORT => {
                Self::Teleport(TeleportData {
                    range: 0, //TODO parse range node
                })
            }
            IL1_SLOW | FP1_SLOW => Self::Slow(MobDebuff::new(
                skill.time_dur(lvl),
                skill.x(lvl),
                SkillChance::always(),
            )),
            FP1_FIRE_ARROW => Self::FireArrow(ElemAttackSkill {
                elem: ElementAttribute::Fire,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            FP1_POISON_BREATH => {
                Self::PoisonBreath(ElemDebuffAttackSkill {
                    elem: ElementAttribute::Poison,
                    skill: AttackSkill::from_skill(skill, lvl),
                    // TODO: must store actual dot data
                    debuff: MobDebuff::from_skill(
                        skill,
                        lvl,
                        SkillChance::always(),
                        mob::Poison(1),
                    ),
                })
            }
            IL1_COLD_BEAM => Self::ColdBeam(ElemDebuffAttackSkill {
                elem: ElementAttribute::Ice,
                skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill(skill, lvl, SkillChance::always(), mob::Freeze(1)),
            }),
            IL1_THUNDER_BOLT => Self::ThunderBolt(ElemAttackSkill {
                elem: ElementAttribute::Light,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            CLERIC_HEAL => Self::Heal(HealData {
                atk: AttackSkill::from_skill(skill, lvl),
                heal_ratio: SkillPerc(skill.hp(lvl)),
            }),
            CLERIC_HOLY_ARROW => Self::HolyArrow(ElemAttackSkill {
                elem: ElementAttribute::Holy,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            CLERIC_INVINCIBLE => Self::Invincible(CharBuffInvincible::from_skill_x(skill, lvl)),
            CLERIC_BLESS => Self::Bless(BlessData {
                pad: CharBuffPad::from_skill(skill, lvl, skill.pad(lvl)),
                mad: CharBuffMad::from_skill(skill, lvl, skill.mad(lvl)),
                pdd: CharBuffPdd::from_skill(skill, lvl, skill.pdd(lvl)),
                mdd: CharBuffMdd::from_skill(skill, lvl, skill.mdd(lvl)),
                acc: CharBuffAcc::from_skill(skill, lvl, skill.accuracy(lvl)),
                evasion: CharBuffEvasion::from_skill(skill, lvl, skill.evasion(lvl)),
            }),
            IL2_SPELL_BOOSTER | FP2_SPELL_BOOSTER => {
                Self::SpellBooster(CharBuffBooster::from_skill_x(skill, lvl))
            }
            IL2_SEAL | FP2_SEAL => Self::Seal(MobDebuff::from_skill_1(skill, lvl)),
            IL2_TELEPORT_MASTERY | FP2_TELEPORT_MASTERY | PRIEST_TELEPORT_MASTERY => {
                Self::TeleportMastery(CharBuffTeleportMasteryOn::new(
                    skill.id.into(),
                    skill.x(lvl).into(),
                    Duration::from_secs(60 * 5), // TODO buff time + add damage stats
                ))
            }

            FP2_ELEMENT_COMPOSITION => {
                Self::ElementCompositionPoison(AttackDebuffSkill {
                    attack_skill: AttackSkill::from_skill(skill, lvl),
                    debuff: MobDebuff::from_skill_1(skill, lvl), //TODO chance is 100 + poison
                })
            }
            FP2_EXPLOSION => Self::Explosion(ElemAttackSkill {
                elem: ElementAttribute::Fire,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            IL2_ELEMENT_COMPOSITION => Self::ElementCompositionFreeze(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::new(skill.time_dur(lvl), 1, SkillChance::always()),
            }),
            IL2_ICE_STRIKE => Self::IceStrike(ElemDebuffAttackSkill {
                elem: ElementAttribute::Ice,
                skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::new(skill.time_dur(lvl), 1, SkillChance::always()),
            }),
            IL2_THUNDER_SPEAR => Self::ThunderSpear(ElemDebuffAttackSkill {
                elem: ElementAttribute::Light,
                skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::new(skill.time_dur(lvl), 1, SkillChance::always()),
            }),
            PRIEST_DOOM => Self::Doom(MobDebuff::from_skill_1(skill, lvl)),
            PRIEST_HOLY_SYMBOL => Self::HolySymbol(CharBuffHolySymbol::from_skill_x(skill, lvl)),
            PRIEST_SHINING_RAY => Self::ShiningRay(ElemDebuffAttackSkill {
                elem: ElementAttribute::Light,
                skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::new(skill.time_dur(lvl), 1, SkillChance::always()),
            }),
            PRIEST_DISPEL => Self::Dispel(SkillChance(skill.prop(lvl))),
            PRIEST_SUMMON_DRAGON => Self::Dragon(MageSummon {
                summon: SummonSkill {
                    move_ability: SummonMoveAbility::CircleFollow,
                    assist_type: SummonAssistType::Attack,
                    dur: skill.time_dur(lvl)
                },
                buff: CharBuffBeholder::from_skill(skill, lvl, 1),
                damage: skill.damage(lvl),
            }),
            PRIEST_MYSTIC_DOOR => Self::MysticDoor(MysticDoor(skill.time_dur(lvl))),
            IL3_BIG_BANG | FP3_BIG_BANG | BISHOP_BIG_BANG => {
                Self::BigBang(AttackSkill::from_skill(skill, lvl))
            }
            IL3_MAPLE_WARRIOR | FP3_MAPLE_WARRIOR | BISHOP_MAPLE_WARRIOR => {
                Self::MapleWarrior(CharBuffBasicStatUp::from_skill_x(skill, lvl))
            }
            IL3_INFINITY | FP3_INFINITY | BISHOP_INFINITY => {
                Self::Infinity(CharBuffInfinity::from_skill(skill, lvl, 1))
            }
            IL3_MANA_REFLECTION | FP3_MANA_REFLECTION | BISHOP_MANA_REFLECTION => {
                Self::ManaReflection(CharBuffManaReflection::from_skill_x(skill, lvl))
            }
            BISHOP_HOLY_SHIELD => Self::HolyShield(CharBuffHolyshield::from_skill_x(skill, lvl)),
            BISHOP_GENESIS => Self::Genesis(ElemAttackSkill {
                elem: ElementAttribute::Holy,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            BISHOP_ANGEL_RAY => Self::AngelyRay(ElemAttackSkill {
                elem: ElementAttribute::Holy,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            BISHOP_BAHAMUT => Self::Bahamut(MageSummon {
                summon: SummonSkill {
                    move_ability: SummonMoveAbility::Follow,
                    assist_type: SummonAssistType::Attack,
                    dur: skill.time_dur(lvl)
                },
                buff: CharBuffBeholder::from_skill(skill, lvl, 1),
                damage: skill.damage(lvl),
            }),
            IL3_CHAIN_LIGHTNING => Self::ChainLightning(ElemAttackSkill {
                elem: ElementAttribute::Light,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            IL3_BLIZZARD => Self::Blizzard(ElemAttackSkill {
                elem: ElementAttribute::Ice,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            IL3_ELQUINES => Self::Elquines(MageSummon {
                summon: SummonSkill {
                    move_ability: SummonMoveAbility::Follow,
                    assist_type: SummonAssistType::Attack,
                    dur: skill.time_dur(lvl)
                },
                buff: CharBuffBeholder::from_skill(skill, lvl, 1),
                damage: skill.damage(lvl),
            }),
            FP3_PARALYZE => Self::Paralyze(AttackSkill::from_skill(skill, lvl)),
            FP3_METEOR_SHOWER => Self::Meteor(ElemAttackSkill {
                elem: ElementAttribute::Fire,
                skill: AttackSkill::from_skill(skill, lvl),
            }),
            FP3_IFRIT => Self::Ifrit(MageSummon {
                summon: SummonSkill {
                    move_ability: SummonMoveAbility::Follow,
                    assist_type: SummonAssistType::Attack,
                    dur: skill.time_dur(lvl)
                },
                buff: CharBuffBeholder::from_skill(skill, lvl, 1),
                damage: skill.damage(lvl),
            }),
            _ => anyhow::bail!("Invalid skill id: {}", skill.id.0),
        })
    }
}
