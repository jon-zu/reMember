use std::collections::HashMap;

use anyhow::Context;
use shroom_pkt::{pkt::EncodeMessage, util::packet_buf::PacketBuf};

use crate::{act::BroadcastSet, net::session::NetMsg, GameTime, Id};

pub trait PoolCtx {
    type Id: Id;
    type Msg: NetMsg + Clone;

    fn t(&self) -> GameTime;
    fn tx(&mut self) -> &mut BroadcastSet<Self::Id, Self::Msg>;
    fn ctrl(&self) -> Option<Self::Id>;
}

pub trait PoolItem {
    type EnterMsg: EncodeMessage;
    type LeaveMsg: EncodeMessage;
    type LeaveParam;
    type Id: Id;

    fn enter_msg(&self, id: Self::Id, t: GameTime) -> Self::EnterMsg;
    fn leave_msg(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeaveMsg;
}

pub trait PoolObject: PoolItem {
    type Item: PoolItem;

    fn id(&self) -> Self::Id;
}

pub trait CtrlPoolItem: PoolObject {
    type ControllerId: Id;
    type AssignControllerMsg: EncodeMessage;

    fn enter_assign_ctrl_msg(
        &self,
        id: Self::Id,
        ctrl: Self::ControllerId,
    ) -> Self::AssignControllerMsg;
    fn assign_ctrl_msg(&self, id: Self::Id, ctrl: Self::ControllerId) -> Self::AssignControllerMsg;
    fn unassign_ctrl_msg(&self, id: Self::Id) -> Self::AssignControllerMsg;
}

#[derive(Debug)]
pub struct Pool<T: PoolObject>(pub HashMap<T::Id, T>);

impl<T: PoolObject> Default for Pool<T> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T: PoolObject> FromIterator<T> for Pool<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().map(|item| (item.id(), item)).collect())
    }
}

impl<T: PoolObject> Pool<T> {
    pub fn insert(&mut self, ctx: &mut impl PoolCtx, item: T) -> anyhow::Result<T::Id> {
        let id = item.id();
        self.0.insert(id, item);
        let item = &self.0[&id];
        let t = ctx.t();
        ctx.tx().broadcast_encode(item.enter_msg(id, t))?;
        Ok(id)
    }

    pub fn remove(
        &mut self,
        ctx: &mut impl PoolCtx,
        id: &T::Id,
        param: T::LeaveParam,
    ) -> anyhow::Result<Option<T>> {
        let Some(item) = self.0.remove(id) else {
            return Ok(None);
        };
        ctx.tx().broadcast_encode(item.leave_msg(*id, param))?;
        Ok(Some(item))
    }

    pub fn must_remove(
        &mut self,
        ctx: &mut impl PoolCtx,
        id: &T::Id,
        param: T::LeaveParam,
    ) -> anyhow::Result<T> {
        self.remove(ctx, id, param)?.context("Remove")
    }

    pub fn get(&self, id: &T::Id) -> Option<&T> {
        self.0.get(id)
    }

    pub fn get_mut(&mut self, id: &T::Id) -> Option<&mut T> {
        self.0.get_mut(id)
    }

    pub fn must_get(&self, id: &T::Id) -> anyhow::Result<&T> {
        self.get(id)
            .ok_or_else(|| anyhow::anyhow!("item not found"))
    }

    pub fn must_get_mut(&mut self, id: &T::Id) -> anyhow::Result<&mut T> {
        self.get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("item not found"))
    }

    pub fn on_enter(&self, packet_buf: &mut PacketBuf, t: GameTime) -> anyhow::Result<()> {
        for (id, item) in &self.0 {
            packet_buf.encode(item.enter_msg(*id, t))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CtrlPool<T: CtrlPoolItem> {
    pub pool: Pool<T>,
    pub controller: Option<T::ControllerId>,
}

impl<T: CtrlPoolItem> FromIterator<T> for CtrlPool<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            pool: iter.into_iter().collect(),
            controller: None,
        }
    }
}

impl<T: CtrlPoolItem> Default for CtrlPool<T> {
    fn default() -> Self {
        Self {
            pool: Pool::default(),
            controller: None,
        }
    }
}

impl<T: CtrlPoolItem> CtrlPool<T> {
    pub fn update_controller(
        &mut self,
        ctx: &mut impl PoolCtx<Id = T::ControllerId>,
        old_ctrl: Option<T::ControllerId>,
        ctrl: Option<T::ControllerId>,
        update: bool,
    ) -> anyhow::Result<()> {
        self.controller = ctrl;
        if update {
            if let Some(old) = old_ctrl {
                ctx.tx().send_all_to_encode(
                    old,
                    self.pool
                        .0
                        .values()
                        .map(|item| item.unassign_ctrl_msg(item.id())),
                )?;
            }

            if let Some(ctrl) = ctrl {
                ctx.tx().send_all_to_encode(
                    ctrl,
                    self.pool
                        .0
                        .values()
                        .map(|item| item.assign_ctrl_msg(item.id(), ctrl)),
                )?;
            }
        }

        Ok(())
    }

    pub fn check_controller(&self, id: &T::ControllerId) -> anyhow::Result<()> {
        if self.controller.as_ref() != Some(id) {
            return Err(anyhow::anyhow!("not controller"));
        }
        Ok(())
    }

    pub fn insert(&mut self, ctx: &mut impl PoolCtx<Id = T::ControllerId>, item: T) -> anyhow::Result<T::Id> {
        let id = item.id();
        self.pool.0.insert(id, item);
        let t = ctx.t();
        let item = &self.pool.0[&id];
        if let Some(ctrl) = self.controller {
            ctx.tx().send_to_encode(ctrl, item.enter_msg(id, t))?;
            ctx.tx().send_to_encode(ctrl, item.assign_ctrl_msg(id, ctrl))?;
        } else {
            ctx.tx().broadcast_encode(item.enter_msg(id, t))?;
        }
        Ok(id)
    }

    pub fn remove(
        &mut self,
        ctx: &mut impl PoolCtx,
        id: &T::Id,
        param: T::LeaveParam,
    ) -> anyhow::Result<Option<T>> {
        self.pool.remove(ctx, id, param)
    }

    pub fn get(&self, id: &T::Id) -> Option<&T> {
        self.pool.get(id)
    }

    pub fn get_mut(&mut self, id: &T::Id) -> Option<&mut T> {
        self.pool.get_mut(id)
    }

    pub fn must_get(&self, id: &T::Id) -> anyhow::Result<&T> {
        self.get(id)
            .ok_or_else(|| anyhow::anyhow!("item not found"))
    }

    pub fn must_get_mut(&mut self, id: &T::Id) -> anyhow::Result<&mut T> {
        self.get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("item not found"))
    }

    pub fn on_enter(
        &self,
        session_id: T::ControllerId,
        packet_buf: &mut PacketBuf,
        t: GameTime
    ) -> anyhow::Result<()> {
        if self.controller == Some(session_id) {
            for (id, item) in &self.pool.0 {
                packet_buf.encode(item.enter_msg(*id, t))?;
                packet_buf.encode(item.assign_ctrl_msg(*id, session_id))?;
            }
        } else {
            for (id, item) in &self.pool.0 {
                packet_buf.encode(item.enter_msg(*id, t))?;
            }
        }
        Ok(())
    }
}
