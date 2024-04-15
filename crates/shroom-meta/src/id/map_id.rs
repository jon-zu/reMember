use std::ops::RangeInclusive;

use crate::shroom_id;

shroom_id!(FieldId, u32);

impl FieldId {
    pub fn is_shroom_island(&self) -> bool {
        Self::MAPLE_ISLAND_RANGE.contains(self)
    }

    // Aran tutorial / burning intro / godly stat
    pub fn is_aran_tutorial_map(&self) -> bool {
        matches!(
            *self,
            Self::BURNING_FOREST_1 | Self::BURNING_FOREST_2 | Self::BURNING_FOREST_3
        )
    }

    pub fn is_cygnus_intro(&self) -> bool {
        Self::CYGNUS_INTRO_LOCATION_RANGE.contains(self)
    }

    pub fn is_physical_fitness(&self) -> bool {
        Self::PHYSICAL_FITNESS_RANGE.contains(self)
    }

    pub fn is_solo_dojo(&self) -> bool {
        Self::DOJO_RANGE.contains(self)
    }

    pub fn is_party_dojo(&self) -> bool {
        Self::DOJO_PARTY_RANGE.contains(self)
    }

    //TODO what's that?
    pub fn is_self_lootable_only(&self) -> bool {
        Self::HAPPYVILLE_TREE_RANGE.contains(self) || Self::GPQ_FOUNTAIN_RANGE.contains(self)
    }

    pub fn is_ola_ola(&self) -> bool {
        Self::OLA_OLA_RANGE.contains(self)
    }

    pub fn is_boss_rush(&self) -> bool {
        Self::BOSS_RUSH_RANGE.contains(self)
    }

    pub fn is_netts_pyramid(&self) -> bool {
        Self::NETTS_PYRAMID_RANGE.contains(self)
    }

    pub fn is_fishing_area(&self) -> bool {
        matches!(
            *self,
            Self::ON_THE_WAY_TO_THE_HARBOR | Self::PIER_ON_THE_BEACH | Self::PEACEFUL_SHIP
        )
    }

    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }
}

impl FieldId {
    // Special
    pub const INVALID: Self = Self(u32::MAX);
    pub const NONE: Self = Self(999999999);
    pub const GM_MAP: Self = Self(180000000);
    pub const JAIL: Self = Self(300000012); // "Cellar: Camp Conference Room"
    pub const DEVELOPERS_HQ: Self = Self(777777777);

    // Misc
    pub const ORBIS_TOWER_BOTTOM: Self = Self(200082300);
    pub const INTERNET_CAFE: Self = Self(193000000);
    pub const CRIMSONWOOD_VALLEY_1: Self = Self(610020000);
    pub const CRIMSONWOOD_VALLEY_2: Self = Self(610020001);
    pub const HENESYS_PQ: Self = Self(910010000);
    pub const ORIGIN_OF_CLOCKTOWER: Self = Self(220080001);
    pub const CAVE_OF_PIANUS: Self = Self(230040420);
    pub const GUILD_HQ: Self = Self(200000301);
    pub const FM_ENTRANCE: Self = Self(910000000);

    // Beginner
    pub const MUSHROOM_TOWN: Self = Self(10000);

    // Town
    pub const SOUTHPERRY: Self = Self(2000000);
    pub const AMHERST: Self = Self(1000000);
    pub const HENESYS: Self = Self(100000000);
    pub const ELLINIA: Self = Self(101000000);
    pub const PERION: Self = Self(102000000);
    pub const KERNING_CITY: Self = Self(103000000);
    pub const LITH_HARBOUR: Self = Self(104000000);
    pub const SLEEPYWOOD: Self = Self(105040300);
    pub const MUSHROOM_KINGDOM: Self = Self(106020000);
    pub const FLORINA_BEACH: Self = Self(110000000);
    pub const EREVE: Self = Self(130000000);
    pub const KERNING_SQUARE: Self = Self(103040000);
    pub const RIEN: Self = Self(140000000);
    pub const ORBIS: Self = Self(200000000);
    pub const EL_NATH: Self = Self(211000000);
    pub const LUDIBRIUM: Self = Self(220000000);
    pub const AQUARIUM: Self = Self(230000000);
    pub const LEAFRE: Self = Self(240000000);
    pub const NEO_CITY: Self = Self(240070000);
    pub const MU_LUNG: Self = Self(250000000);
    pub const HERB_TOWN: Self = Self(251000000);
    pub const OMEGA_SECTOR: Self = Self(221000000);
    pub const KOREAN_FOLK_TOWN: Self = Self(222000000);
    pub const ARIANT: Self = Self(260000000);
    pub const MAGATIA: Self = Self(261000000);
    pub const TEMPLE_OF_TIME: Self = Self(270000100);
    pub const ELLIN_FOREST: Self = Self(300000000);
    pub const SINGAPORE: Self = Self(540000000);
    pub const BOAT_QUAY_TOWN: Self = Self(541000000);
    pub const KAMPUNG_VILLAGE: Self = Self(551000000);
    pub const NEW_LEAF_CITY: Self = Self(600000000);
    pub const MUSHROOM_SHRINE: Self = Self(800000000);
    pub const SHOWA_TOWN: Self = Self(801000000);
    pub const NAUTILUS_HARBOR: Self = Self(120000000);
    pub const HAPPYVILLE: Self = Self(209000000);

    pub const SHOWA_SPA_M: Self = Self(809000101);
    pub const SHOWA_SPA_F: Self = Self(809000201);

    pub(crate) const MAPLE_ISLAND_MIN: Self = Self(0);
    pub(crate) const MAPLE_ISLAND_MAX: Self = Self(2000001);
    pub(crate) const MAPLE_ISLAND_RANGE: RangeInclusive<Self> =
        (Self::MAPLE_ISLAND_MIN..=Self::MAPLE_ISLAND_MAX);

    // Travel
    // There are 10 of each of these travel maps in the files
    pub const FROM_LITH_TO_RIEN: Self = Self(200090060);
    pub const FROM_RIEN_TO_LITH: Self = Self(200090070);
    pub const DANGEROUS_FOREST: Self = Self(140020300); // Rien docks
    pub const FROM_ELLINIA_TO_EREVE: Self = Self(200090030);
    pub const SKY_FERRY: Self = Self(130000210); // Ereve platform
    pub const FROM_EREVE_TO_ELLINIA: Self = Self(200090031);
    pub const ELLINIA_SKY_FERRY: Self = Self(101000400);
    pub const FROM_EREVE_TO_ORBIS: Self = Self(200090021);
    pub const ORBIS_STATION: Self = Self(200000161);
    pub const FROM_ORBIS_TO_EREVE: Self = Self(200090020);

    // Aran
    pub const ARAN_TUTORIAL_START: Self = Self(914000000);
    pub const ARAN_TUTORIAL_MAX: Self = Self(914000500);
    pub const ARAN_INTRO: Self = Self(140090000);
    pub(crate) const BURNING_FOREST_1: Self = Self(914000200);
    pub(crate) const BURNING_FOREST_2: Self = Self(914000210);
    pub(crate) const BURNING_FOREST_3: Self = Self(914000220);

    // Aran intro
    pub const ARAN_TUTO_1: Self = Self(914090010);
    pub const ARAN_TUTO_2: Self = Self(914090011);
    pub const ARAN_TUTO_3: Self = Self(914090012);
    pub const ARAN_TUTO_4: Self = Self(914090013);
    pub const ARAN_POLEARM: Self = Self(914090100);
    pub const ARAN_MAHA: Self = Self(914090200); // Black screen when warped to

    // Starting map Evan
    pub const STARTING_MAP_EVAN: Self = Self(100030100);

    // Starting map
    pub const STARTING_MAP_NOBLESSE: Self = Self(130030000);

    // Edelstein Starting map
    pub const STARTING_MAP_RESISTANCE: Self = Self(310010000);

    // Cygnus intro
    // These are the actual maps
    pub(crate) const CYGNUS_INTRO_LOCATION_MIN: Self = Self(913040000);
    pub(crate) const CYGNUS_INTRO_LOCATION_MAX: Self = Self(913040006);
    pub(crate) const CYGNUS_INTRO_LOCATION_RANGE: RangeInclusive<Self> =
        (Self::CYGNUS_INTRO_LOCATION_MIN..=Self::CYGNUS_INTRO_LOCATION_MAX);

    // Cygnus intro video
    pub const CYGNUS_INTRO_LEAD: Self = Self(913040100);
    pub const CYGNUS_INTRO_WARRIOR: Self = Self(913040101);
    pub const CYGNUS_INTRO_BOWMAN: Self = Self(913040102);
    pub const CYGNUS_INTRO_MAGE: Self = Self(913040103);
    pub const CYGNUS_INTRO_PIRATE: Self = Self(913040104);
    pub const CYGNUS_INTRO_THIEF: Self = Self(913040105);
    pub const CYGNUS_INTRO_CONCLUSION: Self = Self(913040106);

    // Event
    pub const EVENT_COCONUT_HARVEST: Self = Self(109080000);
    pub const EVENT_OX_QUIZ: Self = Self(109020001);
    pub const EVENT_PHYSICAL_FITNESS: Self = Self(109040000);
    pub const EVENT_OLA_OLA_0: Self = Self(109030001);
    pub const EVENT_OLA_OLA_1: Self = Self(109030101);
    pub const EVENT_OLA_OLA_2: Self = Self(109030201);
    pub const EVENT_OLA_OLA_3: Self = Self(109030301);
    pub const EVENT_OLA_OLA_4: Self = Self(109030401);
    pub const EVENT_SNOWBALL: Self = Self(109060000);
    pub const EVENT_FIND_THE_JEWEL: Self = Self(109010000);
    pub const FITNESS_EVENT_LAST: Self = Self(109040004);
    pub const OLA_EVENT_LAST_1: Self = Self(109030003);
    pub const OLA_EVENT_LAST_2: Self = Self(109030103);
    pub const WITCH_TOWER_ENTRANCE: Self = Self(980040000);
    pub const EVENT_WINNER: Self = Self(109050000);
    pub const EVENT_EXIT: Self = Self(109050001);
    pub const EVENT_SNOWBALL_ENTRANCE: Self = Self(109060001);

    pub(crate) const PHYSICAL_FITNESS_MIN: Self = Self::EVENT_PHYSICAL_FITNESS;
    pub(crate) const PHYSICAL_FITNESS_MAX: Self = Self::FITNESS_EVENT_LAST;
    pub(crate) const PHYSICAL_FITNESS_RANGE: RangeInclusive<Self> =
        (Self::PHYSICAL_FITNESS_MIN..=Self::PHYSICAL_FITNESS_MAX);

    pub(crate) const OLA_OLA_MIN: Self = Self::EVENT_OLA_OLA_0;
    pub(crate) const OLA_OLA_MAX: Self = Self(109030403); // OLA_OLA_4 level 3
    pub(crate) const OLA_OLA_RANGE: RangeInclusive<Self> = (Self::OLA_OLA_MIN..=Self::OLA_OLA_MAX);

    // Self lootable maps
    pub(crate) const HAPPYVILLE_TREE_MIN: Self = Self(209000001);
    pub(crate) const HAPPYVILLE_TREE_MAX: Self = Self(209000015);
    pub(crate) const HAPPYVILLE_TREE_RANGE: RangeInclusive<Self> =
        (Self::HAPPYVILLE_TREE_MIN..=Self::HAPPYVILLE_TREE_MAX);

    pub(crate) const GPQ_FOUNTAIN_MIN: Self = Self(990000500);
    pub(crate) const GPQ_FOUNTAIN_MAX: Self = Self(990000502);
    pub(crate) const GPQ_FOUNTAIN_RANGE: RangeInclusive<Self> =
        (Self::GPQ_FOUNTAIN_MIN..=Self::GPQ_FOUNTAIN_MAX);

    // Dojo
    pub const DOJO_SOLO_BASE: Self = Self(925020000);
    pub const DOJO_PARTY_BASE: Self = Self(925030000);
    pub const DOJO_EXIT: Self = Self(925020002);

    pub(crate) const DOJO_MIN: Self = Self::DOJO_SOLO_BASE;
    pub(crate) const DOJO_MAX: Self = Self(925033804);
    pub(crate) const DOJO_RANGE: RangeInclusive<Self> = (Self::DOJO_MIN..=Self::DOJO_MAX);

    pub(crate) const DOJO_PARTY_MIN: Self = Self(925030100);
    pub const DOJO_PARTY_MAX: Self = Self::DOJO_MAX;
    pub(crate) const DOJO_PARTY_RANGE: RangeInclusive<Self> =
        (Self::DOJO_PARTY_MIN..=Self::DOJO_PARTY_MAX);

    // Mini dungeon
    pub const ANT_TUNNEL_2: Self = Self(105050100);
    pub const CAVE_OF_MUSHROOMS_BASE: Self = Self(105050101);
    pub const SLEEPY_DUNGEON_4: Self = Self(105040304);
    pub const GOLEMS_CASTLE_RUINS_BASE: Self = Self(105040320);
    pub const SAHEL_2: Self = Self(260020600);
    pub const HILL_OF_SANDSTORMS_BASE: Self = Self(260020630);
    pub const RAIN_FOREST_EAST_OF_HENESYS: Self = Self(100020000);
    pub const HENESYS_PIG_FARM_BASE: Self = Self(100020100);
    pub const COLD_CRADLE: Self = Self(105090311);
    pub const DRAKES_BLUE_CAVE_BASE: Self = Self(105090320);
    pub const EOS_TOWER_76TH_TO_90TH_FLOOR: Self = Self(221023400);
    pub const DRUMMER_BUNNYS_LAIR_BASE: Self = Self(221023401);
    pub const BATTLEFIELD_OF_FIRE_AND_WATER: Self = Self(240020500);
    pub const ROUND_TABLE_OF_KENTAURUS_BASE: Self = Self(240020512);
    pub const RESTORING_MEMORY_BASE: Self = Self(240040800);
    pub const DESTROYED_DRAGON_NEST: Self = Self(240040520);
    pub const NEWT_SECURED_ZONE_BASE: Self = Self(240040900);
    pub const RED_NOSE_PIRATE_DEN_2: Self = Self(251010402);
    pub const PILLAGE_OF_TREASURE_ISLAND_BASE: Self = Self(251010410);
    pub const LAB_AREA_C1: Self = Self(261020300);
    pub const CRITICAL_ERROR_BASE: Self = Self(261020301);
    pub const FANTASY_THEME_PARK_3: Self = Self(551030000);
    pub const LONGEST_RIDE_ON_BYEBYE_STATION: Self = Self(551030001);

    // Boss rush
    pub(crate) const BOSS_RUSH_MIN: Self = Self(970030100);
    pub(crate) const BOSS_RUSH_MAX: Self = Self(970042711);
    pub(crate) const BOSS_RUSH_RANGE: RangeInclusive<Self> =
        (Self::BOSS_RUSH_MIN..=Self::BOSS_RUSH_MAX);

    // ARPQ
    pub const ARPQ_LOBBY: Self = Self(980010000);
    pub const ARPQ_ARENA_1: Self = Self(980010101);
    pub const ARPQ_ARENA_2: Self = Self(980010201);
    pub const ARPQ_ARENA_3: Self = Self(980010301);
    pub const ARPQ_KINGS_ROOM: Self = Self(980010010);

    // Nett's pyramid
    pub const NETTS_PYRAMID: Self = Self(926010001);
    pub const NETTS_PYRAMID_SOLO_BASE: Self = Self(926010100);
    pub const NETTS_PYRAMID_PARTY_BASE: Self = Self(926020100);
    pub(crate) const NETTS_PYRAMID_MIN: Self = Self::NETTS_PYRAMID_SOLO_BASE;
    pub(crate) const NETTS_PYRAMID_MAX: Self = Self(926023500);
    pub(crate) const NETTS_PYRAMID_RANGE: RangeInclusive<Self> =
        (Self::NETTS_PYRAMID_MIN..=Self::NETTS_PYRAMID_MAX);

    // Fishing
    pub(crate) const ON_THE_WAY_TO_THE_HARBOR: Self = Self(120010000);
    pub(crate) const PIER_ON_THE_BEACH: Self = Self(251000100);
    pub(crate) const PEACEFUL_SHIP: Self = Self(541010110);

    // Wedding
    pub const AMORIA: Self = Self(680000000);
    pub const CHAPEL_WEDDING_ALTAR: Self = Self(680000110);
    pub const CATHEDRAL_WEDDING_ALTAR: Self = Self(680000210);
    pub const WEDDING_PHOTO: Self = Self(680000300);
    pub const WEDDING_EXIT: Self = Self(680000500);

    // Statue
    pub const HALL_OF_WARRIORS: Self = Self(102000004); // Explorer
    pub const HALL_OF_MAGICIANS: Self = Self(101000004);
    pub const HALL_OF_BOWMEN: Self = Self(100000204);
    pub const HALL_OF_THIEVES: Self = Self(103000008);
    pub const NAUTILUS_TRAINING_ROOM: Self = Self(120000105);
    pub const KNIGHTS_CHAMBER: Self = Self(130000100); // Cygnus
    pub const KNIGHTS_CHAMBER_2: Self = Self(130000110);
    pub const KNIGHTS_CHAMBER_3: Self = Self(130000120);
    pub const KNIGHTS_CHAMBER_LARGE: Self = Self(130000101);
    pub const PALACE_OF_THE_MASTER: Self = Self(140010110); // Aran

    // gm-goto
    pub const EXCAVATION_SITE: Self = Self(990000000);
    pub const SOMEONE_ELSES_HOUSE: Self = Self(100000005);
    pub const GRIFFEY_FOREST: Self = Self(240020101);
    pub const MANONS_FOREST: Self = Self(240020401);
    pub const HOLLOWED_GROUND: Self = Self(682000001);
    pub const CURSED_SANCTUARY: Self = Self(105090900);
    pub const DOOR_TO_ZAKUM: Self = Self(211042300);
    pub const DRAGON_NEST_LEFT_BEHIND: Self = Self(240040511);
    pub const HENESYS_PARK: Self = Self(100000200);
    pub const ENTRANCE_TO_HORNTAILS_CAVE: Self = Self(240050400);
    pub const FORGOTTEN_TWILIGHT: Self = Self(270050000);
    pub const CRIMSONWOOD_KEEP: Self = Self(610020006);
    pub const MU_LUNG_DOJO_HALL: Self = Self(925020001);
    pub const EXCLUSIVE_TRAINING_CENTER: Self = Self(970030000);
}
