use shroom_meta::{id::CharacterId, twod::Vec2};
use shroom_pkt::{with_opcode, ShroomPacket};

use crate::{
    send_opcodes::SendOpcodes, recv_opcodes::RecvOpcodes, game::party::PartyID,
};

#[derive(ShroomPacket, Debug)]
pub struct OpenGateCreateResp {
    pub state: u8,
    pub char_id: CharacterId,
    pub pos: Vec2,
    pub first: bool, // Either first or second gate
    pub party_id: PartyID,
}
with_opcode!(OpenGateCreateResp, SendOpcodes::OpenGateCreated);

#[derive(ShroomPacket, Debug)]
pub struct OpenGateRemoveResp {
    pub leave: u8,
    pub char_id: CharacterId,
    pub first: bool,
}
with_opcode!(OpenGateRemoveResp, SendOpcodes::OpenGateRemoved);

#[derive(ShroomPacket, Debug)]
pub struct OpenGateEntryReq {
    pub char_id: CharacterId,
    pub pos: Vec2,
    pub first: bool
}
with_opcode!(OpenGateEntryReq, RecvOpcodes::EnterOpenGateRequest);
