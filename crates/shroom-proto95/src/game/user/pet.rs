use shroom_meta::{
    id::{CharacterId, FootholdId, ItemId},
    twod::Vec2,
};
use shroom_pkt::{
    shroom_enum_code, with_opcode, ShroomList8, ShroomOption8, ShroomPacket, ShroomPacketEnum,
};

use crate::{send_opcodes::SendOpcodes, shared::movement::MovePath};

#[derive(ShroomPacket, Debug)]
pub struct PetMoveResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub move_path: MovePath,
}
with_opcode!(PetMoveResp, SendOpcodes::PetMove);

#[derive(ShroomPacket, Debug)]
pub struct PetNameChangedResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub name: String,
    pub name_tag: bool,
}
with_opcode!(PetNameChangedResp, SendOpcodes::PetNameChanged);

#[derive(ShroomPacket, Debug)]
pub struct PetExceptionListResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub pet_sn: u64,
    pub exception_list: ShroomList8<ItemId>,
}
with_opcode!(PetExceptionListResp, SendOpcodes::PetLoadExceptionList);

#[derive(ShroomPacket, Debug)]
pub struct PetActionResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub ty: u8,
    pub action: u8,
    pub chat: String,
    pub chat_balloon: bool,
}
with_opcode!(PetActionResp, SendOpcodes::PetAction);

shroom_enum_code!(
    PetActivateError,
    u8,
    None = 0,
    PetWentHome = 1,
    PetMagicalTimeExpired = 2,
    UnableToUsePet = 3,
    CannotSummon = 4
);

#[derive(ShroomPacket, Debug)]
pub struct PetInitData {
    pub reset_active: bool,
    pub pet_tmpl_id: u32,
    pub pet_name: String,
    pub pet_locker_sn: u64,
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub name_tag: bool,
    pub chat_balloon: bool,
}

#[derive(ShroomPacket, Debug)]
pub struct PetRemoteActivateResp {
    pub char: CharacterId,
    pub pet_id: u8,
    pub pet_data: ShroomOption8<PetInitData>,
}

#[derive(ShroomPacket, Debug)]
pub struct PetRemoteEnterFieldResp {
    pub char: CharacterId,
    pub pet_id: u8,
    pub pet_data: ShroomOption8<PetInitData>,
}
with_opcode!(PetRemoteEnterFieldResp, SendOpcodes::PetTransferField);


with_opcode!(PetRemoteActivateResp, SendOpcodes::PetActivated);

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum PetLocalActivateResult {
    Ok(PetInitData) = 1,
    Err(PetActivateError) = 0,
}

#[derive(ShroomPacket, Debug)]
pub struct PetLocalActivateResp {
    pub char: CharacterId,
    pub pet_id: u8,
    pub pet_data: PetLocalActivateResult,
}
with_opcode!(PetLocalActivateResp, SendOpcodes::PetActivated);



