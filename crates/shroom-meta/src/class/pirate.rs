use std::time::Duration;

use crate::{
    buffs::{
        char::{
            CharBuff, CharBuffBasicStatUp, CharBuffBooster, CharBuffDashJump, CharBuffDashSpeed,
            CharBuffDice, CharBuffEnergyCharged, CharBuffExtraMaxHp, CharBuffExtraMaxMp,
            CharBuffExtraMdd, CharBuffExtraPad, CharBuffExtraPdd, CharBuffGuidedBullet,
            CharBuffMorph, CharBuffPartyBooster, CharBuffRideVehicle, Dice, EnergyCharged,
            GuidedBullet, RideVehicle,
        },
        mob,
    },
    id::{skill_id::*, ObjectId},
    shared::ElementAttribute,
    skill::SkillLevel,
    SkillMeta,
};

use super::{AttackBuffSkill, AttackDebuffSkill, AttackSkill, ElemDebuffAttackSkill, MobDebuff};

#[derive(Debug)]
pub struct DashData {
    pub speed: CharBuffDashSpeed,
    pub jump: CharBuffDashJump,
}

#[derive(Debug)]
pub struct BattleshipData {
    pub extra_pad: CharBuffExtraPad,
    pub extra_hp: CharBuffExtraMaxHp,
    pub extra_mp: CharBuffExtraMaxMp,
    pub extra_pdd: CharBuffExtraPdd,
    pub extra_mdd: CharBuffExtraMdd,
    pub ship_ride: CharBuffRideVehicle,
}

#[derive(Debug)]
pub enum PirateSkillData {
    FlashFist(AttackSkill),
    SommersaultKick(AttackSkill),
    DoubleShot(AttackSkill),
    Dash(DashData),

    Booster(CharBuffBooster),

    InvisibleShot(AttackSkill),
    Grenade(AttackDebuffSkill<mob::Burned>),
    BlankShot(AttackDebuffSkill<mob::Stun>),
    RecoildShot(AttackSkill),

    BackspinBlow(AttackSkill),
    DoubleUppercut(AttackSkill),
    CorkscrewBlow(AttackSkill),
    // TODO mp recovery
    OakBarrel(CharBuffMorph),

    // Burst first + octopus gaviota
    FlameThrower(ElemDebuffAttackSkill<mob::Burned>),
    IceSplitter(ElemDebuffAttackSkill<mob::Freeze>),
    HomingBeacon(AttackBuffSkill<GuidedBullet>),
    RollDice(CharBuffDice),
    BurstFire(AttackSkill),

    MapleWarrior(CharBuffBasicStatUp),
    AirStike(AttackSkill),
    RapidFire(AttackSkill),
    Battleship(BattleshipData),
    BattleshipCannon(AttackSkill),
    BattleshipTorpedo(AttackSkill),
    Hypnotize(MobDebuff<mob::Dazzle>),

    SpeedInfusion(CharBuffPartyBooster),
    EnergyCharged(CharBuffEnergyCharged),
}

impl PirateSkillData {
    pub fn from_skill(skill: SkillMeta, lvl: SkillLevel) -> anyhow::Result<Self> {
        Ok(match skill.id {
            PIRATE_FLASH_FIST => Self::FlashFist(AttackSkill::from_skill(skill, lvl)),
            PIRATE_SOMMERSAULT_KICK => Self::SommersaultKick(AttackSkill::from_skill(skill, lvl)),
            PIRATE_DOUBLE_SHOT => Self::DoubleShot(AttackSkill::from_skill(skill, lvl)),
            PIRATE_DASH => Self::Dash(DashData {
                speed: CharBuff::from_skill_x(skill, lvl),
                jump: CharBuff::from_skill_y(skill, lvl),
            }),

            GUNSLINGER_INVISIBLE_SHOT => Self::InvisibleShot(AttackSkill::from_skill(skill, lvl)),
            GUNSLINGER_GRENADE => Self::Grenade(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_dot_skill(skill, lvl),
            }),
            GUNSLINGER_BLANK_SHOT => Self::BlankShot(AttackDebuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill_1(skill, lvl),
            }),
            GUNSLINGER_RECOIL_SHOT => Self::RecoildShot(AttackSkill::from_skill(skill, lvl)),

            BRAWLER_BACKSPIN_BLOW => Self::BackspinBlow(AttackSkill::from_skill(skill, lvl)),
            BRAWLER_DOUBLE_UPPERCUT => Self::DoubleUppercut(AttackSkill::from_skill(skill, lvl)),
            BRAWLER_CORKSCREW_BLOW => Self::CorkscrewBlow(AttackSkill::from_skill(skill, lvl)),
            BRAWLER_OAK_BARREL => {
                Self::OakBarrel(CharBuffMorph::from_skill(skill, lvl, skill.morph(lvl)))
            }

            GUNSLINGER_GUN_BOOSTER | BRAWLER_KNUCKLE_BOOSTER => {
                Self::Booster(CharBuffBooster::from_skill_x(skill, lvl))
            }

            OUTLAW_FLAMETHROWER => Self::FlameThrower(ElemDebuffAttackSkill {
                elem: ElementAttribute::Fire,
                skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_dot_skill(skill, lvl),
            }),
            OUTLAW_ICE_SPLITTER => Self::IceSplitter(ElemDebuffAttackSkill {
                elem: ElementAttribute::Ice,
                skill: AttackSkill::from_skill(skill, lvl),
                debuff: MobDebuff::from_skill_1(skill, lvl),
            }),
            OUTLAW_HOMING_BEACON => Self::HomingBeacon(AttackBuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                buff: CharBuffGuidedBullet::new(
                    skill.id.into(),
                    GuidedBullet {
                        value: 0,
                        mob_id: ObjectId(0),
                    },
                    Duration::from_secs(100000),
                ),
            }),
            CORSAIR_BULLSEYE => Self::HomingBeacon(AttackBuffSkill {
                attack_skill: AttackSkill::from_skill(skill, lvl),
                buff: CharBuffGuidedBullet::new(
                    skill.id.into(),
                    GuidedBullet {
                        value: skill.x(lvl) as u32,
                        mob_id: ObjectId(0),
                    },
                    Duration::from_secs(100000),
                ),
            }),
            OUTLAW_ROLL_OF_THE_DICE => Self::RollDice(CharBuffDice::from_skill(
                skill,
                lvl,
                Dice {
                    value: 0,
                    stats: Default::default(),
                },
            )),
            OUTLAW_BURST_FIRE => Self::BurstFire(AttackSkill::from_skill(skill, lvl)),

            MAPLE_WARRIOR => Self::MapleWarrior(CharBuffBasicStatUp::from_skill_x(skill, lvl)),
            CORSAIR_AIR_STRIKE => Self::AirStike(AttackSkill::from_skill(skill, lvl)),
            CORSAIR_RAPID_FIRE => Self::RapidFire(AttackSkill::from_skill(skill, lvl)),
            CORSAIR_BATTLESHIP => Self::Battleship(BattleshipData {
                extra_pad: CharBuffExtraPad::from_skill_x(skill, lvl),
                extra_hp: CharBuffExtraMaxHp::from_skill_x(skill, lvl),
                extra_mp: CharBuffExtraMaxMp::from_skill_x(skill, lvl),
                extra_pdd: CharBuffExtraPdd::from_skill_x(skill, lvl),
                extra_mdd: CharBuffExtraMdd::from_skill_x(skill, lvl),
                ship_ride: CharBuffRideVehicle::from_skill(skill, lvl, RideVehicle(1932000)),
            }),
            CORSAIR_BATTLESHIP_CANNON => {
                Self::BattleshipCannon(AttackSkill::from_skill(skill, lvl))
            }
            CORSAIR_BATTLESHIP_TORPEDO => {
                Self::BattleshipTorpedo(AttackSkill::from_skill(skill, lvl))
            }
            CORSAIR_HYPNOTIZE => Self::Hypnotize(MobDebuff::from_skill_1(skill, lvl)),
            BUCCANEER_SPEED_INFUSION => {
                Self::SpeedInfusion(CharBuffPartyBooster::from_skill_x(skill, lvl))
            }
            MARAUDER_ENERGY_CHARGE => Self::EnergyCharged(CharBuffEnergyCharged::new(
                skill.id.into(),
                EnergyCharged {
                    energy: 0,
                    max_energy: 10_000,
                    energy_per_attack: skill.x(lvl) as u32,
                },
                skill.time_dur(lvl),
            )),

            _ => anyhow::bail!("Invalid skill id: {}", skill.id.0),
        })
    }
}
