use shroom_data::model::inv::EquipItemSlot;
use shroom_meta::{id::{CharacterId, ItemId, ObjectId}, twod::Vec2};
use shroom_proto95::game::drop::{
    DropEnterFieldResp, DropEnterType, DropLeaveFieldResp, DropLeaveType, DropOwner, DropType,
};
use shroom_srv::{game::pool::{Pool, PoolItem}, GameTime};
use std::{ops::Add, time::Duration};

use shroom_pkt::ShroomExpirationTime;

use super::Obj;

#[derive(Debug)]
pub struct DropItem {
    pub owner: DropOwner,
    pub pos: Vec2,
    pub start_pos: Vec2,
    pub value: DropTypeValue,
    pub quantity: usize,
}

impl DropItem {
    pub fn as_money(&self) -> Option<u32> {
        match self.value {
            DropTypeValue::Mesos(m) => Some(m),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum DropTypeValue {
    Mesos(u32),
    Item(ItemId),
    ExistingItem(EquipItemSlot)
}

#[derive(Debug)]
pub enum DropLeaveParam {
    TimeOut,
    ScreenScroll,
    UserPickup(CharacterId),
    MobPickup(u32),
    Explode,
    PetPickup(u32),
    PassConvex,
    PetSkill,
}

impl PoolItem for DropItem {
    type Id = ObjectId;

    type EnterMsg = DropEnterFieldResp;
    type LeaveMsg = DropLeaveFieldResp;
    type LeaveParam = DropLeaveParam;

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        let (drop_type, expiration) = match &self.value {
            DropTypeValue::Item(item) => (
                DropType::Item(*item),
                Some(ShroomExpirationTime::delay(
                    chrono::Duration::new(60, 0).unwrap(),
                )),
            ),
            DropTypeValue::ExistingItem(slot) => (
                DropType::Item(slot.item_id),
                Some(ShroomExpirationTime::delay(
                    chrono::Duration::new(60, 0).unwrap(),
                )),
            ),
            DropTypeValue::Mesos(mesos) => (DropType::Money(*mesos), None),
        };

        let start_pos = (
            self.start_pos.add(Vec2::new(0, -20)),
            Duration::from_millis(100).into(),
        );

        DropEnterFieldResp {
            enter_type: DropEnterType::Create,
            id,
            drop_type,
            drop_owner: self.owner,
            pos: self.pos,
            src_id: 0,
            start_pos: Some(start_pos).into(),
            drop_expiration: expiration.into(),
            by_pet: false,
            u1_flag: false,
        }
    }

    fn leave_msg(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeaveMsg {
        let (leave_type, pickup_id) = match param {
            DropLeaveParam::Explode => (DropLeaveType::Explode, None),
            DropLeaveParam::PassConvex => (DropLeaveType::PassConvex, None),
            DropLeaveParam::PetSkill => (DropLeaveType::PetSkill, None),
            DropLeaveParam::ScreenScroll => (DropLeaveType::ScreenScroll, None),
            DropLeaveParam::TimeOut => (DropLeaveType::TimeOut, None),
            DropLeaveParam::UserPickup(id) => (DropLeaveType::UserPickup, Some(id.0)),
            DropLeaveParam::MobPickup(id) => (DropLeaveType::MobPickup, Some(id)),
            DropLeaveParam::PetPickup(id) => (DropLeaveType::PetPickup, Some(id)),
        };

        DropLeaveFieldResp {
            leave_type,
            id,
            pickup_id: pickup_id.into(),
        }
    }
}

pub type DropItemPool = Pool<Obj<DropItem>>;