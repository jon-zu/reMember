use shroom_meta::{
    id::{CharacterId, FootholdId, NpcId, ObjectId},
    twod::{Range2, Vec2},
};

use shroom_proto95::game::life::npc::{
    NpcChangeControllerResp, NpcData, NpcEnterFieldResp, NpcLeaveFieldResp, NpcMove,
    NpcMoveReq, NpcMoveResp,
};
use shroom_srv::{game::pool::{CtrlPool, CtrlPoolItem, PoolCtx, PoolItem}, GameTime};

use super::Obj;

#[derive(Debug)]
pub struct Npc {
    pub tmpl_id: NpcId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub move_action: u8,
    pub range_horz: Range2,
    pub enabled: bool,
}

impl Npc {
    fn npc_data(&self) -> NpcData {
        NpcData {
            template_id: self.tmpl_id,
            pos: self.pos,
            move_action: self.move_action,
            fh: self.fh,
            range_horz: self.range_horz,
            enabled: self.enabled,
        }
    }
}

impl PoolItem for Npc {
    type Id = ObjectId;

    type EnterMsg = NpcEnterFieldResp;
    type LeaveMsg = NpcLeaveFieldResp;
    type LeaveParam = ();

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        NpcEnterFieldResp {
            id,
            npc: self.npc_data(),
        }
    }

    fn leave_msg(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeaveMsg {
        NpcLeaveFieldResp { id }
    }
}

impl CtrlPoolItem for Obj<Npc> {
    type AssignControllerMsg = NpcChangeControllerResp;
    type ControllerId = CharacterId;

    fn enter_assign_ctrl_msg(
            &self,
            id: Self::Id,
            _ctrl: Self::ControllerId,
        ) -> Self::AssignControllerMsg {
        NpcChangeControllerResp {
            local: true,
            id,
            npc: Some(self.npc_data()).into(),
        }
    }

    fn assign_ctrl_msg(&self, id: Self::Id, _ctrl: Self::ControllerId) -> Self::AssignControllerMsg {
        NpcChangeControllerResp {
            local: true,
            id,
            npc: None.into(),
        }
    }

    fn unassign_ctrl_msg(&self, id: Self::Id) -> Self::AssignControllerMsg {
        NpcChangeControllerResp {
            local: false,
            id,
            npc: None.into(),
        }
    }
}

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct NpcPool(pub CtrlPool<Obj<Npc>>);

impl NpcPool {
    pub fn from_elems(elems: impl Iterator<Item = Obj<Npc>>) -> Self {
        Self(elems.collect())
    }

    pub fn handle_move(&mut self, ctx: &mut impl PoolCtx, req: NpcMoveReq) -> anyhow::Result<()> {
        let Ok(npc) = self.must_get_mut(&ObjectId(req.id.0)) else {
            return Ok(());
        };

        if let Some(ref move_path) = req.move_path.0 {
            let last_pos_fh = move_path.get_last_pos_fh();

            if let Some((pos, fh)) = last_pos_fh {
                //TODO post map state to msg state here
                npc.pos = pos;
                npc.fh = fh.unwrap_or(npc.fh);
            }
        };


        ctx.tx().broadcast_encode(NpcMoveResp {
            id: ObjectId(req.id.0),
            data: NpcMove {
                action: req.action,
                chat: req.chat_idx,
                move_path: req.move_path,
            },
        })?;
        Ok(())
    }
}
