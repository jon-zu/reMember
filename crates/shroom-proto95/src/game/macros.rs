use shroom_meta::id::SkillId;
use shroom_pkt::{with_opcode, ShroomList8, ShroomPacket};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

pub const MAX_MACROS: usize = 5;

#[derive(ShroomPacket, Debug)]
pub struct SkillMacro {
    pub name: String,
    pub mute: bool,
    pub skills: [SkillId; 3],
}

pub type MacroSysData = ShroomList8<SkillMacro>;

#[derive(ShroomPacket, Debug)]
pub struct MacroInitResp(pub MacroSysData);
with_opcode!(MacroInitResp, SendOpcodes::MacroSysDataInit);


#[derive(ShroomPacket, Debug)]
pub struct MacroUpdateReq(pub MacroSysData);
with_opcode!(MacroUpdateReq, RecvOpcodes::UserMacroSysDataModified);
