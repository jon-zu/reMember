use num_enum::{IntoPrimitive, TryFromPrimitive};
use shroom_pkt::ShroomOpCode;

impl ShroomOpCode for SendOpcodes {}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum SendOpcodes {
    CheckPasswordResult = 0x0,
    GuestIDLoginResult = 0x1,
    AccountInfoResult = 0x2,
    CheckUserLimitResult = 0x3,
    SetAccountResult = 0x4,
    ConfirmEULAResult = 0x5,
    CheckPinCodeResult = 0x6,
    UpdatePinCodeResult = 0x7,
    ViewAllCharResult = 0x8,
    SelectCharacterByVACResult = 0x9,
    WorldInformation = 0xa,
    SelectWorldResult = 0xb,
    SelectCharacterResult = 0xc,
    CheckDuplicatedIDResult = 0xd,
    CreateNewCharacterResult = 0xe,
    DeleteCharacterResult = 0xf,
    MigrateCommand = 0x10,
    AliveReq = 0x11,
    AuthenCodeChanged = 0x12,
    AuthenMessage = 0x13,
    SecurityPacket = 0x14,
    EnableSPWResult = 0x15,
    DeleteCharacterOTPRequest = 0x16,
    CheckCrcResult = 0x17,
    LatestConnectedWorld = 0x18,
    RecommendWorldMessage = 0x19,
    CheckExtraCharInfoResult = 0x1a,
    CheckSPWResult = 0x1b,
    InventoryOperation = 0x1c,
    InventoryGrow = 0x1d,
    StatChanged = 0x1e,
    TemporaryStatSet = 0x1f,
    TemporaryStatReset = 0x20,
    ForcedStatSet = 0x21,
    ForcedStatReset = 0x22,
    ChangeSkillRecordResult = 0x23,
    SkillUseResult = 0x24,
    GivePopularityResult = 0x25,
    Message = 0x26,
    SendOpenFullClientLink = 0x27,
    MemoResult = 0x28,
    MapTransferResult = 0x29,
    AntiMacroResult = 0x2a,
    InitialQuizStart = 0x2b,
    ClaimResult = 0x2c,
    SetClaimSvrAvailableTime = 0x2d,
    ClaimSvrStatusChanged = 0x2e,
    SetTamingMobInfo = 0x2f,
    QuestClear = 0x30,
    EntrustedShopCheckResult = 0x31,
    SkillLearnItemResult = 0x32,
    SkillResetItemResult = 0x33,
    GatherItemResult = 0x34,
    SortItemResult = 0x35,
    RemoteShopOpenResult = 0x36,
    SueCharacterResult = 0x37,
    MigrateToCashShopResult = 0x38,
    TradeMoneyLimit = 0x39,
    SetGender = 0x3a,
    GuildBBS = 0x3b,
    PetDeadMessage = 0x3c,
    CharacterInfo = 0x3d,
    PartyResult = 0x3e,
    ExpeditionRequest = 0x3f,
    ExpeditionNoti = 0x40,
    FriendResult = 0x41,
    GuildRequest = 0x42,
    GuildResult = 0x43,
    AllianceResult = 0x44,
    TownPortal = 0x45,
    OpenGate = 0x46,
    BroadcastMsg = 0x47,
    IncubatorResult = 0x48,
    ShopScannerResult = 0x49,
    ShopLinkResult = 0x4a,
    MarriageRequest = 0x4b,
    MarriageResult = 0x4c,
    WeddingGiftResult = 0x4d,
    MarriedPartnerMapTransfer = 0x4e,
    CashPetFoodResult = 0x4f,
    SetWeekEventMessage = 0x50,
    SetPotionDiscountRate = 0x51,
    BridleMobCatchFail = 0x52,
    ImitatedNPCResult = 0x53,
    ImitatedNPCData = 0x54,
    LimitedNPCDisableInfo = 0x55,
    MonsterBookSetCard = 0x56,
    MonsterBookSetCover = 0x57,
    HourChanged = 0x58,
    MiniMapOnOff = 0x59,
    ConsultAuthkeyUpdate = 0x5a,
    ClassCompetitionAuthkeyUpdate = 0x5b,
    WebBoardAuthkeyUpdate = 0x5c,
    SessionValue = 0x5d,
    PartyValue = 0x5e,
    FieldSetVariable = 0x5f,
    BonusExpRateChanged = 0x60,
    PotionDiscountRateChanged = 0x61,
    FamilyChartResult = 0x62,
    FamilyInfoResult = 0x63,
    FamilyResult = 0x64,
    FamilyJoinRequest = 0x65,
    FamilyJoinRequestResult = 0x66,
    FamilyJoinAccepted = 0x67,
    FamilyPrivilegeList = 0x68,
    FamilyFamousPointIncResult = 0x69,
    FamilyNotifyLoginOrLogout = 0x6a,
    FamilySetPrivilege = 0x6b,
    FamilySummonRequest = 0x6c,
    NotifyLevelUp = 0x6d,
    NotifyWedding = 0x6e,
    NotifyJobChange = 0x6f,
    IncRateChanged = 0x70,
    ShroomTVUseRes = 0x71,
    AvatarMegaphoneRes = 0x72,
    AvatarMegaphoneUpdateMessage = 0x73,
    AvatarMegaphoneClearMessage = 0x74,
    CancelNameChangeResult = 0x75,
    CancelTransferWorldResult = 0x76,
    DestroyShopResult = 0x77,
    FAKEGMNOTICE = 0x78,
    SuccessInUseGachaponBox = 0x79,
    NewYearCardRes = 0x7a,
    RandomMorphRes = 0x7b,
    CancelNameChangeByOther = 0x7c,
    SetBuyEquipExt = 0x7d,
    SetPassenserRequest = 0x7e,
    ScriptProgressMessage = 0x7f,
    DataCRCCheckFailed = 0x80,
    CakePieEventResult = 0x81,
    UpdateGMBoard = 0x82,
    ShowSlotMessage = 0x83,
    WildHunterInfo = 0x84,
    AccountMoreInfo = 0x85,
    FindFirend = 0x86,
    StageChange = 0x87,
    DragonBallBox = 0x88,
    AskUserWhetherUsePamsSong = 0x89,
    TransferChannel = 0x8a,
    DisallowedDeliveryQuestList = 0x8b,
    MacroSysDataInit = 0x8c,
    SetField = 0x8d,
    SetITC = 0x8e,
    SetCashShop = 0x8f,
    SetBackgroundEffect = 0x90,
    SetMapObjectVisible = 0x91,
    ClearBackgroundEffect = 0x92,
    TransferFieldReqIgnored = 0x93,
    TransferChannelReqIgnored = 0x94,
    FieldSpecificData = 0x95,
    GroupMessage = 0x96,
    Whisper = 0x97,
    CoupleMessage = 0x98,
    MobSummonItemUseResult = 0x99,
    FieldEffect = 0x9a,
    FieldObstacleOnOff = 0x9b,
    FieldObstacleOnOffStatus = 0x9c,
    FieldObstacleAllReset = 0x9d,
    BlowWeather = 0x9e,
    PlayJukeBox = 0x9f,
    AdminResult = 0xa0,
    FieldQuiz = 0xa1,
    Desc = 0xa2,
    Clock = 0xa3,
    CONTIMOVE = 0xa4,
    CONTISTATE = 0xa5,
    SetQuestClear = 0xa6,
    SetQuestTime = 0xa7,
    Warn = 0xa8,
    SetObjectState = 0xa9,
    DestroyClock = 0xaa,
    ShowArenaResult = 0xab,
    StalkResult = 0xac,
    MassacreIncGauge = 0xad,
    MassacreResult = 0xae,
    QuickslotMappedInit = 0xaf,
    FootHoldInfo = 0xb0,
    RequestFootHoldInfo = 0xb1,
    FieldKillCount = 0xb2,
    UserEnterField = 0xb3,
    UserLeaveField = 0xb4,
    UserChat = 0xb5,
    UserChatNLCPQ = 0xb6,
    UserADBoard = 0xb7,
    UserMiniRoomBalloon = 0xb8,
    UserConsumeItemEffect = 0xb9,
    UserItemUpgradeEffect = 0xba,
    UserItemHyperUpgradeEffect = 0xbb,
    UserItemOptionUpgradeEffect = 0xbc,
    UserItemReleaseEffect = 0xbd,
    UserItemUnreleaseEffect = 0xbe,
    UserHitByUser = 0xbf,
    UserTeslaTriangle = 0xc0,
    UserFollowCharacter = 0xc1,
    UserShowPQReward = 0xc2,
    UserSetPhase = 0xc3,
    SetPortalUsable = 0xc4,
    //ShowPamsSongResSult
    UserShowRecoverUpgradeCountEffect = 0xc5,
    PetActivated = 0xc6,
    PetEvol = 0xc7,
    PetTransferField = 0xc8,
    PetMove = 0xc9,
    PetAction = 0xca,
    PetNameChanged = 0xcb,
    PetLoadExceptionList = 0xcc,
    PetActionCommand = 0xcd,
    DragonEnterField = 0xce,
    DragonMove = 0xcf,
    DragonLeaveField = 0xd0,
    UserMove = 0xd2,
    UserMeleeAttack = 0xd3,
    UserShootAttack = 0xd4,
    UserMagicAttack = 0xd5,
    UserBodyAttack = 0xd6,
    UserSkillPrepare = 0xd7,
    UserMovingShootAttackPrepare = 0xd8,
    UserSkillCancel = 0xd9,
    UserHit = 0xda,
    UserEmotion = 0xdb,
    UserSetActiveEffectItem = 0xdc,
    UserShowUpgradeTombEffect = 0xdd,
    UserSetActivePortableChair = 0xde,
    UserAvatarModified = 0xdf,
    UserEffectRemote = 0xe0,
    UserTemporaryStatSet = 0xe1,
    UserTemporaryStatReset = 0xe2,
    UserHP = 0xe3,
    UserGuildNameChanged = 0xe4,
    UserGuildMarkChanged = 0xe5,
    UserThrowGrenade = 0xe6,
    UserSitResult = 0xe7,
    UserEmotionLocal = 0xe8,
    UserEffectLocal = 0xe9,
    UserTeleport = 0xea,
    Premium = 0xeb,
    MesoGiveSucceeded = 0xec,
    MesoGiveFailed = 0xed,
    RandomMesobagSucceed = 0xee,
    RandomMesobagFailed = 0xef,
    FieldFadeInOut = 0xf0,
    FieldFadeOutForce = 0xf1,
    UserQuestResult = 0xf2,
    NotifyHPDecByField = 0xf3,
    UserPetSkillChanged = 0xf4,
    UserBalloonMsg = 0xf5,
    PlayEventSound = 0xf6,
    PlayMinigameSound = 0xf7,
    UserMakerResult = 0xf8,
    UserOpenConsultBoard = 0xf9,
    UserOpenClassCompetitionPage = 0xfa,
    UserOpenUI = 0xfb,
    UserOpenUIWithOption = 0xfc,
    SetDirectionMode = 0xfd,
    SetStandAloneMode = 0xfe,
    UserHireTutor = 0xff,
    UserTutorMsg = 0x100,
    IncCombo = 0x101,
    UserRandomEmotion = 0x102,
    ResignQuestReturn = 0x103,
    PassMateName = 0x104,
    SetRadioSchedule = 0x105,
    UserOpenSkillGuide = 0x106,
    UserNoticeMsg = 0x107,
    UserChatMsg = 0x108,
    UserBuffzoneEffect = 0x109,
    UserGoToCommoditySN = 0x10a,
    UserDamageMeter = 0x10b,
    UserTimeBombAttack = 0x10c,
    UserPassiveMove = 0x10d,
    UserFollowCharacterFailed = 0x10e,
    UserRequestVengeance = 0x10f,
    UserRequestExJablin = 0x110,
    UserAskAPSPEvent = 0x111,
    QuestGuideResult = 0x112,
    UserDeliveryQuest = 0x113,
    SkillCooltimeSet = 0x114,
    SummonedEnterField = 0x116,
    SummonedLeaveField = 0x117,
    SummonedMove = 0x118,
    SummonedAttack = 0x119,
    SummonedSkill = 0x11a,
    SummonedHit = 0x11b,
    MobEnterField = 0x11c,
    MobLeaveField = 0x11d,
    MobChangeController = 0x11e,
    MobMove = 0x11f,
    MobCtrlAck = 0x120,
    MobCtrlHint = 0x121,
    MobStatSet = 0x122,
    MobStatReset = 0x123,
    MobSuspendReset = 0x124,
    MobAffected = 0x125,
    MobDamaged = 0x126,
    MobSpecialEffectBySkill = 0x127,
    MobHPChange = 0x128,
    MobCrcKeyChanged = 0x129,
    MobHPIndicator = 0x12a,
    MobCatchEffect = 0x12b,
    MobEffectByItem = 0x12c,
    MobSpeaking = 0x12d,
    MobChargeCount = 0x12e,
    MobSkillDelay = 0x12f,
    MobRequestResultEscortInfo = 0x130,
    MobEscortStopEndPermmision = 0x131,
    MobEscortStopSay = 0x132,
    MobEscortReturnBefore = 0x133,
    MobNextAttack = 0x134,
    MobAttackedByMob = 0x135,
    NpcEnterField = 0x137,
    NpcLeaveField = 0x138,
    NpcChangeController = 0x139,
    NpcMove = 0x13a,
    NpcUpdateLimitedInfo = 0x13b,
    NpcSpecialAction = 0x13c,
    NpcSetScript = 0x13d,
    EmployeeEnterField = 0x13f,
    EmployeeLeaveField = 0x140,
    EmployeeMiniRoomBalloon = 0x141,
    DropEnterField = 0x142,
    DropReleaseAllFreeze = 0x143,
    DropLeaveField = 0x144,
    CreateMessgaeBoxFailed = 0x145,
    MessageBoxEnterField = 0x146,
    MessageBoxLeaveField = 0x147,
    AffectedAreaCreated = 0x148,
    AffectedAreaRemoved = 0x149,
    TownPortalCreated = 0x14a,
    TownPortalRemoved = 0x14b,
    OpenGateCreated = 0x14c,
    OpenGateRemoved = 0x14d,
    ReactorChangeState = 0x14e,
    ReactorMove = 0x14f,
    ReactorEnterField = 0x150,
    ReactorLeaveField = 0x151,
    SnowBallState = 0x152,
    SnowBallHit = 0x153,
    SnowBallMsg = 0x154,
    SnowBallTouch = 0x155,
    CoconutHit = 0x156,
    CoconutScore = 0x157,
    HealerMove = 0x158,
    PulleyStateChange = 0x159,
    MCarnivalEnter = 0x15a,
    MCarnivalPersonalCP = 0x15b,
    MCarnivalTeamCP = 0x15c,
    MCarnivalResultSuccess = 0x15d,
    MCarnivalResultFail = 0x15e,
    MCarnivalDeath = 0x15f,
    MCarnivalMemberOut = 0x160,
    MCarnivalGameResult = 0x161,
    ArenaScore = 0x162,
    BattlefieldEnter = 0x163,
    BattlefieldScore = 0x164,
    BattlefieldTeamChanged = 0x165,
    WitchtowerScore = 0x166,
    HontaleTimer = 0x167,
    ChaosZakumTimer = 0x168,
    HontailTimer = 0x169,
    ZakumTimer = 0x16a,
    ScriptMessage = 0x16b,
    OpenShopDlg = 0x16c,
    ShopResult = 0x16d,
    AdminShopResult = 0x16e,
    AdminShopCommodity = 0x16f,
    TrunkResult = 0x170,
    StoreBankGetAllResult = 0x171,
    StoreBankResult = 0x172,
    RPSGame = 0x173,
    Messenger = 0x174,
    MiniRoom = 0x175,
    Tournament = 0x176,
    TournamentMatchTable = 0x177,
    TournamentSetPrize = 0x178,
    TournamentNoticeUEW = 0x179,
    TournamentAvatarInfo = 0x17a,
    WeddingProgress = 0x17b,
    WeddingCremonyEnd = 0x17c,
    Parcel = 0x17d,
    CashShopChargeParamResult = 0x17e,
    CashShopQueryCashResult = 0x17f,
    CashShopCashItemResult = 0x180,
    CashShopPurchaseExpChanged = 0x181,
    CashShopGiftMateInfoResult = 0x182,
    CashShopCheckDuplicatedIDResult = 0x183,
    CashShopCheckNameChangePossibleResult = 0x184,
    CashShopRegisterNewCharacterResult = 0x185,
    CashShopCheckTransferWorldPossibleResult = 0x186,
    CashShopGachaponStampItemResult = 0x187,
    CashShopCashItemGachaponResult = 0x188,
    CashShopCashGachaponOpenResult = 0x189,
    ChangeShroomPointResult = 0x18a,
    CashShopOneADay = 0x18b,
    CashShopNoticeFreeCashItem = 0x18c,
    CashShopMemberShopResult = 0x18d,
    FuncKeyMappedInit = 0x18e,
    PetConsumeItemInit = 0x18f,
    PetConsumeMPItemInit = 0x190,
    CheckSSN2OnCreateNewCharacterResult = 0x191,
    CheckSPWOnCreateNewCharacterResult = 0x192,
    FirstSSNOnCreateNewCharacterResult = 0x193,
    ShroomTVUpdateMessage = 0x195,
    ShroomTVClearMessage = 0x196,
    ShroomTVSendMessageResult = 0x197,
    BroadSetFlashChangeEvent = 0x198,
    ITCChargeParamResult = 0x19a,
    ITCQueryCashResult = 0x19b,
    ITCNormalItemResult = 0x19c,
    CheckDuplicatedIDResultInCS = 0x19d,
    CreateNewCharacterResultInCS = 0x19e,
    CreateNewCharacterFailInCS = 0x19f,
    CharacterSale = 0x1a0,
    GoldHammereS = 0x1a1,
    GoldHammerResult = 0x1a2,
    GoldHammereE = 0x1a3,
    BattleRecordS = 0x1a4,
    BattleRecordDotDamageInfo = 0x1a5,
    BattleRecordRequestResult = 0x1a6,
    ItemUpgradeResult = 0x1a9,
    ItemUpgradeFail = 0x1aa,
    VegaResult = 0x1ad,
    VegaFail = 0x1ae,
    LogoutGift = 0x1b0,
}