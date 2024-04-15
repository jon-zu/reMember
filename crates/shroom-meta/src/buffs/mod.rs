use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod char;
pub mod keys;
pub mod mob;


pub trait BuffKey: Copy + Clone + std::fmt::Debug  {
    type Bits;

    fn as_index(&self) -> usize;
    fn flag(&self) -> Self::Bits;
    fn from_flag(flag: Self::Bits) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SkillChance(pub i16);

impl SkillChance {
    pub const fn always() -> Self {
        Self(100)
    }

    pub const fn never() -> Self {
        Self(0)
    }


    pub fn proc(&self) -> bool {
        if self.0 == 100 {
            return true;
        }
        if self.0 == 0 {
            return false;
        }
        rand::thread_rng().gen_ratio(self.0 as u32, 100)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillPerc(pub i16);

impl SkillPerc {
    pub fn ratio(&self) -> f32 {
        self.0 as f32 / 100.0
    }

    pub fn apply(&self, v: i16) -> i16 {
        (v as f32 * (self.0 as f32 / 100.0)) as i16
    }
}