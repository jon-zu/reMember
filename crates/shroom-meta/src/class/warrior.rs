use std::time::Duration;

use super::{
    AttackDebuffSkill, AttackSkill, HealBuff, MobDebuff, SummonAssistType, SummonMoveAbility,
    SummonSkill,
};
use crate::{
    buffs::{
        char::{
            CharBuffAcc, CharBuffBasicStatUp, CharBuffBeholder, CharBuffBooster,
            CharBuffComboCounter, CharBuffDragonBlood, CharBuffEnrage, CharBuffEvasion,
            CharBuffExtraMdd, CharBuffExtraPad, CharBuffExtraPdd, CharBuffMaxHp,
            CharBuffMaxLevelBuff, CharBuffMaxMp, CharBuffMdd, CharBuffPad, CharBuffPdd,
            CharBuffPowerGuard, CharBuffRegen, CharBuffSpeed, CharBuffStance, CharBuffWeaponCharge,
            ComboCounter, DragonBlood, WeaponCharge,
        },
        mob::{self, MobBuffStat},
        SkillChance, SkillPerc,
    },
    id::skill_id::*,
    shared::ElementAttribute,
    skill::SkillLevel,
    SkillMeta,
};

#[derive(Debug)]
pub struct ComboAttack<T: MobBuffStat> {
    pub attack: AttackDebuffSkill<T>,
    pub min_orbs: usize,
}

#[derive(Debug)]
pub struct ThreatenAttack {
    pub mob_count: usize,
    pub atk_debuff: MobDebuff<mob::Pad>,
    pub def_debuff: MobDebuff<mob::Pdr>,
    pub acc_debuff: MobDebuff<mob::Acc>,
}

#[derive(Debug)]
pub struct BlastAttack {
    pub attack: AttackSkill,
    pub instant_death_chance: SkillChance,
}

#[derive(Debug)]
pub struct SacrificeAttack {
    pub attack: AttackSkill,
    pub hp_cost: i16,
}

#[derive(Debug)]
pub struct Achilles {
    pub damage_reduction_perc: f32,
}

impl Achilles {
    pub fn from_x(x: i16) -> Self {
        Self {
            damage_reduction_perc: (1000 - x) as f32 / 1000.0,
        }
    }
}

#[derive(Debug)]
pub struct Berserk {
    pub hp_threshold: SkillPerc,
    pub damage: i16,
    pub burster_attack_count: i16,
}

#[derive(Debug)]
pub struct BeholderSummon {
    pub buff: CharBuffBeholder,
    pub summon: SummonSkill,
}

#[derive(Debug)]
pub struct BeholderHeal {
    pub heal: HealBuff,
    pub interval: Duration,
}

#[derive(Debug)]
pub struct BeholderBuff {
    pub acc: CharBuffAcc,
    pub eva: CharBuffEvasion,
    pub extra_pad: CharBuffExtraPad,
    pub extra_pdd: CharBuffExtraPdd,
    pub extra_mdd: CharBuffExtraMdd,
    pub interval: Duration,
}

#[derive(Debug)]
pub enum WarriorSkillData {
    Achilles(Achilles),
    Echo(CharBuffMaxLevelBuff),
    Recovery(CharBuffRegen),
    NimbleFeet(CharBuffSpeed),
    Berserk(Berserk),

    PowerStike(AttackSkill),
    SlashBlast(AttackSkill),
    GroundSmash(AttackSkill),
    FinalAttack(AttackSkill),
    Threaten(ThreatenAttack),
    ChargedBlow(AttackDebuffSkill<mob::Stun>),
    HeavensHammer(AttackSkill),
    Blast(BlastAttack),
    DragonBuster(AttackSkill),
    DragonFury(AttackSkill),
    Sacrifice(AttackSkill),
    Roar(AttackSkill),

    Crash(MobDebuff<mob::MagicCrash>),
    Rush(AttackSkill),
    IntrepidSlash(AttackSkill),
    Magnet(AttackSkill),

    IronBody(CharBuffPdd),
    IronWill((CharBuffPdd, CharBuffMdd)),

    Shout(AttackDebuffSkill<mob::Stun>),
    Coma(ComboAttack<mob::Stun>),
    Panic(ComboAttack<mob::Darkness>),
    Brandish(AttackSkill),

    HyperBody((CharBuffMaxHp, CharBuffMaxMp)),
    Rage(CharBuffPad),
    Booster(CharBuffBooster),
    PowerGuard(CharBuffPowerGuard),
    WeaponCharge(CharBuffWeaponCharge),
    Combo(CharBuffComboCounter),
    AdvCombo(CharBuffComboCounter),
    Stance(CharBuffStance),
    DragonBlood(CharBuffDragonBlood),
    Enrage(CharBuffEnrage),
    MapleWarrior(CharBuffBasicStatUp),
    Beholder(BeholderSummon),
    BeholderHeal(BeholderHeal),
    BeholderBuff(BeholderBuff),
    HpRecovery(HealBuff),
}

impl WarriorSkillData {
    pub fn from_skill(skill: SkillMeta, lvl: SkillLevel) -> anyhow::Result<Self> {
        Ok(match skill.id {
            SPEARNMAN_IRON_WILL => Self::IronWill((
                CharBuffPdd::from_skill(skill, lvl, skill.pdd(lvl)),
                CharBuffMdd::from_skill(skill, lvl, skill.mdd(lvl)),
            )),
            WARRIOR_IRON_BODY | DW1_IRON_BODY => {
                Self::IronBody(CharBuffPdd::from_skill(skill, lvl, skill.pdd(lvl)))
            }
            SPEARNMAN_HYPER_BODY => Self::HyperBody((
                CharBuffMaxHp::from_skill(skill, lvl, skill.x(lvl)),
                CharBuffMaxMp::from_skill(skill, lvl, skill.y(lvl)),
            )),
            FIGHTER_RAGE | DW2_RAGE => {
                Self::Rage(CharBuffPad::from_skill(skill, lvl, skill.pad(lvl)))
            }
            FIGHTER_WEAPON_BOOSTER | PAGE_WEAPON_BOOSTER | SPEARNMAN_WEAPON_BOOSTER => {
                Self::Booster(CharBuffBooster::from_skill(skill, lvl, skill.x(lvl)))
            }
            FIGHTER_POWER_GUARD | PAGE_POWER_GUARD => {
                Self::PowerGuard(CharBuffPowerGuard::from_skill_x(skill, lvl))
            }
            WK_LIGHTNING_CHARGE => Self::WeaponCharge(CharBuffWeaponCharge::from_skill(
                skill,
                lvl,
                WeaponCharge {
                    elem: ElementAttribute::Light,
                    damage: skill.damage(lvl),
                    elem_apply: 0,
                },
            )),
            WK_FIRE_CHARGE => Self::WeaponCharge(CharBuffWeaponCharge::from_skill(
                skill,
                lvl,
                WeaponCharge {
                    elem: ElementAttribute::Fire,
                    damage: skill.damage(lvl),
                    elem_apply: 0,
                },
            )),
            WK_ICE_CHARGE => Self::WeaponCharge(CharBuffWeaponCharge::from_skill(
                skill,
                lvl,
                WeaponCharge {
                    elem: ElementAttribute::Ice,
                    damage: skill.damage(lvl),
                    elem_apply: skill.x(lvl),
                },
            )),
            PALADIN_DIVINE_CHARGE => Self::WeaponCharge(CharBuffWeaponCharge::from_skill(
                skill,
                lvl,
                WeaponCharge {
                    elem: ElementAttribute::Holy,
                    damage: skill.damage(lvl),
                    elem_apply: 0,
                },
            )),
            HERO_POWER_STANCE | PALADIN_POWER_STANCE | DRK_POWER_STANCE => {
                Self::Stance(CharBuffStance::from_skill(skill, lvl, skill.prop(lvl)))
            }
            CRUSADER_COMBO_ATTACK | DW3_COMBO_ATTACK => {
                Self::Combo(CharBuffComboCounter::from_skill(
                    skill,
                    lvl,
                    ComboCounter {
                        orbs: 1,
                        max_orbs: skill.x(lvl),
                        double_proc_rate: SkillChance::default(),
                        damage_per_orb: skill.damage_ratio(lvl),
                    },
                ))
            }
            HERO_ADVANCED_COMBO_ATTACK | DW3_ADVANCED_COMBO => {
                Self::AdvCombo(CharBuffComboCounter::new(
                    skill.id.into(),
                    ComboCounter {
                        orbs: 1,
                        max_orbs: skill.x(lvl),
                        double_proc_rate: SkillChance(skill.prop(lvl)),
                        damage_per_orb: skill.damage_ratio(lvl),
                    },
                    Duration::from_secs(120),
                ))
            }
            DK_DRAGON_BLOOD => Self::DragonBlood(CharBuffDragonBlood::from_skill(
                skill,
                lvl,
                DragonBlood {
                    dec_hp: skill.x(lvl),
                    pad: skill.pad(lvl),
                    tick_interval: Duration::from_secs(4),
                },
            )),
            HERO_MAPLE_WARRIOR | PALADIN_MAPLE_WARRIOR | DRK_MAPLE_WARRIOR => {
                Self::MapleWarrior(CharBuffBasicStatUp::from_skill_x(skill, lvl))
            }
            HERO_ENRAGE => Self::Enrage(CharBuffEnrage::from_skill_x(skill, lvl)),
            BEGINNER_ECHO_OF_HERO => Self::Echo(CharBuffMaxLevelBuff::from_skill_x(skill, lvl)),
            CRUSADER_SHOUT => Self::Shout(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill_1(skill, lvl),
            }),
            CRUSADER_PANIC | DW3_PANIC => Self::Panic(ComboAttack {
                attack: AttackDebuffSkill {
                    attack_skill: AttackSkill::from_skill(skill, lvl),
                    debuff: MobDebuff::from_skill_x(skill, lvl),
                },
                min_orbs: 1,
            }),
            CRUSADER_COMA | DW3_COMA => Self::Coma(ComboAttack {
                attack: AttackDebuffSkill {
                    attack_skill: AttackSkill::from_skill(skill, lvl),
                    debuff: MobDebuff::from_skill_1(skill, lvl),
                },
                min_orbs: 1,
            }),
            CRUSADER_MAGIC_CRASH | WK_MAGIC_CRASH | DK_MAGIC_CRASH => {
                Self::Crash(MobDebuff::from_skill_1(skill, lvl))
            }
            CRUSADER_BRANDISH | DW3_BRANDISH => Self::Brandish(AttackSkill::from_skill(skill, lvl)),
            HERO_RUSH | PALADIN_RUSH | DRK_RUSH => Self::Rush(AttackSkill::from_skill(skill, lvl)),
            HERO_INTREPID_SLASH => Self::IntrepidSlash(AttackSkill::from_skill(skill, lvl)),
            FIGHTER_FINAL_ATTACK | PAGE_FINAL_ATTACK | SPEARNMAN_FINAL_ATTACK => {
                Self::FinalAttack(AttackSkill::from_skill(skill, lvl))
            }
            FIGHTER_GROUND_SMASH | PAGE_GROUND_SMASH | SPEARNMAN_GROUND_SMASH => {
                Self::GroundSmash(AttackSkill::from_skill(skill, lvl))
            }
            WARRIOR_POWER_STRIKE | DW1_POWER_STRIKE => {
                Self::PowerStike(AttackSkill::from_skill(skill, lvl))
            }
            WARRIOR_SLASH_BLAST | DW1_SLASH_BLAST => {
                Self::SlashBlast(AttackSkill::from_skill(skill, lvl))
            }
            PAGE_THREATEN => Self::Threaten(ThreatenAttack {
                mob_count: skill.mob_count(lvl),
                atk_debuff: MobDebuff::new(skill.sub_time_dur(lvl), skill.x(lvl), SkillChance(100)),
                def_debuff: MobDebuff::new(skill.sub_time_dur(lvl), skill.y(lvl), SkillChance(100)),
                acc_debuff: MobDebuff::new(
                    skill.sub_time_dur(lvl),
                    skill.x(lvl),
                    SkillChance(skill.prop(lvl)),
                ),
            }),
            WK_CHARGED_BLOW => Self::ChargedBlow(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill_1(skill, lvl),
            }),
            PALADIN_HEAVENS_HAMMER => Self::HeavensHammer(AttackSkill::from_skill(skill, lvl)),
            PALADIN_BLAST => Self::Blast(BlastAttack {
                attack: AttackSkill::from_skill(skill, lvl),
                instant_death_chance: SkillChance(skill.prop(lvl)),
            }),
            DK_DRAGON_BUSTER => Self::DragonBuster(AttackSkill::from_skill(skill, lvl)),
            DK_DRAGON_FURY => Self::DragonFury(AttackSkill::from_skill(skill, lvl)),
            DK_SACRIFICE => Self::Sacrifice(AttackSkill::from_skill(skill, lvl)),
            DK_DRAGON_ROAR => Self::Roar(AttackSkill::from_skill(skill, lvl)),
            HERO_ACHILLES | PALADIN_ACHILLES | DRK_ACHILLES => {
                Self::Achilles(Achilles::from_x(skill.x(lvl)))
            }
            DRK_BERSERK => Self::Berserk(Berserk {
                hp_threshold: SkillPerc(skill.x(lvl)),
                damage: skill.damage(lvl),
                burster_attack_count: skill.y(lvl),
            }),
            HERO_MONSTER_MAGNET | DRK_MONSTER_MAGNET => {
                Self::Magnet(AttackSkill::from_skill(skill, lvl))
            }
            DRK_BEHOLDER => Self::Beholder(BeholderSummon {
                buff: CharBuffBeholder::from_skill_x(skill, lvl),
                summon: SummonSkill {
                    move_ability: SummonMoveAbility::Follow,
                    assist_type: SummonAssistType::Heal,
                    dur: skill.time_dur_min(lvl),
                },
            }),
            DRK_AURA_OF_THE_BEHOLDER => Self::BeholderHeal(BeholderHeal {
                heal: HealBuff::Flat(skill.hp(lvl)),
                interval: Duration::from_secs(skill.x(lvl) as u64),
            }),
            DRK_HEX_OF_THE_BEHOLDER => Self::BeholderBuff(BeholderBuff {
                acc: CharBuffAcc::new(
                    skill.id.into(),
                    skill.accuracy(lvl).into(),
                    skill.time_dur(lvl),
                ),
                eva: CharBuffEvasion::new(
                    skill.id.into(),
                    skill.evasion(lvl).into(),
                    skill.time_dur(lvl),
                ),
                extra_pad: CharBuffExtraPad::new(
                    skill.id.into(),
                    skill.extra_pad(lvl).into(),
                    skill.time_dur(lvl),
                ),
                extra_pdd: CharBuffExtraPdd::new(
                    skill.id.into(),
                    skill.extra_pdd(lvl).into(),
                    skill.time_dur(lvl),
                ),
                extra_mdd: CharBuffExtraMdd::new(
                    skill.id.into(),
                    skill.extra_mdd(lvl).into(),
                    skill.time_dur(lvl),
                ),
                interval: Duration::from_secs(skill.x(lvl) as u64),
            }),
            WK_HP_RECOVERY => Self::HpRecovery(HealBuff::Ratio(SkillPerc(skill.x(lvl)))),
            _ => anyhow::bail!("Invalid skill id: {}", skill.id),
        })
    }
}
