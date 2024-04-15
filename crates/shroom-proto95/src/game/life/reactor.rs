use shroom_meta::{id::{ObjectId, ReactorId}, twod::Vec2};
use shroom_pkt::{with_opcode, ShroomDurationMs16, ShroomPacket};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};


#[derive(ShroomPacket, Debug)]
pub struct ReactorEnterFieldResp {
    pub id: ObjectId,
    pub tmpl_id: ReactorId,
    pub state: u8,
    pub pos: Vec2,
    pub flipped: bool,
    pub name: String,
}
with_opcode!(ReactorEnterFieldResp, SendOpcodes::ReactorEnterField);

#[derive(ShroomPacket, Debug)]
pub struct ReactorLeaveFieldResp {
    pub id: ObjectId,
    pub state: u8,
    pub pos: Vec2,
}
with_opcode!(ReactorLeaveFieldResp, SendOpcodes::ReactorLeaveField);

#[derive(ShroomPacket, Debug)]
pub struct ReactorMoveResp {
    pub id: ObjectId,
    pub pos: Vec2,
}
with_opcode!(ReactorMoveResp, SendOpcodes::ReactorMove);

#[derive(ShroomPacket, Debug)]
pub struct ReactorChangeStateResp {
    pub id: ObjectId,
    pub state: u8,
    pub pos: Vec2,
    pub animation_delay: ShroomDurationMs16,
    pub proper_event_id: u8,
    pub end_state: u8,
}
with_opcode!(ReactorChangeStateResp, SendOpcodes::ReactorChangeState);

#[derive(ShroomPacket, Debug)]
pub struct ReactorHitReq {
    pub id: ObjectId,
    pub skill_reactor: u32,
    pub hit_option: u32,
    pub action_delay: ShroomDurationMs16,
    pub skill_id: u32,
}
with_opcode!(ReactorHitReq, RecvOpcodes::ReactorHit);

#[derive(ShroomPacket, Debug)]
pub struct ReactorTouchReq {
    pub id: ObjectId,
    pub has_reactor: bool, // TODO
}
with_opcode!(ReactorTouchReq, RecvOpcodes::ReactorTouch);
