pub mod char;
pub mod inventory;
pub mod item;
pub mod job;
pub mod movement;

use std::net::{Ipv4Addr, SocketAddr};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use shroom_pkt::{
    mark_shroom_enum, string::FixedPacketString, ShroomPacket, with_opcode
};
use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

pub type NameStr = FixedPacketString<13>;

#[derive(ShroomPacket, Debug)]
pub struct ClientDumpLogReq {
    call_type: u16,
    error_code: u32,
    unknown1: u16,
    unknown2: u32,
    clear_stack_log: u32,
    unknown3: u32,
    //TODO: data: ShroomList16<u8>,
}
with_opcode!(ClientDumpLogReq, RecvOpcodes::ClientDumpLog);

#[derive(ShroomPacket, Debug)]
pub struct ExceptionLogReq {
    pub log: String,
}
with_opcode!(ExceptionLogReq, RecvOpcodes::ExceptionLog);

#[derive(ShroomPacket, Debug)]
pub struct UpdateScreenSettingReq {
    large_screen: bool,
    window_mode: bool,
}
with_opcode!(UpdateScreenSettingReq, RecvOpcodes::UpdateScreenSetting);

#[derive(ShroomPacket, Debug)]
pub struct PongReq;
with_opcode!(PongReq, RecvOpcodes::AliveAck);

#[derive(ShroomPacket, Debug, Clone)]
pub struct PingResp;
with_opcode!(PingResp, SendOpcodes::AliveReq);

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum Gender {
    #[default]
    Male = 0,
    Female = 1,
}
mark_shroom_enum!(Gender);

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum OptionGender {
    #[default]
    Male = 0,
    Female = 1,
    Unset = 10,
}

impl OptionGender {
    #[must_use]
    pub fn is_set(&self) -> bool {
        !self.is_unset()
    }

    #[must_use]
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }

    pub fn as_option(&self) -> Option<Gender> {
        match self {
            Self::Female => Some(Gender::Female),
            Self::Male => Some(Gender::Male),
            Self::Unset => None,
        }
    }
}

impl<T> From<Option<T>> for OptionGender
where
    T: Into<Gender>,
{
    fn from(value: Option<T>) -> Self {
        match value.map(Into::into) {
            None => Self::Unset,
            Some(Gender::Female) => Self::Female,
            Some(Gender::Male) => Self::Male,
        }
    }
}

impl From<OptionGender> for Option<Gender> {
    fn from(val: OptionGender) -> Self {
        match val {
            OptionGender::Female => Some(Gender::Female),
            OptionGender::Male => Some(Gender::Male),
            OptionGender::Unset => None,
        }
    }
}
mark_shroom_enum!(OptionGender);


#[derive(Debug, Clone)]
pub struct ServerAddr(pub Ipv4Addr);

impl From<[u8; 4]> for ServerAddr {
    fn from(value: [u8; 4]) -> Self {
        Self(Ipv4Addr::from(value))
    }
}

impl From<ServerAddr> for [u8; 4] {
    fn from(value: ServerAddr) -> Self {
        value.0.octets()
    }
}


shroom_pkt::packet_wrap!(ServerAddr<>, [u8; 4], [u8; 4]);

#[derive(Debug, Clone, ShroomPacket)]
pub struct ServerSocketAddr {
    pub addr: ServerAddr,
    pub port: u16,
}

impl TryFrom<SocketAddr> for ServerSocketAddr {
    type Error = anyhow::Error;

    fn try_from(value: SocketAddr) -> Result<Self, Self::Error> {
        match value {
            SocketAddr::V4(addr) => Ok(Self {
                addr: ServerAddr(*addr.ip()),
                port: addr.port(),
            }),
            SocketAddr::V6(_) => Err(anyhow::format_err!("Ipv6 not supported")),
        }
    }
}


