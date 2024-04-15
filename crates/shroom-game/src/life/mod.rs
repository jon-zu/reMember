use std::sync::atomic::AtomicU32;

use shroom_meta::id::ObjectId;
use shroom_srv::{game::pool::{CtrlPoolItem, PoolItem, PoolObject}, GameTime};

pub mod char;
pub mod drop_item;
pub mod employee;
pub mod minor;
pub mod mob;
pub mod npc;
pub mod reactor;

pub fn next_id() -> ObjectId {
    static ID: AtomicU32 = AtomicU32::new(0);
    ObjectId(ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
}

#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct Obj<T: PoolItem> {
    pub id: T::Id,
    #[deref]
    #[deref_mut]
    pub item: T,
}

impl<T: PoolItem<Id = ObjectId>> Obj<T> {
    pub fn new(id: T::Id, item: T) -> Self {
        Self { id, item }
    }

    pub fn next(item: T) -> Self {
        Self {
            id: next_id(),
            item,
        }
    }
}

impl<T: PoolItem> PoolItem for Obj<T> {
    type Id = T::Id;
    type EnterMsg = T::EnterMsg;
    type LeaveMsg = T::LeaveMsg;
    type LeaveParam = T::LeaveParam;

    fn enter_msg(&self, id: Self::Id, t: GameTime) -> Self::EnterMsg {
        self.item.enter_msg(id, t)
    }

    fn leave_msg(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeaveMsg {
        self.item.leave_msg(id, param)
    }
}

impl<T: PoolItem> PoolObject for Obj<T> {
    type Item = T;
    fn id(&self) -> T::Id {
        self.id
    }
}

impl<T: CtrlPoolItem> CtrlPoolItem for Obj<T> {
    type ControllerId = T::ControllerId;
    type AssignControllerMsg = T::AssignControllerMsg;

    fn enter_assign_ctrl_msg(
        &self,
        id: Self::Id,
        ctrl: Self::ControllerId,
    ) -> Self::AssignControllerMsg {
        self.item.enter_assign_ctrl_msg(id, ctrl)
    }

    fn assign_ctrl_msg(&self, id: Self::Id, ctrl: Self::ControllerId) -> Self::AssignControllerMsg {
        self.item.assign_ctrl_msg(id, ctrl)
    }

    fn unassign_ctrl_msg(&self, id: Self::Id) -> Self::AssignControllerMsg {
        self.item.unassign_ctrl_msg(id)
    }
}
