use shroom_meta::{id::{NpcId, QuestId}, twod::Vec2};
use shroom_pkt::{with_opcode, DecodePacket, EncodePacket, ShroomPacket, ShroomPacketEnum, ShroomTime, SizeHint};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};


#[derive(Debug)]
pub struct ConstantU8<const C: u8>;

impl<const C: u8> EncodePacket for ConstantU8<C> {
    const SIZE_HINT: shroom_pkt::SizeHint = SizeHint::new(1);

    fn encode<B: bytes::BufMut>(&self, pw: &mut shroom_pkt::PacketWriter<B>) -> shroom_pkt::PacketResult<()> {
        C.encode(pw)?;
        Ok(())
    }
}

impl<'de, const C: u8> DecodePacket<'de> for ConstantU8<C> {
    fn decode(pr: &mut shroom_pkt::PacketReader<'de>) -> shroom_pkt::PacketResult<Self> {
        let v = u8::decode(pr)?;
        if v != C {
            //TODO
            return Err(shroom_pkt::Error::InvalidEnumDiscriminant(v as usize));
        }
        Ok(Self)
    }
}


#[derive(Debug, ShroomPacket)]
pub struct LostItem {
    pub id: QuestId,
    pub npc_tmpl_id: NpcId,
    pub delivery_item_pos: u32,

}

#[derive(Debug, ShroomPacket)]
pub struct AcceptQuest {
    pub id: QuestId,
    pub npc_tmpl_id: NpcId,
    pub delivery_item_pos: u32, //TODO only if there's an item
    /* TODO pub pos: Vec2 only if  quest is not auto alert*/

}

#[derive(Debug, ShroomPacket)]
pub struct CompleteQuest {
    pub id: QuestId,
    pub npc_tmpl_id: NpcId,
    pub delivery_item_pos: u32,
    /* TODO pub pos: Vec2 only if  quest is not auto alert*/
    pub quest_reward_list: u32
    
}

#[derive(Debug, ShroomPacket)]
pub struct QuestScript {
    pub id: QuestId,
    pub npc_tmpl_id: NpcId,
    pub pos: Vec2
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum UserQuestReq {
    LostItem(QuestId) = 0, //TODO
    AcceptQuest(AcceptQuest) = 1,
    CompleteQuest(CompleteQuest) = 2,
    ResignQuest(QuestId) = 3,
    OpenScript(QuestScript) = 4,
    CompleteScript(QuestScript) = 5

}

with_opcode!(UserQuestReq, RecvOpcodes::UserQuestRequest);


#[derive(Debug, ShroomPacket)]
pub struct UserQuestSuccessResult {
    pub id: QuestId,
    pub npc_tmpl_id: NpcId,
    /// 0 means none
    pub next_quest: QuestId,
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum UserQuestResultResp {
    StartTimer(()) = 6,
    EndTimer(()) = 7,
    ResetTimer(QuestId) = 18,

    StartTimeKeepTimer(()) = 8,
    EndTimeKeepTimer(()) = 9,

    Success(UserQuestSuccessResult) = 10,

    FailedUnknown(()) = 11,
    FailedInventory(QuestId) = 12,
    FailedMoney(()) = 13,
    FailedPet(()) = 14,
    FailedEquipped(()) = 15,
    FailedOnlyItem(()) = 16,
    FailedTimeOver(QuestId) = 17,
}
with_opcode!(UserQuestResultResp, SendOpcodes::UserQuestResult);



#[derive(Debug)]
pub struct QuestRecordValue(pub String);



#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum QuestState {
    NotStarted(u8) = 0,
    Accept(String) = 1,
    Complete(ShroomTime) = 2,
    Resign(()) = 3,
    Fail(()) = 4
}

#[derive(Debug, ShroomPacket)]
pub struct QuestRecordMessageResp {
    pub marker: ConstantU8<1>,
    pub id: QuestId,
    pub state: QuestState
}

with_opcode!(QuestRecordMessageResp, SendOpcodes::Message);
