pub mod buffs;
pub mod class;
pub mod drops;
pub mod exp_table;
pub mod fmt;
pub mod id;
pub mod mob;
pub mod shared;
pub mod skill;
pub mod srv;
pub mod svc;
pub mod twod;
//pub mod wz2;
pub mod quest;
pub mod tmpl;
pub mod item;
pub mod field;
pub mod npc;

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use shroom_pkt::ShroomPacket;
pub use svc::*;

pub mod util {
    pub mod search;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Deserialize, Serialize)]
#[repr(u8)]
pub enum MoveAbility {
    Stop = 0,
    Walk = 1,
    WalkRandom = 2,
    Jump = 3,
    Fly = 4,
    FlyRandom = 5,
    Escort = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Deserialize, Serialize)]
#[repr(u8)]
pub enum MobMoveAbility {
    Jump,
    Fly,
    Move,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Deserialize, Serialize)]
#[repr(u8)]
pub enum MoveActionType {
    Walk = 1,
    Stand = 2,
    Jump = 3,
    Alert = 4,
    Prone = 5,
    Fly1 = 6,
    Ladder = 7,
    Rope = 8,
    Dead = 9,
    Sit = 0xA,
    Stand0 = 0xB,
    Hungry = 0xC,
    Rest0 = 0xD,
    Rest1 = 0xE,
    Hang = 0xF,
    Chase = 0x10,
    Fly2 = 0x11,
    Fly2Move = 0x12,
    Dash2 = 0x13,
    RocketBooster = 0x14,
    TeslaCoilTriangle = 0x15,
    No = 0x16,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, ShroomPacket)]
pub struct CharLevel(pub u8);

impl From<u8> for CharLevel {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Money(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProcChance(pub u8);

/// Popularity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pop(pub i16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct QuestDataId(pub u32);

impl ProcChance {
    pub fn as_percent(&self) -> f32 {
        self.0 as f32 / 100.0
    }

    pub fn proc(&self, rng: &mut impl rand::Rng) -> bool {
        rng.gen_range(0..100) < self.0
    }
}



pub type PetTameness = u8;
pub type TamingMobLevel = u8;
pub type World = u8;
pub type PetSkill = u8;

pub const FIELD_REGIONS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
