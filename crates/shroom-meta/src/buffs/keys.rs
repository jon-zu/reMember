#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum CharBuffKey {
    Pad = 0,
    Pdd = 1,
    Mad = 2,
    Mdd = 3,
    Acc = 4,
    Evasion = 5,
    CriticalRate = 6,
    Speed = 7,
    Jump = 8,
    ExtraMaxHp = 0x5D,
    ExtraMaxMp = 0x5E,
    ExtraPad = 0x5F,
    ExtraPdd = 0x60,
    ExtraMdd = 0x61,
    MagicGuard = 9,
    DarkSight = 0xa,
    Booster = 0xb,
    PowerGuard = 0xc,
    Guard = 0x62,
    SafetyDamage = 0x63,
    SafetyAbsorb = 0x64,
    MaxHp = 0xd,
    MaxMp = 0xe,
    Invincible = 0xf,
    SoulArrow = 0x10,
    Stun = 0x11,
    Poison = 0x12,
    Seal = 0x13,
    Darkness = 0x14,
    ComboCounter = 0x15,
    WeaponCharge = 0x16,
    DragonBlood = 0x17,
    HolySymbol = 0x18,
    MesoUp = 0x19,
    ShadowPartner = 0x1A,
    PickPocket = 0x1B,
    MesoGuard = 0x1C,
    Thaw = 0x1D,
    Weakness = 0x1E,
    Curse = 0x1F,
    Slow = 0x20, // Done
    Morph = 0x21,
    Ghost = 0x31,          // ghost morph
    Regen = 0x22,          // recovery
    BasicStatUp = 0x23,    // shroom warrior
    Stance = 0x24,         // Done
    SharpEyes = 0x25,      // Done
    ManaReflection = 0x26, // Done
    Attract = 0x27,        // seduce
    SpiritJavelin = 0x28,  // shadow claw
    Infinity = 0x29,       // Done
    Holyshield = 0x2A,     // Done
    HamString = 0x2B,      // Done
    Blind = 0x2C,          // Done
    Concentration = 0x2D,  // Done
    BanMap = 0x2E,
    MaxLevelBuff = 0x2F, // echo of hero
    Barrier = 0x32,
    DojangShield = 0x3E,
    ReverseInput = 0x33, // confuse
    MesoUpByItem = 0x30, // Done
    ItemUpByItem = 0x34, // Done
    RespectPImmune = 0x35,
    RespectMImmune = 0x36,
    DefenseAtt = 0x37,
    DefenseState = 0x38,
    DojangBerserk = 0x3B,    // berserk fury
    DojangInvincible = 0x3C, // divine body
    Spark = 0x3D,            // Done
    SoulMasterFinal = 0x3F,  // Done ?
    WindBreakerFinal = 0x40, // Done ?
    ElementalReset = 0x41,   // Done
    WindWalk = 0x42,         // Done
    EventRate = 0x43,
    ComboAbilityBuff = 0x44, // aran combo
    ComboDrain = 0x45,       // Done
    ComboBarrier = 0x46,     // Done
    BodyPressure = 0x47,     // Done
    SmartKnockback = 0x48,   // Done
    RepeatEffect = 0x49,
    ExpBuffRate = 0x4A, // Done
    IncEffectHPPotion = 0x39,
    IncEffectMPPotion = 0x3A,
    StopPortion = 0x4B,
    StopMotion = 0x4C,
    Fear = 0x4D,            // debuff done
    EvanSlow = 0x4E,        // Done
    MagicShield = 0x4F,     // Done
    MagicResistance = 0x50, // Done
    SoulStone = 0x51,
    Flying = 0x52,
    Frozen = 0x53,
    AssistCharge = 0x54,
    Enrage = 0x55, //mirror imaging
    SuddenDeath = 0x56,
    NotDamaged = 0x57,
    FinalCut = 0x58,
    ThornsEffect = 0x59,
    SwallowAttackDamage = 0x5A,
    MorewildDamageUp = 0x5B,
    Mine = 0x5C,
    Cyclone = 0x65,
    SwallowCritical = 0x66,
    SwallowMaxMP = 0x67,
    SwallowDefence = 0x68,
    SwallowEvasion = 0x69,
    Conversion = 0x6A,
    Revive = 0x6B, // summon reaper buff
    Sneak = 0x6C,
    Mechanic = 0x6D,
    Aura = 0x6E,
    DarkAura = 0x6F,
    BlueAura = 0x70,
    YellowAura = 0x71,
    SuperBody = 0x72, // body boost
    MorewildMaxHP = 0x73,
    Dice = 0x74,
    BlessingArmor = 0x75, // Paladin Divine Shield
    DamR = 0x76,
    TeleportMasteryOn = 0x77,
    CombatOrders = 0x78,
    Beholder = 0x79,
    EnergyCharged = 0x7A, // TODO What does >= 10_000 mean
    DashSpeed = 0x7B,
    DashJump = 0x7C,
    RideVehicle = 0x7D,
    PartyBooster = 0x7E,
    GuidedBullet = 0x7F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum MobBuffKey {
    Pad = 0,
    Pdr = 1,
    Mad = 2,
    Mdr = 3,
    Acc = 4,
    Eva = 5,
    Speed = 6,
    Stun = 7,
    Freeze = 8,
    Poison = 9,
    Seal = 10,
    Darkness = 11,
    PowerUp = 12,
    MagicUp = 13,
    PGuardUp = 14,
    MGuardUp = 15,
    Doom = 16,
    Web = 17,
    PImmune = 18,
    MImmune = 19,
    HardSkin = 21,
    Ambush = 22,
    Venom = 24,
    Blind = 25,
    SealSkill = 26,
    Dazzle = 28,
    PCounter = 29,
    MCounter = 30,
    RiseByToss = 32,
    BodyPressure = 33,
    Weakness = 34,
    TimeBomb = 35,
    Showdown = 20,
    MagicCrash = 36,
    DamagedElemAttr = 23,
    HealByDamage = 37,
    Burned = 27,
    Disable = 31,
}

impl MobBuffKey {
    pub fn movement_affecting_keys() -> [MobBuffKey; 5] {
        [
            MobBuffKey::Speed,
            MobBuffKey::Stun,
            MobBuffKey::Freeze,
            MobBuffKey::Doom,
            MobBuffKey::RiseByToss,
        ]
    }

    pub fn is_movement_affecting(&self) -> bool {
        matches!(self, MobBuffKey::Speed
            | MobBuffKey::Stun
            | MobBuffKey::Freeze
            | MobBuffKey::Doom
            | MobBuffKey::RiseByToss)
    }
}
