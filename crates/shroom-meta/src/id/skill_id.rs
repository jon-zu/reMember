use crate::shroom_id;

shroom_id!(SkillId, u32);

impl SkillId {
    pub fn page_ix(&self) -> usize {
        //TODO
        0
    }

    pub fn is_dispel(&self) -> bool {
        self.0 == 2311001
    }

    pub fn is_spirit_javelin(&self) -> bool {
        self.0 == 4121006
    }

    pub fn is_monster_magnet(&self) -> bool {
        self.0 % 10000000 == 1004
    }

    pub fn is_charge_skill(&self) -> bool {
        //TODO
        [
            33101005, 33121009, 35001001, 35101009, 22121000, 22151001, 14111006, 15101003,
            3221001, 5201002, 5221004, 2321001, 3121004, 2121001, 4341003,
        ]
        .contains(&self.0)
    }

    pub fn has_targets(&self) -> bool {
        self.0 == 0
    }

    pub fn is_grenade_skill(&self) -> bool {
        [14111006].contains(&self.0)
    }

    pub fn is_keydown(&self) -> bool {
        //TODO
        matches!(
            *self,
            WA3_HURRICANE
                | BOWMASTER_HURRICANE
                | MARKSMAN_PIERCING_ARROW
                | BISHOP_BIG_BANG
                | FP3_BIG_BANG
                | IL3_BIG_BANG
                | EVAN4_ICE_BREATH
                | DB5_FINAL_CUT
                | DB5_MONSTER_BOMB
                | BRAWLER_CORKSCREW_BLOW
                | GUNSLINGER_GRENADE
                | CORSAIR_RAPID_FIRE
                | NW3_POISON_BOMB
                | TB2_CORKSCREW_BLOW
                | WH2_JAGUAROSHI
                | WH4_WILD_ARROW_BLAST
                | MECH1_FLAME_LAUNCHER
                | MECH2_ENHANCED_FLAME_LAUNCHER
        )
    }

    pub fn is_not_using_shooting_weapon(&self) -> bool {
        matches!(
            self.0,
            4121003
                | 4221003
                | 5121002
                | 11101004
                | 15111006
                | 15111007
                | 21100004
                | 21110004
                | 21120006
                | 33101007
        )
    }

    pub fn is_not_consuming_bullet(&self) -> bool {
        if self.is_not_using_shooting_weapon() {
            return true;
        }
        matches!(
            self.0,
            3101003
                | 3201003
                | 4111004
                | 13101005
                | 14101006
                | 33101002
                | 35001001
                | 35001004
                | 35101009
                | 35101010
                | 35111004
                | 35111015
                | 35121005
                | 35121012
                | 35121013
        )
    }

    pub fn is_anti_repeat_buff_skill(&self) -> bool {
        matches!(
            self.0,
            1001003
                | 1101006
                | 1111007
                | 1121000
                | 1201006
                | 1211009
                | 1211010
                | 1221000
                | 1301006
                | 1301007
                | 1311007
                | 1321000
                | 2101001
                | 2101003
                | 2121000
                | 2201001
                | 2201003
                | 2221000
                | 2301004
                | 2311001
                | 2311003
                | 2321000
                | 2321005
                | 3121000
                | 3121002
                | 3221000
                | 4101004
                | 4111001
                | 4121000
                | 4201003
                | 4221000
                | 4311001
                | 4341000
                | 4341007
                | 5111007
                | 5121000
                | 5121009
                | 5211007
                | 5221000
                | 11001001
                | 11101003
                | 12101000
                | 12101001
                | 14101003
                | 15111005
                | 21121000
                | 22141003
                | 22171000
                | 22181000
                | 32111004
                | 32121007
                | 33121007
                | 35111013
        )
    }

    pub fn has_master_level(&self) -> bool {
        matches!(
            self.0,
            1120004
                | 1120003
                | 1121000
                | 1121001
                | 1121002
                | 1121008
                | 1121010
                | 1121011
                | 1121006
                | 1220005
                | 1220006
                | 1221000
                | 1221002
                | 1221012
                | 1220010
                | 1221011
                | 1221007
                | 1221009
                | 1221004
                | 1320008
                | 1321010
                | 1320009
                | 1321002
                | 1321007
                | 1320005
                | 1320006
                | 1321001
                | 1321000
                | 1321003
                | 21120002
                | 21120001
                | 21120005
                | 21121003
                | 21120004
                | 21120006
                | 21120007
                | 21121008
                | 21120009
                | 21120010
                | 21121000
                | 2121001
                | 2121002
                | 2121004
                | 2121005
                | 2121008
                | 2121000
                | 2121003
                | 2121006
                | 2121007
                | 22111001
                | 22140000
                | 22141002
                | 22171000
                | 22170001
                | 22171002
                | 22171003
                | 22171004
                | 22181000
                | 22181001
                | 22181002
                | 22181003
                | 2221001
                | 2221002
                | 2221004
                | 2221005
                | 2221008
                | 2221000
                | 2221003
                | 2221006
                | 2221007
                | 2321000
                | 2321001
                | 2321002
                | 2321004
                | 2321007
                | 2321003
                | 2321006
                | 2321008
                | 2321009
                | 2321005
                | 3121000
                | 3121002
                | 3121004
                | 3121007
                | 3121006
                | 3121009
                | 3120005
                | 3121008
                | 3121003
                | 32121002
                | 32121003
                | 32121004
                | 32120000
                | 32120001
                | 32121005
                | 32121006
                | 32121007
                | 32121008
                | 3220004
                | 3221000
                | 3221002
                | 3221001
                | 3221006
                | 3221005
                | 3221007
                | 3221008
                | 3221003
                | 33121009
                | 33121002
                | 33121001
                | 33120000
                | 33121004
                | 33121005
                | 33121006
                | 33121007
                | 33121008
                | 35120000
                | 35121005
                | 35121012
                | 35121006
                | 35121003
                | 35121009
                | 35121010
                | 35121011
                | 35120001
                | 35121007
                | 35121008
                | 35121013
                | 4121000
                | 4121003
                | 4121007
                | 4121006
                | 4121009
                | 4120002
                | 4120005
                | 4121004
                | 4121008
                | 4221003
                | 4221008
                | 4220002
                | 4221004
                | 4221007
                | 4221006
                | 4220005
                | 4221000
                | 4221001
                | 4311003
                | 4321000
                | 4331002
                | 4331005
                | 4341000
                | 4340001
                | 4341002
                | 4341003
                | 4341004
                | 4341005
                | 4341006
                | 4341007
                | 4341008
                | 5121002
                | 5121003
                | 5121009
                | 5121010
                | 5121008
                | 5121004
                | 5121005
                | 5121000
                | 5121001
                | 5121007
                | 5220001
                | 5220011
                | 5220002
                | 5221000
                | 5221003
                | 5221004
                | 5221009
                | 5221006
                | 5221007
                | 5221008
                | 5221010
        )
    }
}

pub const BEGINNER_FOLLOW_THE_LEAD: SkillId = SkillId(8);
pub const BEGINNER_BLESSING_OF_THE_FAIRY: SkillId = SkillId(12);
pub const BEGINNER_THREE_SNAILS: SkillId = SkillId(1000);
pub const BEGINNER_RECOVERY: SkillId = SkillId(1001);
pub const BEGINNER_NIMBLE_FEET: SkillId = SkillId(1002);
pub const BEGINNER_LEGENDARY_SPIRIT: SkillId = SkillId(1003);
pub const BEGINNER_MONSTER_RIDER: SkillId = SkillId(1004);
pub const BEGINNER_ECHO_OF_HERO: SkillId = SkillId(1005);
pub const BEGINNER_TEST: SkillId = SkillId(1006);
pub const BEGINNER_MAKER: SkillId = SkillId(1007);
pub const BEGINNER_BAMBOO_RAIN: SkillId = SkillId(1009);
pub const BEGINNER_INVINCIBILITY: SkillId = SkillId(1010);
pub const BEGINNER_POWER_EXPLOSION: SkillId = SkillId(1011);
pub const BEGINNER_SPACESHIP: SkillId = SkillId(1013);
pub const BEGINNER_SPACE_DASH: SkillId = SkillId(1014);
pub const BEGINNER_SPACE_BEAM: SkillId = SkillId(1015);
pub const BEGINNER_YETI_RIDER: SkillId = SkillId(1017);
pub const BEGINNER_YETI_MOUNT: SkillId = SkillId(1018);
pub const BEGINNER_WITCHS_BROOMSTICK: SkillId = SkillId(1019);
pub const BEGINNER_RAGE_OF_PHARAOH: SkillId = SkillId(1020);
pub const BEGINNER_CHARGE_WOODEN_PONY: SkillId = SkillId(1025);
pub const BEGINNER_SOARING: SkillId = SkillId(1026);
pub const BEGINNER_CROCO: SkillId = SkillId(1027);
pub const BEGINNER_BLACK_SCOOTER: SkillId = SkillId(1028);
pub const BEGINNER_PINK_SCOOTER: SkillId = SkillId(1029);
pub const BEGINNER_NIMBUS_CLOUD: SkillId = SkillId(1030);
pub const BEGINNER_BALROG: SkillId = SkillId(1031);
pub const BEGINNER_: SkillId = SkillId(1033);
pub const BEGINNER_ZD_TIGER: SkillId = SkillId(1034);
pub const BEGINNER_MIST_BALROG: SkillId = SkillId(1035);
pub const BEGINNER_LION: SkillId = SkillId(1036);
pub const BEGINNER_UNICORN: SkillId = SkillId(1037);
pub const BEGINNER_LOW_RIDER: SkillId = SkillId(1038);
pub const BEGINNER_RED_TRUCK: SkillId = SkillId(1039);
pub const BEGINNER_GARGOYLE: SkillId = SkillId(1040);
pub const BEGINNER_SHINJO: SkillId = SkillId(1042);
pub const BEGINNER_ORANGE_MUSHROOM: SkillId = SkillId(1044);
pub const BEGINNER_HELICOPTER: SkillId = SkillId(1045);
pub const BEGINNER_SPACESHIP_1: SkillId = SkillId(1046);
pub const BEGINNER_SPACE_DASH_1: SkillId = SkillId(1047);
pub const BEGINNER_SPACE_BEAM_1: SkillId = SkillId(1048);
pub const BEGINNER_NIGHTMARE: SkillId = SkillId(1049);
pub const BEGINNER_YETI: SkillId = SkillId(1050);
pub const BEGINNER_OSTRICH: SkillId = SkillId(1051);
pub const BEGINNER_PINK_BEAR_HOTAIR_BALLOON: SkillId = SkillId(1052);
pub const BEGINNER_TRANSFORMATION_ROBOT: SkillId = SkillId(1053);
pub const BEGINNER_CHICKEN: SkillId = SkillId(1054);
pub const BEGINNER_MOTORCYCLE: SkillId = SkillId(1063);
pub const BEGINNER_POWER_SUIT: SkillId = SkillId(1064);
pub const BEGINNER_OS4_SHUTTLE: SkillId = SkillId(1065);
pub const BEGINNER_VISITOR_MELEE_ATTACK: SkillId = SkillId(1066);
pub const BEGINNER_VISITOR_RANGE_ATTACK: SkillId = SkillId(1067);
pub const BEGINNER_OWL: SkillId = SkillId(1069);
pub const BEGINNER_MOTHERSHIP: SkillId = SkillId(1070);
pub const BEGINNER_OS3A_MACHINE: SkillId = SkillId(1071);
pub const BEGINNER_DECENT_HASTE: SkillId = SkillId(8000);
pub const BEGINNER_DECENT_MYSTIC_DOOR: SkillId = SkillId(8001);
pub const BEGINNER_DECENT_SHARP_EYES: SkillId = SkillId(8002);
pub const BEGINNER_DECENT_HYPER_BODY: SkillId = SkillId(8003);
pub const BEGINNER_PIGS_WEAKNESS: SkillId = SkillId(9000);
pub const BEGINNER_STUMPS_WEAKNESS: SkillId = SkillId(9001);
pub const BEGINNER_SLIMES_WEAKNESS: SkillId = SkillId(9002);
pub const WARRIOR_HP_BOOST: SkillId = SkillId(1000006);
pub const WARRIOR_IRON_BODY: SkillId = SkillId(1001003);
pub const WARRIOR_POWER_STRIKE: SkillId = SkillId(1001004);
pub const WARRIOR_SLASH_BLAST: SkillId = SkillId(1001005);
pub const FIGHTER_WEAPON_MASTERY: SkillId = SkillId(1100000);
pub const FIGHTER_FINAL_ATTACK: SkillId = SkillId(1100002);
pub const FIGHTER_ENHANCED_BASICS: SkillId = SkillId(1100009);
pub const FIGHTER_WEAPON_BOOSTER: SkillId = SkillId(1101004);
pub const FIGHTER_RAGE: SkillId = SkillId(1101006);
pub const FIGHTER_POWER_GUARD: SkillId = SkillId(1101007);
pub const FIGHTER_GROUND_SMASH: SkillId = SkillId(1101008);
pub const CRUSADER_IMPROVING_MP_RECOVERY: SkillId = SkillId(1110000);
pub const CRUSADER_CHANCE_ATTACK: SkillId = SkillId(1110009);
pub const CRUSADER_COMBO_ATTACK: SkillId = SkillId(1111002);
pub const CRUSADER_PANIC: SkillId = SkillId(1111003);
pub const CRUSADER_COMA: SkillId = SkillId(1111005);
pub const CRUSADER_MAGIC_CRASH: SkillId = SkillId(1111007);
pub const CRUSADER_SHOUT: SkillId = SkillId(1111008);
pub const CRUSADER_BRANDISH: SkillId = SkillId(1111010);
pub const HERO_ADVANCED_COMBO_ATTACK: SkillId = SkillId(1120003);
pub const HERO_ACHILLES: SkillId = SkillId(1120004);
pub const HERO_COMBAT_MASTERY: SkillId = SkillId(1120012);
pub const HERO_MAPLE_WARRIOR: SkillId = SkillId(1121000);
pub const HERO_MONSTER_MAGNET: SkillId = SkillId(1121001);
pub const HERO_POWER_STANCE: SkillId = SkillId(1121002);
pub const HERO_RUSH: SkillId = SkillId(1121006);
pub const HERO_INTREPID_SLASH: SkillId = SkillId(1121008);
pub const HERO_ENRAGE: SkillId = SkillId(1121010);
pub const HERO_HEROS_WILL: SkillId = SkillId(1121011);
pub const PAGE_WEAPON_MASTERY: SkillId = SkillId(1200000);
pub const PAGE_FINAL_ATTACK: SkillId = SkillId(1200002);
pub const PAGE_ENHANCED_BASICS: SkillId = SkillId(1200009);
pub const PAGE_WEAPON_BOOSTER: SkillId = SkillId(1201004);
pub const PAGE_THREATEN: SkillId = SkillId(1201006);
pub const PAGE_POWER_GUARD: SkillId = SkillId(1201007);
pub const PAGE_GROUND_SMASH: SkillId = SkillId(1201008);
pub const WK_SHIELD_MASTERY: SkillId = SkillId(1210001);
pub const WK_CHARGED_BLOW: SkillId = SkillId(1211002);
pub const WK_FIRE_CHARGE: SkillId = SkillId(1211004);
pub const WK_ICE_CHARGE: SkillId = SkillId(1211006);
pub const WK_LIGHTNING_CHARGE: SkillId = SkillId(1211008);
pub const WK_MAGIC_CRASH: SkillId = SkillId(1211009);
pub const WK_HP_RECOVERY: SkillId = SkillId(1211010);
pub const WK_COMBAT_ORDERS: SkillId = SkillId(1211011);
pub const PALADIN_ACHILLES: SkillId = SkillId(1220005);
pub const PALADIN_GUARDIAN: SkillId = SkillId(1220006);
pub const PALADIN_ADVANCED_CHARGE: SkillId = SkillId(1220010);
pub const PALADIN_DIVINE_SHIELD: SkillId = SkillId(1220013);
pub const PALADIN_MAPLE_WARRIOR: SkillId = SkillId(1221000);
pub const PALADIN_POWER_STANCE: SkillId = SkillId(1221002);
pub const PALADIN_DIVINE_CHARGE: SkillId = SkillId(1221004);
pub const PALADIN_RUSH: SkillId = SkillId(1221007);
pub const PALADIN_BLAST: SkillId = SkillId(1221009);
pub const PALADIN_HEAVENS_HAMMER: SkillId = SkillId(1221011);
pub const PALADIN_HEROS_WILL: SkillId = SkillId(1221012);
pub const SPEARNMAN_WEAPON_MASTERY: SkillId = SkillId(1300000);
pub const SPEARNMAN_FINAL_ATTACK: SkillId = SkillId(1300002);
pub const SPEARNMAN_ENHANCED_BASICS: SkillId = SkillId(1300009);
pub const SPEARNMAN_WEAPON_BOOSTER: SkillId = SkillId(1301004);
pub const SPEARNMAN_IRON_WILL: SkillId = SkillId(1301006);
pub const SPEARNMAN_HYPER_BODY: SkillId = SkillId(1301007);
pub const SPEARNMAN_GROUND_SMASH: SkillId = SkillId(1301008);
pub const DK_ELEMENTAL_RESISTANCE: SkillId = SkillId(1310000);
pub const DK_DRAGON_WISDOM: SkillId = SkillId(1310009);
pub const DK_DRAGON_BUSTER: SkillId = SkillId(1311001);
pub const DK_DRAGON_FURY: SkillId = SkillId(1311003);
pub const DK_SACRIFICE: SkillId = SkillId(1311005);
pub const DK_DRAGON_ROAR: SkillId = SkillId(1311006);
pub const DK_MAGIC_CRASH: SkillId = SkillId(1311007);
pub const DK_DRAGON_BLOOD: SkillId = SkillId(1311008);
pub const DRK_ACHILLES: SkillId = SkillId(1320005);
pub const DRK_BERSERK: SkillId = SkillId(1320006);
pub const DRK_AURA_OF_THE_BEHOLDER: SkillId = SkillId(1320008);
pub const DRK_HEX_OF_THE_BEHOLDER: SkillId = SkillId(1320009);
pub const DRK_HEX_OF_THE_BEHOLDER_1: SkillId = SkillId(1320011);
pub const DRK_MAPLE_WARRIOR: SkillId = SkillId(1321000);
pub const DRK_MONSTER_MAGNET: SkillId = SkillId(1321001);
pub const DRK_POWER_STANCE: SkillId = SkillId(1321002);
pub const DRK_RUSH: SkillId = SkillId(1321003);
pub const DRK_BEHOLDER: SkillId = SkillId(1321007);
pub const DRK_HEROS_WILL: SkillId = SkillId(1321010);
pub const MAGE_MP_BOOST: SkillId = SkillId(2000006);
pub const MAGE_MAGIC_GUARD: SkillId = SkillId(2001002);
pub const MAGE_MAGIC_ARMOR: SkillId = SkillId(2001003);
pub const MAGE_ENERGY_BOLT: SkillId = SkillId(2001004);
pub const MAGE_MAGIC_CLAW: SkillId = SkillId(2001005);
pub const FP1_MP_EATER: SkillId = SkillId(2100000);
pub const FP1_SPELL_MASTERY: SkillId = SkillId(2100006);
pub const FP1_MEDITATION: SkillId = SkillId(2101001);
pub const FP1_TELEPORT: SkillId = SkillId(2101002);
pub const FP1_SLOW: SkillId = SkillId(2101003);
pub const FP1_FIRE_ARROW: SkillId = SkillId(2101004);
pub const FP1_POISON_BREATH: SkillId = SkillId(2101005);
pub const FP2_PARTIAL_RESISTANCE: SkillId = SkillId(2110000);
pub const FP2_ELEMENT_AMPLIFICATION: SkillId = SkillId(2110001);
pub const FP2_EXPLOSION: SkillId = SkillId(2111002);
pub const FP2_POISON_MIST: SkillId = SkillId(2111003);
pub const FP2_SEAL: SkillId = SkillId(2111004);
pub const FP2_SPELL_BOOSTER: SkillId = SkillId(2111005);
pub const FP2_ELEMENT_COMPOSITION: SkillId = SkillId(2111006);
pub const FP2_TELEPORT_MASTERY: SkillId = SkillId(2111007);
pub const FP2_ELEMENTAL_DECREASE: SkillId = SkillId(2111008);
pub const FP3_BUFF_MASTERY: SkillId = SkillId(2120009);
pub const FP3_MAPLE_WARRIOR: SkillId = SkillId(2121000);
pub const FP3_BIG_BANG: SkillId = SkillId(2121001);
pub const FP3_MANA_REFLECTION: SkillId = SkillId(2121002);
pub const FP3_FIRE_DEMON: SkillId = SkillId(2121003);
pub const FP3_INFINITY: SkillId = SkillId(2121004);
pub const FP3_IFRIT: SkillId = SkillId(2121005);
pub const FP3_PARALYZE: SkillId = SkillId(2121006);
pub const FP3_METEOR_SHOWER: SkillId = SkillId(2121007);
pub const FP3_HEROS_WILL: SkillId = SkillId(2121008);
pub const IL1_MP_EATER: SkillId = SkillId(2200000);
pub const IL1_SPELL_MASTERY: SkillId = SkillId(2200006);
pub const IL1_MEDITATION: SkillId = SkillId(2201001);
pub const IL1_TELEPORT: SkillId = SkillId(2201002);
pub const IL1_SLOW: SkillId = SkillId(2201003);
pub const IL1_COLD_BEAM: SkillId = SkillId(2201004);
pub const IL1_THUNDER_BOLT: SkillId = SkillId(2201005);
pub const IL2_PARTIAL_RESISTANCE: SkillId = SkillId(2210000);
pub const IL2_ELEMENT_AMPLIFICATION: SkillId = SkillId(2210001);
pub const IL2_ICE_STRIKE: SkillId = SkillId(2211002);
pub const IL2_THUNDER_SPEAR: SkillId = SkillId(2211003);
pub const IL2_SEAL: SkillId = SkillId(2211004);
pub const IL2_SPELL_BOOSTER: SkillId = SkillId(2211005);
pub const IL2_ELEMENT_COMPOSITION: SkillId = SkillId(2211006);
pub const IL2_TELEPORT_MASTERY: SkillId = SkillId(2211007);
pub const IL2_ELEMENTAL_DECREASE: SkillId = SkillId(2211008);
pub const IL3_BUFF_MASTERY: SkillId = SkillId(2220009);
pub const IL3_MAPLE_WARRIOR: SkillId = SkillId(2221000);
pub const IL3_BIG_BANG: SkillId = SkillId(2221001);
pub const IL3_MANA_REFLECTION: SkillId = SkillId(2221002);
pub const IL3_ICE_DEMON: SkillId = SkillId(2221003);
pub const IL3_INFINITY: SkillId = SkillId(2221004);
pub const IL3_ELQUINES: SkillId = SkillId(2221005);
pub const IL3_CHAIN_LIGHTNING: SkillId = SkillId(2221006);
pub const IL3_BLIZZARD: SkillId = SkillId(2221007);
pub const IL3_HEROS_WILL: SkillId = SkillId(2221008);
pub const CLERIC_MP_EATER: SkillId = SkillId(2300000);
pub const CLERIC_SPELL_MASTERY: SkillId = SkillId(2300006);
pub const CLERIC_TELEPORT: SkillId = SkillId(2301001);
pub const CLERIC_HEAL: SkillId = SkillId(2301002);
pub const CLERIC_INVINCIBLE: SkillId = SkillId(2301003);
pub const CLERIC_BLESS: SkillId = SkillId(2301004);
pub const CLERIC_HOLY_ARROW: SkillId = SkillId(2301005);
pub const PRIEST_ELEMENTAL_RESISTANCE: SkillId = SkillId(2310000);
pub const PRIEST_HOLY_FOCUS: SkillId = SkillId(2310008);
pub const PRIEST_DISPEL: SkillId = SkillId(2311001);
pub const PRIEST_MYSTIC_DOOR: SkillId = SkillId(2311002);
pub const PRIEST_HOLY_SYMBOL: SkillId = SkillId(2311003);
pub const PRIEST_SHINING_RAY: SkillId = SkillId(2311004);
pub const PRIEST_DOOM: SkillId = SkillId(2311005);
pub const PRIEST_SUMMON_DRAGON: SkillId = SkillId(2311006);
pub const PRIEST_TELEPORT_MASTERY: SkillId = SkillId(2311007);
pub const BISHOP_BUFF_MASTERY: SkillId = SkillId(2320010);
pub const BISHOP_MAPLE_WARRIOR: SkillId = SkillId(2321000);
pub const BISHOP_BIG_BANG: SkillId = SkillId(2321001);
pub const BISHOP_MANA_REFLECTION: SkillId = SkillId(2321002);
pub const BISHOP_BAHAMUT: SkillId = SkillId(2321003);
pub const BISHOP_INFINITY: SkillId = SkillId(2321004);
pub const BISHOP_HOLY_SHIELD: SkillId = SkillId(2321005);
pub const BISHOP_RESURRECTION: SkillId = SkillId(2321006);
pub const BISHOP_ANGEL_RAY: SkillId = SkillId(2321007);
pub const BISHOP_GENESIS: SkillId = SkillId(2321008);
pub const BISHOP_HEROS_WILL: SkillId = SkillId(2321009);
pub const BOWMAN_CRITICAL_SHOT: SkillId = SkillId(3000001);
pub const BOWMAN_THE_EYE_OF_AMAZON: SkillId = SkillId(3000002);
pub const BOWMAN_FOCUS: SkillId = SkillId(3001003);
pub const BOWMAN_ARROW_BLOW: SkillId = SkillId(3001004);
pub const BOWMAN_DOUBLE_SHOT: SkillId = SkillId(3001005);
pub const HUNTER_BOW_MASTERY: SkillId = SkillId(3100000);
pub const HUNTER_FINAL_ATTACK_BOW: SkillId = SkillId(3100001);
pub const HUNTER_ENHANCED_BASICS: SkillId = SkillId(3100006);
pub const HUNTER_BOW_BOOSTER: SkillId = SkillId(3101002);
pub const HUNTER_POWER_KNOCKBACK: SkillId = SkillId(3101003);
pub const HUNTER_SOUL_ARROW_BOW: SkillId = SkillId(3101004);
pub const HUNTER_ARROW_BOMB_BOW: SkillId = SkillId(3101005);
pub const RANGER_THRUST: SkillId = SkillId(3110000);
pub const RANGER_MORTAL_BLOW: SkillId = SkillId(3110001);
pub const RANGER_EVASION_BOOST: SkillId = SkillId(3110007);
pub const RANGER_PUPPET: SkillId = SkillId(3111002);
pub const RANGER_INFERNO: SkillId = SkillId(3111003);
pub const RANGER_ARROW_RAIN: SkillId = SkillId(3111004);
pub const RANGER_SILVER_HAWK: SkillId = SkillId(3111005);
pub const RANGER_STRAFE: SkillId = SkillId(3111006);
pub const BOWMASTER_BOW_EXPERT: SkillId = SkillId(3120005);
pub const BOWMASTER_VENGEANCE: SkillId = SkillId(3120010);
pub const BOWMASTER_MARKSMANSHIP: SkillId = SkillId(3120011);
pub const BOWMASTER_MAPLE_WARRIOR: SkillId = SkillId(3121000);
pub const BOWMASTER_SHARP_EYES: SkillId = SkillId(3121002);
pub const BOWMASTER_DRAGONS_BREATH: SkillId = SkillId(3121003);
pub const BOWMASTER_HURRICANE: SkillId = SkillId(3121004);
pub const BOWMASTER_PHOENIX: SkillId = SkillId(3121006);
pub const BOWMASTER_HAMSTRING: SkillId = SkillId(3121007);
pub const BOWMASTER_CONCENTRATE: SkillId = SkillId(3121008);
pub const BOWMASTER_HEROS_WILL: SkillId = SkillId(3121009);
pub const CROSSBOWMAN_CROSSBOW_MASTERY: SkillId = SkillId(3200000);
pub const CROSSBOWMAN_FINAL_ATTACK_CROSSBOW: SkillId = SkillId(3200001);
pub const CROSSBOWMAN_ENHANCED_BASICS: SkillId = SkillId(3200006);
pub const CROSSBOWMAN_CROSSBOW_BOOSTER: SkillId = SkillId(3201002);
pub const CROSSBOWMAN_POWER_KNOCKBACK: SkillId = SkillId(3201003);
pub const CROSSBOWMAN_SOUL_ARROW_CROSSBOW: SkillId = SkillId(3201004);
pub const CROSSBOWMAN_IRON_ARROW_CROSSBOW: SkillId = SkillId(3201005);
pub const SNIPER_THRUST: SkillId = SkillId(3210000);
pub const SNIPER_MORTAL_BLOW: SkillId = SkillId(3210001);
pub const SNIPER_EVASION_BOOST: SkillId = SkillId(3210007);
pub const SNIPER_PUPPET: SkillId = SkillId(3211002);
pub const SNIPER_BLIZZARD: SkillId = SkillId(3211003);
pub const SNIPER_ARROW_ERUPTION: SkillId = SkillId(3211004);
pub const SNIPER_GOLDEN_EAGLE: SkillId = SkillId(3211005);
pub const SNIPER_STRAFE: SkillId = SkillId(3211006);
pub const MARKSMAN_MARKSMAN_BOOST: SkillId = SkillId(3220004);
pub const MARKSMAN_MARKSMANSHIP: SkillId = SkillId(3220009);
pub const MARKSMAN_ULTIMATE_STRAFE: SkillId = SkillId(3220010);
pub const MARKSMAN_MAPLE_WARRIOR: SkillId = SkillId(3221000);
pub const MARKSMAN_PIERCING_ARROW: SkillId = SkillId(3221001);
pub const MARKSMAN_SHARP_EYES: SkillId = SkillId(3221002);
pub const MARKSMAN_DRAGONS_BREATH: SkillId = SkillId(3221003);
pub const MARKSMAN_FROSTPREY: SkillId = SkillId(3221005);
pub const MARKSMAN_BLIND: SkillId = SkillId(3221006);
pub const MARKSMAN_SNIPE: SkillId = SkillId(3221007);
pub const MARKSMAN_HEROS_WILL: SkillId = SkillId(3221008);
pub const THIEF_NIMBLE_BODY: SkillId = SkillId(4000000);
pub const THIEF_KEEN_EYES: SkillId = SkillId(4000001);
pub const THIEF_DISORDER: SkillId = SkillId(4001002);
pub const THIEF_DARK_SIGHT: SkillId = SkillId(4001003);
pub const THIEF_DOUBLE_STAB: SkillId = SkillId(4001334);
pub const THIEF_LUCKY_SEVEN: SkillId = SkillId(4001344);
pub const ASSASSIN_CLAW_MASTERY: SkillId = SkillId(4100000);
pub const ASSASSIN_CRITICAL_THROW: SkillId = SkillId(4100001);
pub const ASSASSIN_SHADOW_RESISTANCE: SkillId = SkillId(4100006);
pub const ASSASSIN_CLAW_BOOSTER: SkillId = SkillId(4101003);
pub const ASSASSIN_HASTE: SkillId = SkillId(4101004);
pub const ASSASSIN_DRAIN: SkillId = SkillId(4101005);
pub const HERMIT_ALCHEMIST: SkillId = SkillId(4110000);
pub const HERMIT_MESO_UP: SkillId = SkillId(4111001);
pub const HERMIT_SHADOW_PARTNER: SkillId = SkillId(4111002);
pub const HERMIT_SHADOW_WEB: SkillId = SkillId(4111003);
pub const HERMIT_SHADOW_MESO: SkillId = SkillId(4111004);
pub const HERMIT_AVENGER: SkillId = SkillId(4111005);
pub const HERMIT_FLASH_JUMP: SkillId = SkillId(4111006);
pub const HERMIT_DARK_FLARE: SkillId = SkillId(4111007);
pub const NIGHTLORD_SHADOW_SHIFTER: SkillId = SkillId(4120002);
pub const NIGHTLORD_VENOMOUS_STAR: SkillId = SkillId(4120005);
pub const NIGHTLORD_EXPERT_THROWING_STAR_HANDLING: SkillId = SkillId(4120010);
pub const NIGHTLORD_MAPLE_WARRIOR: SkillId = SkillId(4121000);
pub const NIGHTLORD_TAUNT: SkillId = SkillId(4121003);
pub const NIGHTLORD_NINJA_AMBUSH: SkillId = SkillId(4121004);
pub const NIGHTLORD_SHADOW_STARS: SkillId = SkillId(4121006);
pub const NIGHTLORD_TRIPLE_THROW: SkillId = SkillId(4121007);
pub const NIGHTLORD_NINJA_STORM: SkillId = SkillId(4121008);
pub const NIGHTLORD_HEROS_WILL: SkillId = SkillId(4121009);
pub const BANDIT_DAGGER_MASTERY: SkillId = SkillId(4200000);
pub const BANDIT_SHADOW_RESISTANCE: SkillId = SkillId(4200006);
pub const BANDIT_DAGGER_BOOSTER: SkillId = SkillId(4201002);
pub const BANDIT_HASTE: SkillId = SkillId(4201003);
pub const BANDIT_STEAL: SkillId = SkillId(4201004);
pub const BANDIT_SAVAGE_BLOW: SkillId = SkillId(4201005);
pub const CHIEFBANDIT_SHIELD_MASTERY: SkillId = SkillId(4210000);
pub const CHIEFBANDIT_CHAKRA: SkillId = SkillId(4211001);
pub const CHIEFBANDIT_ASSAULTER: SkillId = SkillId(4211002);
pub const CHIEFBANDIT_PICKPOCKET: SkillId = SkillId(4211003);
pub const CHIEFBANDIT_BAND_OF_THIEVES: SkillId = SkillId(4211004);
pub const CHIEFBANDIT_MESO_GUARD: SkillId = SkillId(4211005);
pub const CHIEFBANDIT_MESO_EXPLOSION: SkillId = SkillId(4211006);
pub const CHIEFBANDIT_DARK_FLARE: SkillId = SkillId(4211007);
pub const CHIEFBANDIT_SHADOW_PARTNER: SkillId = SkillId(4211008);
pub const CHIEFBANDIT_FLASH_JUMP: SkillId = SkillId(4211009);
pub const SHADOWER_SHADOW_SHIFTER: SkillId = SkillId(4220002);
pub const SHADOWER_VENOMOUS_STAB: SkillId = SkillId(4220005);
pub const SHADOWER_MESO_MASTERY: SkillId = SkillId(4220009);
pub const SHADOWER_MAPLE_WARRIOR: SkillId = SkillId(4221000);
pub const SHADOWER_ASSASSINATE: SkillId = SkillId(4221001);
pub const SHADOWER_TAUNT: SkillId = SkillId(4221003);
pub const SHADOWER_NINJA_AMBUSH: SkillId = SkillId(4221004);
pub const SHADOWER_SMOKESCREEN: SkillId = SkillId(4221006);
pub const SHADOWER_BOOMERANG_STEP: SkillId = SkillId(4221007);
pub const SHADOWER_HEROS_WILL: SkillId = SkillId(4221008);
pub const DB1_KATARA_MASTERY: SkillId = SkillId(4300000);
pub const DB1_TRIPLE_STAB: SkillId = SkillId(4301001);
pub const DB1_KATARA_BOOSTER: SkillId = SkillId(4301002);
pub const DB2_SHADOW_RESISTANCE: SkillId = SkillId(4310004);
pub const DB2_SELF_HASTE: SkillId = SkillId(4311001);
pub const DB2_FATAL_BLOW: SkillId = SkillId(4311002);
pub const DB2_SLASH_STORM: SkillId = SkillId(4311003);
pub const DB3_TORNADO_SPIN: SkillId = SkillId(4321000);
pub const DB3_TORNADO_SPIN_ATTACK: SkillId = SkillId(4321001);
pub const DB3_FLASHBANG: SkillId = SkillId(4321002);
pub const DB3_FLASH_JUMP: SkillId = SkillId(4321003);
pub const DB4_ADVANCED_DARK_SIGHT: SkillId = SkillId(4330001);
pub const DB4_BLOODY_STORM: SkillId = SkillId(4331000);
pub const DB4_MIRROR_IMAGE: SkillId = SkillId(4331002);
pub const DB4_OWL_SPIRIT: SkillId = SkillId(4331003);
pub const DB4_UPPER_STAB: SkillId = SkillId(4331004);
pub const DB4_FLYING_ASSAULTER: SkillId = SkillId(4331005);
pub const DB5_VENOM: SkillId = SkillId(4340001);
pub const DB5_MAPLE_WARRIOR: SkillId = SkillId(4341000);
pub const DB5_FINAL_CUT: SkillId = SkillId(4341002);
pub const DB5_MONSTER_BOMB: SkillId = SkillId(4341003);
pub const DB5_SUDDEN_RAID: SkillId = SkillId(4341004);
pub const DB5_CHAINS_OF_HELL: SkillId = SkillId(4341005);
pub const DB5_MIRRORED_TARGET: SkillId = SkillId(4341006);
pub const DB5_THORNS: SkillId = SkillId(4341007);
pub const DB5_HEROS_WILL: SkillId = SkillId(4341008);
pub const PIRATE_BULLET_TIME: SkillId = SkillId(5000000);
pub const PIRATE_FLASH_FIST: SkillId = SkillId(5001001);
pub const PIRATE_SOMMERSAULT_KICK: SkillId = SkillId(5001002);
pub const PIRATE_DOUBLE_SHOT: SkillId = SkillId(5001003);
pub const PIRATE_DASH: SkillId = SkillId(5001005);
pub const BRAWLER_KNUCKLE_MASTERY: SkillId = SkillId(5100001);
pub const BRAWLER_CRITICAL_PUNCH: SkillId = SkillId(5100008);
pub const BRAWLER_HP_BOOST: SkillId = SkillId(5100009);
pub const BRAWLER_BACKSPIN_BLOW: SkillId = SkillId(5101002);
pub const BRAWLER_DOUBLE_UPPERCUT: SkillId = SkillId(5101003);
pub const BRAWLER_CORKSCREW_BLOW: SkillId = SkillId(5101004);
pub const BRAWLER_MP_RECOVERY: SkillId = SkillId(5101005);
pub const BRAWLER_KNUCKLE_BOOSTER: SkillId = SkillId(5101006);
pub const BRAWLER_OAK_BARREL: SkillId = SkillId(5101007);
pub const MARAUDER_STUN_MASTERY: SkillId = SkillId(5110000);
pub const MARAUDER_ENERGY_CHARGE: SkillId = SkillId(5110001);
pub const MARAUDER_BRAWLING_MASTERY: SkillId = SkillId(5110008);
pub const MARAUDER_ENERGY_BLAST: SkillId = SkillId(5111002);
pub const MARAUDER_ENERGY_DRAIN: SkillId = SkillId(5111004);
pub const MARAUDER_TRANSFORMATION: SkillId = SkillId(5111005);
pub const MARAUDER_SHOCKWAVE: SkillId = SkillId(5111006);
pub const MARAUDER_ROLL_OF_THE_DICE: SkillId = SkillId(5111007);
pub const BUCCANEER_PIRATES_REVENGE: SkillId = SkillId(5120011);
pub const BUCCANEER_MAPLE_WARRIOR: SkillId = SkillId(5121000);
pub const BUCCANEER_DRAGON_STRIKE: SkillId = SkillId(5121001);
pub const BUCCANEER_ENERGY_ORB: SkillId = SkillId(5121002);
pub const BUCCANEER_SUPER_TRANSFORMATION: SkillId = SkillId(5121003);
pub const BUCCANEER_DEMOLITION: SkillId = SkillId(5121004);
pub const BUCCANEER_SNATCH: SkillId = SkillId(5121005);
pub const BUCCANEER_BARRAGE: SkillId = SkillId(5121007);
pub const BUCCANEER_PIRATES_RAGE: SkillId = SkillId(5121008);
pub const BUCCANEER_SPEED_INFUSION: SkillId = SkillId(5121009);
pub const BUCCANEER_TIME_LEAP: SkillId = SkillId(5121010);
pub const GUNSLINGER_GUN_MASTERY: SkillId = SkillId(5200000);
pub const GUNSLINGER_CRITICAL_SHOT: SkillId = SkillId(5200007);
pub const GUNSLINGER_INVISIBLE_SHOT: SkillId = SkillId(5201001);
pub const GUNSLINGER_GRENADE: SkillId = SkillId(5201002);
pub const GUNSLINGER_GUN_BOOSTER: SkillId = SkillId(5201003);
pub const GUNSLINGER_BLANK_SHOT: SkillId = SkillId(5201004);
pub const GUNSLINGER_WINGS: SkillId = SkillId(5201005);
pub const GUNSLINGER_RECOIL_SHOT: SkillId = SkillId(5201006);
pub const OUTLAW_BURST_FIRE: SkillId = SkillId(5210000);
pub const OUTLAW_OCTOPUS: SkillId = SkillId(5211001);
pub const OUTLAW_GAVIOTA: SkillId = SkillId(5211002);
pub const OUTLAW_FLAMETHROWER: SkillId = SkillId(5211004);
pub const OUTLAW_ICE_SPLITTER: SkillId = SkillId(5211005);
pub const OUTLAW_HOMING_BEACON: SkillId = SkillId(5211006);
pub const OUTLAW_ROLL_OF_THE_DICE: SkillId = SkillId(5211007);
pub const CORSAIR_ELEMENTAL_BOOST: SkillId = SkillId(5220001);
pub const CORSAIR_WRATH_OF_THE_OCTOPI: SkillId = SkillId(5220002);
pub const CORSAIR_BULLSEYE: SkillId = SkillId(5220011);
pub const CORSAIR_PIRATES_REVENGE: SkillId = SkillId(5220012);
pub const CORSAIR_MAPLE_WARRIOR: SkillId = SkillId(5221000);
pub const CORSAIR_AIR_STRIKE: SkillId = SkillId(5221003);
pub const CORSAIR_RAPID_FIRE: SkillId = SkillId(5221004);
pub const CORSAIR_BATTLESHIP: SkillId = SkillId(5221006);
pub const CORSAIR_BATTLESHIP_CANNON: SkillId = SkillId(5221007);
pub const CORSAIR_BATTLESHIP_TORPEDO: SkillId = SkillId(5221008);
pub const CORSAIR_HYPNOTIZE: SkillId = SkillId(5221009);
pub const CORSAIR_HEROS_WILL: SkillId = SkillId(5221010);
pub const SHROOMLEAFBRIGADIER_: SkillId = SkillId(8001000);
pub const SHROOMLEAFBRIGADIER__: SkillId = SkillId(8001001);
pub const GM_HASTE_NORMAL: SkillId = SkillId(9001000);
pub const GM_SUPER_DRAGON_ROAR: SkillId = SkillId(9001001);
pub const GM_TELEPORT: SkillId = SkillId(9001002);
pub const GM_BLESS: SkillId = SkillId(9001003);
pub const GM_HIDE: SkillId = SkillId(9001004);
pub const GM_RESURRECTION: SkillId = SkillId(9001005);
pub const GM_SUPER_DRAGON_ROAR_1: SkillId = SkillId(9001006);
pub const GM_TELEPORT_1: SkillId = SkillId(9001007);
pub const GM_HYPER_BODY: SkillId = SkillId(9001008);
pub const GM_ADMIN_ANTIMACRO: SkillId = SkillId(9001009);
pub const SUPERGM_HEAL_DISPEL: SkillId = SkillId(9101000);
pub const SUPERGM_HASTE_SUPER: SkillId = SkillId(9101001);
pub const SUPERGM_HOLY_SYMBOL: SkillId = SkillId(9101002);
pub const SUPERGM_BLESS: SkillId = SkillId(9101003);
pub const SUPERGM_HIDE: SkillId = SkillId(9101004);
pub const SUPERGM_RESURRECTION: SkillId = SkillId(9101005);
pub const SUPERGM_SUPER_DRAGON_ROAR: SkillId = SkillId(9101006);
pub const SUPERGM_TELEPORT: SkillId = SkillId(9101007);
pub const SUPERGM_HYPER_BODY: SkillId = SkillId(9101008);
pub const NOBLESSE_BLESSING_OF_THE_FAIRY: SkillId = SkillId(10000012);
pub const NOBLESSE_HELPER: SkillId = SkillId(10000013);
pub const NOBLESSE_FOLLOW_THE_LEAD: SkillId = SkillId(10000018);
pub const NOBLESSE_THREE_SNAILS: SkillId = SkillId(10001000);
pub const NOBLESSE_RECOVERY: SkillId = SkillId(10001001);
pub const NOBLESSE_NIMBLE_FEET: SkillId = SkillId(10001002);
pub const NOBLESSE_LEGENDARY_SPIRIT: SkillId = SkillId(10001003);
pub const NOBLESSE_MONSTER_RIDER: SkillId = SkillId(10001004);
pub const NOBLESSE_ECHO_OF_HERO: SkillId = SkillId(10001005);
pub const NOBLESSE_JUMP_DOWN: SkillId = SkillId(10001006);
pub const NOBLESSE_MAKER: SkillId = SkillId(10001007);
pub const NOBLESSE_BAMBOO_THRUST: SkillId = SkillId(10001009);
pub const NOBLESSE_INVINCIBLE_BARRIER: SkillId = SkillId(10001010);
pub const NOBLESSE_METEO_SHOWER: SkillId = SkillId(10001011);
pub const NOBLESSE_SPACESHIP: SkillId = SkillId(10001014);
pub const NOBLESSE_SPACE_DASH: SkillId = SkillId(10001015);
pub const NOBLESSE_SPACE_BEAM: SkillId = SkillId(10001016);
pub const NOBLESSE_YETI_RIDER: SkillId = SkillId(10001019);
pub const NOBLESSE_RAGE_OF_PHARAOH: SkillId = SkillId(10001020);
pub const NOBLESSE_YETI_MOUNT: SkillId = SkillId(10001022);
pub const NOBLESSE_WITCHS_BROOMSTICK: SkillId = SkillId(10001023);
pub const NOBLESSE_CHARGE_WOODEN_PONY: SkillId = SkillId(10001025);
pub const NOBLESSE_SOARING: SkillId = SkillId(10001026);
pub const NOBLESSE_CROCO: SkillId = SkillId(10001027);
pub const NOBLESSE_BLACK_SCOOTER: SkillId = SkillId(10001028);
pub const NOBLESSE_PINK_SCOOTER: SkillId = SkillId(10001029);
pub const NOBLESSE_NIMBUS_CLOUD: SkillId = SkillId(10001030);
pub const NOBLESSE_BALROG: SkillId = SkillId(10001031);
pub const NOBLESSE_: SkillId = SkillId(10001033);
pub const NOBLESSE_ZD_TIGER: SkillId = SkillId(10001034);
pub const NOBLESSE_MIST_BALROG: SkillId = SkillId(10001035);
pub const NOBLESSE_LION: SkillId = SkillId(10001036);
pub const NOBLESSE_UNICORN: SkillId = SkillId(10001037);
pub const NOBLESSE_LOW_RIDER: SkillId = SkillId(10001038);
pub const NOBLESSE_RED_TRUCK: SkillId = SkillId(10001039);
pub const NOBLESSE_GARGOYLE: SkillId = SkillId(10001040);
pub const NOBLESSE_SHINJO: SkillId = SkillId(10001042);
pub const NOBLESSE_ORANGE_MUSHROOM: SkillId = SkillId(10001044);
pub const NOBLESSE_HELICOPTER: SkillId = SkillId(10001045);
pub const NOBLESSE_SPACESHIP_1: SkillId = SkillId(10001046);
pub const NOBLESSE_SPACE_DASH_1: SkillId = SkillId(10001047);
pub const NOBLESSE_SPACE_BEAM_1: SkillId = SkillId(10001048);
pub const NOBLESSE_NIGHTMARE: SkillId = SkillId(10001049);
pub const NOBLESSE_YETI: SkillId = SkillId(10001050);
pub const NOBLESSE_OSTRICH: SkillId = SkillId(10001051);
pub const NOBLESSE_PINK_BEAR_HOTAIR_BALLOON: SkillId = SkillId(10001052);
pub const NOBLESSE_TRANSFORMATION_ROBOT: SkillId = SkillId(10001053);
pub const NOBLESSE_CHICKEN: SkillId = SkillId(10001054);
pub const NOBLESSE_MOTORCYCLE: SkillId = SkillId(10001063);
pub const NOBLESSE_POWER_SUIT: SkillId = SkillId(10001064);
pub const NOBLESSE_OS4_SHUTTLE: SkillId = SkillId(10001065);
pub const NOBLESSE_VISITOR_MELEE_ATTACK: SkillId = SkillId(10001066);
pub const NOBLESSE_VISITOR_RANGE_ATTACK: SkillId = SkillId(10001067);
pub const NOBLESSE_OWL: SkillId = SkillId(10001069);
pub const NOBLESSE_MOTHERSHIP: SkillId = SkillId(10001070);
pub const NOBLESSE_OS3A_MACHINE: SkillId = SkillId(10001071);
pub const NOBLESSE_DECENT_HASTE: SkillId = SkillId(10008000);
pub const NOBLESSE_DECENT_MYSTIC_DOOR: SkillId = SkillId(10008001);
pub const NOBLESSE_DECENT_SHARP_EYES: SkillId = SkillId(10008002);
pub const NOBLESSE_DECENT_HYPER_BODY: SkillId = SkillId(10008003);
pub const NOBLESSE_PIGS_WEAKNESS: SkillId = SkillId(10009000);
pub const NOBLESSE_STUMPS_WEAKNESS: SkillId = SkillId(10009001);
pub const NOBLESSE_SLIMES_WEAKNESS: SkillId = SkillId(10009002);
pub const DW1_HP_BOOST: SkillId = SkillId(11000005);
pub const DW1_IRON_BODY: SkillId = SkillId(11001001);
pub const DW1_POWER_STRIKE: SkillId = SkillId(11001002);
pub const DW1_SLASH_BLAST: SkillId = SkillId(11001003);
pub const DW1_SOUL: SkillId = SkillId(11001004);
pub const DW2_SWORD_MASTERY: SkillId = SkillId(11100000);
pub const DW2_SWORD_BOOSTER: SkillId = SkillId(11101001);
pub const DW2_FINAL_ATTACK: SkillId = SkillId(11101002);
pub const DW2_RAGE: SkillId = SkillId(11101003);
pub const DW2_SOUL_BLADE: SkillId = SkillId(11101004);
pub const DW2_SOUL_RUSH: SkillId = SkillId(11101005);
pub const DW3_MP_RECOVERY_RATE_ENHANCEMENT: SkillId = SkillId(11110000);
pub const DW3_ADVANCED_COMBO: SkillId = SkillId(11110005);
pub const DW3_COMBO_ATTACK: SkillId = SkillId(11111001);
pub const DW3_PANIC: SkillId = SkillId(11111002);
pub const DW3_COMA: SkillId = SkillId(11111003);
pub const DW3_BRANDISH: SkillId = SkillId(11111004);
pub const DW3_SOUL_DRIVER: SkillId = SkillId(11111006);
pub const DW3_SOUL_CHARGE: SkillId = SkillId(11111007);
pub const BW1_MP_BOOST: SkillId = SkillId(12000005);
pub const BW1_MAGIC_GUARD: SkillId = SkillId(12001001);
pub const BW1_MAGIC_ARMOR: SkillId = SkillId(12001002);
pub const BW1_MAGIC_CLAW: SkillId = SkillId(12001003);
pub const BW1_FLAME: SkillId = SkillId(12001004);
pub const BW2_SPELL_MASTERY: SkillId = SkillId(12100007);
pub const BW2_MEDITATION: SkillId = SkillId(12101000);
pub const BW2_SLOW: SkillId = SkillId(12101001);
pub const BW2_FIRE_ARROW: SkillId = SkillId(12101002);
pub const BW2_TELEPORT: SkillId = SkillId(12101003);
pub const BW2_SPELL_BOOSTER: SkillId = SkillId(12101004);
pub const BW2_ELEMENTAL_RESET: SkillId = SkillId(12101005);
pub const BW2_FIRE_PILLAR: SkillId = SkillId(12101006);
pub const BW3_ELEMENTAL_RESISTANCE: SkillId = SkillId(12110000);
pub const BW3_ELEMENT_AMPLIFICATION: SkillId = SkillId(12110001);
pub const BW3_SEAL: SkillId = SkillId(12111002);
pub const BW3_METEOR_SHOWER: SkillId = SkillId(12111003);
pub const BW3_IFRIT: SkillId = SkillId(12111004);
pub const BW3_FLAME_GEAR: SkillId = SkillId(12111005);
pub const BW3_FIRE_STRIKE: SkillId = SkillId(12111006);
pub const WA1_CRITICAL_SHOT: SkillId = SkillId(13000000);
pub const WA1_THE_EYE_OF_AMAZON: SkillId = SkillId(13000001);
pub const WA1_FOCUS: SkillId = SkillId(13001002);
pub const WA1_DOUBLE_SHOT: SkillId = SkillId(13001003);
pub const WA1_STORM: SkillId = SkillId(13001004);
pub const WA2_BOW_MASTERY: SkillId = SkillId(13100000);
pub const WA2_THRUST: SkillId = SkillId(13100004);
pub const WA2_BOW_BOOSTER: SkillId = SkillId(13101001);
pub const WA2_FINAL_ATTACK: SkillId = SkillId(13101002);
pub const WA2_SOUL_ARROW: SkillId = SkillId(13101003);
pub const WA2_STORM_BREAK: SkillId = SkillId(13101005);
pub const WA2_WIND_WALK: SkillId = SkillId(13101006);
pub const WA3_BOW_EXPERT: SkillId = SkillId(13110003);
pub const WA3_ARROW_RAIN: SkillId = SkillId(13111000);
pub const WA3_STRAFE: SkillId = SkillId(13111001);
pub const WA3_HURRICANE: SkillId = SkillId(13111002);
pub const WA3_PUPPET: SkillId = SkillId(13111004);
pub const WA3_EAGLE_EYE: SkillId = SkillId(13111005);
pub const WA3_WIND_PIERCING: SkillId = SkillId(13111006);
pub const WA3_WIND_SHOT: SkillId = SkillId(13111007);
pub const NW1_NIMBLE_BODY: SkillId = SkillId(14000000);
pub const NW1_KEEN_EYES: SkillId = SkillId(14000001);
pub const NW1_DISORDER: SkillId = SkillId(14001002);
pub const NW1_DARK_SIGHT: SkillId = SkillId(14001003);
pub const NW1_LUCKY_SEVEN: SkillId = SkillId(14001004);
pub const NW1_DARKNESS: SkillId = SkillId(14001005);
pub const NW2_CLAW_MASTERY: SkillId = SkillId(14100000);
pub const NW2_CRITICAL_THROW: SkillId = SkillId(14100001);
pub const NW2_VANISH: SkillId = SkillId(14100005);
pub const NW2_CLAW_BOOSTER: SkillId = SkillId(14101002);
pub const NW2_HASTE: SkillId = SkillId(14101003);
pub const NW2_FLASH_JUMP: SkillId = SkillId(14101004);
pub const NW2_VAMPIRE: SkillId = SkillId(14101006);
pub const NW3_ALCHEMIST: SkillId = SkillId(14110003);
pub const NW3_VENOM: SkillId = SkillId(14110004);
pub const NW3_SHADOW_PARTNER: SkillId = SkillId(14111000);
pub const NW3_SHADOW_WEB: SkillId = SkillId(14111001);
pub const NW3_AVENGER: SkillId = SkillId(14111002);
pub const NW3_TRIPLE_THROW: SkillId = SkillId(14111005);
pub const NW3_POISON_BOMB: SkillId = SkillId(14111006);
pub const TB1_QUICK_MOTION: SkillId = SkillId(15000000);
pub const TB1_STRAIGHT: SkillId = SkillId(15001001);
pub const TB1_SOMERSAULT_KICK: SkillId = SkillId(15001002);
pub const TB1_DASH: SkillId = SkillId(15001003);
pub const TB1_LIGHTNING: SkillId = SkillId(15001004);
pub const TB2_KNUCKLE_MASTERY: SkillId = SkillId(15100001);
pub const TB2_ENERGY_CHARGE: SkillId = SkillId(15100004);
pub const TB2_HP_BOOST: SkillId = SkillId(15100007);
pub const TB2_KNUCKLE_BOOSTER: SkillId = SkillId(15101002);
pub const TB2_CORKSCREW_BLOW: SkillId = SkillId(15101003);
pub const TB2_ENERGY_BLAST: SkillId = SkillId(15101005);
pub const TB2_LIGHTNING_CHARGE: SkillId = SkillId(15101006);
pub const TB3_CRITICAL_PUNCH: SkillId = SkillId(15110000);
pub const TB3_ENERGY_DRAIN: SkillId = SkillId(15111001);
pub const TB3_TRANSFORMATION: SkillId = SkillId(15111002);
pub const TB3_SHOCKWAVE: SkillId = SkillId(15111003);
pub const TB3_BARRAGE: SkillId = SkillId(15111004);
pub const TB3_SPEED_INFUSION: SkillId = SkillId(15111005);
pub const TB3_SPARK: SkillId = SkillId(15111006);
pub const TB3_SHARK_WAVE: SkillId = SkillId(15111007);
pub const LEGEND_BLESSING_OF_THE_FAIRY: SkillId = SkillId(20000012);
pub const LEGEND_TUTORIAL_SKILL: SkillId = SkillId(20000014);
pub const LEGEND_TUTORIAL_SKILL_1: SkillId = SkillId(20000015);
pub const LEGEND_TUTORIAL_SKILL_2: SkillId = SkillId(20000016);
pub const LEGEND_TUTORIAL_SKILL_3: SkillId = SkillId(20000017);
pub const LEGEND_TUTORIAL_SKILL_4: SkillId = SkillId(20000018);
pub const LEGEND_FOLLOW_THE_LEAD: SkillId = SkillId(20000024);
pub const LEGEND_THREE_SNAILS: SkillId = SkillId(20001000);
pub const LEGEND_RECOVERY: SkillId = SkillId(20001001);
pub const LEGEND_AGILE_BODY: SkillId = SkillId(20001002);
pub const LEGEND_LEGENDARY_SPIRIT: SkillId = SkillId(20001003);
pub const LEGEND_MONSTER_RIDER: SkillId = SkillId(20001004);
pub const LEGEND_ECHO_OF_HERO: SkillId = SkillId(20001005);
pub const LEGEND_JUMP_DOWN: SkillId = SkillId(20001006);
pub const LEGEND_MAKER: SkillId = SkillId(20001007);
pub const LEGEND_BAMBOO_THRUST: SkillId = SkillId(20001009);
pub const LEGEND_INVINCIBLE_BARRIER: SkillId = SkillId(20001010);
pub const LEGEND_METEO_SHOWER: SkillId = SkillId(20001011);
pub const LEGEND_HELPER: SkillId = SkillId(20001013);
pub const LEGEND_YETI_RIDER: SkillId = SkillId(20001019);
pub const LEGEND_RAGE_OF_PHARAOH: SkillId = SkillId(20001020);
pub const LEGEND_YETI_MOUNT: SkillId = SkillId(20001022);
pub const LEGEND_WITCHS_BROOMSTICK: SkillId = SkillId(20001023);
pub const LEGEND_CHARGE_WOODEN_PONY: SkillId = SkillId(20001025);
pub const LEGEND_SOARING: SkillId = SkillId(20001026);
pub const LEGEND_CROCO: SkillId = SkillId(20001027);
pub const LEGEND_BLACK_SCOOTER: SkillId = SkillId(20001028);
pub const LEGEND_PINK_SCOOTER: SkillId = SkillId(20001029);
pub const LEGEND_NIMBUS_CLOUD: SkillId = SkillId(20001030);
pub const LEGEND_BALROG: SkillId = SkillId(20001031);
pub const LEGEND_: SkillId = SkillId(20001033);
pub const LEGEND_ZD_TIGER: SkillId = SkillId(20001034);
pub const LEGEND_MIST_BALROG: SkillId = SkillId(20001035);
pub const LEGEND_LION: SkillId = SkillId(20001036);
pub const LEGEND_UNICORN: SkillId = SkillId(20001037);
pub const LEGEND_LOW_RIDER: SkillId = SkillId(20001038);
pub const LEGEND_RED_TRUCK: SkillId = SkillId(20001039);
pub const LEGEND_GARGOYLE: SkillId = SkillId(20001040);
pub const LEGEND_SHINJO: SkillId = SkillId(20001042);
pub const LEGEND_ORANGE_MUSHROOM: SkillId = SkillId(20001044);
pub const LEGEND_HELICOPTER: SkillId = SkillId(20001045);
pub const LEGEND_SPACESHIP: SkillId = SkillId(20001046);
pub const LEGEND_SPACE_DASH: SkillId = SkillId(20001047);
pub const LEGEND_SPACE_BEAM: SkillId = SkillId(20001048);
pub const LEGEND_NIGHTMARE: SkillId = SkillId(20001049);
pub const LEGEND_YETI: SkillId = SkillId(20001050);
pub const LEGEND_OSTRICH: SkillId = SkillId(20001051);
pub const LEGEND_PINK_BEAR_HOTAIR_BALLOON: SkillId = SkillId(20001052);
pub const LEGEND_TRANSFORMATION_ROBOT: SkillId = SkillId(20001053);
pub const LEGEND_CHICKEN: SkillId = SkillId(20001054);
pub const LEGEND_MOTORCYCLE: SkillId = SkillId(20001063);
pub const LEGEND_POWER_SUIT: SkillId = SkillId(20001064);
pub const LEGEND_OS4_SHUTTLE: SkillId = SkillId(20001065);
pub const LEGEND_VISITOR_MELEE_ATTACK: SkillId = SkillId(20001066);
pub const LEGEND_VISITOR_RANGE_ATTACK: SkillId = SkillId(20001067);
pub const LEGEND_OWL: SkillId = SkillId(20001069);
pub const LEGEND_MOTHERSHIP: SkillId = SkillId(20001070);
pub const LEGEND_OS3A_MACHINE: SkillId = SkillId(20001071);
pub const LEGEND_DECENT_HASTE: SkillId = SkillId(20008000);
pub const LEGEND_DECENT_MYSTIC_DOOR: SkillId = SkillId(20008001);
pub const LEGEND_DECENT_SHARP_EYES: SkillId = SkillId(20008002);
pub const LEGEND_DECENT_HYPER_BODY: SkillId = SkillId(20008003);
pub const LEGEND_PIGS_WEAKNESS: SkillId = SkillId(20009000);
pub const LEGEND_STUMPS_WEAKNESS: SkillId = SkillId(20009001);
pub const LEGEND_SLIMES_WEAKNESS: SkillId = SkillId(20009002);
pub const EVANBEGINNER_BLESSING_OF_THE_FAIRY: SkillId = SkillId(20010012);
pub const EVANBEGINNER_THREE_SNAILS: SkillId = SkillId(20011000);
pub const EVANBEGINNER_RECOVER: SkillId = SkillId(20011001);
pub const EVANBEGINNER_NIMBLE_FEET: SkillId = SkillId(20011002);
pub const EVANBEGINNER_LEGENDARY_SPIRIT: SkillId = SkillId(20011003);
pub const EVANBEGINNER_MONSTER_RIDER: SkillId = SkillId(20011004);
pub const EVANBEGINNER_HEROS_ECHO: SkillId = SkillId(20011005);
pub const EVANBEGINNER_JUMP_DOWN: SkillId = SkillId(20011006);
pub const EVANBEGINNER_MAKER: SkillId = SkillId(20011007);
pub const EVANBEGINNER_BAMBOO_THRUST: SkillId = SkillId(20011009);
pub const EVANBEGINNER_INVINCIBLE_BARRIER: SkillId = SkillId(20011010);
pub const EVANBEGINNER_METEO_SHOWER: SkillId = SkillId(20011011);
pub const EVANBEGINNER_YETI_RIDER: SkillId = SkillId(20011018);
pub const EVANBEGINNER_WITCHS_BROOMSTICK: SkillId = SkillId(20011019);
pub const EVANBEGINNER_RAGE_OF_PHARAOH: SkillId = SkillId(20011020);
pub const EVANBEGINNER_FOLLOW_THE_LEAD: SkillId = SkillId(20011024);
pub const EVANBEGINNER_CHARGE_WOODEN_PONY: SkillId = SkillId(20011025);
pub const EVANBEGINNER_SOARING: SkillId = SkillId(20011026);
pub const EVANBEGINNER_CROCO: SkillId = SkillId(20011027);
pub const EVANBEGINNER_BLACK_SCOOTER: SkillId = SkillId(20011028);
pub const EVANBEGINNER_PINK_SCOOTER: SkillId = SkillId(20011029);
pub const EVANBEGINNER_NIMBUS_CLOUD: SkillId = SkillId(20011030);
pub const EVANBEGINNER_BALROG: SkillId = SkillId(20011031);
pub const EVANBEGINNER_RACE_KART: SkillId = SkillId(20011033);
pub const EVANBEGINNER_ZD_TIGER: SkillId = SkillId(20011034);
pub const EVANBEGINNER_MIST_BALROG: SkillId = SkillId(20011035);
pub const EVANBEGINNER_LION: SkillId = SkillId(20011036);
pub const EVANBEGINNER_UNICORN: SkillId = SkillId(20011037);
pub const EVANBEGINNER_LOW_RIDER: SkillId = SkillId(20011038);
pub const EVANBEGINNER_RED_TRUCK: SkillId = SkillId(20011039);
pub const EVANBEGINNER_GARGOYLE: SkillId = SkillId(20011040);
pub const EVANBEGINNER_SHINJO: SkillId = SkillId(20011042);
pub const EVANBEGINNER_ORANGE_MUSHROOM: SkillId = SkillId(20011044);
pub const EVANBEGINNER_HELICOPTER: SkillId = SkillId(20011045);
pub const EVANBEGINNER_SPACESHIP: SkillId = SkillId(20011046);
pub const EVANBEGINNER_SPACE_DASH: SkillId = SkillId(20011047);
pub const EVANBEGINNER_SPACE_BEAM: SkillId = SkillId(20011048);
pub const EVANBEGINNER_NIGHTMARE: SkillId = SkillId(20011049);
pub const EVANBEGINNER_YETI: SkillId = SkillId(20011050);
pub const EVANBEGINNER_OSTRICH: SkillId = SkillId(20011051);
pub const EVANBEGINNER_PINK_BEAR_HOTAIR_BALLOON: SkillId = SkillId(20011052);
pub const EVANBEGINNER_TRANSFORMATION_ROBOT: SkillId = SkillId(20011053);
pub const EVANBEGINNER_CHICKEN: SkillId = SkillId(20011054);
pub const EVANBEGINNER_MOTORCYCLE: SkillId = SkillId(20011063);
pub const EVANBEGINNER_POWER_SUIT: SkillId = SkillId(20011064);
pub const EVANBEGINNER_OS4_SHUTTLE: SkillId = SkillId(20011065);
pub const EVANBEGINNER_VISITOR_MELEE_ATTACK: SkillId = SkillId(20011066);
pub const EVANBEGINNER_VISITOR_RANGE_ATTACK: SkillId = SkillId(20011067);
pub const EVANBEGINNER_OWL: SkillId = SkillId(20011069);
pub const EVANBEGINNER_MOTHERSHIP: SkillId = SkillId(20011070);
pub const EVANBEGINNER_OS3A_MACHINE: SkillId = SkillId(20011071);
pub const EVANBEGINNER_DECENT_HASTE: SkillId = SkillId(20018000);
pub const EVANBEGINNER_DECENT_MYSTIC_DOOR: SkillId = SkillId(20018001);
pub const EVANBEGINNER_DECENT_SHARP_EYES: SkillId = SkillId(20018002);
pub const EVANBEGINNER_DECENT_HYPER_BODY: SkillId = SkillId(20018003);
pub const EVANBEGINNER_PIGS_WEAKNESS: SkillId = SkillId(20019000);
pub const EVANBEGINNER_STUMPS_WEAKNESS: SkillId = SkillId(20019001);
pub const EVANBEGINNER_SLIMES_WEAKNESS: SkillId = SkillId(20019002);
pub const ARAN1_COMBO_ABILITY: SkillId = SkillId(21000000);
pub const ARAN1_DOUBLE_SWING: SkillId = SkillId(21000002);
pub const ARAN1_COMBAT_STEP: SkillId = SkillId(21001001);
pub const ARAN1_POLEARM_BOOSTER: SkillId = SkillId(21001003);
pub const ARAN2_POLEARM_MASTERY: SkillId = SkillId(21100000);
pub const ARAN2_TRIPLE_SWING: SkillId = SkillId(21100001);
pub const ARAN2_FINAL_CHARGE: SkillId = SkillId(21100002);
pub const ARAN2_COMBO_SMASH: SkillId = SkillId(21100004);
pub const ARAN2_COMBO_DRAIN: SkillId = SkillId(21100005);
pub const ARAN2_BODY_PRESSURE: SkillId = SkillId(21101003);
pub const ARAN3_COMBO_CRITICAL: SkillId = SkillId(21110000);
pub const ARAN3_FULL_SWING: SkillId = SkillId(21110002);
pub const ARAN3_FINAL_TOSS: SkillId = SkillId(21110003);
pub const ARAN3_COMBO_FENRIR: SkillId = SkillId(21110004);
pub const ARAN3_ROLLING_SPIN: SkillId = SkillId(21110006);
pub const ARAN3_HIDDEN_FULL_SWING_DOUBLE_SWING: SkillId = SkillId(21110007);
pub const ARAN3_HIDDEN_FULL_SWING_TRIPLE_SWING: SkillId = SkillId(21110008);
pub const ARAN3_SMART_KNOCKBACK: SkillId = SkillId(21111001);
pub const ARAN3_SNOW_CHARGE: SkillId = SkillId(21111005);
pub const ARAN4_HIGH_MASTERY: SkillId = SkillId(21120001);
pub const ARAN4_OVER_SWING: SkillId = SkillId(21120002);
pub const ARAN4_HIGH_DEFENSE: SkillId = SkillId(21120004);
pub const ARAN4_FINAL_BLOW: SkillId = SkillId(21120005);
pub const ARAN4_COMBO_TEMPEST: SkillId = SkillId(21120006);
pub const ARAN4_COMBO_BARRIER: SkillId = SkillId(21120007);
pub const ARAN4_HIDDEN_OVER_SWING_DOUBLE_SWING: SkillId = SkillId(21120009);
pub const ARAN4_HIDDEN_OVER_SWING_TRIPLE_SWING: SkillId = SkillId(21120010);
pub const ARAN4_MAPLE_WARRIOR: SkillId = SkillId(21121000);
pub const ARAN4_FREEZE_STANDING: SkillId = SkillId(21121003);
pub const ARAN4_HEROS_WILL: SkillId = SkillId(21121008);
pub const EVAN1_DRAGON_SOUL: SkillId = SkillId(22000000);
pub const EVAN1_MAGIC_MISSILE: SkillId = SkillId(22001001);
pub const EVAN2_FIRE_CIRCLE: SkillId = SkillId(22101000);
pub const EVAN2_TELEPORT: SkillId = SkillId(22101001);
pub const EVAN3_LIGHTNING_BOLT: SkillId = SkillId(22111000);
pub const EVAN3_MAGIC_GUARD: SkillId = SkillId(22111001);
pub const EVAN4_SPELL_MASTERY: SkillId = SkillId(22120002);
pub const EVAN4_ICE_BREATH: SkillId = SkillId(22121000);
pub const EVAN4_ELEMENTAL_RESET: SkillId = SkillId(22121001);
pub const EVAN5_MAGIC_FLARE: SkillId = SkillId(22131000);
pub const EVAN5_MAGIC_SHIELD: SkillId = SkillId(22131001);
pub const EVAN6_CRITICAL_MAGIC: SkillId = SkillId(22140000);
pub const EVAN6_DRAGON_THRUST: SkillId = SkillId(22141001);
pub const EVAN6_MAGIC_BOOSTER: SkillId = SkillId(22141002);
pub const EVAN6_SLOW: SkillId = SkillId(22141003);
pub const EVAN7_MAGIC_AMPLIFICATION: SkillId = SkillId(22150000);
pub const EVAN7_FIRE_BREATH: SkillId = SkillId(22151001);
pub const EVAN7_KILLER_WINGS: SkillId = SkillId(22151002);
pub const EVAN7_MAGIC_RESISTANCE: SkillId = SkillId(22151003);
pub const EVAN8_DRAGON_FURY: SkillId = SkillId(22160000);
pub const EVAN8_EARTHQUAKE: SkillId = SkillId(22161001);
pub const EVAN8_PHANTOM_IMPRINT: SkillId = SkillId(22161002);
pub const EVAN8_RECOVERY_AURA: SkillId = SkillId(22161003);
pub const EVAN9_MAGIC_MASTERY: SkillId = SkillId(22170001);
pub const EVAN9_MAPLE_WARRIOR: SkillId = SkillId(22171000);
pub const EVAN9_ILLUSION: SkillId = SkillId(22171002);
pub const EVAN9_FLAME_WHEEL: SkillId = SkillId(22171003);
pub const EVAN9_HEROS_WILL: SkillId = SkillId(22171004);
pub const EVAN10_BLESSING_OF_THE_ONYX: SkillId = SkillId(22181000);
pub const EVAN10_BLAZE: SkillId = SkillId(22181001);
pub const EVAN10_DARK_FOG: SkillId = SkillId(22181002);
pub const EVAN10_SOUL_STONE: SkillId = SkillId(22181003);
pub const CITIZEN_POTION_MASTERY: SkillId = SkillId(30000002);
pub const CITIZEN_BLESSING_OF_THE_FAIRY: SkillId = SkillId(30000012);
pub const CITIZEN_DEADLY_CRITS: SkillId = SkillId(30000022);
pub const CITIZEN_CRYSTAL_THROW: SkillId = SkillId(30001000);
pub const CITIZEN_INFILTRATE: SkillId = SkillId(30001001);
pub const CITIZEN_LEGENDARY_SPIRIT: SkillId = SkillId(30001003);
pub const CITIZEN_MONSTER_RIDING: SkillId = SkillId(30001004);
pub const CITIZEN_HEROS_ECHO: SkillId = SkillId(30001005);
pub const CITIZEN_TEST: SkillId = SkillId(30001006);
pub const CITIZEN_MAKER: SkillId = SkillId(30001007);
pub const CITIZEN_BAMBOO_RAIN: SkillId = SkillId(30001009);
pub const CITIZEN_INVINCIBILITY: SkillId = SkillId(30001010);
pub const CITIZEN_POWER_EXPLOSION: SkillId = SkillId(30001011);
pub const CITIZEN_SPACESHIP: SkillId = SkillId(30001013);
pub const CITIZEN_SPACE_DASH: SkillId = SkillId(30001014);
pub const CITIZEN_SPACE_BEAM: SkillId = SkillId(30001015);
pub const CITIZEN_YETI_MOUNT: SkillId = SkillId(30001017);
pub const CITIZEN_YETI_MOUNT_1: SkillId = SkillId(30001018);
pub const CITIZEN_WITCHS_BROOMSTICK: SkillId = SkillId(30001019);
pub const CITIZEN_RAGE_OF_PHARAOH: SkillId = SkillId(30001020);
pub const CITIZEN_FOLLOW_THE_LEAD: SkillId = SkillId(30001024);
pub const CITIZEN_CHARGE_TOY_TROJAN: SkillId = SkillId(30001025);
pub const CITIZEN_SOARING: SkillId = SkillId(30001026);
pub const CITIZEN_CROCO: SkillId = SkillId(30001027);
pub const CITIZEN_BLACK_SCOOTER: SkillId = SkillId(30001028);
pub const CITIZEN_PINK_SCOOTER: SkillId = SkillId(30001029);
pub const CITIZEN_NIMBUS_CLOUD: SkillId = SkillId(30001030);
pub const CITIZEN_BALROG: SkillId = SkillId(30001031);
pub const CITIZEN_RACE_KART: SkillId = SkillId(30001033);
pub const CITIZEN_ZD_TIGER: SkillId = SkillId(30001034);
pub const CITIZEN_MIST_BALROG: SkillId = SkillId(30001035);
pub const CITIZEN_LION: SkillId = SkillId(30001036);
pub const CITIZEN_UNICORN: SkillId = SkillId(30001037);
pub const CITIZEN_LOW_RIDER: SkillId = SkillId(30001038);
pub const CITIZEN_RED_TRUCK: SkillId = SkillId(30001039);
pub const CITIZEN_GARGOYLE: SkillId = SkillId(30001040);
pub const CITIZEN_SHINJO: SkillId = SkillId(30001042);
pub const CITIZEN_ORANGE_MUSHROOM: SkillId = SkillId(30001044);
pub const CITIZEN_NIGHTMARE: SkillId = SkillId(30001049);
pub const CITIZEN_YETI: SkillId = SkillId(30001050);
pub const CITIZEN_OSTRICH: SkillId = SkillId(30001051);
pub const CITIZEN_PINK_BEAR_HOTAIR_BALLOON: SkillId = SkillId(30001052);
pub const CITIZEN_TRANSFORMED_ROBOT: SkillId = SkillId(30001053);
pub const CITIZEN_CAPTURE: SkillId = SkillId(30001061);
pub const CITIZEN_CALL_OF_THE_HUNTER: SkillId = SkillId(30001062);
pub const CITIZEN_MOTORCYCLE: SkillId = SkillId(30001063);
pub const CITIZEN_POWER_SUIT: SkillId = SkillId(30001064);
pub const CITIZEN_OS4_SHUTTLE: SkillId = SkillId(30001065);
pub const CITIZEN_VISITOR_MELEE_ATTACK: SkillId = SkillId(30001066);
pub const CITIZEN_VISITOR_RANGE_ATTACK: SkillId = SkillId(30001067);
pub const CITIZEN_MECHANIC_DASH: SkillId = SkillId(30001068);
pub const CITIZEN_OWL: SkillId = SkillId(30001069);
pub const CITIZEN_MOTHERSHIP: SkillId = SkillId(30001070);
pub const CITIZEN_OS3A_MACHINE: SkillId = SkillId(30001071);
pub const CITIZEN_DECENT_HASTE: SkillId = SkillId(30008000);
pub const CITIZEN_DECENT_MYSTIC_DOOR: SkillId = SkillId(30008001);
pub const CITIZEN_DECENT_SHARP_EYES: SkillId = SkillId(30008002);
pub const CITIZEN_DECENT_HYPER_BODY: SkillId = SkillId(30008003);
pub const TRIPLE_BLOW: SkillId = SkillId(32001000);
pub const THE_FINISHER: SkillId = SkillId(32001001);
pub const TELEPORT: SkillId = SkillId(32001002);
pub const DARK_AURA: SkillId = SkillId(32001003);
pub const STAFF_MASTERY: SkillId = SkillId(32100006);
pub const QUAD_BLOW: SkillId = SkillId(32101000);
pub const DARK_CHAIN: SkillId = SkillId(32101001);
pub const BLUE_AURA: SkillId = SkillId(32101002);
pub const YELLOW_AURA: SkillId = SkillId(32101003);
pub const BLOOD_DRAIN: SkillId = SkillId(32101004);
pub const STAFF_BOOST: SkillId = SkillId(32101005);
pub const ADVANCED_BLUE_AURA: SkillId = SkillId(32110000);
pub const BATTLE_MASTERY: SkillId = SkillId(32110001);
pub const QUINTUPLE_BLOW: SkillId = SkillId(32111002);
pub const DARK_SHOCK: SkillId = SkillId(32111003);
pub const CONVERSION: SkillId = SkillId(32111004);
pub const BODY_BOOST: SkillId = SkillId(32111005);
pub const SUMMON_REAPER_BUFF: SkillId = SkillId(32111006);
pub const TELEPORT_MASTERY: SkillId = SkillId(32111010);
pub const ADVANCED_DARK_CHAIN: SkillId = SkillId(32111011);
pub const ADVANCED_DARK_AURA: SkillId = SkillId(32120000);
pub const ADVANCED_YELLOW_AURA: SkillId = SkillId(32120001);
pub const ENERGIZE: SkillId = SkillId(32120009);
pub const FINISHING_BLOW: SkillId = SkillId(32121002);
pub const TWISTER_SPIN: SkillId = SkillId(32121003);
pub const DARK_GENESIS: SkillId = SkillId(32121004);
pub const STANCE: SkillId = SkillId(32121005);
pub const PARTY_SHIELD: SkillId = SkillId(32121006);
pub const MAPLE_WARRIOR: SkillId = SkillId(32121007);
pub const HEROS_WILL: SkillId = SkillId(32121008);
pub const WH1_TRIPLE_SHOT: SkillId = SkillId(33001000);
pub const WH1_JAGUAR_RIDER: SkillId = SkillId(33001001);
pub const WH1_JAG_JUMP: SkillId = SkillId(33001002);
pub const WH1_CROSSBOW_BOOSTER: SkillId = SkillId(33001003);
pub const WH2_CROSSBOW_MASTERY: SkillId = SkillId(33100000);
pub const WH2_FINAL_ATTACK: SkillId = SkillId(33100009);
pub const WH2_RICOCHET: SkillId = SkillId(33101001);
pub const WH2_JAGUAR_RAWR: SkillId = SkillId(33101002);
pub const WH2_SOUL_ARROW_CROSSBOW: SkillId = SkillId(33101003);
pub const WH2_ITS_RAINING_MINES: SkillId = SkillId(33101004);
pub const WH2_JAGUAROSHI: SkillId = SkillId(33101005);
pub const WH2_JAGUAROSHI_1: SkillId = SkillId(33101006);
pub const WH2_JAGUAROSHI_2: SkillId = SkillId(33101007);
pub const WH2_ITS_RAINING_MINESHIDDEN_SELFDESTRUCT: SkillId = SkillId(33101008);
pub const WH3_JAGUAR_BOOST: SkillId = SkillId(33110000);
pub const WH3_ENDURING_FIRE: SkillId = SkillId(33111001);
pub const WH3_DASH_N_SLASH: SkillId = SkillId(33111002);
pub const WH3_WILD_TRAP: SkillId = SkillId(33111003);
pub const WH3_BLIND: SkillId = SkillId(33111004);
pub const WH3_SILVER_HAWK: SkillId = SkillId(33111005);
pub const WH3_SWIPE: SkillId = SkillId(33111006);
pub const WH4_CROSSBOW_EXPERT: SkillId = SkillId(33120000);
pub const WH4_WILD_INSTINCT: SkillId = SkillId(33120010);
pub const WH4_EXPLODING_ARROWS: SkillId = SkillId(33121001);
pub const WH4_SONIC_ROAR: SkillId = SkillId(33121002);
pub const WH4_SHARP_EYES: SkillId = SkillId(33121004);
pub const WH4_STINK_BOMB_SHOT: SkillId = SkillId(33121005);
pub const WH4_FELINE_BERSERK: SkillId = SkillId(33121006);
pub const WH4_MAPLE_WARRIOR: SkillId = SkillId(33121007);
pub const WH4_HEROS_WILL: SkillId = SkillId(33121008);
pub const WH4_WILD_ARROW_BLAST: SkillId = SkillId(33121009);
pub const MECH1_FLAME_LAUNCHER: SkillId = SkillId(35001001);
pub const MECH1_MECH_PROTOTYPE: SkillId = SkillId(35001002);
pub const MECH1_ME07_DRILLHANDS: SkillId = SkillId(35001003);
pub const MECH1_GATLING_GUN: SkillId = SkillId(35001004);
pub const MECH2_MECHANIC_MASTERY: SkillId = SkillId(35100000);
pub const MECH2_HEAVY_WEAPON_MASTERY: SkillId = SkillId(35100008);
pub const MECH2_ATOMIC_HAMMER: SkillId = SkillId(35101003);
pub const MECH2_ROCKET_BOOSTER: SkillId = SkillId(35101004);
pub const MECH2_OPEN_PORTAL_GX9: SkillId = SkillId(35101005);
pub const MECH2_MECHANIC_RAGE: SkillId = SkillId(35101006);
pub const MECH2_PERFECT_ARMOR: SkillId = SkillId(35101007);
pub const MECH2_ENHANCED_FLAME_LAUNCHER: SkillId = SkillId(35101009);
pub const MECH2_ENHANCED_GATLING_GUN: SkillId = SkillId(35101010);
pub const MECH3_METAL_FIST_MASTERY: SkillId = SkillId(35110014);
pub const MECH3_SATELLITE: SkillId = SkillId(35111001);
pub const MECH3_ROCK_N_SHOCK: SkillId = SkillId(35111002);
pub const MECH3_MECH_SIEGE_MODE: SkillId = SkillId(35111004);
pub const MECH3_ACCELERATION_BOT_EX7: SkillId = SkillId(35111005);
pub const MECH3_SATELLITE_1: SkillId = SkillId(35111009);
pub const MECH3_SATELLITE_2: SkillId = SkillId(35111010);
pub const MECH3_HEALING_ROBOT_HLX: SkillId = SkillId(35111011);
pub const MECH3_ROLL_OF_THE_DICE: SkillId = SkillId(35111013);
pub const MECH3_PUNCH_LAUNCHER: SkillId = SkillId(35111015);
pub const MECH4_EXTREME_MECH: SkillId = SkillId(35120000);
pub const MECH4_ROBOT_MASTERY: SkillId = SkillId(35120001);
pub const MECH4_GIANT_ROBOT_SG88: SkillId = SkillId(35121003);
pub const MECH4_MECH_MISSILE_TANK: SkillId = SkillId(35121005);
pub const MECH4_SATELLITE_SAFETY: SkillId = SkillId(35121006);
pub const MECH4_MAPLE_WARRIOR: SkillId = SkillId(35121007);
pub const MECH4_HEROS_WILL: SkillId = SkillId(35121008);
pub const MECH4_BOTS_N_TOTS: SkillId = SkillId(35121009);
pub const MECH4_AMPLIFIER_ROBOT_AF11: SkillId = SkillId(35121010);
pub const MECH4_LASER_BLAST: SkillId = SkillId(35121012);
pub const MECH4_MECH_SIEGE_MODE: SkillId = SkillId(35121013);
pub const INSTANT_DEATH: SkillId = SkillId(90000000);
pub const KNOCK_DOWN: SkillId = SkillId(90001001);
pub const SLOW: SkillId = SkillId(90001002);
pub const POISON: SkillId = SkillId(90001003);
pub const DARKNESS: SkillId = SkillId(90001004);
pub const SEAL: SkillId = SkillId(90001005);
pub const FREEZE: SkillId = SkillId(90001006);
