use shroom_meta::{
    id::{CharacterId, FieldId, ItemId, ObjectId, SkillId},
    twod::{Rect32, Vec2},
};
use shroom_proto95::game::{
    life::{
        affected_area::{AffectedAreaCreateResp, AffectedAreaRemoveResp, AffectedAreaType},
        message_box::{MessageBoxCreateResp, MessageBoxRemoveResp},
        open_gate::{OpenGateCreateResp, OpenGateRemoveResp},
        town_portal::{TownPortalCreateResp, TownPortalRemoveResp},
    },
    party::PartyID,
};
use shroom_srv::{
    game::pool::{Pool, PoolItem},
    GameTime,
};

use super::Obj;

#[derive(Debug)]
pub struct AffectedArea {
    pub ty: AffectedAreaType,
    pub owner_id: CharacterId,
    pub skill_id: SkillId,
    pub skill_level: u8,
    pub start_delay: u16,
    pub area: Rect32,
    pub phase: u32,
    pub elem_attr: u32,
}

impl PoolItem for AffectedArea {
    type Id = ObjectId;

    type EnterMsg = AffectedAreaCreateResp;
    type LeaveMsg = AffectedAreaRemoveResp;
    type LeaveParam = ();

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        AffectedAreaCreateResp {
            id,
            ty: self.ty,
            owner_id: self.owner_id,
            skill_id: self.skill_id,
            skill_level: self.skill_level,
            area: self.area.clone(),
            phase: self.phase,
            start_delay: self.start_delay,
            elem_attr: self.elem_attr,
        }
    }

    fn leave_msg(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeaveMsg {
        AffectedAreaRemoveResp { id }
    }
}

pub type AffectedAreaPool = Pool<Obj<AffectedArea>>;

#[derive(Debug)]
pub struct MessageBox {
    pub item_id: ItemId,
    pub message: String,
    pub char_name: String,
    pub host_pos: Vec2,
}

impl PoolItem for MessageBox {
    type Id = ObjectId;
    type EnterMsg = MessageBoxCreateResp;
    type LeaveMsg = MessageBoxRemoveResp;
    type LeaveParam = bool;

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        MessageBoxCreateResp {
            id,
            item_id: self.item_id,
            message: self.message.clone(),
            char_name: self.char_name.clone(),
            host_pos: self.host_pos,
        }
    }

    fn leave_msg(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeaveMsg {
        MessageBoxRemoveResp {
            id,
            no_fade_out: param,
        }
    }
}

pub type MessageBoxPool = Pool<Obj<MessageBox>>;

#[derive(Debug)]
pub struct OpenGate {
    pub state: u8,
    pub char_id: CharacterId,
    pub pos: Vec2,
    pub first: bool, // Either first or second gate
    pub party_id: PartyID,
}

impl PoolItem for OpenGate {
    type Id = ObjectId;
    type EnterMsg = OpenGateCreateResp;
    type LeaveMsg = OpenGateRemoveResp;
    type LeaveParam = ();

    fn enter_msg(&self, _id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        OpenGateCreateResp {
            state: self.state,
            char_id: self.char_id,
            pos: self.pos,
            first: self.first,
            party_id: self.party_id,
        }
    }

    fn leave_msg(&self, _id: Self::Id, _param: Self::LeaveParam) -> Self::LeaveMsg {
        OpenGateRemoveResp {
            leave: self.state,
            char_id: self.char_id,
            first: self.first,
        }
    }
}

pub type OpenGatePool = Pool<Obj<OpenGate>>;

#[derive(Debug)]
pub struct TownPortal {
    pub char_id: CharacterId,
    pub state: u8,
    pub pos: Vec2,
    pub target_map: FieldId,
    //TODO expiration + skill id
}

impl PoolItem for TownPortal {
    type Id = ObjectId;
    type EnterMsg = TownPortalCreateResp;
    type LeaveMsg = TownPortalRemoveResp;

    type LeaveParam = bool;

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        dbg!(TownPortalCreateResp {
            state: self.state,
            id: id.0.into(),
            pos: self.pos,
        })
    }

    fn leave_msg(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeaveMsg {
        TownPortalRemoveResp {
            display: param,
            id: id.0.into(),
        }
    }
}

pub type TownPortalPool = Pool<Obj<TownPortal>>;
