//TODO find a way to auto generate this from wz files or verify this with files during build time

pub mod item_id;
pub mod job_id;
pub mod map_id;
pub mod mob_skill_id;
pub mod skill_id;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use shroom_pkt::{shroom_enum_code, DecodePacket, EncodePacket};

use crate::fmt::ShroomDisplay;

pub use self::item_id::ItemId;
pub use self::job_id::JobClass;
pub use self::map_id::FieldId;
pub use mob_skill_id::MobSkillId;
pub use skill_id::SkillId;

#[macro_export]
macro_rules! shroom_id {
    ($name:ident, $ty:ty) => {
        #[derive(
            Default,
            Debug,
            PartialEq,
            Eq,
            Clone,
            Copy,
            Hash,
            Ord,
            PartialOrd,
            serde::Serialize,
            serde::Deserialize,
        )]
        pub struct $name(pub $ty);

        impl From<&$name> for $ty {
            fn from(val: &$name) -> Self {
                val.0
            }
        }

        impl From<$name> for $ty {
            fn from(val: $name) -> Self {
                val.0
            }
        }

        impl From<$ty> for $name {
            fn from(val: $ty) -> Self {
                $name(val)
            }
        }

        shroom_pkt::packet_wrap!($name<>, $ty, $ty);
        /*
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }*/
    };
}

shroom_id!(ItemOptionId, u16);

shroom_id!(FaceId, u32);

impl FaceId {
    pub const MOTIVATED_LOOK_M: Self = Self(20000); // Face
    pub const PERPLEXED_STARE: Self = Self(20001);
    pub const LEISURE_LOOK_M: Self = Self(20002);
    pub const MOTIVATED_LOOK_F: Self = Self(21000);
    pub const FEARFUL_STARE_M: Self = Self(21001);
    pub const LEISURE_LOOK_F: Self = Self(21002);
    pub const FEARFUL_STARE_F: Self = Self(21201);
    pub const PERPLEXED_STARE_HAZEL: Self = Self(20401);
    pub const LEISURE_LOOK_HAZEL: Self = Self(20402);
    pub const MOTIVATED_LOOK_AMETHYST: Self = Self(21700);
    pub const MOTIVATED_LOOK_BLUE: Self = Self(20100);
}

shroom_id!(HairId, u32);

impl HairId {
    pub const BLACK_TOBEN: Self = Self(30000); // Hair
    pub const ZETA: Self = Self(30010);
    pub const BLACK_REBEL: Self = Self(30020);
    pub const BLACK_BUZZ: Self = Self(30030);
    pub const BLACK_SAMMY: Self = Self(31000);
    pub const BLACK_EDGY: Self = Self(31040);
    pub const BLACK_CONNIE: Self = Self(31050);
}

shroom_enum_code!(
    Skin,
    u8,
    Normal = 0,
    Dark = 1,
    Black = 2,
    Pale = 3,
    Blue = 4,
    Green = 5,
    White = 9,
    Pink = 10
);

shroom_id!(MobId, u32);

impl std::fmt::Display for MobId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl ShroomDisplay for MobId {}

shroom_id!(FootholdId, u16);

impl FootholdId {
    pub const fn none() -> Self {
        Self(0)
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0
    }
}

impl FromStr for FootholdId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

shroom_id!(ReactorId, u32);
shroom_id!(NpcId, u32);
impl std::fmt::Display for NpcId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl NpcId {
    pub const ADMIN: NpcId = NpcId(9010000);
}

impl ShroomDisplay for NpcId {}

shroom_id!(ObjectId, u32);

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

shroom_id!(CharacterId, u32);
impl std::fmt::Display for CharacterId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
shroom_id!(QuestId, u16);

pub type CashID = u64;
pub type Money = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffId {
    Skill(SkillId),
    MobSkill(MobSkillId, u8),
    Item(ItemId),
}

impl BuffId {
    pub fn to_src32(&self) -> u32 {
        match self {
            Self::Skill(id) => id.0,
            Self::MobSkill(id, lvl) => (*lvl as u32) << 16 | (*id as u32),
            Self::Item(id) => -(id.0 as i32) as u32,
        }
    }
}

impl EncodePacket for BuffId {
    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::new(4);

    fn encode<B: bytes::BufMut>(
        &self,
        pw: &mut shroom_pkt::PacketWriter<B>,
    ) -> shroom_pkt::PacketResult<()> {
        match self {
            Self::Skill(id) => id.0.encode(pw),
            Self::MobSkill(id, level) => {
                (*id as u16).encode(pw)?;
                (*level as u16).encode(pw)?;
                Ok(())
            }
            Self::Item(id) => {
                let id = id.0 as i32;
                (-id).encode(pw)?;
                Ok(())
            }
        }
    }
}

impl<'de> DecodePacket<'de> for BuffId {
    fn decode(pr: &mut shroom_pkt::PacketReader<'de>) -> shroom_pkt::PacketResult<Self> {
        let v = pr.read_i32()?;
        //TODO mob buff?
        Ok(if v < 0 {
            BuffId::Item(ItemId((-v) as u32))
        } else {
            BuffId::Skill(SkillId(v as u32))
        })
    }
}

impl From<SkillId> for BuffId {
    fn from(id: SkillId) -> Self {
        Self::Skill(id)
    }
}

impl From<ItemId> for BuffId {
    fn from(id: ItemId) -> Self {
        Self::Item(id)
    }
}

impl Default for BuffId {
    fn default() -> Self {
        Self::Skill(SkillId::default())
    }
}
