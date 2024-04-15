use shroom_meta::{
    id::{FootholdId, ObjectId},
    twod::Vec2,
};
use shroom_proto95::game::life::employee::{
    EmployeeCreateResp, EmployeeMiniRoomBalloon, EmployeeMiniRoomBalloonResp, EmployeeRemoveResp,
};
use shroom_srv::{
    game::pool::{Pool, PoolCtx, PoolItem},
    GameTime,
};

use super::Obj;

#[derive(Debug)]
pub struct Employee {
    pub tmpl_id: u32,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub char_name: String,
    pub room_type: u8,
    pub balloon: Option<EmployeeMiniRoomBalloon>,
}

impl PoolItem for Employee {
    type Id = ObjectId;
    type EnterMsg = EmployeeCreateResp;
    type LeaveMsg = EmployeeRemoveResp;
    type LeaveParam = ();

    fn enter_msg(&self, id: Self::Id, _t: GameTime) -> Self::EnterMsg {
        EmployeeCreateResp {
            id,
            employee_tmpl_id: self.tmpl_id,
            pos: self.pos,
            fh: self.fh,
            char_name: self.char_name.clone(),
            balloon: self.balloon.clone().into(),
        }
    }

    fn leave_msg(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeaveMsg {
        EmployeeRemoveResp { id }
    }
}

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct EmployeePool(pub Pool<Obj<Employee>>);

impl EmployeePool {
    pub fn update_balloon(
        &mut self,
        ctx: &mut impl PoolCtx,
        id: ObjectId,
        balloon: Option<EmployeeMiniRoomBalloon>,
    ) -> anyhow::Result<()> {
        let employee = self.must_get_mut(&id)?;
        employee.balloon = balloon.clone();

        ctx.tx().broadcast_encode(EmployeeMiniRoomBalloonResp {
            employee_id: id,
            balloon: balloon.into(),
        })?;
        Ok(())
    }
}
