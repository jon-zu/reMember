use std::num::Saturating;

use shroom_meta::{
    drops::QuestDropFlags, field::FieldReactor, id::{ObjectId, ReactorId}, twod::Vec2
};
use shroom_proto95::game::life::reactor::{ReactorEnterFieldResp, ReactorLeaveFieldResp};
use shroom_srv::{
    game::pool::{Pool, PoolCtx, PoolItem},
    GameTime,
};

use crate::field::AttackerContext;

use super::Obj;

#[derive(Debug)]
pub struct Reactor {
    pub pos: Vec2,
    pub tmpl_id: ReactorId,
    pub state: Saturating<u8>,
    pub name: Option<String>,
    pub quest_drop_flags: QuestDropFlags,
}

impl Reactor {
    pub fn new(id: ReactorId, reactor: &FieldReactor) -> Self {
        Self {
            pos: reactor.pos,
            tmpl_id: id,
            state: Saturating(0),
            name: reactor.name.clone(), // TODO remove that allocation
            quest_drop_flags: Default::default()
        }
    }

    pub fn attack(&mut self, flags: Option<&QuestDropFlags>) {
        self.state -= 1;
        if let Some(flags) = flags {
            self.quest_drop_flags.union(flags);
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state.0 == 0
    }
}

impl PoolItem for Reactor {
    type Id = ObjectId;

    type EnterMsg = ReactorEnterFieldResp;
    type LeaveMsg = ReactorLeaveFieldResp;
    type LeaveParam = ();

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        ReactorEnterFieldResp {
            id,
            tmpl_id: self.tmpl_id,
            state: self.state.0,
            pos: self.pos,
            flipped: false,
            name: self.name.clone().unwrap_or_default(),
        }
    }

    fn leave_msg(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeaveMsg {
        ReactorLeaveFieldResp {
            id,
            state: self.state.0,
            pos: self.pos,
        }
    }
}

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct ReactorPool(pub Pool<Obj<Reactor>>);

impl ReactorPool {
    pub fn from_elems(elems: impl Iterator<Item = Obj<Reactor>>) -> Self {
        Self(elems.collect())
    }

    pub fn attack(
        &mut self,
        ctx: &mut impl PoolCtx,
        atk: &impl AttackerContext,
        id: ObjectId
    ) -> anyhow::Result<Option<Reactor>> {
        let reactor = self.must_get_mut(&id)?;
        let quest_flags = atk.get_reactor_quest_flag(reactor.tmpl_id);
        reactor.attack(quest_flags);
        Ok(if reactor.is_dead() {
            self.remove(ctx, &id, ())?.map(|r| r.item)
        } else {
            None
        })
    }
}
