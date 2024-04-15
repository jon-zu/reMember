use shroom_meta::id::Money;
use shroom_pkt::{partial_data, ShroomExpirationTime, ShroomIndexListZ8, ShroomList16};

use crate::shared::{
    char::{
        CharDataEquipped, CharDataStat, InventorySize, MiniGameInfo, NewYearCardInfo,
        QuestCompleteInfo, QuestCompleteOldInfo, QuestInfo, QuestRecordExpired, SkillCooltime,
        SkillInfo, SocialRecords, TeleportRockInfo, VisitorQuestLogInfo,
    },
    item::Item,
};

partial_data!(
    CharData,
    CharDataFlags,
    u64,
    derive(Debug),
    Stat(CharDataStat) => 1 << 0,
    Money(Money) => 1 << 1,
    InvSize(InventorySize) => 1 << 7,
    EquipExtSlotExpiration(ShroomExpirationTime) => 1 << 20,
    Equipped(CharDataEquipped) => 1 << 2,
    ConsumeInv(ShroomIndexListZ8<Item>) => 1 << 3,
    SetupInv(ShroomIndexListZ8<Item>) => 1 << 4,
    EtcInv(ShroomIndexListZ8<Item>) => 1 << 5,
    CashInv(ShroomIndexListZ8<Item>) => 1 << 6,
    // InvSize 1 << 7
    SkillRecords(ShroomList16<SkillInfo>) => 1 << 8,
    SkllCooltime(ShroomList16<SkillCooltime>) => 1 << 15,
    Quests(ShroomList16<QuestInfo>) => 1 << 9,
    QuestsCompleted(ShroomList16<QuestCompleteInfo>) => 1 << 14,
    MiniGameRecords(ShroomList16<MiniGameInfo>) => 1 << 10,
    SocialRecords(SocialRecords) => 1 << 11,
    TeleportRockInfo(TeleportRockInfo) => 1 << 12,
    // Avatar 1 << 13
    // QuestsCompleted 1 << 14
    // SkillCooltimes 1 << 15
    // Monsterbook Card 1 << 16
    // Monster Book Cover  1 << 17
    NewYearCards(ShroomList16<NewYearCardInfo>) => 1 << 18,
    QuestRecordsExpired(ShroomList16<QuestRecordExpired>) => 1 << 19,
    // EquipExtExpire 1 << 20
    //TODO this has to be optional in the all struct, bneed to implement this later 1 << somehow
    // this only affects the all struct, partial struct can opt to not encode 1 << it
    //WILD_HUNTER_INFO(WildHunterInfo) => 1 << 21,
    QuestCompleteOld(ShroomList16<QuestCompleteOldInfo>) => 1 << 22,
    VisitorQuestLogInfo(ShroomList16<VisitorQuestLogInfo>) => 1 << 23,

);