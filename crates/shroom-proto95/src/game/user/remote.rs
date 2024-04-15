use shroom_meta::{
    id::{job_id::JobId, CharacterId, FootholdId, ItemId, SkillId},
    twod::{TagPoint, Vec2},
};
use shroom_pkt::{
    partial::PartialFlag, with_opcode, ShroomDurationMs32, ShroomIndexListZ8, ShroomList32,
    ShroomOption8, ShroomPacket,
};

use crate::{
    send_opcodes::SendOpcodes,
    shared::{char::AvatarData, movement::MovePath},
};

use super::{
    secondary_stats::{RemoteCharSecondaryStatFlags, RemoteCharSecondaryStatPartial},
    ActionDir,
};

#[derive(ShroomPacket, Default, Debug)]
pub struct GuildMarkData {
    bg: u16,
    bg_color: u8,
    mark: u16,
    mark_color: u8,
}

#[derive(ShroomPacket, Debug)]
pub struct PetInitInfo {
    tmpl_id: u32,
    name: String,
    pet_locker_sn: u64,
    pos_prev: Vec2,
    move_action: u8,
    fh: FootholdId,
}

#[derive(ShroomPacket, Debug, Default)]
pub struct TamingMobData {
    level: u32,
    exp: u32,
    fatigue: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct MiniRoomData {
    pub sn: u32,
    pub title: String,
    pub private: bool,
    pub game_kind: bool,
    pub cur_users: u8,
    pub max_users: u8,
    pub game_on: bool,
}

#[derive(ShroomPacket, Debug)]
pub struct ADBoardRemoteData {
    title: String,
}

#[derive(ShroomPacket, Debug)]
pub struct CoupleRingData {
    item_sn: u64,
    pair_item_sn: u64,
    item_id: ItemId,
}

#[derive(ShroomPacket, Debug)]
pub struct FriendshipRingData {
    item_sn: u64,
    pair_item_sn: u64,
    item_id: ItemId,
}

#[derive(ShroomPacket, Debug)]
pub struct MarriageData {
    char_id: CharacterId,
    pair_char_id: CharacterId,
    wedding_ring_id: ItemId,
}

pub type PartialSecondaryStats = PartialFlag<(), RemoteCharSecondaryStatPartial>;

#[derive(ShroomPacket, Debug)]
pub struct UserRemoteInitData {
    pub level: u8,
    pub name: String,
    pub guild_name: String,
    pub guild_mark: GuildMarkData,
    pub secondary_stat: PartialSecondaryStats, //TODO
    pub defense_att: u8,
    pub defense_state: u8,
    pub job: JobId,
    pub avatar: AvatarData,
    pub driver_id: CharacterId,
    pub passenger_id: CharacterId,
    pub choco_count: u32,
    pub active_effect_item: ItemId, //Active Item portable chair?
    pub completed_set_item_id: ItemId,
    pub portable_chair: ItemId,
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub show_admin_effects: bool,
    pub pet_infos: ShroomIndexListZ8<PetInitInfo>,
    pub taming_mob: TamingMobData,
    // TODO: in theory the u8 should be the mini room type,
    // != 0 => read data
    pub mini_room: ShroomOption8<MiniRoomData>,
    pub ad_board: ShroomOption8<ADBoardRemoteData>,
    pub couple: ShroomOption8<CoupleRingData>,
    pub friendship: ShroomOption8<FriendshipRingData>,
    pub marriage: ShroomOption8<MarriageData>,
    pub load_flags: u8, // 0: load dark force, 2: load dragon, 4: swallow,
    pub new_year_cards: ShroomOption8<ShroomList32<CharacterId>>,
    pub phase: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct UserEnterFieldResp {
    pub char_id: CharacterId,
    pub user_init_data: UserRemoteInitData,
}
with_opcode!(UserEnterFieldResp, SendOpcodes::UserEnterField);

#[derive(ShroomPacket, Debug)]
pub struct UserLeaveFieldResp {
    pub char_id: CharacterId,
}
with_opcode!(UserLeaveFieldResp, SendOpcodes::UserLeaveField);

#[derive(ShroomPacket, Debug)]
pub struct UserSkillPrepareResp {
    pub char_id: CharacterId,
    pub skill_id: SkillId,
    pub slv: u8,
    pub action_dir: ActionDir,
    pub action_speed: u8,
}
with_opcode!(UserSkillPrepareResp, SendOpcodes::UserSkillPrepare);

#[derive(ShroomPacket, Debug)]
pub struct UserMovingShootPrepareResp {
    pub char_id: CharacterId,
    pub level: u8,
    pub skill: ShroomOption8<SkillId>,
    pub action_dir: ActionDir,
    pub action_speed: u8,
}
with_opcode!(
    UserMovingShootPrepareResp,
    SendOpcodes::UserMovingShootAttackPrepare
);

#[derive(ShroomPacket, Debug)]
pub struct UserSkillCancelResp {
    pub char_id: CharacterId,
    pub skill: SkillId,
}
with_opcode!(UserSkillCancelResp, SendOpcodes::UserSkillCancel);

#[derive(ShroomPacket, Debug)]
pub struct UserEmotionResp {
    pub char_id: CharacterId,
    pub emotion: u32,
    pub dur: ShroomDurationMs32,
}
with_opcode!(UserEmotionResp, SendOpcodes::UserEmotion);

#[derive(ShroomPacket, Debug)]
pub struct UserSetActiveItemEffectResp {
    pub char_id: CharacterId,
    pub item: ItemId,
}
with_opcode!(
    UserSetActiveItemEffectResp,
    SendOpcodes::UserSetActiveEffectItem
);

#[derive(ShroomPacket, Debug)]
pub struct UserShowUpgradeTombEffectResp {
    pub char_id: CharacterId,
    pub item: ItemId,
    pub pos: TagPoint,
}
with_opcode!(
    UserShowUpgradeTombEffectResp,
    SendOpcodes::UserShowUpgradeTombEffect
);

#[derive(ShroomPacket, Debug)]
pub struct UserThrowGrenadeResp {
    pub char_id: CharacterId,
    pub pos: TagPoint,
    pub key_down: ShroomDurationMs32,
    pub skill: SkillId,
    pub slv: u32,
}
with_opcode!(UserThrowGrenadeResp, SendOpcodes::UserThrowGrenade);

#[derive(ShroomPacket, Debug)]
pub struct UserMoveResp {
    pub char_id: CharacterId,
    pub move_path: MovePath,
}
with_opcode!(UserMoveResp, SendOpcodes::UserMove);

#[derive(ShroomPacket, Debug)]
pub struct UserSetActivePortablChairResp {
    pub char_id: CharacterId,
    pub chair_id: ItemId,
}
with_opcode!(
    UserSetActivePortablChairResp,
    SendOpcodes::UserSetActivePortableChair
);

#[derive(ShroomPacket, Debug)]
pub struct UserAvatarModifiedResp {
    pub char_id: CharacterId,
    pub flags: u8,
    pub avatar_data: AvatarData,
    pub speed: u8,
    pub carry_item_effect: u8,
    /*
    TODO
        u8 flag set

        1 => AvatarData
        2 => speed(u8)
        3 => carry item effect(u8)



     */
    pub couple: ShroomOption8<CoupleRingData>,
    pub marriage: ShroomOption8<MarriageData>,
    pub completed_set_item_id: u32,
}
with_opcode!(UserAvatarModifiedResp, SendOpcodes::UserAvatarModified);

#[derive(ShroomPacket, Debug)]
pub struct UserSetTemporaryStatResp {
    stats: PartialSecondaryStats,
}
with_opcode!(UserSetTemporaryStatResp, SendOpcodes::UserTemporaryStatSet);

#[derive(ShroomPacket, Debug)]
pub struct UserResetTemporaryStatResp {
    flags: RemoteCharSecondaryStatFlags,
}
with_opcode!(
    UserResetTemporaryStatResp,
    SendOpcodes::UserTemporaryStatReset
);

#[derive(ShroomPacket, Debug)]
pub struct UserReceiveHPResp {
    pub hp: u32,
    pub max_hp: u32,
}
with_opcode!(UserReceiveHPResp, SendOpcodes::UserHP);

#[derive(ShroomPacket, Debug)]
pub struct UserGuildNameChangedResp {
    pub guild_name: String,
}
with_opcode!(UserGuildNameChangedResp, SendOpcodes::UserGuildNameChanged);

#[derive(ShroomPacket, Debug)]
pub struct UserGuildMarkChangedResp {
    pub guild_mark: GuildMarkData,
}
with_opcode!(UserGuildMarkChangedResp, SendOpcodes::UserGuildMarkChanged);
