use std::ops::RangeInclusive;

use shroom_pkt::shroom_enum_code;

use crate::shroom_id;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum ItemType {
    Equip,
    Consume,
    Install,
    Etc,
    Cash,
    No,
    ExNo,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum CashItemType {
    None,
    Hair,
    Face,
    Skin,
    Shop,
    SetPetLife,
    Emotion,
    ProtectOnDie,
    Pet,
    Effect,
    Bullet,
    ShopEmployee,
    SpeakerChannel,
    SpeakerWorld,
    ItemSpeaker,
    SpeakerBridge,
    Weather,
    SetPetName,
    MessageBox,
    MoneyPocket,
    Jukebox,
    SendMemo,
    MapTransfer,
    StatChange,
    SkillChange,
    Naming,
    Protecting,
    Incubator,
    PetSkill,
    ShopScanner,
    PetFood,
    QuickDelivery,
    AdBoard,
    ConsumeEffectItem,
    ConsumeAreaBuffItem,
    ColorLens,
    WeddingTicket,
    InvitationTicket,
    SelectNpc,
    RemoteShop,
    GachaponCoupon,
    Morph,
    PetEvol,
    AvatarMegaphone,
    HeartSpeaker,
    SkullSpeaker,
    Removable,
    MapleTv,
    MapleSoleTv,
    MapleLoveTv,
    MegaTv,
    MegaSoleTv,
    MegaLoveTv,
    ChangeCharacterName,
    TransferWorldCoupon,
    HairShopMembershipCoupon,
    FaceShopMembershipCoupon,
    SkinShopMembershipCoupon,
    PetSnack,
    GachaponBoxMasterKey,
    GachaponRemote,
    ArtSpeakerWorld,
    ExtendExpireDate,
    UpgradeTomb,
    KarmaScissors,
    ExpiredProtecting,
    CharacterSale,
    ItemUpgrade,
    CashItemGachapon,
    CashGachaponOpen,
    ChangeMaplePoint,
    Vega,
    Reward,
    MasteryBook,
    ItemUnrelease,
    SkillReset,
    DragonBall,
    RecoverUpgradeCount,
    QuestDelivery,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum WeaponType {
    None,
    OneHandSword,
    OneHandAxe,
    OneHandMace,
    Dagger,
    SubDagger,
    Wand,
    Staff,
    BareHand,
    TwoHandSword,
    TwoHandAxe,
    TwoHandMace,
    Spear,
    PoleArm,
    Bow,
    Crossbow,
    Claw,
    Knuckle,
    Gun,
}

impl WeaponType {
    pub fn is_two_handed(&self) -> bool {
        matches!(
            self,
            Self::TwoHandSword
                | Self::TwoHandAxe
                | Self::TwoHandMace
                | Self::Spear
                | Self::PoleArm
                | Self::Bow
                | Self::Crossbow
                | Self::Claw
                | Self::Knuckle
                | Self::Gun
        )
    }
}

impl TryFrom<u8> for WeaponType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            30 => Self::OneHandSword,
            31 => Self::OneHandAxe,
            32 => Self::OneHandMace,
            33 => Self::Dagger,
            34 => Self::SubDagger,
            37 => Self::Wand,
            38 => Self::Staff,
            39 => Self::BareHand,
            40 => Self::TwoHandSword,
            41 => Self::TwoHandAxe,
            42 => Self::TwoHandMace,
            43 => Self::Spear,
            44 => Self::PoleArm,
            45 => Self::Bow,
            46 => Self::Crossbow,
            47 => Self::Claw,
            48 => Self::Knuckle,
            49 => Self::Gun,
            _ => anyhow::bail!("Unknown weapon type: {}", value),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum EquipType {
    Cap,
    FaceAcc,
    EyeAcc,
    EarAcc,
    Shirt,
    Coat,
    Pants,
    Shoes,
    Gloves,
    Shield,
    Cape,
    Ring,
    Pendant,
    Belt,
    Medal,
    Shoulder,
    MechanicBase,
    MechanicArm,
    MechanicLeg,
    MechanicFrame,
    MechanicTransistor,
    PetEquip,
    TamingMob,
    Saddle,
    MobEquip,
    DragonBase,
    DragonPendant,
    DragonWing,
    DragonShoes,
    Weapon,
}

impl EquipType {
    pub fn is_accessory(&self) -> bool {
        matches!(
            self,
            Self::FaceAcc | Self::EyeAcc | Self::EarAcc | Self::Ring | Self::Pendant | Self::Belt
        )
    }
}

shroom_id!(ItemId, u32);

shroom_enum_code!(
    ItemGrade,
    u8,
    Normal = 0,
    HiddenRare = 1,
    HiddenEpic = 2,
    HiddenUnique = 3,
    // Visible normal?
    VisibleRare = 5,
    VisibleEpic = 6,
    VisibleUnique = 7
);

shroom_enum_code!(
    InventoryType,
    u8,
    Equip = 1,
    Consume = 2,
    Install = 3,
    Etc = 4,
    Cash = 5,
    Equipped = 6,
    Special = 9,
    DragonEquipped = 10,
    MechanicEquipped = 11
);

impl InventoryType {
    pub fn is_equip(&self) -> bool {
        // TODO make this correct
        matches!(self, Self::Equipped | Self::Equip)
    }

    pub fn is_stack(&self) -> bool {
        !self.is_equip()
    }

    pub fn is_equipped(&self) -> bool {
        matches!(
            self,
            Self::Equipped | Self::Special | Self::DragonEquipped | Self::MechanicEquipped
        )
    }
}

impl ItemId {
    pub fn is_black_upgrade(&self) -> bool {
        self.0 / 100 == 20491
    }
    pub fn is_durability_upgrade(&self) -> bool {
        self.0 / 1000 == 2047
    }

    pub fn is_new_upgrade(&self) -> bool {
        self.0 / 1000 == 2046
    }

    pub fn is_hyper_upgrade(&self) -> bool {
        self.0 / 100 == 20493
    }

    pub fn is_item_option_upgrade(&self) -> bool {
        self.0 / 100 == 20494
    }

    pub fn is_acc_upgrade(&self) -> bool {
        self.0 / 100 == 20492
    }

    pub fn is_pet_equip(&self) -> bool {
        self.0 / 10000 == 18
    }

    pub fn is_upgrade(&self) -> bool {
        let upgrade_ty = self.0 / 10_000;

        matches!(upgrade_ty, 247 | 249 | 204)
    }

    pub fn is_consumable(&self) -> bool {
        matches!(self.0 / 10_000, 200..=250)
    }

    pub fn can_upgrade_with(&self, upgrade_item: ItemId) -> bool {
        let upgrade_ty = upgrade_item.0 / 10_000;

        // Visitor extension or vicious hammer
        // TODO further checks or cleaner way
        if matches!(upgrade_ty, 247 | 249) {
            return true;
        }

        // Check for scroll item and if the current item is an equip
        if upgrade_ty != 204 || self.item_type() != ItemType::Equip {
            return false;
        }

        // Check for common upgrade items
        if upgrade_item.0 / 100 == 20490
            || upgrade_item.is_black_upgrade() && !self.is_pet_equip()
            || upgrade_item.is_hyper_upgrade()
            || upgrade_item.is_item_option_upgrade()
        {
            return true;
        }

        let scroll_cat = (upgrade_item.0 - 2040000) / 100;
        let eq_cat = (self.0 / 10_000) % 100;

        // Accessory upgrade
        if upgrade_item.is_acc_upgrade()  {
            return (11..=13).contains(&eq_cat);
        }

        if upgrade_item.is_new_upgrade() || upgrade_item.is_durability_upgrade() {
            return match scroll_cat % 10 {
                // One-Handed
                0 => (30..=39).contains(&eq_cat),
                // Two-Handed
                1 => (40..=49).contains(&eq_cat),
                // Armor
                2 => matches!(eq_cat, 4..=10),
                // Accessory
                3 => matches!(eq_cat, 1..=3 | 11..=14),

                _ => true
            };
        }

        scroll_cat == eq_cat
    }

    pub fn weapon_type(&self) -> Option<WeaponType> {
        if self.item_type() != ItemType::Equip {
            return None;
        }
        WeaponType::try_from((self.0 / 10000) as u8).ok()
    }

    pub fn equip_type(&self) -> Option<EquipType> {
        if self.item_type() != ItemType::Equip {
            return None;
        }

        Some(match self.0 / 10000 {
            100 => EquipType::Cap,
            101 => EquipType::FaceAcc,
            102 => EquipType::EyeAcc,
            103 => EquipType::EarAcc,
            104 => EquipType::Shirt,
            105 => EquipType::Coat,
            106 => EquipType::Pants,
            107 => EquipType::Shoes,
            108 => EquipType::Gloves,
            109 => EquipType::Shield,
            110 => EquipType::Cape,
            111 => EquipType::Ring,
            112 => EquipType::Pendant,
            113 => EquipType::Belt,
            114 => EquipType::Medal,
            115 => EquipType::Shoulder,
            161 => EquipType::MechanicBase,
            162 => EquipType::MechanicArm,
            163 => EquipType::MechanicLeg,
            164 => EquipType::MechanicFrame,
            165 => EquipType::MechanicTransistor,
            180..=183 => EquipType::PetEquip,
            190 => EquipType::TamingMob,
            191 => EquipType::Saddle,
            192 => EquipType::MobEquip,
            194 => EquipType::DragonBase,
            195 => EquipType::DragonPendant,
            196 => EquipType::DragonWing,
            197 => EquipType::DragonShoes,
            130..=159 => EquipType::Weapon,
            _ => {
                return None;
            }
        })
    }

    pub fn item_type(&self) -> ItemType {
        match self.0 / 1000000 {
            1 => ItemType::Equip,
            2 => ItemType::Consume,
            3 => ItemType::Install,
            4 => ItemType::Etc,
            5 => ItemType::Cash,
            6 => ItemType::No,
            9 => ItemType::ExNo,
            _ => unreachable!(),
        }
    }

    pub fn get_inv_type(&self) -> anyhow::Result<InventoryType> {
        let ty = self.0 / 1000000;
        Ok(match ty {
            1 => InventoryType::Equip,
            2 => InventoryType::Consume,
            3 => InventoryType::Install,
            4 => InventoryType::Etc,
            5 => InventoryType::Cash,
            _ => anyhow::bail!("Unknown inv type for item: {self:?}"),
        })
    }

    pub fn is_arrow_for_bow(&self) -> bool {
        (2060000..=2061000).contains(&self.0)
    }

    pub fn is_arrow_for_crossbow(&self) -> bool {
        (2061000..=2062000).contains(&self.0)
    }
    

    pub fn is_summon_sack(&self) -> bool {
        self.0 / 10000 == 210
    }

    pub fn is_throwing_star(&self) -> bool {
        Self::THROWING_STAR_RANGE.contains(self)
    }

    pub fn is_rechargable(&self) -> bool {
        self.0 / 10000 == 233 || self.0 / 10000 == 207
    }

    pub fn is_exp_increase(&self) -> bool {
        (2022450..=2022452).contains(&self.0)
    }

    pub fn is_rate_coupon(&self) -> bool {
        let ty = self.0 / 1000;
        ty == 5211 || ty == 5360
    }

    pub fn is_monster_card(&self) -> bool {
        let ty = self.0 / 10000;
        ty == 238
    }

    pub fn is_pyramid_buff(&self) -> bool {
        (2022585..=2022588).contains(&self.0) || (2022616..=2022617).contains(&self.0)
    }

    pub fn is_dojo_buff(&self) -> bool {
        (2022359..=2022421).contains(&self.0)
    }

    pub fn is_chair(&self) -> bool {
        Self::CHAIR_RANGE.contains(self)
    }

    pub fn is_wedding_ring(&self) -> bool {
        matches!(
            *self,
            Self::WEDDING_RING_GOLDEN
                | Self::WEDDING_RING_MOONSTONE
                | Self::WEDDING_RING_SILVER
                | Self::WEDDING_RING_STAR
        )
    }

    pub fn is_wedding_token(&self) -> bool {
        (Self::EMPTY_ENGAGEMENT_BOX_MOONSTONE..=Self::ENGAGEMENT_BOX_SILVER).contains(self)
    }

    pub fn is_party_all_cure(&self) -> bool {
        matches!(
            *self,
            Self::DOJO_PARTY_ALL_CURE | Self::CARNIVAL_PARTY_ALL_CURE
        )
    }

    pub fn is_pet(&self) -> bool {
        self.0 / 1000 == 5000
    }

    pub fn is_nx_card(&self) -> bool {
        matches!(*self, Self::NX_CARD_100 | Self::NX_CARD_250)
    }

    pub fn is_facial_expression(&self) -> bool {
        Self::FACE_EXPRESSION_RANGE.contains(self)
    }

    pub fn is_cygnus_mount(&self) -> bool {
        (Self::MIMIANA..=Self::SHINJOU).contains(self) || *self == Self::CYGNUS_SADDLE
    }

    pub fn is_explorer_mount(&self) -> bool {
        (Self::HOG..=Self::RED_DRACO).contains(self) || *self == Self::EXPLORER_SADDLE
    }


    pub fn is_state_change(&self) -> bool {
        let ty = self.0 / 10000;
        matches!(
            ty,
            200 | 201 | 202 | 205 | 221 | 236 | 238 | 245
        )
    }
}

impl ItemId {
    // Misc
    pub const PENDANT_OF_THE_SPIRIT: Self = Self(1122017);
    pub const HEART_SHAPED_CHOCOLATE: Self = Self(5110000);
    pub const HAPPY_BIRTHDAY: Self = Self(2022153);
    pub const FISHING_CHAIR: Self = Self(3011000);
    pub const MINI_GAME_BASE: Self = Self(4080000);
    pub const MATCH_CARDS: Self = Self(4080100);
    pub const MAGICAL_MITTEN: Self = Self(1472063);
    pub const RPS_CERTIFICATE_BASE: Self = Self(4031332);
    pub const GOLDEN_MAPLE_LEAF: Self = Self(4000313);
    pub const PERFECT_PITCH: Self = Self(4310000);
    pub const MAGIC_ROCK: Self = Self(4006000);
    pub const GOLDEN_CHICKEN_EFFECT: Self = Self(4290000);
    pub const BUMMER_EFFECT: Self = Self(4290001);
    pub const ARPQ_SHIELD: Self = Self(2022269);
    pub const ROARING_TIGER_MESSENGER: Self = Self(5390006);
    // Potion
    pub const WHITE_POTION: Self = Self(2000002);
    pub const BLUE_POTION: Self = Self(2000003);
    pub const ORANGE_POTION: Self = Self(2000001);
    pub const MANA_ELIXIR: Self = Self(2000006);

    // HP/MP recovery
    pub const SORCERERS_POTION: Self = Self(2022337);
    pub const RUSSELLONS_PILLS: Self = Self(2022198);

    // Environment
    pub const RED_BEAN_PORRIDGE: Self = Self(2022001);
    pub const SOFT_WHITE_BUN: Self = Self(2022186);
    pub const AIR_BUBBLE: Self = Self(2022040);

    // Chair
    pub const RELAXER: Self = Self(3010000);
    pub const CHAIR_MIN: Self = Self::RELAXER;
    pub const CHAIR_MAX: Self = Self::FISHING_CHAIR;
    pub const CHAIR_RANGE: RangeInclusive<Self> = Self::CHAIR_MIN..=Self::CHAIR_MAX;

    // Throwing star
    pub const SUBI_THROWING_STARS: Self = Self(2070000);
    pub const HWABI_THROWING_STARS: Self = Self(2070007);
    pub const BALANCED_FURY: Self = Self(2070018);
    pub const DEVIL_RAIN_THROWING_STAR: Self = Self(2070014);
    pub const CRYSTAL_ILBI_THROWING_STARS: Self = Self(2070016);
    pub const THROWING_STAR_MIN: Self = Self::SUBI_THROWING_STARS;
    //TODO MAX is  wrong(balanced fury not throwing stars???)
    pub const THROWING_STAR_MAX: Self = Self(2070018);
    pub const THROWING_STAR_RANGE: RangeInclusive<Self> =
        Self::THROWING_STAR_MIN..=Self::THROWING_STAR_MAX;

    // Bullet
    pub const BULLET: Self = Self(2330000);
    pub const BULLET_MIN: Self = Self::BULLET;
    pub const BULLET_MAX: Self = Self(2330005);
    pub const BULLET_RANGE: RangeInclusive<Self> = Self::BULLET_MIN..=Self::BULLET_MAX;
    pub const BLAZE_CAPSULE: Self = Self(2331000);
    pub const GLAZE_CAPSULE: Self = Self(2332000);

    // Starter
    pub const BEGINNERS_GUIDE: Self = Self(4161001);
    pub const LEGENDS_GUIDE: Self = Self(4161048);
    pub const NOBLESSE_GUIDE: Self = Self(4161047);
    pub const SWORD: Self = Self(1302000); // Weapon
    pub const HAND_AXE: Self = Self(1312004);
    pub const WOODEN_CLUB: Self = Self(1322005);
    pub const BASIC_POLEARM: Self = Self(1442079);
    pub const WHITE_UNDERSHIRT: Self = Self(1040002); // Top
    pub const UNDERSHIRT: Self = Self(1040006);
    pub const GREY_TSHIRT: Self = Self(1040010);
    pub const WHITE_TUBETOP: Self = Self(1041002);
    pub const YELLOW_TSHIRT: Self = Self(1041006);
    pub const GREEN_TSHIRT: Self = Self(1041010);
    pub const RED_STRIPED_TOP: Self = Self(1041011);
    pub const SIMPLE_WARRIOR_TOP: Self = Self(1042167);
    pub const BLUE_JEAN_SHORTS: Self = Self(1060002); // Bottom
    pub const BROWN_COTTON_SHORTS: Self = Self(1060006);
    pub const RED_MINISKIRT: Self = Self(1061002);
    pub const INDIGO_MINISKIRT: Self = Self(1061008);
    pub const SIMPLE_WARRIOR_PANTS: Self = Self(1062115);
    pub const RED_RUBBER_BOOTS: Self = Self(1072001);
    pub const LEATHER_SANDALS: Self = Self(1072005);
    pub const YELLOW_RUBBER_BOOTS: Self = Self(1072037);
    pub const BLUE_RUBBER_BOOTS: Self = Self(1072038);
    pub const AVERAGE_MUSASHI_SHOES: Self = Self(1072383);

    // Warrior
    pub const RED_HWARANG_SHIRT: Self = Self(1040021);
    pub const BLACK_MARTIAL_ARTS_PANTS: Self = Self(1060016);
    pub const MITHRIL_BATTLE_GRIEVES: Self = Self(1072039);
    pub const GLADIUS: Self = Self(1302008);
    pub const MITHRIL_POLE_ARM: Self = Self(1442001);
    pub const MITHRIL_MAUL: Self = Self(1422001);
    pub const FIREMANS_AXE: Self = Self(1312005);
    pub const DARK_ENGRIT: Self = Self(1051010);

    // Bowman
    pub const GREEN_HUNTERS_ARMOR: Self = Self(1040067);
    pub const GREEN_HUNTRESS_ARMOR: Self = Self(1041054);
    pub const GREEN_HUNTERS_PANTS: Self = Self(1060056);
    pub const GREEN_HUNTRESS_PANTS: Self = Self(1061050);
    pub const GREEN_HUNTER_BOOTS: Self = Self(1072081);
    pub const RYDEN: Self = Self(1452005);
    pub const MOUNTAIN_CROSSBOW: Self = Self(1462000);

    // Magician
    pub const BLUE_WIZARD_ROBE: Self = Self(1050003);
    pub const PURPLE_FAIRY_TOP: Self = Self(1041041);
    pub const PURPLE_FAIRY_SKIRT: Self = Self(1061034);
    pub const RED_MAGICSHOES: Self = Self(1072075);
    pub const MITHRIL_WAND: Self = Self(1372003);
    pub const CIRCLE_WINDED_STAFF: Self = Self(1382017);

    // Thief
    pub const DARK_BROWN_STEALER: Self = Self(1040057);
    pub const RED_STEAL: Self = Self(1041047);
    pub const DARK_BROWN_STEALER_PANTS: Self = Self(1060043);
    pub const RED_STEAL_PANTS: Self = Self(1061043);
    pub const BRONZE_CHAIN_BOOTS: Self = Self(1072032);
    pub const STEEL_GUARDS: Self = Self(1472008);
    pub const REEF_CLAW: Self = Self(1332012);

    // Pirate
    pub const BROWN_PAULIE_BOOTS: Self = Self(1072294);
    pub const PRIME_HANDS: Self = Self(1482004);
    pub const COLD_MIND: Self = Self(1492004);
    pub const BROWN_POLLARD: Self = Self(1052107);

    // Three snails
    pub const SNAIL_SHELL: Self = Self(4000019);
    pub const BLUE_SNAIL_SHELL: Self = Self(4000000);
    pub const RED_SNAIL_SHELL: Self = Self(4000016);

    // Special SCROLL
    pub const COLD_PROTECTION_SCROLL: Self = Self(2041058);
    pub const SPIKES_SCROLL: Self = Self(2040727);
    pub const VEGAS_SPELL_10: Self = Self(5610000);
    pub const VEGAS_SPELL_60: Self = Self(5610001);
    pub const CHAOS_SCROLL_60: Self = Self(2049100);
    pub const LIAR_TREE_SAP: Self = Self(2049101);
    pub const MAPLE_SYRUP: Self = Self(2049102);
    pub const WHITE_SCROLL: Self = Self(2340000);
    pub const CLEAN_SLATE_1: Self = Self(2049000);
    pub const CLEAN_SLATE_3: Self = Self(2049001);
    pub const CLEAN_SLATE_5: Self = Self(2049002);
    pub const CLEAN_SLATE_20: Self = Self(2049003);
    pub const RING_STR_100_SCROLL: Self = Self(2041100);
    pub const DRAGON_STONE_SCROLL: Self = Self(2041200);
    pub const BELT_STR_100_SCROLL: Self = Self(2041300);

    // Cure debuff
    pub const ALL_CURE_POTION: Self = Self(2050004);
    pub const EYEDROP: Self = Self(2050001);
    pub const TONIC: Self = Self(2050002);
    pub const HOLY_WATER: Self = Self(2050003);
    pub const ANTI_BANISH_SCROLL: Self = Self(2030100);
    pub const DOJO_PARTY_ALL_CURE: Self = Self(2022433);
    pub const CARNIVAL_PARTY_ALL_CURE: Self = Self(2022163);
    pub const WHITE_ELIXIR: Self = Self(2022544);

    // Special effect
    pub const PHARAOHS_BLESSING_1: Self = Self(2022585);
    pub const PHARAOHS_BLESSING_2: Self = Self(2022586);
    pub const PHARAOHS_BLESSING_3: Self = Self(2022587);
    pub const PHARAOHS_BLESSING_4: Self = Self(2022588);

    // Evolve pet
    pub const DRAGON_PET: Self = Self(5000028);
    pub const ROBO_PET: Self = Self(5000047);

    // Pet equip
    pub const MESO_MAGNET: Self = Self(1812000);
    pub const ITEM_POUCH: Self = Self(1812001);
    pub const ITEM_IGNORE: Self = Self(1812007);

    // Expirable pet
    pub const PET_SNAIL: Self = Self(5000054);

    // Permanent pet
    pub const PERMA_PINK_BEAN: Self = Self(5000060);
    pub const PERMA_KINO: Self = Self(5000100);
    pub const PERMA_WHITE_TIGER: Self = Self(5000101);
    pub const PERMA_MINI_YETI: Self = Self(5000102);

    // Maker
    pub const BASIC_MONSTER_CRYSTAL_1: Self = Self(4260000);
    pub const BASIC_MONSTER_CRYSTAL_2: Self = Self(4260001);
    pub const BASIC_MONSTER_CRYSTAL_3: Self = Self(4260002);
    pub const INTERMEDIATE_MONSTER_CRYSTAL_1: Self = Self(4260003);
    pub const INTERMEDIATE_MONSTER_CRYSTAL_2: Self = Self(4260004);
    pub const INTERMEDIATE_MONSTER_CRYSTAL_3: Self = Self(4260005);
    pub const ADVANCED_MONSTER_CRYSTAL_1: Self = Self(4260006);
    pub const ADVANCED_MONSTER_CRYSTAL_2: Self = Self(4260007);
    pub const ADVANCED_MONSTER_CRYSTAL_3: Self = Self(4260008);

    // NPC weather (PQ)
    pub const NPC_WEATHER_GROWLIE: Self = Self(5120016); // Henesys PQ

    // Safety charm
    pub const SAFETY_CHARM: Self = Self(5130000);
    pub const EASTER_BASKET: Self = Self(4031283);
    pub const EASTER_CHARM: Self = Self(4140903);

    // Engagement box
    pub const ENGAGEMENT_BOX_MOONSTONE: Self = Self(2240000);
    pub const ENGAGEMENT_BOX_STAR: Self = Self(2240001);
    pub const ENGAGEMENT_BOX_GOLDEN: Self = Self(2240002);
    pub const ENGAGEMENT_BOX_SILVER: Self = Self(2240003);
    pub const EMPTY_ENGAGEMENT_BOX_MOONSTONE: Self = Self(4031357);
    pub const ENGAGEMENT_RING_MOONSTONE: Self = Self(4031358);
    pub const EMPTY_ENGAGEMENT_BOX_STAR: Self = Self(4031359);
    pub const ENGAGEMENT_RING_STAR: Self = Self(4031360);
    pub const EMPTY_ENGAGEMENT_BOX_GOLDEN: Self = Self(4031361);
    pub const ENGAGEMENT_RING_GOLDEN: Self = Self(4031362);
    pub const EMPTY_ENGAGEMENT_BOX_SILVER: Self = Self(4031363);
    pub const ENGAGEMENT_RING_SILVER: Self = Self(4031364);

    // Wedding etc
    pub const PARENTS_BLESSING: Self = Self(4031373);
    pub const OFFICIATORS_PERMISSION: Self = Self(4031374);
    pub const ONYX_CHEST_FOR_COUPLE: Self = Self(4031424);

    // Wedding ticket
    pub const NORMAL_WEDDING_TICKET_CATHEDRAL: Self = Self(5251000);
    pub const NORMAL_WEDDING_TICKET_CHAPEL: Self = Self(5251001);
    pub const PREMIUM_WEDDING_TICKET_CHAPEL: Self = Self(5251002);
    pub const PREMIUM_WEDDING_TICKET_CATHEDRAL: Self = Self(5251003);

    // Wedding reservation
    pub const PREMIUM_CATHEDRAL_RESERVATION_RECEIPT: Self = Self(4031375);
    pub const PREMIUM_CHAPEL_RESERVATION_RECEIPT: Self = Self(4031376);
    pub const NORMAL_CATHEDRAL_RESERVATION_RECEIPT: Self = Self(4031480);
    pub const NORMAL_CHAPEL_RESERVATION_RECEIPT: Self = Self(4031481);

    // Wedding invite
    pub const INVITATION_CHAPEL: Self = Self(4031377);
    pub const INVITATION_CATHEDRAL: Self = Self(4031395);
    pub const RECEIVED_INVITATION_CHAPEL: Self = Self(4031406);
    pub const RECEIVED_INVITATION_CATHEDRAL: Self = Self(4031407);

    pub const CARAT_RING_BASE: Self = Self(1112300); // Unsure about math on this and the following one
    pub const CARAT_RING_BOX_BASE: Self = Self(2240004);
    pub const CARAT_RING_BOX_MAX: Self = Self(2240015);

    pub const ENGAGEMENT_BOX_MIN: Self = Self::ENGAGEMENT_BOX_MOONSTONE;
    pub const ENGAGEMENT_BOX_MAX: Self = Self::CARAT_RING_BOX_MAX;
    pub const ENGAGEMENT_BOX_RANGE: RangeInclusive<Self> =
        Self::ENGAGEMENT_BOX_MIN..=Self::ENGAGEMENT_BOX_MAX;

    // Wedding ring
    pub const WEDDING_RING_MOONSTONE: Self = Self(1112803);
    pub const WEDDING_RING_STAR: Self = Self(1112806);
    pub const WEDDING_RING_GOLDEN: Self = Self(1112807);
    pub const WEDDING_RING_SILVER: Self = Self(1112809);

    // Priority buff
    pub const ROSE_SCENT: Self = Self(2022631);
    pub const FREESIA_SCENT: Self = Self(2022632);
    pub const LAVENDER_SCENT: Self = Self(2022633);

    // Cash shop
    pub const WHEEL_OF_FORTUNE: Self = Self(5510000);
    pub const CASH_SHOP_SURPRISE: Self = Self(5222000);
    pub const EXP_COUPON_2X_4H: Self = Self(5211048);
    pub const DROP_COUPON_2X_4H: Self = Self(5360042);
    pub const EXP_COUPON_3X_2H: Self = Self(5211060);
    pub const QUICK_DELIVERY_TICKET: Self = Self(5330000);
    pub const CHALKBOARD_1: Self = Self(5370000);
    pub const CHALKBOARD_2: Self = Self(5370001);
    pub const REMOTE_GACHAPON_TICKET: Self = Self(5451000);
    pub const AP_RESET: Self = Self(5050000);
    pub const NAME_CHANGE: Self = Self(5400000);
    pub const WORLD_TRANSFER: Self = Self(5401000);
    pub const MAPLE_LIFE_B: Self = Self(5432000);
    pub const VICIOUS_HAMMER: Self = Self(5570000);

    pub const NX_CARD_100: Self = Self(4031865);
    pub const NX_CARD_250: Self = Self(4031866);

    // <Face> expression
    pub const FACE_EXPRESSION_MIN: Self = Self(5160000);
    pub const FACE_EXPRESSION_MAX: Self = Self(5160014);
    pub const FACE_EXPRESSION_RANGE: RangeInclusive<Self> =
        Self::FACE_EXPRESSION_MIN..=Self::FACE_EXPRESSION_MAX;

    // New Year card
    pub const NEW_YEARS_CARD: Self = Self(2160101);
    pub const NEW_YEARS_CARD_SEND: Self = Self(4300000);
    pub const NEW_YEARS_CARD_RECEIVED: Self = Self(4301000);

    // Popular owl items
    pub const WORK_GLOVES: Self = Self(1082002);
    pub const STEELY_THROWING_KNIVES: Self = Self(2070005);
    pub const ILBI_THROWING_STARS: Self = Self(2070006);
    pub const OWL_BALL_MASK: Self = Self(1022047);
    pub const PINK_ADVENTURER_CAPE: Self = Self(1102041);
    pub const CLAW_30_SCROLL: Self = Self(2044705);
    pub const HELMET_60_ACC_SCROLL: Self = Self(2040017);
    pub const MAPLE_SHIELD: Self = Self(1092030);
    pub const GLOVES_ATT_60_SCROLL: Self = Self(2040804);

    // Henesys PQ
    pub const GREEN_PRIMROSE_SEED: Self = Self(4001095);
    pub const PURPLE_PRIMROSE_SEED: Self = Self(4001096);
    pub const PINK_PRIMROSE_SEED: Self = Self(4001097);
    pub const BROWN_PRIMROSE_SEED: Self = Self(4001098);
    pub const YELLOW_PRIMROSE_SEED: Self = Self(4001099);
    pub const BLUE_PRIMROSE_SEED: Self = Self(4001100);
    pub const MOON_BUNNYS_RICE_CAKE: Self = Self(4001101);

    // Catch mobs items
    pub const PHEROMONE_PERFUME: Self = Self(2270000);
    pub const POUCH: Self = Self(2270001);
    pub const GHOST_SACK: Self = Self(4031830);
    pub const ARPQ_ELEMENT_ROCK: Self = Self(2270002);
    pub const ARPQ_SPIRIT_JEWEL: Self = Self(4031868);
    pub const MAGIC_CANE: Self = Self(2270003);
    pub const TAMED_RUDOLPH: Self = Self(4031887);
    pub const TRANSPARENT_MARBLE_1: Self = Self(2270005);
    pub const MONSTER_MARBLE_1: Self = Self(2109001);
    pub const TRANSPARENT_MARBLE_2: Self = Self(2270006);
    pub const MONSTER_MARBLE_2: Self = Self(2109002);
    pub const TRANSPARENT_MARBLE_3: Self = Self(2270007);
    pub const MONSTER_MARBLE_3: Self = Self(2109003);
    pub const EPQ_PURIFICATION_MARBLE: Self = Self(2270004);
    pub const EPQ_MONSTER_MARBLE: Self = Self(4001169);
    pub const FISH_NET: Self = Self(2270008);
    pub const FISH_NET_WITH_A_CATCH: Self = Self(2022323);

    // Mount
    pub const BATTLESHIP: Self = Self(1932000);

    // Explorer mount
    pub const HOG: Self = Self(1902000);
    pub const SILVER_MANE: Self = Self(1902001);
    pub const RED_DRACO: Self = Self(1902002);
    pub const EXPLORER_SADDLE: Self = Self(1912000);

    // Cygnus mount
    pub const MIMIANA: Self = Self(1902005);
    pub const MIMIO: Self = Self(1902006);
    pub const SHINJOU: Self = Self(1902007);
    pub const CYGNUS_SADDLE: Self = Self(1912005);

    // Dev equips
    pub const GREEN_HEADBAND: Self = Self(1002067);
    pub const TIMELESS_NIBLEHEIM: Self = Self(1402046);
    pub const BLUE_KORBEN: Self = Self(1082140);
    pub const MITHRIL_PLATINE_PANTS: Self = Self(1060091);
    pub const BLUE_CARZEN_BOOTS: Self = Self(1072154);
    pub const MITHRIL_PLATINE: Self = Self(1040103);

    pub const PERMANENT_PETS: [Self; 4] = [
        Self::PERMA_PINK_BEAN,
        Self::PERMA_KINO,
        Self::PERMA_WHITE_TIGER,
        Self::PERMA_MINI_YETI,
    ];

    pub const OWL_ITEMS: [Self; 10] = [
        Self::WORK_GLOVES,
        Self::STEELY_THROWING_KNIVES,
        Self::ILBI_THROWING_STARS,
        Self::OWL_BALL_MASK,
        Self::PINK_ADVENTURER_CAPE,
        Self::CLAW_30_SCROLL,
        Self::WHITE_SCROLL,
        Self::HELMET_60_ACC_SCROLL,
        Self::MAPLE_SHIELD,
        Self::GLOVES_ATT_60_SCROLL,
    ];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade() {
        let claw = ItemId::STEEL_GUARDS;
        let dagger = ItemId::REEF_CLAW;
        let scroll = ItemId::CLAW_30_SCROLL;
        let chaos = ItemId::CHAOS_SCROLL_60;

        assert!(claw.can_upgrade_with(scroll));
        assert!(claw.can_upgrade_with(chaos));
        assert!(!dagger.can_upgrade_with(scroll));
        assert!(dagger.can_upgrade_with(chaos));
    }
}