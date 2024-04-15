use shroom_meta::id::ItemId;
use shroom_pkt::{with_opcode, ShroomList32, ShroomPacket, ShroomPacketEnum};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

pub const FUNC_KEYS: usize = 89;
pub const QUICK_SLOTS: usize = 8;

#[derive(Debug, ShroomPacket, Default, Clone, Copy)]
pub struct FuncKey {
    pub ty: u8,
    pub action_id: u32,
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum FuncKeyMapInitResp {
    DefaultMap(()) = 1,
    KeyMap(Box<[FuncKey; FUNC_KEYS]>) = 0
}

impl From<Option<[FuncKey; FUNC_KEYS]>> for FuncKeyMapInitResp {
    fn from(map: Option<[FuncKey; FUNC_KEYS]>) -> Self {
        match map {
            Some(map) => FuncKeyMapInitResp::KeyMap(Box::new(map)),
            None => FuncKeyMapInitResp::DefaultMap(())
        }
    }
}

with_opcode!(FuncKeyMapInitResp, SendOpcodes::FuncKeyMappedInit);

#[derive(Debug, ShroomPacket)]
pub struct FuncKeyMapPetConsumeInitResp(pub ItemId);
with_opcode!(
    FuncKeyMapPetConsumeInitResp,
    SendOpcodes::PetConsumeItemInit
);

#[derive(Debug, ShroomPacket)]
pub struct FuncKeyMapPetConsumeMpInitResp(pub ItemId);
with_opcode!(
    FuncKeyMapPetConsumeMpInitResp,
    SendOpcodes::PetConsumeMPItemInit
);

#[derive(Debug, ShroomPacketEnum)]
#[repr(u32)]
pub enum FuncKeyMapChangeReq {
    Changed(ShroomList32<(u32, FuncKey)>) = 0,
    PetConsumeItem(ItemId) = 1,
    PetMpConsumeItem(ItemId) = 2,
}

with_opcode!(FuncKeyMapChangeReq, RecvOpcodes::FuncKeyMappedModified);


#[derive(Debug, ShroomPacket)]
pub struct QuickslotKeyMapChangedReq(pub [u32; QUICK_SLOTS]);
with_opcode!(QuickslotKeyMapChangedReq, RecvOpcodes::QuickslotKeyMappedModified);


#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum QuickSlotInitResp {
    DefaultMap(()) = 0,
    KeyMap([u32; QUICK_SLOTS]) = 1
}
impl From<Option<[u32; QUICK_SLOTS]>> for QuickSlotInitResp {
    fn from(map: Option<[u32; QUICK_SLOTS]>) -> Self {
        match map {
            Some(map) => Self::KeyMap(map),
            None => Self::DefaultMap(())
        }
    }
}
with_opcode!(QuickSlotInitResp, SendOpcodes::QuickslotMappedInit);