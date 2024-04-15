use shroom_pkt::{
    partial::PartialFlag, partial_data, with_opcode, CondEither, CondOption, ShroomDurationMs16,
    ShroomExpirationTime, ShroomIndexList8, ShroomIndexListZ16, ShroomList16, ShroomList32,
    ShroomOption8, ShroomPacket, ShroomTime,
};

use crate::{
    game::{
        user::secondary_stats::{CharSecondaryStatFlags, CharSecondaryStatPartial}, /*user::secondary_stats::{CharSecondaryStatPartial, CharSecondaryStatFlags},*/
    },
    send_opcodes::SendOpcodes,
};

use shroom_meta::id::{
    job_id::{JobId, SubJob},
    CashID, CharacterId, FaceId, FieldId, HairId, ItemId, MobId, QuestId, SkillId, Skin,
};

use super::{item::Item, job::Job, Gender, NameStr};

const CHAR_PET_COUNT: usize = 3;
pub type PetIds = [ItemId; CHAR_PET_COUNT];
//TODO:
pub type Pets = [u64; CHAR_PET_COUNT];
pub type PetCashIds = [CashID; CHAR_PET_COUNT];

#[derive(ShroomPacket, Debug)]
pub struct SkillPointPage {
    pub index: u8,
    pub value: u8,
}

pub type SkillPointPages = [SkillPointPage; 10];

#[derive(ShroomPacket, Debug)]
pub struct CharStat {
    pub char_id: CharacterId,
    pub name: NameStr,
    pub gender: Gender,
    pub skin_color: Skin,
    pub face: FaceId,
    pub hair: HairId,
    pub pets: Pets,
    pub level: u8,
    pub job_id: JobId,
    pub str: u16,
    pub dex: u16,
    pub int: u16,
    pub luk: u16,
    pub hp: u32,
    pub max_hp: u32,
    pub mp: u32,
    pub max_mp: u32,
    pub ap: u16,
    #[pkt(either(field = "job_id", cond = "JobId::has_extended_sp"))]
    pub sp: CondEither<SkillPointPages, u16>,
    pub exp: i32,
    pub fame: u16,
    pub tmp_exp: u32,
    pub map_id: FieldId,
    pub portal: u8,
    // TODO: Is this playtime in seconds
    pub playtime: u32,
    pub sub_job: SubJob,
}

impl CharStat {
    pub fn get_job(&self) -> Job {
        Job::new(self.job_id, self.sub_job)
    }

    pub fn set_job(&mut self, job: Job) {
        //TODO: maybe define a transparent mapping layer like
        // like ignoring to (de)serialiaze job and allow mapping fields
        // for job_id, sub_job which reference to the job field
        self.job_id = job.job_id;
        self.sub_job = job.sub_job;
    }
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct AvatarEquips {
    pub equips: ShroomIndexList8<ItemId>,
    pub masked_equips: ShroomIndexList8<ItemId>,
    pub weapon_sticker_id: ItemId,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct AvatarData {
    pub gender: Gender,
    pub skin: Skin,
    pub face: FaceId,
    pub mega: bool,
    pub hair: HairId,
    pub equips: AvatarEquips,
    pub pets: PetIds,
}

#[derive(Debug, ShroomPacket)]
pub struct CharExpirationData {
    u1: u8,
    /// Ignored
    remove_sn: ShroomList32<u64>,
    remove_sn_time: ShroomList32<ShroomTime>,
}

#[derive(Debug, ShroomPacket)]
pub struct SkillInfo {
    pub id: SkillId,
    pub level: u32,
    pub expiration: ShroomExpirationTime,
    #[pkt(check(field = "id", cond = "SkillId::has_master_level"))]
    pub master_level: CondOption<u32>,
}

#[derive(Debug, ShroomPacket)]
pub struct SkillCooltime {
    pub id: SkillId,
    pub time_left: ShroomDurationMs16,
}

/*
limits:
 class ZRef<GW_ItemSlotBase> aEquipped[0x3c];
    class ZRef<GW_ItemSlotBase> aEquipped2[0x3c];
    class ZRef<GW_ItemSlotBase> aDragonEquipped[0x4];
    class ZRef<GW_ItemSlotBase> aMechanicEquipped[0x5];
    class ZArray<ZRef<GW_ItemSlotBase> > aaItemSlot[0x6];


*/

#[derive(Debug, ShroomPacket)]
pub struct QuestInfo {
    pub id: QuestId,
    pub value: String,
}

#[derive(Debug, ShroomPacket)]
pub struct QuestCompleteInfo {
    pub id: QuestId,
    pub time: ShroomTime,
}

#[derive(Debug, ShroomPacket)]
pub struct MiniGameInfo {
    pub game_id: u32,
    pub win: u32,
    pub draw: u32,
    pub score: u32,
    pub u1: u32,
}

pub type CharId = u32;

#[derive(Debug, ShroomPacket)]
pub struct CoupleRecord {
    pub pair_char_id: CharId,
    pub pair_char_name: NameStr,
    pub sn: CashID,
    pub pair_sn: CashID,
}

#[derive(Debug, ShroomPacket)]
pub struct FriendRecord {
    pub pair_char_id: CharId,
    pub pair_char_name: NameStr,
    pub sn: CashID,
    pub pair_sn: CashID,
    pub friend_item_id: ItemId,
}

#[derive(Debug, ShroomPacket)]
pub struct MarriageRecord {
    pub marriage_no: u32,
    pub groom_id: CharId,
    pub bride_id: CharId,
    pub status: u16, // 3 == married?
    pub groom_item_id: ItemId,
    pub bride_item_id: ItemId,
    pub groom_name: NameStr,
    pub bride_name: NameStr,
}

#[derive(Debug, ShroomPacket, Default)]
pub struct SocialRecords {
    pub couple_records: ShroomList16<CoupleRecord>,
    pub friend_records: ShroomList16<FriendRecord>,
    pub marriage_records: ShroomList16<MarriageRecord>,
}

#[derive(Debug, ShroomPacket, Default)]
pub struct TeleportRockInfo {
    //TODO allow MapID
    pub maps: [FieldId; 5],
    pub vip_maps: [FieldId; 10],
}

#[derive(Debug, ShroomPacket)]
pub struct NewYearCardInfo {
    pub id: u32, //sn
    pub sender_id: CharId,
    pub sender_name: String,
    pub is_sender_discarded: bool,
    pub data_sent: ShroomTime,
    pub receiver_id: CharId,
    pub receiver_name: String,
    pub is_receiver_discarded: bool,
    pub is_receiver_received: bool,
    pub date_deceived: ShroomTime,
    pub content: String,
}

#[derive(Debug, ShroomPacket)]
pub struct QuestRecordExpired {
    pub id: QuestId,
    pub value: String,
}

#[derive(Debug, ShroomPacket, Default)]
pub struct WildHunterInfo {
    //TODO proper typing
    pub riding_ty_id: u8,
    pub captured_mobs: [MobId; 5],
}

#[derive(Debug, ShroomPacket)]
pub struct QuestCompleteOldInfo {
    pub id: QuestId,
    pub time: ShroomTime,
}

#[derive(Debug, ShroomPacket)]
pub struct VisitorQuestLogInfo {
    pub id: QuestId,
    pub unknown: u16,
}

#[derive(ShroomPacket, Debug)]
pub struct CharDataStat {
    pub stat: CharStat,
    pub friend_max: u8,
    pub linked_character: ShroomOption8<String>,
}

#[derive(ShroomPacket, Debug, Default)]
pub struct CharDataEquipped {
    pub equipped: ShroomIndexListZ16<Item>,
    pub equipped_cash: ShroomIndexListZ16<Item>,
    pub equip: ShroomIndexListZ16<Item>,
    pub dragon_equipped: ShroomIndexListZ16<Item>,
    pub mechanic_equipped: ShroomIndexListZ16<Item>,
}

partial_data!(
    CharForcedStat,
    CharForcedStatFlags,
    u32,
    derive(Debug, Clone),
    Str(u16) => 1 << 0,
    Dex(u16) => 1 << 1,
    Int(u16) => 1 << 2,
    Luk(u16) => 1 << 3,
    Pad(u16) => 1 << 4,
    Pdd(u16) => 1 << 5,
    Mad(u16) => 1 << 6,
    Mdd(u16) => 1 << 7,
    Acc(u16) => 1 << 8,
    Eva(u16) => 1 << 9,
    Speed(u8) => 1 << 10,
    Jump(u8) => 1 << 11,
    SpeedMax(u8) => 1 << 12
);

#[derive(ShroomPacket, Debug)]
pub struct CharForcedStatSetResp {
    pub stats: PartialFlag<(), CharForcedStatPartial>,
}
with_opcode!(CharForcedStatSetResp, SendOpcodes::ForcedStatSet);

#[derive(ShroomPacket, Debug)]
pub struct CharForcedStatResetResp;
with_opcode!(CharForcedStatResetResp, SendOpcodes::ForcedStatReset);

partial_data!(
    CharStat,
    CharStatFlags,
    u32,
    derive(Debug, Clone),
    Skin(Skin) => 1 << 0,
    Face(FaceId) => 1 << 1,
    Hair(HairId) => 1 << 2,
    Pet1(CashID) => 1 << 3,
    Pet2(CashID) => 1 << 19,
    Pet3(CashID) => 1 << 20,
    Level(u8) => 1 << 4,
    Job(JobId) => 1 << 5,
    Str(u16) => 1 << 6,
    Dex(u16) => 1 << 7,
    Int(u16) => 1 << 8,
    Luk(u16) => 1 << 9,
    Hp(u32) => 1 << 10,
    MaxHp(u32) => 1 << 11,
    Mp(u32) => 1 << 12,
    MaxMp(u32) => 1 << 13,
    Ap(u16) => 1 << 14,
    // TODO handle extended SP
    Sp(u16) => 1 << 15,
    Exp(u32) => 1 << 16,
    Fame(u16) => 1 << 17,
    Money(u32) => 1 << 18,
    TempExp(u32) => 1 << 21
);

#[derive(Debug, ShroomPacket)]
pub struct CharStatChangedResp {
    pub excl: bool,
    pub stats: PartialFlag<(), CharStatPartial>,
    //TODO Tail has to be decoded properly
    pub secondary_stat: bool,
    pub battle_recovery: bool,
}
with_opcode!(CharStatChangedResp, SendOpcodes::StatChanged);

#[derive(ShroomPacket, Debug)]
pub struct CharTempStatSetResp {
    pub temp_stats: PartialFlag<(), CharSecondaryStatPartial>,
    pub unknown: u16, // Delay?
    pub movement_affecting: bool,
}
with_opcode!(CharTempStatSetResp, SendOpcodes::TemporaryStatSet);

#[derive(ShroomPacket, Debug)]
pub struct CharTempStatResetResp {
    pub flags: CharSecondaryStatFlags,
}
with_opcode!(CharTempStatResetResp, SendOpcodes::TemporaryStatReset);


// TODO always has combat orders + extra data

pub type InventorySize = [u8; 5];

#[derive(ShroomPacket, Debug)]
pub struct CharDataHeader {
    pub combat_orders: u8,
    pub extra_data: ShroomOption8<CharExpirationData>,
}
