use num::Saturating;
use rand::{thread_rng, Rng};
use shroom_data::{entities::character, entity_ext::SkillPointPages};
use shroom_meta::id::{job_id::JobId, JobClass};
use shroom_proto95::shared::char::CharStatPartial;

pub trait ClampedStatNum: num::Unsigned + Saturating + Ord + Clone + Copy {
    type Signed: num::Signed + Ord;

    fn try_clamp_add(&self, max: Self, delta: Self::Signed) -> Option<Self>;
    fn clamp_add(&mut self, max: Self, delta: Self::Signed);
    fn clamp_add_unsigned(&mut self, max: Self, delta: Self);
    fn max_ratio(max: Self, r: f32) -> Self;

    fn ratio100(&self, max: Self) -> u8;
}

impl ClampedStatNum for u16 {
    type Signed = i16;

    fn try_clamp_add(&self, max: Self, delta: Self::Signed) -> Option<Self> {
        if let Some(v) = self.checked_add_signed(delta) {
            if v <= max {
                return Some(v);
            }
        }

        None
    }

    fn clamp_add(&mut self, max: Self, delta: Self::Signed) {
        let v = self.saturating_add_signed(delta);
        *self = v.min(max);
    }

    fn max_ratio(max: Self, r: f32) -> Self {
        (max as f32 * r).round() as Self
    }

    fn clamp_add_unsigned(&mut self, max: Self, delta: Self) {
        let v = self.saturating_add(delta);
        *self = v.min(max);
    }

    fn ratio100(&self, max: Self) -> u8 {
        ((*self as u32 * 100) / max as u32) as u8
    }
}

impl ClampedStatNum for u32 {
    type Signed = i32;

    fn try_clamp_add(&self, max: Self, delta: Self::Signed) -> Option<Self> {
        if let Some(v) = self.checked_add_signed(delta) {
            if v <= max {
                return Some(v);
            }
        }

        None
    }

    fn clamp_add(&mut self, max: Self, delta: Self::Signed) {
        let v = self.saturating_add_signed(delta);
        *self = v.min(max);
    }

    fn max_ratio(max: Self, r: f32) -> Self {
        (max as f32 * r).round() as Self
    }

    fn clamp_add_unsigned(&mut self, max: Self, delta: Self) {
        let v = self.saturating_add(delta);
        *self = v.min(max);
    }

    fn ratio100(&self, max: Self) -> u8 {
        ((*self as u64 * 100) / max as u64) as u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClampedStat<T> {
    pub value: T,
    pub max: T,
}

impl<T: ClampedStatNum> ClampedStat<T> {
    pub fn new(value: T, max: T) -> Self {
        Self { value, max }
    }

    pub fn maxed(v: T) -> Self {
        Self::new(v, v)
    }

    pub fn ratio100(&self) -> u8 {
        self.value.ratio100(self.max)
    }

    pub fn add_signed(&mut self, delta: T::Signed) {
        self.value.clamp_add(self.max, delta);
    }

    pub fn try_add(&self, delta: T::Signed) -> Option<Self> {
        let max = self.max;
        self.value
            .try_clamp_add(max, delta)
            .map(|value| Self { value, max })
    }

    pub fn set_stat(&mut self, val: T) {
        self.value = self.max.min(val);
    }

    pub fn update_max(&mut self, max: T) {
        self.max = max;
        self.value = self.max.min(self.value);
    }

    pub fn add_ratio(&mut self, ratio: f32) {
        let value = T::max_ratio(self.max, ratio);
        self.value.clamp_add_unsigned(self.max, value);
    }
}

macro_rules! map_partial_stats {
    ($stats:expr, $update_stats:ident, $($stat:ident,)*) => {
        $(if $stats.flags.contains(CharStatsFlags::$stat) {
            $update_stats.$stat = Some($stats.$stat.into()).into();
        })*
    };
}

#[derive(Debug, trackr::Tracked)]
pub struct CharStats {
    #[track(flag)]
    flags: CharStatsFlags,
    pub hp: ClampedStat<u32>,
    pub mp: ClampedStat<u32>,
    pub str: u16,
    pub dex: u16,
    pub int: u16,
    pub luk: u16,
    pub money: u32,
    pub exp: u32,
    pub job: JobId,
    pub ap: u16,
    pub skill_points: SkillPointPages,
    pub fame: u16,
    pub level: u8,
    pub action_locked: bool,
}

impl From<&character::Model> for CharStats {
    fn from(value: &character::Model) -> Self {
        Self {
            hp: ClampedStat::new(value.hp as u32, value.max_hp as u32),
            mp: ClampedStat::new(value.mp as u32, value.max_mp as u32),
            str: value.str as u16,
            dex: value.dex as u16,
            int: value.int as u16,
            luk: value.luk as u16,
            money: value.mesos as u32,
            exp: value.exp as u32,
            job: JobId::try_from(value.job as u16).expect("Job"),
            ap: value.ap as u16,
            fame: value.fame as u16,
            level: value.level as u8,
            action_locked: true,
            flags: Default::default(),
            skill_points: value.get_skill_pages(),
        }
    }
}

impl CharStats {
    pub fn reset(&mut self) {
        self.flags = CharStatsFlags::empty();
        self.action_locked = true;
    }

    pub fn process_level_up(&mut self) {
        let mut r = thread_rng();
        *self.ap_mut() += 5;
        self.skill_points_mut().force_update(|p| *p.get_mut(0) += 3);

        let (hp_gain, mut mp_gain) = match self.job.class() {
            JobClass::Warrior | JobClass::DawnWarrior | JobClass::Aran => {
                (r.gen_range(48..=52), r.gen_range(4..=6))
            }
            JobClass::Magician | JobClass::BlazeWizard => {
                (r.gen_range(10..=14), r.gen_range(48..=52))
            }
            JobClass::Bowman | JobClass::WildHunter | JobClass::WindArcher => {
                (r.gen_range(20..=24), r.gen_range(14..=16))
            }
            JobClass::Thief | JobClass::NightWalker => (r.gen_range(20..=24), r.gen_range(14..=16)),
            JobClass::Pirate | JobClass::ThunderBreaker | JobClass::Mechanic => {
                (r.gen_range(37..=41), r.gen_range(18..=22))
            }
            JobClass::Evan => (r.gen_range(12..=16), r.gen_range(50..=52)),
            JobClass::BattleMage => (r.gen_range(20..=24), r.gen_range(42..=44)),
            JobClass::Beginner | JobClass::Noblesse | JobClass::LegendBeginner => {
                (r.gen_range(12..=16), r.gen_range(10..=12))
            }
            JobClass::GM => (r.gen_range(12..=16), r.gen_range(10..=12)),
            JobClass::Unknown => todo!(),
        };

        mp_gain += self.int as u32 / 10;

        self.hp_mut().max += hp_gain;
        self.mp_mut().max += mp_gain;

        self.heal_hp_ratio(1.);
        self.heal_mp_ratio(1.);

        *self.level_mut() += 1;
    }

    pub fn heal_hp_ratio(&mut self, ratio: f32) {
        self.hp_mut().add_ratio(ratio);
    }

    pub fn heal_mp_ratio(&mut self, ratio: f32) {
        self.mp_mut().add_ratio(ratio);
    }

    pub fn set_hp(&mut self, hp: u32) {
        self.hp_mut().value = hp;
    }

    pub fn update_hp(&mut self, d: i32) {
        self.hp_mut().add_signed(d);
    }

    pub fn update_mp(&mut self, d: i32) {
        self.mp_mut().add_signed(d);
    }

    pub fn try_update_hp(&mut self, d: i32) -> bool {
        if let Some(hp) = self.hp.try_add(d) {
            self.hp_mut().force_set(hp);
            true
        } else {
            false
        }
    }

    pub fn try_update_mp(&mut self, d: i32) -> bool {
        if let Some(mp) = self.mp.try_add(d) {
            self.mp_mut().force_set(mp);
            true
        } else {
            false
        }
    }

    pub fn try_take_sp(&mut self, page: usize) -> bool {
        if *self.skill_points.get(page) > 0 {
            self.skill_points_mut()
                .force_update(|p| *p.get_mut(page) -= 1);
            true
        } else {
            false
        }
    }

    pub fn get_stats_partial(&mut self) -> Option<CharStatPartial> {
        if self.flags.is_empty() {
            return None;
        }

        let mut update_stats = CharStatPartial::default();

        if self.flags.contains(CharStatsFlags::hp) {
            update_stats.hp = Some(self.hp.value).into();
            update_stats.maxhp = Some(self.hp.max).into();
        }

        if self.flags.contains(CharStatsFlags::mp) {
            update_stats.mp = Some(self.mp.value).into();
            update_stats.maxmp = Some(self.mp.max).into();
        }

        if self.flags.contains(CharStatsFlags::skill_points) {
            //TODO use pages
            update_stats.sp = Some(*self.skill_points.get(0)).into();
        }

        map_partial_stats!(
            self,
            update_stats,
            money,
            exp,
            job,
            str,
            dex,
            int,
            luk,
            ap,
            fame,
            level,
        );

        self.reset();

        Some(update_stats)
    }
}
