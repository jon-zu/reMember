use std::ops::Range;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use shroom_pkt::mark_shroom_enum;

//TODO model sub job for dual blade
// Which is actually a beginner but sub job is set to 1

// TODO also add dual blade class for job advancements

use super::{FaceId, FieldId, HairId, ItemId, SkillId};

#[derive(
    Debug, Hash, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u16)]
pub enum JobId {
    Beginner = 0,

    Warrior = 100,
    Fighter = 110,
    Crusader = 111,
    Hero = 112,

    Page = 120,
    WhiteKnight = 121,
    Paladin = 122,
    Spearman = 130,
    DragonKnight = 131,
    DarkKnight = 132,

    Magician = 200,
    WizardFirePoison = 210,
    MageFirePoison = 211,
    ArchMageFirePoinson = 212,

    WizardIceLightning = 220,
    MageIceLightning = 221,
    ArchMageIceLightning = 222,

    Cleric = 230,
    Priest = 231,
    Bishop = 232,

    Bowman = 300,
    Hunter = 310,
    Ranger = 311,
    BowMaster = 312,

    Crossbowman = 320,
    Sniper = 321,
    Marksman = 322,

    Thief = 400,
    Assassin = 410,
    Hermit = 411,
    NightLord = 412,

    Bandit = 420,
    ChiefBandit = 421,
    Shadower = 422,

    BladeRecruit = 430,
    BladeAcolyte = 431,
    BladeSpecialist = 432,
    BladeLord = 433,
    BladeMaster = 434,

    Pirate = 500,
    Brawler = 510,
    Marauder = 511,
    Buccaneer = 512,

    Gunslinger = 520,
    Outlaw = 521,
    Corsair = 522,

    //What's that?
    ShroomLeafBrigadier = 800,
    GM = 900,
    SuperGM = 910,

    Noblesse = 1000,

    DawnWarrior1 = 1100,
    DawnWarrior2 = 1110,
    DawnWarrior3 = 1111,
    DawnWarrior4 = 1112,

    BlazeWizard1 = 1200,
    BlazeWizard2 = 1210,
    BlazeWizard3 = 1211,
    BlazeWizard4 = 1212,

    WindArcher1 = 1300,
    WindArcher2 = 1310,
    WindArcher3 = 1311,
    WindArcher4 = 1312,

    NightWalker1 = 1400,
    NightWalker2 = 1410,
    NightWalker3 = 1411,
    NightWalker4 = 1412,

    ThunderBreaker1 = 1500,
    ThunderBreaker2 = 1510,
    ThunderBreaker3 = 1511,
    ThunderBreaker4 = 1512,

    Legend = 2000,
    Aran1 = 2100,
    Aran2 = 2110,
    Aran3 = 2111,
    Aran4 = 2112,

    EvanBeginner = 2001,
    Evan1 = 2200,
    Evan2 = 2210,
    Evan3 = 2211,
    Evan4 = 2212,
    Evan5 = 2213,
    Evan6 = 2214,
    Evan7 = 2215,
    Evan8 = 2216,
    Evan9 = 2217,
    Evan10 = 2218,

    Citizen = 3000,

    BattleMage1 = 3200,
    BattleMage2 = 3210,
    BattleMage3 = 3211,
    BattleMage4 = 3212,

    WildHunter1 = 3300,
    WildHunter2 = 3310,
    WildHunter3 = 3311,
    WildHunter4 = 3312,

    Mechanic1 = 3500,
    Mechanic2 = 3510,
    Mechanic3 = 3511,
    Mechanic4 = 3512,
}
mark_shroom_enum!(JobId);

pub enum NextJobIterator {
    Choices(&'static [JobId], usize),
    Single(JobId),
    None,
}

impl Iterator for NextJobIterator {
    type Item = JobId;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Choices(choices, idx) => {
                if *idx < choices.len() {
                    let next = choices[*idx];
                    *idx += 1;
                    Some(next)
                } else {
                    None
                }
            }
            Self::Single(job) => {
                let next = *job;
                *self = Self::None;
                Some(next)
            }
            Self::None => None,
        }
    }
}

impl JobId {
    pub fn skill_range(&self) -> Range<SkillId> {
        let start = SkillId(*self as u32 * 10_000);
        let end = SkillId(start.0 + 10_000);

        start..end
    }

    pub fn job_group(&self) -> JobGroup {
        let id = *self as u16;
        match id / 1000 {
            0 => JobGroup::Adventurer,
            1 => JobGroup::KnightsOfCygnus,
            2 if *self == Self::EvanBeginner || id / 100 == 22 => JobGroup::Resistance,
            2 => JobGroup::Legend,
            3 => JobGroup::Resistance,
            _ => unreachable!("Invalid job id {id} has not group "),
        }
    }

    pub fn class(&self) -> JobClass {
        let id = *self as u16;
        match id / 100 {
            //Adventurer
            0 => JobClass::Beginner,
            1 => JobClass::Warrior,
            2 => JobClass::Magician,
            3 => JobClass::Bowman,
            4 => JobClass::Thief,
            5 => JobClass::Pirate,
            8 => JobClass::Unknown,
            9 => JobClass::GM,

            // Cygnus
            10 => JobClass::Noblesse,
            11 => JobClass::DawnWarrior,
            12 => JobClass::BlazeWizard,
            13 => JobClass::WindArcher,
            14 => JobClass::NightWalker,
            15 => JobClass::ThunderBreaker,

            // Legends
            20 => JobClass::LegendBeginner,
            21 => JobClass::Aran,
            22 => JobClass::Evan,

            32 => JobClass::BattleMage,
            33 => JobClass::WildHunter,
            35 => JobClass::Mechanic,

            _ => unreachable!("Invalid job id {id} has not class "),
        }
    }

    pub fn max_job_level(&self) -> usize {
        if self.is_noob() || self.is_admin() {
            return 0;
        }

        if self.class() == JobClass::Evan {
            return 10;
        }

        // TODO add check for dual blade

        4
    }

    pub fn level(&self) -> usize {
        if self.is_noob() || self.is_admin() {
            return 0;
        }

        let id = *self as u16;
        let lvl = id % 10;

        match lvl {
            0 if id % 100 == 0 => 1,
            lvl => (lvl + 2) as usize,
        }
    }

    pub fn is_max_level(&self) -> bool {
        self.level() == self.max_job_level()
    }

    pub fn next_jobs(&self) -> NextJobIterator {
        if self.is_max_level() {
            return NextJobIterator::None;
        }

        if let Some(choices) = self.next_job_choices() {
            return NextJobIterator::Choices(choices, 0);
        }

        let offset = if self.level() == 1 { 10 } else { 1 };
        let next = Self::try_from_primitive(*self as u16 + offset).expect("next job");
        NextJobIterator::Single(next)
    }

    pub fn next_job_choices(&self) -> Option<&'static [Self]> {
        Some(match self {
            Self::Beginner => &[
                Self::Warrior,
                Self::Magician,
                Self::Bowman,
                Self::Thief,
                Self::Pirate,
            ],
            Self::Noblesse => &[
                Self::DawnWarrior1,
                Self::BlazeWizard1,
                Self::WindArcher1,
                Self::NightWalker1,
                Self::ThunderBreaker1,
            ],
            Self::Legend => &[Self::Aran1],
            Self::EvanBeginner => &[Self::Evan1],
            Self::Citizen => &[Self::BattleMage1, Self::WildHunter1, Self::Mechanic1],
            Self::Warrior => &[Self::Fighter, Self::Page, Self::Spearman],
            Self::Hunter => &[Self::Ranger, Self::Crossbowman],
            Self::Magician => &[
                Self::WizardFirePoison,
                Self::WizardIceLightning,
                Self::Cleric,
            ],
            Self::Thief => &[Self::Hermit, Self::ChiefBandit],
            Self::Pirate => &[Self::Marauder, Self::Outlaw],
            _ => return None,
        })
    }

    pub fn is_noob(&self) -> bool {
        matches!(
            *self,
            Self::Beginner | Self::Noblesse | Self::Legend | Self::EvanBeginner | Self::Citizen
        )
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Self::GM | Self::SuperGM | Self::ShroomLeafBrigadier) // todo brigadier might not be correct
    }

    pub fn has_extended_sp(&self) -> bool {
        self.job_group() == JobGroup::Resistance || self.class() == JobClass::Evan
    }

    pub fn prev_job(&self) -> Option<Self> {
        if self.is_noob() {
            return None;
        }

        let v = *self as u16;
        // >= 2nd job
        if v % 10 != 0 {
            return Some(Self::try_from_primitive(*self as u16 - 1).expect("prev job"));
        }

        // 1st job
        Some(Self::try_from_primitive((v / 100) * 100).unwrap())
    }

    pub fn prev_jobs(&self) -> Vec<Self> {
        let mut prev = Vec::new();
        let mut v = *self as u16;

        while v % 10 != 0 {
            v -= 1;
            prev.push(Self::try_from_primitive(v).unwrap());
        }

        prev.push(Self::try_from_primitive((v / 100) * 100).unwrap());
        prev.push(self.job_group().get_noob_job_id());
        prev
    }
}

pub type SubJob = u16;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum JobGroup {
    Resistance = 0,
    Adventurer = 1,
    KnightsOfCygnus = 2,
    Legend = 3,
    Evan = 4,
}

mark_shroom_enum!(JobGroup);

impl JobGroup {
    pub fn get_noob_job_id(&self) -> JobId {
        match *self {
            Self::Adventurer => JobId::Beginner,
            Self::Legend => JobId::Legend,
            Self::KnightsOfCygnus => JobId::Noblesse,
            Self::Evan => JobId::EvanBeginner,
            Self::Resistance => JobId::Citizen,
        }
    }

    pub fn get_starter_weapons(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::SWORD,
            ItemId::HAND_AXE,
            ItemId::WOODEN_CLUB,
            ItemId::BASIC_POLEARM,
        ]
        .into_iter()
    }

    pub fn get_starter_tops(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::WHITE_UNDERSHIRT,
            ItemId::UNDERSHIRT,
            ItemId::GREY_TSHIRT,
            ItemId::WHITE_TUBETOP,
            ItemId::YELLOW_TSHIRT,
            // Aran
            ItemId::SIMPLE_WARRIOR_TOP,
        ]
        .into_iter()
    }

    pub fn get_starter_bottoms(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::BLUE_JEAN_SHORTS,
            ItemId::BROWN_COTTON_SHORTS,
            ItemId::RED_MINISKIRT,
            ItemId::INDIGO_MINISKIRT,
            // Aran
            ItemId::SIMPLE_WARRIOR_PANTS,
        ]
        .into_iter()
    }

    pub fn get_starter_shoes(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::RED_RUBBER_BOOTS,
            ItemId::LEATHER_SANDALS,
            ItemId::YELLOW_RUBBER_BOOTS,
            ItemId::BLUE_RUBBER_BOOTS,
            // Aran
            ItemId::AVERAGE_MUSASHI_SHOES,
        ]
        .into_iter()
    }

    pub fn get_starter_face(&self) -> impl Iterator<Item = FaceId> {
        [
            FaceId::MOTIVATED_LOOK_M,
            FaceId::MOTIVATED_LOOK_F,
            FaceId::PERPLEXED_STARE,
            FaceId::PERPLEXED_STARE_HAZEL,
            FaceId::LEISURE_LOOK_M,
            FaceId::LEISURE_LOOK_F,
            FaceId::FEARFUL_STARE_M,
            FaceId::FEARFUL_STARE_F,
            FaceId::LEISURE_LOOK_HAZEL,
            FaceId::MOTIVATED_LOOK_AMETHYST,
            FaceId::MOTIVATED_LOOK_BLUE,
        ]
        .into_iter()
    }

    pub fn get_starter_hair(&self) -> impl Iterator<Item = HairId> {
        [
            HairId::BLACK_TOBEN,
            HairId::ZETA,
            HairId::BLACK_REBEL,
            HairId::BLACK_BUZZ,
            HairId::BLACK_SAMMY,
            HairId::BLACK_EDGY,
            HairId::BLACK_CONNIE,
        ]
        .into_iter()
    }

    pub fn get_guide_item(&self) -> ItemId {
        match *self {
            Self::KnightsOfCygnus => ItemId::NOBLESSE_GUIDE,
            Self::Legend => ItemId::LEGENDS_GUIDE,
            Self::Evan => ItemId::LEGENDS_GUIDE,
            Self::Adventurer => ItemId::BEGINNERS_GUIDE,
            //TODO
            Self::Resistance => ItemId::BEGINNERS_GUIDE,
        }
    }

    pub fn get_start_field(&self) -> FieldId {
        match *self {
            Self::Adventurer => FieldId::MUSHROOM_TOWN,
            Self::Legend => FieldId::ARAN_TUTORIAL_START,
            Self::Evan => FieldId::STARTING_MAP_EVAN,
            Self::KnightsOfCygnus => FieldId::STARTING_MAP_NOBLESSE,
            Self::Resistance => FieldId::STARTING_MAP_RESISTANCE,
        }
    }
}

#[derive(
    Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum JobClass {
    // Noobs
    Beginner,
    Noblesse,
    LegendBeginner,

    // Adventurer
    Warrior,
    Magician,
    Bowman,
    Thief,
    Pirate,

    //Cygnus
    DawnWarrior,
    BlazeWizard,
    WindArcher,
    NightWalker,
    ThunderBreaker,

    // Legends
    Aran,
    Evan,

    // Resistance
    BattleMage,
    WildHunter,
    Mechanic,

    //GM
    GM,
    //TODO: MAPLE LEAF BRIGADIER
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_job() {
        assert_eq!(
            JobId::Warrior.next_jobs().collect::<Vec<_>>(),
            vec![JobId::Fighter, JobId::Page, JobId::Spearman]
        );
        assert_eq!(
            JobId::Spearman.next_jobs().collect::<Vec<_>>(),
            vec![JobId::DragonKnight]
        );
        assert_eq!(
            JobId::BlazeWizard1.next_jobs().collect::<Vec<_>>(),
            vec![JobId::BlazeWizard2]
        );
        assert_eq!(
            JobId::BlazeWizard2.next_jobs().collect::<Vec<_>>(),
            vec![JobId::BlazeWizard3]
        );
        assert_eq!(JobId::BlazeWizard4.next_jobs().collect::<Vec<_>>(), vec![]);
    }
}
