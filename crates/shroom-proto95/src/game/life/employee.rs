use shroom_meta::{
    id::{FootholdId, ObjectId},
    twod::Vec2,
};
use shroom_pkt::{with_opcode, ShroomOption8, ShroomPacket};

use crate::send_opcodes::SendOpcodes;

#[derive(ShroomPacket, Debug, Clone)]
pub struct EmployeeMiniRoomBalloon {
    pub sn: u32,
    pub text: String,
    pub spec: u8,
    pub cur_users: u8,
    pub max_users: u8,
}

#[derive(ShroomPacket, Debug)]
pub struct EmployeeCreateResp {
    pub id: ObjectId,
    pub employee_tmpl_id: u32,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub char_name: String,
    // TODO: the u8 is mini room ty
    pub balloon: ShroomOption8<EmployeeMiniRoomBalloon>,
}
with_opcode!(EmployeeCreateResp, SendOpcodes::EmployeeEnterField);

#[derive(ShroomPacket, Debug)]
pub struct EmployeeMiniRoomBalloonResp {
    pub employee_id: ObjectId,
    // TODO: the u8 is mini room ty
    pub balloon: ShroomOption8<EmployeeMiniRoomBalloon>,
}
with_opcode!(
    EmployeeMiniRoomBalloonResp,
    SendOpcodes::EmployeeMiniRoomBalloon
);

#[derive(ShroomPacket, Debug)]
pub struct EmployeeRemoveResp {
    pub id: ObjectId,
}
with_opcode!(EmployeeRemoveResp, SendOpcodes::EmployeeLeaveField);
