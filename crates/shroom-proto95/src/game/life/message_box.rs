use shroom_meta::{id::{ItemId, ObjectId}, twod::Vec2};
use shroom_pkt::{with_opcode, ShroomPacket};

use crate::send_opcodes::SendOpcodes;

#[derive(ShroomPacket, Debug)]
pub struct MessageBoxCreateResp {
    pub id: ObjectId,
    pub item_id: ItemId,
    pub message: String,
    pub char_name: String,
    pub host_pos: Vec2,
}
with_opcode!(MessageBoxCreateResp, SendOpcodes::MessageBoxEnterField);

#[derive(ShroomPacket, Debug)]
pub struct MessageBoxCreateFailedResp;
with_opcode!(
    MessageBoxCreateFailedResp,
    SendOpcodes::CreateMessgaeBoxFailed
);

#[derive(ShroomPacket, Debug)]
pub struct MessageBoxRemoveResp {
    pub no_fade_out: bool,
    pub id: ObjectId,
}
with_opcode!(MessageBoxRemoveResp, SendOpcodes::MessageBoxLeaveField);
