use shroom_meta::{
    id::{CharacterId, ItemId, ObjectId},
    twod::Vec2,
};
use shroom_pkt::{
    shroom_enum_code, with_opcode, CondOption, DecodePacket, EncodePacket, PacketResult,
    ShroomDurationMs16, ShroomExpirationTime, ShroomPacket, ShroomPacketEnum,
};

use crate::send_opcodes::SendOpcodes;

use super::party::PartyID;

pub type DropId = ObjectId;

#[derive(Debug, Clone, Copy)]
pub enum DropOwner {
    User(CharacterId),
    // TODO: Party ID
    Party(PartyID),
    None,
    Explosive,
}

impl From<DropOwner> for (u32, u8) {
    fn from(val: DropOwner) -> Self {
        match val {
            DropOwner::User(user) => (user.0, 0),
            DropOwner::Party(party) => (party, 1),
            DropOwner::None => (0, 2),
            DropOwner::Explosive => (0, 3),
        }
    }
}

impl TryFrom<(u32, u8)> for DropOwner {
    type Error = shroom_pkt::Error;

    fn try_from(v: (u32, u8)) -> PacketResult<Self> {
        Ok(match v.1 {
            0 => Self::User(v.0.into()),
            1 => Self::Party(v.0),
            2 => Self::None,
            3 => Self::Explosive,
            _ => return Err(shroom_pkt::Error::InvalidEnumPrimitive(v.1 as u32)),
        })
    }
}

impl EncodePacket for DropOwner {
    const SIZE_HINT: shroom_pkt::SizeHint = <(u32, u8) as EncodePacket>::SIZE_HINT;

    fn encode<B: bytes::BufMut>(&self, pw: &mut shroom_pkt::PacketWriter<B>) -> PacketResult<()> {
        <(u32, u8) as EncodePacket>::encode(&(*self).into(), pw)
    }
}

impl<'de> DecodePacket<'de> for DropOwner {
    fn decode(pr: &mut shroom_pkt::PacketReader<'de>) -> PacketResult<Self> {
        <(u32, u8) as DecodePacket>::decode(pr)?.try_into()
    }
}

shroom_enum_code!(
    DropEnterType,
    u8,
    Default = 0,
    Create = 1,     // Basic floating
    OnFoothold = 2, // Instant attached to fh
    FadingOut = 3,  // Fading away
    Unknown4 = 4    // ?
);

impl DropEnterType {
    fn has_start_pos(&self) -> bool {
        matches!(
            self,
            Self::Default | Self::Create | Self::FadingOut | Self::Unknown4
        )
    }
}
#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum DropType {
    Item(ItemId) = 0,
    Money(u32) = 1,
}
impl DropType {
    fn has_expiration(&self) -> bool {
        !matches!(self, Self::Money(_))
    }
}

#[derive(ShroomPacket, Debug)]
pub struct DropEnterFieldResp {
    pub enter_type: DropEnterType,
    pub id: ObjectId,
    pub drop_type: DropType,
    pub drop_owner: DropOwner,
    pub pos: Vec2,
    pub src_id: u32,
    #[pkt(check(field = "enter_type", cond = "DropEnterType::has_start_pos"))]
    pub start_pos: CondOption<(Vec2, ShroomDurationMs16)>,
    #[pkt(check(field = "drop_type", cond = "DropType::has_expiration"))]
    pub drop_expiration: CondOption<ShroomExpirationTime>,
    pub by_pet: bool,
    // If this is set to true It throws an exception
    pub u1_flag: bool,
}
with_opcode!(DropEnterFieldResp, SendOpcodes::DropEnterField);

shroom_enum_code!(
    DropLeaveType,
    u8,
    TimeOut = 0,
    ScreenScroll = 1,
    UserPickup = 2,
    MobPickup = 3,
    Explode = 4,
    PetPickup = 5,
    PassConvex = 6,
    PetSkill = 7
);

impl DropLeaveType {
    fn has_pickup_id(&self) -> bool {
        matches!(self, Self::UserPickup | Self::MobPickup | Self::PetSkill)
    }
}

#[derive(ShroomPacket, Debug)]
pub struct DropLeaveFieldResp {
    pub leave_type: DropLeaveType,
    pub id: DropId,
    #[pkt(check(field = "leave_type", cond = "DropLeaveType::has_pickup_id"))]
    pub pickup_id: CondOption<u32>,
}
with_opcode!(DropLeaveFieldResp, SendOpcodes::DropLeaveField);

#[cfg(test)]
mod tests {
    use shroom_pkt::{DecodePacket, PacketReader};

    use super::*;

    #[test]
    fn drop_enter() {
        let data = [
            0x01, 0x3B, 0x00, 0x00, 0x00, 0x01, 0x5E, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x59, 0x01, 0xC7, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xEC, 0xFF, 0x64,
            0x00, 0x01, 0x00,
        ];
        let drop = DropEnterFieldResp::decode_complete(&mut PacketReader::new(&data)).unwrap();
        dbg!(&drop);
    }

    #[test]
    fn drop_enter2() {
        let data = [
            01, 0x68, 0x00, 0x00, 0x00, 0x01, 0x5E, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0xC3, 0x02, 0x31, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xEC, 0xFF, 0x64, 0x00,
            0x01, 0x00,
        ];
        let drop = DropEnterFieldResp::decode_complete(&mut PacketReader::new(&data)).unwrap();
        dbg!(&drop);
    }
}
