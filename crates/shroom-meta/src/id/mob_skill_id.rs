use num_enum::TryFromPrimitive;

#[allow(non_camel_case_types)]
#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Copy,
    Hash,
    Ord,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    TryFromPrimitive,
)]
#[repr(u8)]
pub enum MobSkillId {
    POWERUP = 100,
    MAGICUP = 0x65,
    PGUARDUP = 0x66,
    MGUARDUP = 0x67,
    HASTE = 0x68,
    POWERUP_M = 0x6E,
    MAGICUP_M = 0x6F,
    PGUARDUP_M = 0x70,
    MGUARDUP_M = 0x71,
    HEAL_M = 0x72,
    HASTE_M = 0x73,
    SEAL = 120,
    DARKNESS = 121,
    WEAKNESS = 0x7A,
    STUN = 0x7B,
    CURSE = 124,
    POISON = 125,
    SLOW = 126,
    DISPEL = 127,
    ATTRACT = 0x80,
    BANMAP = 0x81,
    AREA_FIRE = 0x82,
    AREA_POISON = 0x83,
    REVERSE_INPUT = 0x84,
    UNDEAD = 0x85,
    STOPPORTION = 0x86,
    STOPMOTION = 0x87,
    FEAR = 0x88,
    FROZEN = 0x89,
    PHYSICAL_IMMUNE = 0x8C,
    MAGIC_IMMUNE = 0x8D,
    HARDSKIN = 142,

    PCOUNTER = 0x8F,
    MCOUNTER = 0x90,
    PMCOUNTER = 0x91,
    PAD = 0x96,
    MAD = 0x97,
    PDR = 0x98,
    MDR = 0x99,
    ACC = 0x9A,
    EVA = 0x9B,
    SPEED = 0x9C,
    SEALSKILL = 0x9D,
    BALROGCOUNTER = 0x9E,
    MOBSKILLL_SPREADSKILLFROMUSER = 0xA0,
    HEALBYDAMAGE = 0xA1,
    BIND = 0xA2,
    SUMMON = 0xC8,
    SUMMON_CUBE = 0xC9,
}

impl MobSkillId {
    pub fn is_stat_change(&self) -> bool {
        matches!(
            self,
            Self::POWERUP
                | Self::MAGICUP
                | Self::PGUARDUP
                | Self::MGUARDUP
                | Self::HASTE
                | Self::SPEED
                | Self::PHYSICAL_IMMUNE
                | Self::MAGIC_IMMUNE
                | Self::HARDSKIN
                | Self::PAD
                | Self::MAD
                | Self::PDR
                | Self::MDR
                | Self::ACC
                | Self::EVA
                | Self::SEALSKILL
                | Self::PCOUNTER
                | Self::MCOUNTER
                | Self::PMCOUNTER
        )
    }

    pub fn is_user_stat_change(&self) -> bool {
        matches!(
            self,
            Self::SEAL
                | Self::DARKNESS
                | Self::WEAKNESS
                | Self::STUN
                | Self::CURSE
                | Self::POISON
                | Self::SLOW
                | Self::ATTRACT
                | Self::BANMAP
                | Self::DISPEL
                | Self::REVERSE_INPUT
        )
    }

    pub fn is_partizan_stat_change(&self) -> bool {
        matches!(
            self,
            Self::POWERUP_M | Self::MAGICUP_M | Self::PGUARDUP_M | Self::MGUARDUP_M | Self::HASTE_M
        )
    }

    pub fn is_partizan_one_time_stat_change(&self) -> bool {
        matches!(self, Self::HEAL_M)
    }

    pub fn is_summon(&self) -> bool {
        matches!(self, Self::SUMMON | Self::SUMMON_CUBE)
    }

    pub fn is_affect_area(&self) -> bool {
        matches!(self, Self::AREA_FIRE | Self::AREA_POISON)
    }
}
