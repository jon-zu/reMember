pub mod it;

use bitflags::bitflags;
use derive_more::{Add, AddAssign, Deref, DerefMut};
use enum_map::{Enum, EnumMap};
use num_enum::TryFromPrimitive;
use rand::distributions::uniform::SampleRange;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;


use crate::id::job_id::JobId;
use crate::id::JobClass;


use crate::ProcChance;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Deserialize, Serialize)]
pub struct ItemStat(pub u16);

impl Add for ItemStat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Deserialize, Serialize)]
pub struct ItemStatRatio(pub u16);

impl ItemStat {
    pub fn rnd_stat(mut rng: impl Rng, stat: u16, variance: u16) -> Self {
        if stat == 0 {
            return Self(0);
        }

        Self(rng.gen_range(stat.wrapping_sub(variance)..=stat.wrapping_add(variance)).max(1))
    }

    pub fn apply_chaos(&self, mut rng: impl Rng, range: impl SampleRange<i16>) -> Self {
        if self.0 == 0 {
            return Self(0);
        }

        let stat_diff = rng.gen_range(range);
        Self(self.0.saturating_add_signed(stat_diff))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStatRange(pub RangeInclusive<ItemStat>);

impl ItemStatRange {
    pub fn new(min: ItemStat, max: ItemStat) -> Self {
        Self(min..=max)
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
    pub struct JobFlag: u8 {
        const WARRIOR = 1 << 0;
        const MAGICIAN = 1 << 1;
        const BOWMAN = 1 << 2;
        const THIEF = 1 << 3;
        const PIRATE = 1 << 4;
        const BEGINNER = 1 << 5;
    }
}

impl From<i8> for JobFlag {
    fn from(value: i8) -> Self {
        match value {
            -1 => JobFlag::BEGINNER,
            0 => JobFlag::all(),
            _ => JobFlag::from_bits_truncate(value as u8),
        }
    }
}

impl JobFlag {
    pub fn contains_job(&self, job: JobId) -> bool {
        match job.class() {
            JobClass::Warrior | JobClass::DawnWarrior | JobClass::Aran => {
                self.contains(JobFlag::WARRIOR)
            }
            JobClass::Magician | JobClass::BlazeWizard | JobClass::Evan | JobClass::BattleMage => {
                self.contains(JobFlag::MAGICIAN)
            }
            JobClass::Bowman | JobClass::WindArcher | JobClass::WildHunter => {
                self.contains(JobFlag::BOWMAN)
            }
            JobClass::Thief | JobClass::NightWalker => self.contains(JobFlag::THIEF),
            JobClass::Pirate | JobClass::ThunderBreaker | JobClass::Mechanic => {
                self.contains(JobFlag::PIRATE)
            }
            JobClass::Beginner | JobClass::LegendBeginner | JobClass::Noblesse => {
                self.contains(JobFlag::BEGINNER)
            }
            JobClass::GM => true,
            JobClass::Unknown => unreachable!(),
        }
    }
}



#[derive(Debug, Enum, Clone, Deserialize, Serialize)]
pub enum EquipStat {
    Str,
    Dex,
    Int,
    Luk,
    Hp,
    Mp,
    Pad,
    Mad,
    Pdd,
    Mdd,
    Acc,
    Eva,
    Craft,
    Speed,
    Jump,
}

impl EquipStat {
    pub fn max_enhance_stat(&self) -> ItemStat {
        match self {
            EquipStat::Str | EquipStat::Dex | EquipStat::Int | EquipStat::Luk => ItemStat(3),
            EquipStat::Hp | EquipStat::Mp => ItemStat(10),
            EquipStat::Pad | EquipStat::Mad => ItemStat(2),
            EquipStat::Pdd | EquipStat::Mdd => ItemStat(5),
            EquipStat::Acc | EquipStat::Eva => ItemStat(5),
            EquipStat::Craft => ItemStat(0),
            EquipStat::Speed => ItemStat(1),
            EquipStat::Jump => ItemStat(1),
        }
    }

    pub fn add_on_enhance(&self) -> bool {
        matches!(self, Self::Str | Self::Dex | Self::Int | Self::Luk)
    }
}

#[derive(Debug, Clone, Deref, DerefMut, Default,  Deserialize, Serialize)]
pub struct EquipBaseStats(pub EnumMap<EquipStat, ItemStat>);

impl Add for EquipBaseStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.map(|stat, value| value + other.0[stat]))
    }
}

impl AddAssign<&EquipBaseStats> for EquipBaseStats {
    fn add_assign(&mut self, other: &Self) {
        for (stat, value) in self.0.iter_mut() {
            *value = *value + other.0[stat];
        }
    }
}

impl EquipBaseStats {
    pub fn from_fn(f: impl Fn(EquipStat) -> ItemStat) -> Self {
        Self(EnumMap::from_fn(f))
    }
    pub fn apply_chaos_scroll(&mut self, mut rng: impl rand::Rng, range: RangeInclusive<i16>) {
        for stat in self.0.values_mut() {
            *stat = stat.apply_chaos(&mut rng, range.clone());
        }
    }

    pub fn apply_enhancement(&mut self, mut rng: impl Rng) {
        for (stat, value) in self.0.iter_mut() {
            if value.0 == 0 {
                // Skip
                if !stat.add_on_enhance() {
                    continue;
                }

                // Check for add chance
                /*if !rng.gen_bool(0.01) {
                    continue;
                }*/
            }

            let range = 0..=stat.max_enhance_stat().0;
            value.0 = value.0.saturating_add(rng.gen_range(range));
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EquipStats {
    pub base: EquipBaseStats,
    pub inc_max_hp_r: ItemStatRatio,
    pub inc_max_mp_r: ItemStatRatio,
}

impl EquipStats {
    pub fn apply_enhancement(&mut self, mut rng: impl Rng) {
        for (stat, value) in self.base.0.iter_mut() {
            if value.0 == 0 {
                // Skip
                if !stat.add_on_enhance() {
                    continue;
                }

                // Check for add chance
                /*if !rng.gen_bool(0.01) {
                    continue;
                }*/
            }

            let range = 0..=stat.max_enhance_stat().0;
            value.0 = value.0.saturating_add(rng.gen_range(range));
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ScrollChance {
    Perfect,
    Fixed(ProcChance),
    Dynamic(Vec<ProcChance>),
}

impl ScrollChance {
    pub fn proc(&self, lvl: usize, rng: &mut impl Rng) -> bool {
        match self {
            ScrollChance::Fixed(chance) => chance.proc(rng),
            ScrollChance::Dynamic(chances) => chances.get(lvl).unwrap_or(&ProcChance(0)).proc(rng),
            //TODO fix that
            ScrollChance::Perfect => false,
        }
    }
}


#[derive(Debug, Deserialize, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum EnchantCategory {
    None = 0,
    Normal = 1,
    Visitor = 2,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive, Deserialize, Serialize)]
#[repr(u8)]
pub enum AttackSpeedDegree {
    FASTEST = 0,
    FASTEST_ = 1,
    FASTER = 2,
    FASTER_ = 3,
    FAST = 4,
    FAST_ = 5,
    NORMAL = 6,
    NORMAL_ = 7,
    SLOW = 8,
    SLOW_ = 9,
    SLOWER = 10,
    SLOWER_ = 11,
    SLOWEST = 12,
}



/*
<enum name="ITEMVARIATIONOPTION" type="int" length="0x4" >
    <member name="ITEMVARIATION_NONE" value="0" />
    <member name="ITEMVARIATION_BETTER" value="1" />
    <member name="ITEMVARIATION_NORMAL" value="2" />
    <member name="ITEMVARIATION_GREAT" value="3" />
    <member name="ITEMVARIATION_PERPECT" value="4" />
    <member name="ITEMVARIATION_GACHAPON" value="5" />
</enum>



<enum name="&lt;unnamed_02f0&gt;" type="int" length="0x4" >
    <member name="ITEMQUALITY_COLOR_BAD" value="819" />
    <member name="ITEMQUALITY_COLOR_GOOD" value="879" />
    <member name="ITEMQUALITY_COLOR_VERYGOOD" value="3183" />
    <member name="ITEMQUALITY_COLOR_PREMIUM" value="4032" />
    <member name="ITEMQUALITY_COLOR_EXCELLENT" value="1008" />
    <member name="ITEMQUALITY_COLOR_SPECIAL" value="3848" />
</enum>

<enum name="&lt;unnamed_023c&gt;" type="int" length="0x4" >
    <member name="RECOVERYHP" value="10151" />
    <member name="RECOVERYMP" value="10156" />
    <member name="RECOVERYHPMP_STATE_SIT" value="20181" />
    <member name="HP_STATE_KILL" value="20401" />
    <member name="MP_STATE_KILL" value="20406" />
    <member name="INVINCIBLE_INC1" value="20366" />
    <member name="INVINCIBLE_INC2" value="30366" />
    <member name="STATUS_TIME" value="20369" />
    <member name="INVINCIBLE" value="30371" />
    <member name="LEARN_SKILL_HASTE" value="31001" />
    <member name="LEARN_SKILL_MYSTIC_DOOR" value="31002" />
    <member name="LEARN_SKILL_SHARP_EYES" value="31003" />
    <member name="LEARN_SKILL_HYPER_BODY" value="31004" />
    <member name="AUTOSTEAL1" value="30701" />
    <member name="AUTOSTEAL2" value="30702" />
</enum>
<enum name="&lt;unnamed_023d&gt;" type="int" length="0x4" >
    <member name="MONSTER_BOMB_START" value="30" />
    <member name="MONSTER_BOMB_START_HEIGHT" value="160" />
    <member name="MONSTER_BOMB_START_DELAY" value="450" />
    <member name="MONSTER_BOMB_LIMITED_TIME" value="500" />
</enum>


<enum name="ENCHANT_SCROLL_CATEGORY" type="int" length="0x4" >
    <member name="ENCHANT_SCROLL_CATEGORY_NORMAL" value="1" />
    <member name="ENCHANT_SCROLL_CATEGORY_VISITOR" value="2" />
</enum>


<enum name="&lt;unnamed_02bb&gt;" type="int" length="0x4" >
    <member name="ITEM_CATEGORY_STRENGTHEN_GEM" value="425" />
    <member name="ITEM_CATEGORY_MONSTER_CRYSTAL" value="426" />
    <member name="ITEM_CATEGORY_HIDDEN" value="999" />
    <member name="ITEM_CATEGORY_EQUIP_DISASSEMBLE" value="998" />
    <member name="ITEM_CATEGORY_CONSUME" value="200" />
    <member name="ITEM_CATEGORY_INSTALL" value="300" />
    <member name="ITEM_CATEGORY_ETC" value="400" />
    <member name="ITEM_CATEGORY_CATALYST" value="4130" />
</enum>
*/
