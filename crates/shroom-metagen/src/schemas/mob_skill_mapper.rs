use std::{collections::BTreeMap, time::Duration};

use shroom_meta::{
    buffs::{
        char::{CharBuff, CharBuffStat},
        mob::{CounterData, MobBuff, MobBuffStat},
        SkillChance,
    },
    id::{BuffId, CharacterId, MobId, MobSkillId},
    mob::{
        MobAffectedAreaSkill, MobBuffSkill, MobBuffSkillData, MobCharBuffSkill,
        MobCharBuffSkillData, MobHealSkill, MobPartizanBuffSkill, MobPartizanBuffSkillData,
        MobSkill, MobSkillInfo, MobSkillRange, MobSkills, MobSpreadFromUserSkill, MobSummonSkill,
    },
    twod::{Box2, Vec2},
};

use super::{
    shroom_schemas, IntoBool, IntoNum
};


trait IntoMobBuffValue: Sized {
    fn into_mob_buff_value(value: &shroom_schemas::MobSkillValueLevelValue)
        -> anyhow::Result<Self>;
}


impl IntoMobBuffValue for i16 {
    fn into_mob_buff_value(
        value: &shroom_schemas::MobSkillValueLevelValue,
    ) -> anyhow::Result<Self> {
        Ok(value.x.into_num() as i16)
    }
}

impl IntoMobBuffValue for CounterData {
    fn into_mob_buff_value(
        value: &shroom_schemas::MobSkillValueLevelValue,
    ) -> anyhow::Result<Self> {
        Ok(CounterData {
            n: value.x.into_num() as i16,
            w: value.y.into_num() as u32,
        })
    }
}


pub trait SingularCharBuffStat: CharBuffStat { }
impl SingularCharBuffStat for shroom_meta::buffs::char::Seal {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Darkness {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Weakness {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Stun {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Curse {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Poison {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Slow {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Attract {}
impl SingularCharBuffStat for shroom_meta::buffs::char::BanMap {}
impl SingularCharBuffStat for shroom_meta::buffs::char::ReverseInput {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Fear {}
impl SingularCharBuffStat for shroom_meta::buffs::char::Frozen {}

impl<T: SingularCharBuffStat + From<i16>> IntoMobBuffValue for T {
    fn into_mob_buff_value(
        value: &shroom_schemas::MobSkillValueLevelValue,
    ) -> anyhow::Result<Self> {
        let v = value.x.into_num() as i16;
        Ok(if v == 0 { T::from(1i16) } else { T::from(v) })
    }
}

impl TryFrom<&shroom_schemas::MobSkillValueLevelValue> for Option<MobSkillRange> {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        Ok(match (&value.lt, &value.rb) {
            (Some(lt), Some(rb)) => Some(MobSkillRange(Box2::new(
                Vec2::new(lt.x as i16, lt.y as i16).to_point(),
                Vec2::new(rb.x as i16, rb.y as i16).to_point(),
            ))),
            _ => None,
        })
    }
}

impl TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobHealSkill {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        Ok(Self {
            info: value.try_into()?,
            amount: value.x.into_num() as u16,
            variance: value.y.into_num() as u16,
        })
    }
}

impl TryFrom<&shroom_schemas::MobSkillValueLevelValue> for i16 {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        Ok(value.x.into_num() as i16)
    }
}

impl TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobSkillInfo {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        Ok(Self {
            hp_threshold: value.hp.as_ref().map(|v| v.into_num() as u8),
            mp_cost: value.mp_con.into_num() as u16,
            interval: value
                .interval
                .as_ref()
                .map(|v| Duration::from_secs(v.into_num() as u64)),
        })
    }
}

impl<U: IntoMobBuffValue, T: MobBuffStat<Inner = U>>
    TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobPartizanBuffSkill<T>
{
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        //TODO range
        let data = T::from_inner(U::into_mob_buff_value(value)?);
        let time = Duration::from_secs(value.time.into_num() as u64);

        Ok(Self {
            info: value.try_into()?,
            stat: MobBuff {
                id: BuffId::default(),
                data,
                dur: time,
                src: CharacterId(0),
            },
            range: value.try_into()?,
        })
    }
}
impl<U: IntoMobBuffValue, T: MobBuffStat<Inner = U>>
    TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobBuffSkill<T>
{
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        let data = T::from_inner(U::into_mob_buff_value(value)?);
        let time = Duration::from_secs(value.time.into_num() as u64);

        Ok(Self {
            info: value.try_into()?,
            stat: MobBuff {
                id: BuffId::default(),
                data,
                dur: time,
                src: CharacterId(0),
            },
        })
    }
}

impl<U: IntoMobBuffValue, T: CharBuffStat<Inner = U>>
    TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobCharBuffSkill<T>
{
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        let data = T::from_inner(U::into_mob_buff_value(value)?);
        let time = Duration::from_secs(value.time.into_num() as u64);

        Ok(Self {
            info: value.try_into()?,
            stat: CharBuff {
                id: BuffId::default(),
                data,
                dur: time,
            },
            skill_chance: value
                .prop
                .as_ref()
                .map(|v| SkillChance(v.into_num() as i16)),
            range: value.try_into()?,
        })
    }
}

impl TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobSummonSkill {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        let mobs = [
            value._0.into_num(),
            value._1.into_num(),
            value._2.into_num(),
            value._3.into_num(),
            value._4.into_num(),
            value._5.into_num(),
            value._6.into_num(),
            value._7.into_num(),
            value._8.into_num(),
            value._9.into_num(),
            value._10.into_num(),
            value._11.into_num(),
        ]
        .iter()
        .filter_map(|v| {
            if *v == 0 {
                None
            } else {
                Some(MobId(*v as u32))
            }
        })
        .collect();
        Ok(Self {
            info: value.try_into()?,
            mobs,
            limit: value.limit.into_num() as usize,
            summon_effect: value.summon_effect.into_num() as usize,
        })
    }
}

impl TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobAffectedAreaSkill {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        Ok(Self {
            info: value.try_into()?,
            range: Option::<MobSkillRange>::try_from(value).unwrap().unwrap(),
            elem_attr: value
                .elem_attr
                .as_ref()
                .map(|v| v.as_str().try_into().unwrap()),
            dur: Duration::from_secs(value.time.into_num() as u64),
            value: value.x.into_num() as i16,
            prop: SkillChance(value.prop.into_num() as i16),
            count: value.count.into_num() as usize,
        })
    }
}

impl TryFrom<&shroom_schemas::MobSkillValueLevelValue> for MobSpreadFromUserSkill {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkillValueLevelValue) -> Result<Self, Self::Error> {
        Ok(Self {
            info: value.try_into()?,
            count: value.count.into_num() as usize,
            random_target: value.random_target.into_bool(),
            range: Option::<MobSkillRange>::try_from(value).unwrap().unwrap(),
            spread_skill: (value.x.into_num() as u8).try_into()?,
        })
    }
}

pub struct MobSkillWithId<'a>(pub MobSkillId, &'a shroom_schemas::MobSkillValueLevelValue);

impl<'a> TryFrom<MobSkillWithId<'a>> for MobSkill {
    type Error = anyhow::Error;

    fn try_from(v: MobSkillWithId<'a>) -> Result<Self, Self::Error> {
        let MobSkillWithId(id, value) = v;
        Ok(match id {
            MobSkillId::POWERUP => Self::Buff(MobBuffSkillData::PowerUp(value.try_into()?)),
            MobSkillId::MAGICUP => Self::Buff(MobBuffSkillData::MagicUp(value.try_into()?)),
            MobSkillId::PGUARDUP => Self::Buff(MobBuffSkillData::PGuardUp(value.try_into()?)),
            MobSkillId::MGUARDUP => Self::Buff(MobBuffSkillData::MGuardUp(value.try_into()?)),
            MobSkillId::HASTE => Self::Buff(MobBuffSkillData::Haste(value.try_into()?)),
            MobSkillId::PHYSICAL_IMMUNE => {
                Self::Buff(MobBuffSkillData::PhysicalImmune(value.try_into()?))
            }
            MobSkillId::MAGIC_IMMUNE => {
                Self::Buff(MobBuffSkillData::MagicImmune(value.try_into()?))
            }
            MobSkillId::HARDSKIN => Self::Buff(MobBuffSkillData::HardSkin(value.try_into()?)),
            MobSkillId::PCOUNTER => Self::Buff(MobBuffSkillData::PhyicalCounter(value.try_into()?)),
            MobSkillId::MCOUNTER => Self::Buff(MobBuffSkillData::MagicCounter(value.try_into()?)),
            MobSkillId::PMCOUNTER => {
                let p = value.try_into()?;
                let m = value.try_into()?;
                Self::Buff(MobBuffSkillData::PMCounter(p, m))
            }
            MobSkillId::PAD => Self::Buff(MobBuffSkillData::Pad(value.try_into()?)),
            MobSkillId::MAD => Self::Buff(MobBuffSkillData::Mad(value.try_into()?)),
            MobSkillId::PDR => Self::Buff(MobBuffSkillData::Pdr(value.try_into()?)),
            MobSkillId::MDR => Self::Buff(MobBuffSkillData::Mdr(value.try_into()?)),
            MobSkillId::ACC => Self::Buff(MobBuffSkillData::Acc(value.try_into()?)),
            MobSkillId::EVA => Self::Buff(MobBuffSkillData::Eva(value.try_into()?)),
            MobSkillId::SPEED => Self::Buff(MobBuffSkillData::Speed(value.try_into()?)),

            MobSkillId::POWERUP_M => {
                Self::PartizanBuff(MobPartizanBuffSkillData::PowerUpM(value.try_into()?))
            }
            MobSkillId::MAGICUP_M => {
                Self::PartizanBuff(MobPartizanBuffSkillData::MagicUpM(value.try_into()?))
            }
            MobSkillId::PGUARDUP_M => {
                Self::PartizanBuff(MobPartizanBuffSkillData::PGuardUpM(value.try_into()?))
            }
            MobSkillId::MGUARDUP_M => {
                Self::PartizanBuff(MobPartizanBuffSkillData::MGuardUpM(value.try_into()?))
            }
            MobSkillId::HASTE_M => {
                Self::PartizanBuff(MobPartizanBuffSkillData::HasteM(value.try_into()?))
            }
            MobSkillId::HEAL_M => {
                Self::PartizanBuff(MobPartizanBuffSkillData::HealM(value.try_into()?))
            }

            MobSkillId::SEAL => Self::CharBuff(MobCharBuffSkillData::Seal(value.try_into()?)),
            MobSkillId::DARKNESS => {
                Self::CharBuff(MobCharBuffSkillData::Darkness(value.try_into()?))
            }
            MobSkillId::WEAKNESS => {
                Self::CharBuff(MobCharBuffSkillData::Weakness(value.try_into()?))
            }
            MobSkillId::STUN => Self::CharBuff(MobCharBuffSkillData::Stun(value.try_into()?)),
            MobSkillId::CURSE => Self::CharBuff(MobCharBuffSkillData::Curse(value.try_into()?)),
            MobSkillId::POISON => Self::CharBuff(MobCharBuffSkillData::Poison(value.try_into()?)),
            MobSkillId::SLOW => Self::CharBuff(MobCharBuffSkillData::Slow(value.try_into()?)),
            MobSkillId::DISPEL => Self::CharBuff(MobCharBuffSkillData::Dispel(value.try_into()?)),
            MobSkillId::ATTRACT => Self::CharBuff(MobCharBuffSkillData::Attract(value.try_into()?)),
            MobSkillId::BANMAP => Self::CharBuff(MobCharBuffSkillData::BanMap(value.try_into()?)),
            MobSkillId::REVERSE_INPUT => {
                Self::CharBuff(MobCharBuffSkillData::ReverseInput(value.try_into()?))
            }
            MobSkillId::FEAR => Self::CharBuff(MobCharBuffSkillData::Fear(value.try_into()?)),
            MobSkillId::FROZEN => Self::CharBuff(MobCharBuffSkillData::Frozen(value.try_into()?)),

            MobSkillId::AREA_POISON => Self::AreaPoison(value.try_into()?),
            MobSkillId::AREA_FIRE => Self::AreaFire(value.try_into()?),

            MobSkillId::SUMMON => Self::Summon(value.try_into()?),
            MobSkillId::SUMMON_CUBE => Self::SummonCube(value.try_into()?),

            //TODO
            MobSkillId::MOBSKILLL_SPREADSKILLFROMUSER => {
                Self::MobSpreadFromUserSkill(value.try_into()?)
            }
            MobSkillId::SEALSKILL => Self::SealSkill,
            MobSkillId::STOPMOTION => Self::StopMotion,
            MobSkillId::STOPPORTION => Self::StopPortion,
            MobSkillId::HEALBYDAMAGE => Self::HealByDamage,
            MobSkillId::BALROGCOUNTER => Self::BalrogCounter,
            MobSkillId::BIND => Self::Bind,
            MobSkillId::UNDEAD => Self::Undead,
        })
    }
}

impl TryFrom<&shroom_schemas::MobSkill> for MobSkills {
    type Error = anyhow::Error;

    fn try_from(value: &shroom_schemas::MobSkill) -> Result<Self, Self::Error> {
        let mut skills: BTreeMap<MobSkillId, BTreeMap<u8, MobSkill>> = BTreeMap::new();
        for (id, skill) in value.0.iter() {
            let id: MobSkillId = id.parse::<u8>()?.try_into()?;
            for (lvl, skill) in skill.level.iter() {
                let lvl = lvl.parse::<u8>()?;
                skills
                    .entry(id)
                    .or_default()
                    .insert(lvl, MobSkillWithId(id, skill).try_into()?);
            }
        }
        Ok(Self(skills))
    }
}
