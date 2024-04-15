use std::sync::Arc;

use shroom_meta::{
    drops::QuestDropFlags,
    field::{FhTree, FieldLife},
    id::{CharacterId, FieldId, ItemId, MobId, Money, NpcId, ObjectId, ReactorId, SkillId},
    twod::{Box2, Range2, Vec2},
    FieldMeta, MetaService,
};
use shroom_pkt::util::packet_buf::PacketBuf;
use shroom_proto95::{
    game::{
        drop::DropOwner,
        life::{
            mob::{MobLeaveType, MobMoveReq},
            npc::NpcMoveReq,
        },
        user::remote::UserMoveResp,
    },
    shared::movement::MovePath,
};
use shroom_srv::{
    act::{
        room::{ControlMessage, RoomActor, RoomId},
        system::SystemRoomController,
        BroadcastSet, Context,
    },
    net::session::NetSession,
    util::delay_queue::DelayQueue,
};
use shroom_srv::{
    act::{session::SessionCell, SessionSet},
    game::pool::PoolCtx,
    GameTime,
};

use crate::{
    game::{GameMessage, GameSession},
    life::{
        char::Character,
        drop_item::{DropItem, DropItemPool, DropLeaveParam, DropTypeValue},
        employee::EmployeePool,
        minor::{
            AffectedArea, AffectedAreaPool, MessageBoxPool, OpenGatePool, TownPortal,
            TownPortalPool,
        },
        mob::{buffs::MobApplyDebuff, pool::MobPool, Mob},
        npc::{Npc, NpcPool},
        reactor::{Reactor, ReactorPool},
        Obj,
    },
    system::GameSystem,
};

pub trait AttackerContext {
    fn attacker(&self) -> CharacterId;

    fn get_reactor_quest_flag(&self, rid: ReactorId) -> Option<&QuestDropFlags>;
    fn get_mob_quest_flag(&self, mid: MobId) -> Option<&QuestDropFlags>;
}

pub struct CharSetRef<'a>(
    &'a mut [SessionCell<FieldHandler, <FieldHandler as RoomActor>::Session>],
);

impl<'a> CharSetRef<'a> {
    pub fn iter_mut_rect(&mut self, range: Option<Box2>) -> impl Iterator<Item = &mut Character> {
        self.0.iter_mut().filter_map(move |sess| {
            let chr = &mut sess.inner_mut().handler.session.char;
            range
                .map(|range| range.contains(chr.pos.to_point()))
                .unwrap_or(true)
                .then_some(chr)
        })
    }
}

type Tx = BroadcastSet<CharacterId, GameMessage>;

#[derive(Debug)]
pub enum FieldEvent {
    DropTimeout(ObjectId),
    AffectedAreaTimeout(ObjectId),
}

#[derive(Debug)]
pub struct SharedFieldState {
    pub field_meta: FieldMeta,
    pub field_fh: &'static FhTree,
}

#[derive(Debug)]
pub struct FieldHandler {
    meta: &'static MetaService,
    field_id: FieldId,

    shared: Arc<SharedFieldState>,
    events: DelayQueue<FieldEvent>,
    controller: Option<CharacterId>,

    drop_pool: DropItemPool,
    mob_pool: MobPool,
    npc_pool: NpcPool,
    reactor_pool: ReactorPool,
    affected_area_pool: AffectedAreaPool,
    employee_pool: EmployeePool,
    message_box_pool: MessageBoxPool,
    open_gate_pool: OpenGatePool,
    town_portal_pool: TownPortalPool,
}

impl FieldHandler {
    pub fn new(meta_svc: &'static MetaService, t: GameTime, shared: Arc<SharedFieldState>) -> Self {
        let meta = shared.field_meta;
        let npcs = meta
            .life
            .values()
            .filter_map(|life| match &life {
                FieldLife::Npc(n) => Some(n),
                _ => None,
            })
            .map(|npc| Npc {
                tmpl_id: npc.id,
                pos: npc.pos,
                fh: npc.fh,
                move_action: !npc.flip as u8,
                range_horz: Range2 {
                    low: *npc.range_x.start(),
                    high: *npc.range_x.end(),
                },
                enabled: true,
            })
            .map(Obj::next);

        let mobs = meta
            .life
            .values()
            .filter_map(|life| match life {
                FieldLife::Mob(m) if !m.hide => Some(m),
                _ => None,
            })
            .map(|m| {
                let meta = meta_svc.get_mob_data(m.id).unwrap();
                (m.id, meta, m)
            });

        let reactors = meta
            .reactors
            .values()
            .map(|r| Reactor::new(r.id, r))
            .map(Obj::next);

        Self {
            field_id: shared.field_meta.id,
            shared,
            drop_pool: DropItemPool::default(),
            mob_pool: MobPool::from_spawns(meta_svc, t, mobs),
            npc_pool: NpcPool::from_elems(npcs),
            reactor_pool: ReactorPool::from_elems(reactors),
            affected_area_pool: AffectedAreaPool::default(),
            message_box_pool: Default::default(),
            employee_pool: Default::default(),
            open_gate_pool: Default::default(),
            town_portal_pool: Default::default(),
            events: DelayQueue::new(),
            meta: meta_svc,
            controller: None,
        }
    }

    pub fn update_controller(
        &mut self,
        ctx: &mut FieldPoolCtx<'_>,
        old_ctrl: Option<CharacterId>,
        ctrl: Option<CharacterId>,
        update: bool,
    ) -> anyhow::Result<()> {
        self.controller = ctrl;
        self.mob_pool
            .update_controller(ctx, old_ctrl, ctrl, update)?;
        self.npc_pool
            .update_controller(ctx, old_ctrl, ctrl, update)?;
        Ok(())
    }
}

impl RoomActor for FieldHandler {
    type Session = NetSession<GameSession>;
    type Controller = SystemRoomController<GameSystem>;
    type Error = anyhow::Error;

    fn on_tick(ctx: &mut SessionSet<Self>) -> Result<(), Self::Error> {
        let t = ctx.time();

        ctx.ctx.room.mob_pool.on_tick(
            &mut FieldPoolCtx {
                tx: &mut ctx.ctx.tx,
                t,
                ctrl: ctx.ctx.room.controller,
            },
            &mut CharSetRef(&mut ctx.actors),
        )?;

        for event in ctx.ctx.room.events.drain_expired(t) {
            log::info!("Field event: {:?}", event);
            match event {
                FieldEvent::DropTimeout(id) => {
                    // Remove fail is not a problem
                    let _ = ctx.ctx.room.drop_pool.remove(
                        &mut FieldPoolCtx {
                            tx: &mut ctx.ctx.tx,
                            t,
                            ctrl: ctx.ctx.room.controller,
                        },
                        &id,
                        DropLeaveParam::TimeOut,
                    )?;
                }
                FieldEvent::AffectedAreaTimeout(id) => {
                    let _ = ctx.ctx.room.affected_area_pool.remove(
                        &mut FieldPoolCtx {
                            tx: &mut ctx.ctx.tx,
                            t,
                            ctrl: ctx.ctx.room.controller,
                        },
                        &id,
                        (),
                    );
                }
            }
        }

        Ok(())
    }

    fn on_msg(_ctx: &mut SessionSet<Self>, _msg: ControlMessage<Self>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn id(&self) -> RoomId<Self> {
        self.field_id
    }

    fn on_enter_session(
        ctx: &mut SessionSet<Self>,
        session: &mut Self::Session,
    ) -> Result<(), Self::Error> {
        session.handler.send_set_field(&mut session.socket)?;
        let t = ctx.ctx.time();
        let char = &session.handler.session.char;

        // Assign a new controller if there's none
        if ctx.ctx.room.controller.is_none() {
            ctx.ctx.room.update_controller(
                &mut FieldPoolCtx {
                    tx: &mut ctx.ctx.tx,
                    t,
                    ctrl: ctx.ctx.room.controller,
                },
                None,
                Some(char.id),
                false,
            )?;
        }

        let field = &mut ctx.ctx.room;

        // Send spawn packets
        let mut buf = PacketBuf::default();
        //TODO self.field.user_pool.on_enter(char.id, &mut buf)?;
        field.drop_pool.on_enter(&mut buf, t)?;
        field.npc_pool.on_enter(char.id, &mut buf, t)?;
        field.mob_pool.on_enter(char.id, &mut buf, t)?;
        field.reactor_pool.on_enter(&mut buf, t)?;
        field.affected_area_pool.on_enter(&mut buf, t)?;
        field.employee_pool.on_enter(&mut buf, t)?;
        field.message_box_pool.on_enter(&mut buf, t)?;
        field.open_gate_pool.on_enter(&mut buf, t)?;
        field.town_portal_pool.on_enter(&mut buf, t)?;
        session.socket.send_buf(buf)?;

        // Do the post init
        session.handler.init_char(&mut session.socket)?;

        Ok(())
    }

    fn on_leave_session(
        ctx: &mut SessionSet<Self>,
        session: &mut Self::Session,
    ) -> Result<(), Self::Error> {
        let field = &mut ctx.ctx.room;
        let char = &session.handler.session.char;
        let char_id = char.id;

        if field.controller == Some(char_id) {
            let next_controller = ctx
                .sessions()
                .map(|sess| sess.id())
                .find(|id| id != &char_id);
            let t = ctx.ctx.time();

            ctx.ctx.room.update_controller(
                &mut FieldPoolCtx {
                    tx: &mut ctx.ctx.tx,
                    t,
                    ctrl: next_controller,
                },
                None,
                next_controller,
                true,
            )?;
        }

        Ok(())
    }
}

pub struct FieldPoolCtx<'a> {
    pub tx: &'a mut Tx,
    pub t: GameTime,
    pub ctrl: Option<CharacterId>,
}

impl PoolCtx for FieldPoolCtx<'_> {
    type Id = CharacterId;
    type Msg = GameMessage;

    fn tx(&mut self) -> &mut Tx {
        self.tx
    }

    fn t(&self) -> GameTime {
        self.t
    }

    fn ctrl(&self) -> Option<Self::Id> {
        self.ctrl
    }
}

macro_rules! pool_ctx {
    ($ctx:ident) => {
        &mut FieldPoolCtx {
            tx: &mut $ctx.tx,
            t: $ctx.t,
            ctrl: $ctx.field.controller,
        }
    };
}
pub struct FieldContext<'a> {
    pub field: &'a mut FieldHandler,
    pub tx: &'a mut Tx,
    pub t: GameTime,
}

impl<'a> FieldContext<'a> {
    pub fn verify_controller(&mut self, id: CharacterId) -> anyhow::Result<()> {
        if self.field.controller != Some(id) {
            return Err(anyhow::anyhow!("Not controller"));
        }
        Ok(())
    }

    pub fn handle_user_move(
        &mut self,
        char_id: CharacterId,
        move_path: MovePath,
    ) -> anyhow::Result<()> {
        self.tx
            .broadcast_filter_encode(UserMoveResp { char_id, move_path }, char_id)?;
        Ok(())
    }

    pub fn add_npc(&mut self, npc: Npc) -> anyhow::Result<()> {
        self.field
            .npc_pool
            .insert(pool_ctx!(self), Obj::next(npc))?;
        Ok(())
    }

    pub fn remove_npc(&mut self, id: ObjectId) -> anyhow::Result<()> {
        self.field.npc_pool.remove(pool_ctx!(self), &id, ())?;
        Ok(())
    }

    pub fn handle_npc_move(
        &mut self,
        controller: CharacterId,
        req: NpcMoveReq,
    ) -> anyhow::Result<()> {
        self.verify_controller(controller)?;
        self.field.npc_pool.handle_move(pool_ctx!(self), req)?;
        Ok(())
    }

    pub fn add_mob(&mut self, mob: Mob) -> anyhow::Result<()> {
        self.field.mob_pool.spawn(pool_ctx!(self), mob)?;
        Ok(())
    }

    pub fn remove_mob(&mut self, id: ObjectId, leave: MobLeaveType) -> anyhow::Result<()> {
        //TODO
        self.field
            .mob_pool
            .remove(pool_ctx!(self), id, leave)
            .unwrap();
        Ok(())
    }

    pub fn update_mob_pos(
        &mut self,
        mv: MobMoveReq,
        controller: CharacterId,
    ) -> anyhow::Result<()> {
        self.field
            .mob_pool
            .handle_move(pool_ctx!(self), mv.id, mv, controller)?;
        Ok(())
    }

    pub fn set_mob_aggro(&mut self, mob: ObjectId, ctrl: CharacterId) -> anyhow::Result<()> {
        self.field
            .mob_pool
            .set_mob_aggro(pool_ctx!(self), mob, ctrl)?;
        Ok(())
    }

    pub fn debuff_mob(
        &mut self,
        id: ObjectId,
        debuff: &dyn MobApplyDebuff,
        attacker: CharacterId,
        src: SkillId,
    ) -> anyhow::Result<()> {
        self.field
            .mob_pool
            .debuff(pool_ctx!(self), id, debuff, attacker, src)?;
        Ok(())
    }

    pub fn attack_mob(
        &mut self,
        id: ObjectId,
        dmg: u32,
        attacker: impl AttackerContext,
        debuff: &Option<Box<dyn MobApplyDebuff>>,
        src: SkillId,
    ) -> anyhow::Result<()> {
        let Some(mob) =
            self.field
                .mob_pool
                .attack(pool_ctx!(self), &attacker, id, dmg)?
        else {
            if let Some(debuff) = debuff {
                self.field.mob_pool.debuff(
                    pool_ctx!(self),
                    id,
                    debuff.as_ref(),
                    attacker.attacker(),
                    src,
                )?;
                log::info!("Debuffed mob");
            }
            return Ok(());
        };

        let (items, money) = self
            .field
            .meta
            .get_drops_and_money_for_mob(mob.tmpl_id, dbg!(&mob.quest_drop_flags));
        self.spread_drops(mob.pos, DropOwner::User(attacker.attacker()), &items, money)?;

        Ok(())
    }

    pub fn drop_money(&mut self, money: Money, pos: Vec2) -> anyhow::Result<()> {
        self.add_drop(DropItem {
            owner: DropOwner::None,
            pos,
            start_pos: pos,
            value: DropTypeValue::Mesos(money),
            quantity: 1,
        })
    }

    pub fn add_drop(&mut self, drop: DropItem) -> anyhow::Result<()> {
        let id = self
            .field
            .drop_pool
            .insert(pool_ctx!(self), Obj::next(drop))?;
        self.field
            .events
            .push(FieldEvent::DropTimeout(id), self.t.add_ms(60_000));
        Ok(())
    }

    pub fn spread_drops(
        &mut self,
        pos: Vec2,
        owner: DropOwner,
        drops: &[(ItemId, usize)],
        money: Money,
    ) -> anyhow::Result<()> {
        let fh = self
            .field
            .shared
            .field_fh
            .get_foothold_below((pos.x as f32, pos.y as f32 - 20.).into());

        let n = drops.len() + usize::from(money > 0);
        if n == 0 {
            return Ok(());
        }
        // Get spread for items + mesos, TODO mesos are optional, fix items being zero
        let mut spread = fh.map(|fh| fh.get_item_spread(pos.x as f32, n));

        fn map_coord(c: geo::Coord<f32>) -> Vec2 {
            Vec2::new(c.x as i16, c.y as i16)
        }

        if money > 0 {
            self.add_drop(DropItem {
                owner,
                pos: Vec2::from(
                    spread
                        .as_mut()
                        .and_then(|fh| fh.next().map(map_coord))
                        .unwrap_or(pos),
                ),
                start_pos: pos,
                value: DropTypeValue::Mesos(money),
                quantity: 1,
            })?;
        }

        for (item, quantity) in drops.iter().copied() {
            self.add_drop(DropItem {
                owner,
                pos: spread
                    .as_mut()
                    .and_then(|fh| fh.next().map(map_coord))
                    .unwrap_or(pos),
                start_pos: pos,
                value: DropTypeValue::Item(item),
                quantity,
            })?;
        }

        Ok(())
    }

    pub fn try_loot_drop(&mut self, id: ObjectId, looter: CharacterId) -> Option<DropItem> {
        self.remove_drop(id, DropLeaveParam::UserPickup(looter))
            .unwrap()
    }

    pub fn remove_drop(
        &mut self,
        id: ObjectId,
        leave: DropLeaveParam,
    ) -> anyhow::Result<Option<DropItem>> {
        Ok(self
            .field
            .drop_pool
            .remove(pool_ctx!(self), &id, leave)?
            .map(|m| m.item))
    }

    pub fn chat(&mut self, msg: shroom_proto95::game::chat::UserChatMsgResp) -> anyhow::Result<()> {
        self.tx.broadcast_encode(msg)?;
        Ok(())
    }

    pub fn add_town_portal(&mut self, portal: TownPortal) -> anyhow::Result<()> {
        self.field
            .town_portal_pool
            .insert(pool_ctx!(self), Obj::next(portal))?;
        Ok(())
    }

    pub fn get_town_portal_target(&self, id: ObjectId) -> anyhow::Result<FieldId> {
        Ok(self.field.town_portal_pool.must_get(&id)?.target_map)
    }

    pub fn attack_reactor(
        &mut self,
        id: ObjectId,
        atk: impl AttackerContext,
    ) -> anyhow::Result<()> {
        let meta = self.field.meta;
        if let Some(reactor) = self.field.reactor_pool.attack(pool_ctx!(self), &atk, id)? {
            let drops = meta.get_reactor_drops(reactor.tmpl_id, &reactor.quest_drop_flags);
            self.spread_drops(reactor.pos, DropOwner::User(atk.attacker()), &drops, 10)?;
        }

        Ok(())
    }

    pub fn update_controller(
        &mut self,
        old_ctrl: Option<CharacterId>,
        ctrl: Option<CharacterId>,
        update: bool,
    ) -> anyhow::Result<()> {
        self.field.controller = ctrl;
        self.field
            .mob_pool
            .update_controller(pool_ctx!(self), old_ctrl, ctrl, update)?;
        self.field
            .npc_pool
            .update_controller(pool_ctx!(self), old_ctrl, ctrl, update)?;
        Ok(())
    }

    pub fn create_affected_area(&mut self, aff: AffectedArea) -> anyhow::Result<()> {
        let id = self
            .field
            .affected_area_pool
            .insert(pool_ctx!(self), Obj::next(aff))?;
        log::info!("Added effected area with id: {}", id);
        //self.field.events.push(FieldEvent::AffectedAreaTimeout(id), self.t.add_ms(aff.duration));
        Ok(())
    }

    pub fn get_npc_tmpl_id(&self, id: ObjectId) -> Option<NpcId> {
        self.field.npc_pool.get(&id).map(|n| n.tmpl_id)
    }
}
