use std::ops::RangeInclusive;

use array_init::array_init;

use shroom_meta::{item::{EquipBaseStats, EquipStat, ItemStat}, tmpl::equip::EquipItemTmpl, Meta};

use crate::entities::equip_item;

/*
#[derive(Debug, From, Into, PartialEq, Eq, Clone, Copy, Default)]
pub struct ItemStat(pub u16);

impl ItemStat {
    pub fn rnd_stat(mut rng: impl Rng, stat: u16) -> Self {
        if stat == 0 {
            return Self(0);
        }

        Self(rng.gen_range(stat.wrapping_sub(2)..=stat).max(1))
    }

    pub fn apply_chaos(&self, mut rng: impl Rng, range: impl SampleRange<i16>) -> Self {
        if self.0 == 0 {
            return Self(0);
        }

        let stat_diff = rng.gen_range(range);
        Self(self.0.saturating_add_signed(stat_diff))
    }
}

#[derive(Debug, Enum, Clone)]
pub enum EquipStat {
    Str,
    Dex,
    Int,
    Luk,
    Hp,
    Mp,
    WeaponAtk,
    MagicAtk,
    WeaponDef,
    MagicDef,
    Accuracy,
    Avoid,
    Craft,
    Speed,
    Jump,
}

pub type EquipStats = EnumMap<EquipStat, ItemStat>;*/

pub trait StatsExt {
    fn from_db_stats(v: &equip_item::Model) -> EquipBaseStats;
    fn from_equip_meta(meta: Meta<EquipItemTmpl>) -> EquipBaseStats;
    fn as_game_stats(&self) -> shroom_proto95::shared::item::EquipStats;
    fn from_game_stats(v: shroom_proto95::shared::item::EquipStats) -> EquipBaseStats;

    fn apply_chaos_scroll(&mut self, rng: impl rand::Rng, range: RangeInclusive<i16>);
    fn add(&self, other: &Self) -> Self;
    fn sum(stats: impl Iterator<Item = Self>) -> Self;
}

impl StatsExt for EquipBaseStats {
    fn add(&self, other: &Self) -> Self {
        let a = self.as_array();
        let b = other.as_array();

        Self::from_array(array_init(|i| ItemStat(a[i].0.saturating_add(b[i].0))))
    }

    fn sum(stats: impl Iterator<Item = Self>) -> Self {
        stats.fold(Self::default(), |acc, next| acc.add(&next))
    }

    fn from_db_stats(v: &equip_item::Model) -> EquipBaseStats {
        enum_map::enum_map! {
            EquipStat::Str => ItemStat(v.str as u16),
            EquipStat::Dex => ItemStat(v.dex as u16),
            EquipStat::Int => ItemStat(v.int as u16),
            EquipStat::Luk => ItemStat(v.luk as u16),
            EquipStat::Hp => ItemStat(v.hp as u16),
            EquipStat::Mp => ItemStat(v.mp as u16),
            EquipStat::Pad => ItemStat(v.weapon_atk as u16),
            EquipStat::Mad => ItemStat(v.magic_atk as u16),
            EquipStat::Pdd => ItemStat(v.weapon_def as u16),
            EquipStat::Mdd => ItemStat(v.magic_def as u16),
            EquipStat::Acc => ItemStat(v.accuracy as u16),
            EquipStat::Eva => ItemStat(v.avoid as u16),
            EquipStat::Craft => ItemStat(v.craft as u16),
            EquipStat::Speed => ItemStat(v.speed as u16),
            EquipStat::Jump => ItemStat(v.jump as u16)
        }
    }

    fn from_game_stats(v: shroom_proto95::shared::item::EquipStats) -> EquipBaseStats {
        enum_map::enum_map! {
            EquipStat::Str => ItemStat(v.str),
            EquipStat::Dex => ItemStat(v.dex),
            EquipStat::Int => ItemStat(v.int),
            EquipStat::Luk => ItemStat(v.luk),
            EquipStat::Hp => ItemStat(v.hp),
            EquipStat::Mp => ItemStat(v.mp),
            EquipStat::Pad => ItemStat(v.pad),
            EquipStat::Mad => ItemStat(v.mad),
            EquipStat::Pdd => ItemStat(v.pdd),
            EquipStat::Mdd => ItemStat(v.mdd),
            EquipStat::Acc => ItemStat(v.acc),
            EquipStat::Eva => ItemStat(v.eva),
            EquipStat::Craft => ItemStat(v.craft),
            EquipStat::Speed => ItemStat(v.speed),
            EquipStat::Jump => ItemStat(v.jump)
        }
    }

    fn as_game_stats(&self) -> shroom_proto95::shared::item::EquipStats {
        shroom_proto95::shared::item::EquipStats {
            str: self[EquipStat::Str].0,
            dex: self[EquipStat::Dex].0,
            int: self[EquipStat::Int].0,
            luk: self[EquipStat::Luk].0,
            hp: self[EquipStat::Hp].0,
            mp: self[EquipStat::Mp].0,
            pad: self[EquipStat::Pad].0,
            mad: self[EquipStat::Mad].0,
            pdd: self[EquipStat::Pdd].0,
            mdd: self[EquipStat::Mdd].0,
            acc: self[EquipStat::Acc].0,
            eva: self[EquipStat::Eva].0,
            craft: self[EquipStat::Craft].0,
            speed: self[EquipStat::Speed].0,
            jump: self[EquipStat::Jump].0,
        }
    }

    fn from_equip_meta(meta: Meta<EquipItemTmpl>) -> EquipBaseStats {
        meta.stats.base.clone()
    }

    fn apply_chaos_scroll(&mut self, mut rng: impl rand::Rng, range: RangeInclusive<i16>) {
        for stat in self.values_mut() {
            *stat = stat.apply_chaos(&mut rng, range.clone());
        }
    }
}
