use shroom_meta::id::CharacterId;
use shroom_pkt::{
    string::FixedPacketString, with_opcode, ShroomOption8, ShroomPacket, ShroomPacketEnum,
};

use crate::{send_opcodes::SendOpcodes, shared::NameStr};

//TODO in_shop is an u8 idk

#[derive(ShroomPacket, Debug)]
pub struct FriendRecord {
    pub id: CharacterId,
    pub name: NameStr,
    pub flag: u8,
    pub channel_id: u32,
    pub friend_group: FixedPacketString<0x11>,
}

#[derive(ShroomPacket, Debug)]
pub struct FriendList {
    pub len: u8,
    #[pkt(size = "len")]
    pub friends: Vec<FriendRecord>,
    #[pkt(size = "len")]
    pub in_shop: Vec<u32>, // 4 bit boolean
}

impl FriendList {
    pub fn empty() -> Self {
        Self {
            len: 0,
            friends: Vec::new(),
            in_shop: Vec::new(),
        }
    }
}

#[derive(ShroomPacket, Debug)]
pub struct FriendUpdate {
    pub friend_id: CharacterId,
    pub record: FriendRecord,
    pub in_shop: bool,
}

#[derive(ShroomPacket, Debug)]
pub struct FriendChangeChannel {
    pub friend_id: CharacterId,
    pub in_shop: bool,
    pub channel: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct FriendUnknown9 {
    pub friend_id: CharacterId,
    pub in_shop: bool,
    pub channel_id: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct FriendReq {
    pub friend_id: CharacterId,
    pub friend_name: String,
    pub level: u32,
    pub job_code: u32, //TODO: job id?
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum FriendResultResp {
    Reset(FriendList) = 0,
    Update(FriendUpdate) = 1,
    Req(FriendReq) = 2,
    Reset3(FriendList) = 3,
    Unknown4(()) = 4,
    Unknown5(()) = 5,
    Unknown6(()) = 6,
    Unknown7(()) = 7,
    Unknown8(()) = 8,
    Unknown9(ShroomOption8<String>) = 9,
    UnknownA(ShroomOption8<String>) = 0xa,
    // Blocked is alwayws true fo this
    ResetB(FriendList) = 0xb,
    UnknownC(ShroomOption8<String>) = 0xc,
    ChangeChannel(FriendChangeChannel) = 0xd,
    MaxFriends(u8) = 0xe,
    UnknownF(ShroomOption8<String>) = 0xf,
}
with_opcode!(FriendResultResp, SendOpcodes::FriendResult);
