use shroom_meta::{id::{CharacterId, ItemId, SkillId}, skill::SkillLevel, CharLevel};
use shroom_pkt::{with_opcode, CondOption, ShroomOption8, ShroomPacket, ShroomPacketEnum};

use crate::send_opcodes::SendOpcodes;

#[derive(Debug, ShroomPacket)]
pub struct SkillUseData {
    pub skill_id: SkillId,
    pub lvl: CharLevel,
    pub skill_level: SkillLevel,
    /*
        TODO:
            Drk DarkForce -> bDarkForceActive
            Evan DragonFury -> bDragonFuryActive
            Dual5 Assasinate -> bLeft, mobId
            WH Capture -> capture msg
            WH Summon -> bLeft, ptX, ptY

     */
}

#[derive(Debug, ShroomPacket)]
pub struct SkillAffectedData {
    pub skill_id: SkillId,
    pub lvl: CharLevel,
}

#[derive(Debug, ShroomPacket)]
pub struct SkillAffectedSelectData {
    pub skill_id: SkillId,
    pub lvl: CharLevel,
}

#[derive(Debug, ShroomPacket)]
pub struct SkillSpecialEffectData {
    pub skill_id: SkillId,
    pub lvl: CharLevel,
    /*
            if skill dual5 dual monster bomb ->
            ptX(4),ptY(4), slv(4), unk(4)    
     */
}

#[derive(Debug, ShroomPacket)]
pub struct PetEffectData {
    pub ty: u8,
    pub pet_ix: u8
}

fn is_true(v: &bool) -> bool {
    *v
}

#[derive(Debug, ShroomPacket)]
pub struct ProtectOnDieItemEffectData {
    pub use_item: bool,
    pub day: u8,
    pub times: u8,
    #[pkt(check(field = "use_item", cond = "is_true"))]
    pub item_id: CondOption<ItemId>
}

#[derive(Debug, ShroomPacket)]
pub struct QuestEffectData {
    pub quest_info: u8,
    pub item_id: ItemId,
    pub quest_item_fmt: u32,
    pub name: String,
    pub effect: u32
}

#[derive(Debug, ShroomPacket)]
pub struct LotteryUseEffectData {
    pub item_id: ItemId,
    /// path is played if succcesful
    pub path: ShroomOption8<String>
}

#[derive(Debug, ShroomPacket)]
pub struct AvatarOrientedData {
    pub path: String,
    pub unused: u8
}

#[derive(Debug, ShroomPacket)]
pub struct IncubatorUseData {
    pub item_id: ItemId,
    pub path: String
}


#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum UserEffect {
    LevelUp(()) = 0,
    SkillUse(SkillUseData) = 1,
    SkillAffected(SkillAffectedData) = 2,
    SkillAffectedSelect(SkillAffectedSelectData) = 3,
    SkillAffectedSpecial(SkillAffectedData) = 4,
    Quest(QuestEffectData) = 5,
    PetShowEffect(PetEffectData) = 6,
    ShowSkillSpecialEffect(SkillSpecialEffectData) = 7,
    ProtectOnDieItemUse(ProtectOnDieItemEffectData) = 8,
    PortalSoundEffect(()) = 9,
    JobChanged(()) = 10,
    QuestComplete(()) = 11,
    /// Delta
    IncDecHPEffect(i8) = 12,
    BuffItemEffect(ItemId) = 13,
    SquibEffect(String) = 14,
    MonsterBookCard(()) = 15,
    LotteryUse(LotteryUseEffectData) = 16,
    ItemLevelUp(()) = 17,
    /// Fail(1), Success(0)
    ItemMaker(u32) = 18,
    ExpItemConsumed(()) = 19,
    /// Path
    ReservedEffect(String) = 0x14,
    Buff(()) = 21,
    ConsumeEffect(ItemId) = 22,
    /// Wheels left
    UpgradeTombItemUse(u8) = 23,
    BattlefieldItemUse(String) = 24,
    AvatarOriented(AvatarOrientedData) = 25,
    IncubatorUse(IncubatorUseData) = 26,
    PlaySoundWithMuteBgm(String) = 27,
    SoulStoneUse(()) = 28,
    /// Delta
    MakeIncDecHPEffect(i32) = 29,
    DeliveryQuestItemUse(u32) = 30,
    RepeatEffectRemove(()) = 31,
    EvolRing(()) = 32,
}

#[derive(Debug, ShroomPacket)]
pub struct LocalUserEffectResp(pub UserEffect);
with_opcode!(LocalUserEffectResp, SendOpcodes::UserEffectLocal);

#[derive(Debug, ShroomPacket)]
pub struct RemoteUserEffect {
    pub char_id: CharacterId,
    pub effect: UserEffect
}
with_opcode!(RemoteUserEffect, SendOpcodes::UserEffectRemote);


#[derive(Debug, Default, ShroomPacket)]
pub struct ItemUpgradeEffectResp {
    pub char_id: CharacterId,
    pub success: bool,
    pub destroyed: bool,
    pub enchant_skill: bool,
    pub enchant_category: u32,
    pub white_scroll: bool,
    pub recoverable: bool
}
with_opcode!(ItemUpgradeEffectResp, SendOpcodes::UserItemUpgradeEffect);

#[derive(Debug, Default, ShroomPacket)]
pub struct ItemHyperUpgradeEffectResp {
    pub char_id: CharacterId,
    pub success: bool,
    pub destroyed: bool,
    pub enchant_skill: bool,
    pub enchant_category: u32
}
with_opcode!(ItemHyperUpgradeEffectResp, SendOpcodes::UserItemHyperUpgradeEffect);