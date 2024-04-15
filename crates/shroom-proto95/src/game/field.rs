use shroom_meta::{id::{CharacterId, FieldId, MobId}, twod::TagPoint};
use shroom_pkt::{
    list::ShroomListLen, with_opcode, CondEither, ShroomDurationMs32, ShroomList, ShroomList16, ShroomOption8, ShroomPacket, ShroomPacketEnum, ShroomTime
};

use crate::{send_opcodes::SendOpcodes, shared::char::CharDataHeader};

use super::user::char::{CharDataAll, CharDataFlags};

#[derive(ShroomPacket, Debug)]
pub struct ClientOption {
    pub key: u32,
    pub value: u32,
}

#[derive(ShroomPacket, Debug, Default)]
pub struct CrcSeed {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct LogoutGiftConfig {
    pub predict_quit: u32,
    pub gift_commodity_id: [u32; 3],
}

/// Dirty hack to work around the problem
/// that when there's a notification, there's always n + 1
/// First entry is chatblock reason
#[derive(ShroomPacket, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlusOneListIndex(pub u16);

impl ShroomListLen for PlusOneListIndex {
    fn to_len(&self) -> usize {
        match self.0 {
            0 => 0,
            n => (n + 1) as usize,
        }
    }

    fn from_len(ix: usize) -> Self {
        Self(ix as u16)
    }
}
#[derive(ShroomPacket, Debug, Default)]
pub struct NotificationList(ShroomList<PlusOneListIndex, String>);

impl NotificationList {
    pub fn chat_banned<'s>(ban_reason: &str, extra: impl Iterator<Item = &'s str>) -> Self {
        let mut list = Self::default();
        list.0.push(ban_reason.to_string());
        list.0.extend(extra.map(|s| s.to_string()));
        list
    }

    pub fn ban_reason(&self) -> Option<&str> {
        self.0.first().map(|s| s.as_str())
    }

    pub fn extra(&self) -> impl Iterator<Item = &str> {
        self.0.iter().skip(1).map(|s| s.as_str())
    }
}

#[derive(ShroomPacket, Debug)]
pub struct FieldCharData {
    pub seed: CrcSeed,
    pub char_data_flags: CharDataFlags,
    pub char_data_hdr: CharDataHeader,
    pub char_data: CharDataAll,
    pub logout_gift_config: LogoutGiftConfig,
}

#[derive(ShroomPacket, Debug)]
pub struct FieldTransferData {
    pub revive: bool,
    pub map: FieldId,
    pub portal: u8,
    pub hp: u32,
    pub chase_target_pos: ShroomOption8<TagPoint>,
}

impl FieldTransferData {
    pub fn is_chase_enabled(&self) -> bool {
        self.chase_target_pos.opt.is_some()
    }
}

fn is_true(b: &bool) -> bool {
    *b
}

#[derive(ShroomPacket, Debug)]
pub struct SetFieldResp {
    pub client_option: ShroomList16<ClientOption>,
    pub channel_id: u32,
    pub old_driver_id: CharacterId,
    pub field_key: u8,
    pub has_char_data: bool,
    pub notifications: NotificationList,
    #[pkt(either(field = "has_char_data", cond = "is_true"))]
    pub char_data: CondEither<FieldCharData, FieldTransferData>,
    pub server_time: ShroomTime,
}
with_opcode!(SetFieldResp, SendOpcodes::SetField);


#[derive(ShroomPacket, Debug)]
pub struct SummonEffectData {
    pub uol: bool,
    pub pos: TagPoint
}

#[derive(ShroomPacket, Debug)]
pub struct TrembleEffectData {
    /// Else heavy
    pub heavy_n_short: bool,
    pub delay: ShroomDurationMs32
}

#[derive(ShroomPacket, Debug)]
pub struct MobHpTagEffect {
    pub mob_tmpl_id: MobId,
    pub hp: u32,
    pub max_hp: u32,
    pub color: u8,
    pub bg_color: u8
}

#[derive(ShroomPacket, Debug)]
pub struct FieldEffectData {
    pub reward_job_id: u32,
    pub reward_party_id: u32,
    pub reward_lev_id: u32
}


pub enum CakeEventEffect {
    CakeWin,
    PieWin,
    Start,
    TimeOver
}

impl From<CakeEventEffect> for FieldEffectResp {
    fn from(event: CakeEventEffect) -> Self {
        Self::Screen(match event {
            CakeEventEffect::CakeWin => "event/5th/cakewin".to_string(),
            CakeEventEffect::PieWin => "event/5th/piewin".to_string(),
            CakeEventEffect::Start => "event/5th/start".to_string(),
            CakeEventEffect::TimeOver => "event/5th/timeover".to_string(),
        })
    }
}

pub enum DojangEffect {
    Clear,
    TimeOver
}

impl From<DojangEffect> for FieldEffectResp {
    fn from(event: DojangEffect) -> Self {
        Self::Screen(match event {
            DojangEffect::Clear => "dojang/end/clear".to_string(),
            DojangEffect::TimeOver => "dojang/timeOver".to_string(),
        })
    }
}


#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum FieldEffectResp {
    Summon(SummonEffectData) = 0,
    Tremble(TrembleEffectData) = 1,
    Object(String) = 2,
    Screen(String) = 3,
    Sound(String) = 4,
    MobHpTag(MobHpTagEffect) = 5,
    ChangeBgm(String) = 6,
    RewardBullet(FieldEffectData) = 7,
}
with_opcode!(FieldEffectResp, SendOpcodes::FieldEffect);