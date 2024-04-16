use std::{net::IpAddr, num::Wrapping, ops::Neg, time::Duration};

use either::Either;
use scripts_lib::NpcHandle;
use shroom_data::entity_ext::FuncKey;
use shroom_meta::{
    buffs::char::{CharBuffMad, CharBuffPad},
    id::{
        item_id::InventoryType, BuffId, CharacterId, FieldId, MobId, Money, NpcId, ObjectId,
        QuestId, SkillId,
    },
    tmpl::item::BundleItemValue,
    FieldMeta, MetaService, QuestDataId,
};
use shroom_pkt::{
    partial::PartialFlag, pkt::Message, DecodePacket, ShroomExpirationTime, ShroomIndexListZ16,
    ShroomList16, ShroomTime,
};
use shroom_proto95::{
    game::{
        chat::{ChatMsgReq, UserChatMsgResp},
        field::{
            CrcSeed, FieldCharData, FieldTransferData, LogoutGiftConfig, NotificationList,
            SetFieldResp,
        },
        friend::{FriendList, FriendResultResp},
        key_map::{
            FuncKeyMapChangeReq, FuncKeyMapInitResp, QuickSlotInitResp, QuickslotKeyMapChangedReq,
        },
        life::{
            mob::{MobApplyCtrlReq, MobMoveReq},
            npc::{NpcMoveReq, UserSelectNpcReq},
            reactor::ReactorHitReq,
            summon::{SummonAttackReq, SummonSkillReq},
            town_portal::TownPortalEnterReq,
        },
        quest::{
            ConstantU8, QuestRecordMessageResp, QuestState, UserQuestReq, UserQuestResultResp,
            UserQuestSuccessResult,
        },
        script::{ScriptAnswerReq, ScriptMessageResp},
        user::{
            char::{CharDataAll, CharDataFlags},
            effect::{
                ItemHyperUpgradeEffectResp, ItemUpgradeEffectResp, LocalUserEffectResp, UserEffect,
            },
            secondary_stats::{LocalSecondaryStatResetResp, LocalSecondaryStatSetResp2},
            AttackTargetInfo, ChangeSkillRecordResp, Hits, UserBodyAttackReq, UserDropMoneyReq,
            UserDropPickUpReq, UserHitReq, UserMagicAttackReq, UserMeleeAttackReq, UserMoveReq,
            UserShotAttackReq, UserSkillCancelReq, UserSkillUpReq, UserSkillUseReq,
            UserStatChangeReq, UserTransferFieldReq,
        },
        BroadcastMessageResp, ClaimSvrStatusChangedResp, CtxSetGenderResp, UserPortalScriptReq,
    },
    login::{ChannelId, ClientKey, WorldId},
    recv_opcodes::RecvOpcodes,
    shared::{
        char::{
            CharDataEquipped, CharDataHeader, CharDataStat, CharStatChangedResp, QuestCompleteInfo,
            QuestInfo, SkillInfo, SocialRecords, TeleportRockInfo,
        },
        inventory::{
            InvChangeSlotPosReq, InventoryOperationsResp, ItemHyperUpgradeReq,
            ItemStatChangeItemUseReq, ItemUpgradeReq,
        },
        item::Item,
    },
};
use shroom_script::npc::NpcAction;
use shroom_srv::{
    act::Context,
    net::{
        session::{Handler, NetMsg, NetSocket},
        socket::PktMsg,
    },
};

use crate::{
    life::{
        char::{buffs::CharBuffPacket, class::UseSkillData, quest::QuestCheckError, Character},
        drop_item::{DropItem, DropTypeValue},
    },
    repl::GameRepl,
    services::shared::SharedServices,
    session::{
        shroom_session_backend::SessionIngameData, shroom_session_manager::OwnedShroomGameSession,
    },
};

use super::field::FieldHandler;

pub type SessionId = CharacterId;

#[derive(Debug, Clone)]
pub enum GameMessage {
    Pkt(PktMsg),
    MobExp(MobId, u32, u8),
    ExpGain(u32),
}

impl From<PktMsg> for GameMessage {
    fn from(pkt: PktMsg) -> Self {
        GameMessage::Pkt(pkt)
    }
}

impl TryFrom<GameMessage> for PktMsg {
    type Error = GameMessage;

    fn try_from(value: GameMessage) -> Result<Self, Self::Error> {
        match value {
            GameMessage::Pkt(pkt) => Ok(pkt),
            m => Err(m),
        }
    }
}

impl NetMsg for GameMessage {
    fn into_pkg_msg(self) -> Result<PktMsg, Self> {
        match self {
            GameMessage::Pkt(pkt) => Ok(pkt),
            m => Err(m),
        }
    }
}

pub struct GameSession {
    pub services: SharedServices,
    pub session: OwnedShroomGameSession,
    pub addr: IpAddr,
    pub channel_id: ChannelId,
    pub world_id: WorldId,
    pub client_key: ClientKey,
    pub field_id: FieldId,
    pub field_meta: FieldMeta,
    pub repl: GameRepl,
    pub current_script: Option<NpcHandle>,
    pub field_key: Wrapping<u8>,
}

#[macro_export]
macro_rules! field {
    ($ctx:ident) => {
        $crate::field::FieldContext {
            t: $ctx.time(),
            field: &mut $ctx.room.room,
            tx: &mut $ctx.room.tx,
        }
    };
}

macro_rules! op_handler {
    ($this:ident, $op:ident, $ctx:ident, $msg:ident, $default:ident, $($ty:ty => $handler:ident),*) => {
        match  $op {
            $(
                <$ty as shroom_pkt::HasOpCode>::OPCODE => $this.$handler($ctx, $this.decode_msg::<$ty>(&$msg)?),
            )*
            _ => {
                $this.$default($ctx, $msg)?;
                Ok(())
            }
        }
    };
}

pub type GameContext<'a> = shroom_srv::net::session::NetSessionContext<'a, GameSession>;

impl Handler for GameSession {
    type Id = SessionId;
    type RoomId = FieldId;
    type Room = FieldHandler;
    type Msg = GameMessage;
    type Error = anyhow::Error;

    fn id(&self) -> Self::Id {
        self.session.char.id
    }

    fn room_id(&self) -> Self::RoomId {
        self.field_id
    }

    fn on_enter_room(
        &mut self,
        _ctx: &mut shroom_srv::net::session::NetSessionContext<Self>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_leave_room(
        &mut self,
        _ctx: &mut shroom_srv::net::session::NetSessionContext<Self>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_msg(&mut self, _ctx: &mut GameContext, msg: Self::Msg) -> anyhow::Result<()> {
        match msg {
            // Will be handled earlier
            GameMessage::Pkt(_) => {}
            GameMessage::ExpGain(exp) => {
                self.session.char.add_exp(exp);
            }
            GameMessage::MobExp(mob_id, exp, _perc) => {
                self.session.char.add_exp(exp);
                self.session.char.quests.on_mob_killed(mob_id, 1);
            }
        }
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        self.update_char_stats(ctx)?;
        self.session.char.last_update = ctx.time();

        Ok(())
    }

    fn on_net_msg(&mut self, ctx: &mut GameContext, msg: Message) -> anyhow::Result<()> {
        let op: RecvOpcodes = msg.opcode()?;
        //log::info!("Handling op: {op:?}");
        let res = op_handler!(
            self,
            op,
            ctx,
            msg,
            handle_default,
            UserSkillUseReq =>  handle_use_skill,
            ReactorHitReq => handle_reactor_hit,
            MobMoveReq => handle_mob_move,
            NpcMoveReq => handle_npc_move,
            UserPortalScriptReq => handle_portal_script,
            UserMoveReq => handle_user_move,
            ChatMsgReq => handle_chat_msg,
            UserDropMoneyReq => handle_drop_money,
            UserDropPickUpReq => handle_drop_pick_up,
            UserMeleeAttackReq => handle_melee_attack,
            UserShotAttackReq => handle_shoot_attack,
            UserMagicAttackReq => handle_magic_attack,
            UserBodyAttackReq => handle_body_attack,
            UserSkillUpReq => handle_skill_up,
            InvChangeSlotPosReq => handle_inv_change_slot,
            UserStatChangeReq => handle_stat_change,
            UserTransferFieldReq => handle_field_transfer,
            ScriptAnswerReq => handle_script_answer_req,
            UserHitReq => handle_user_hit_req,
            UserSkillCancelReq => handle_skill_cancel,
            SummonSkillReq => handle_summon_use_skill,
            SummonAttackReq => handle_summon_attack,
            TownPortalEnterReq => handle_town_portal_enter,
            FuncKeyMapChangeReq => handle_func_key_map_change,
            QuickslotKeyMapChangedReq => handle_quick_slot_changed,
            MobApplyCtrlReq => handle_mob_apply_ctrl,
            UserQuestReq => handle_user_quest_req,
            ItemUpgradeReq => handle_item_upgrade,
            ItemHyperUpgradeReq => handle_item_hyper_upgrade,
            ItemStatChangeItemUseReq => handle_item_stat_change_use,
            UserSelectNpcReq => handle_select_npc
        );
        if res.is_err() {
            log::error!("Error handling op: {op:?}");
        }
        res?;
        self.update_char_stats(ctx)?;
        Ok(())
    }
}

impl shroom_script::SessionCtx for Character {
    fn meta(&self) -> &'static MetaService {
        self.game.meta
    }

    fn search_fields(&self, query: &str) -> Result<FieldId, Vec<(FieldId, String)>> {
        let res = self.game.meta.goto().get_or_query(query);
        match res {
            Ok(field) => Ok(field.0),
            Err(res) => Err(res.into_iter().map(|v| (v.1 .0, v.0.to_string())).collect()),
        }
    }

    fn send_msg(&mut self, msg: shroom_proto95::game::script::ScriptMessage) {
        self.npc_msg.push_back(msg);
    }

    fn set_money(&mut self, money: Money) {
        *self.stats.money_mut() = money;
    }

    fn update_money(&mut self, delta: i32) -> bool {
        let mut m = self.stats.money_mut();
        if let Some(new) = m.checked_add_signed(delta) {
            *m = new;
            true
        } else {
            false
        }
    }

    fn money(&self) -> Money {
        *(self.stats.money())
    }

    fn level(&self) -> u8 {
        *self.stats.level()
    }

    fn set_level(&mut self, level: u8) {
        *self.stats.level_mut() = level;
    }

    fn has_item(&self, id: shroom_meta::id::ItemId) -> bool {
        self.inventory.contains_id(&id).unwrap_or(false)
    }

    fn try_give_item(
        &mut self,
        id: shroom_meta::id::ItemId,
        quantity: usize,
    ) -> anyhow::Result<bool> {
        self.add_items(id, Some(quantity))?;
        Ok(true)
    }

    fn try_give_items(
        &mut self,
        items: &[(shroom_meta::id::ItemId, usize)],
    ) -> anyhow::Result<bool> {
        for (id, qty) in items {
            self.add_items(*id, Some(*qty))?;
        }

        log::info!("Adding items: {:?}", items);
        Ok(true)
    }

    fn job(&self) -> shroom_meta::id::job_id::JobId {
        self.stats.job
    }

    fn set_job(&mut self, job: shroom_meta::id::job_id::JobId) {
        self.change_job(job, false).unwrap();
    }

    fn say(&self, msg: &str) {
        log::info!("Saying: {}", msg);
    }

    fn transfer_field(&mut self, field_id: FieldId) {
        self.do_script_transfer = Some(field_id);
    }

    fn has_item_quantity(&self, id: shroom_meta::id::ItemId, count: usize) -> bool {
        self.inventory.get_quantity(id).unwrap_or(0) >= count
    }

    fn try_take_item(
        &mut self,
        item: shroom_meta::id::ItemId,
        count: usize,
    ) -> anyhow::Result<bool> {
        self.inventory.try_take_by_id(item, count)?;
        Ok(true)
    }

    fn try_take_items(
        &mut self,
        items: &[(shroom_meta::id::ItemId, usize)],
    ) -> anyhow::Result<bool> {
        for (id, qty) in items {
            self.inventory.try_take_by_id(*id, *qty)?;
        }
        Ok(true)
    }

    fn try_take_all_items(&mut self, id: shroom_meta::id::ItemId) -> anyhow::Result<usize> {
        self.inventory.try_take_all(id)
    }

    fn get_quest_state_data(&self, id: QuestDataId) -> Option<Vec<u8>> {
        self.quests.quest_data.get(&id).cloned()
    }

    fn set_quest_state_data(&mut self, id: QuestDataId, data: Vec<u8>) -> anyhow::Result<()> {
        self.quests.quest_data.insert(id, data);
        Ok(())
    }

    fn current_npc_id(&self) -> Option<NpcId> {
        self.npc_id
    }

    fn set_npc_id(&mut self, id: Option<NpcId>) {
        self.npc_id = id;
    }

    fn has_completed_quest(&self, id: QuestId) -> bool {
        self.quests.is_completed(id)
    }

    fn is_active_quest(&self, id: QuestId) -> bool {
        self.quests.is_active(id)
    }
}

impl GameSession {
    fn handle_select_npc(
        &mut self,
        ctx: &mut GameContext,
        req: UserSelectNpcReq,
    ) -> anyhow::Result<()> {
        /*let npc = self.field_meta.get_npc(req.npc_id).unwrap();
        let script = self.meta().get_script(npc.script_id).unwrap();
        let mut script = NpcHandle::new(script, self.session.char.id);
        self.start_script(ctx, script)?;*/

       /*  let script = match npc_id {
            NpcId(1022000) => "npc_job_adv",
            NpcId(1072000) => "npc_job2_adv",
            NpcId(1072004) => "npc_job2_inside",
            NpcId(2020008) => "npc_chief_warrior",
            NpcId(2030006) => "npc_holy_stone",
            NpcId(1061009) => "npc_mirror",
            NpcId(1061010) => "npc_mirror_inside",
            _ => todo!(),
        };*/

        let npc = field!(ctx).get_npc_tmpl_id(ObjectId(req.id.0)).unwrap();
        let script = self.services.game.scripts.get_npc_script_or_fallback(npc);
        self.start_script(ctx, script)?;

        Ok(())
    }

    fn handle_item_stat_change_use(
        &mut self,
        ctx: &mut GameContext,
        req: ItemStatChangeItemUseReq,
    ) -> anyhow::Result<()> {
        dbg!(&req);

        let slot = (InventoryType::Consume, req.use_slot as i16).try_into()?;
        let it =
            self.session
                .char
                .inventory
                .drop_stack_item(InventoryType::Consume, slot, Some(1))?;

        let meta = self.meta().items().consume.get(&it.0).unwrap();
        dbg!(&meta);

        let BundleItemValue::StateChange(ref state) = meta.value else {
            log::info!("Invalid state change item: {:?}", it);
            return Ok(());
        };

        let Some(_time) = state.time else {
            log::info!("No time set for state change item: {:?}", it);
            return Ok(());
        };

        let t = state.time.unwrap_or(Duration::ZERO);

        if state.pad.0 != 0 {
            self.session.char.buffs.set(
                ctx.time(),
                CharBuffPad::new(BuffId::Item(it.0), (state.pad.0 as i16).into(), t),
            );
        }

        if state.mad.0 != 0 {
            self.session.char.buffs.set(
                ctx.time(),
                CharBuffMad::new(BuffId::Item(it.0), (state.mad.0 as i16).into(), t),
            );
        }

        self.user_effect(ctx, UserEffect::BuffItemEffect(it.0))?;

        Ok(())
    }

    fn user_effect(&mut self, ctx: &mut GameContext, eff: UserEffect) -> anyhow::Result<()> {
        ctx.socket.reply(LocalUserEffectResp(eff))?;
        Ok(())
    }

    fn handle_item_hyper_upgrade(
        &mut self,
        ctx: &mut GameContext,
        req: ItemHyperUpgradeReq,
    ) -> anyhow::Result<()> {
        let eq = (InventoryType::Equipped, req.equip_slot as i16).try_into()?;
        let use_slot = (InventoryType::Consume, req.use_slot as i16).try_into()?;

        let res =
            self.session
                .char
                .inventory
                .apply_enhance(eq, use_slot, self.services.game.meta)?;

        ctx.socket.reply(ItemHyperUpgradeEffectResp {
            char_id: self.char_id(),
            enchant_skill: req.enchant_skill,
            success: res.is_success(),
            destroyed: res.is_destroyed(),
            ..Default::default()
        })?;

        Ok(())
    }

    fn handle_item_upgrade(
        &mut self,
        ctx: &mut GameContext,
        req: ItemUpgradeReq,
    ) -> anyhow::Result<()> {
        let eq = (InventoryType::Equipped, req.equip_slot as i16).try_into()?;
        let use_slot = (InventoryType::Consume, req.use_slot as i16).try_into()?;

        let res = self.session.char.inventory.apply_scroll(
            eq,
            use_slot,
            req.using_white_scroll.into(),
            self.services.game.meta,
        )?;

        ctx.socket.reply(ItemUpgradeEffectResp {
            char_id: self.char_id(),
            enchant_skill: req.enchant_skill,
            white_scroll: req.using_white_scroll.into(),
            success: res.is_success(),
            destroyed: res.is_destroyed(),
            ..Default::default()
        })?;

        Ok(())
    }

    fn handle_user_quest_req(
        &mut self,
        ctx: &mut GameContext,
        req: UserQuestReq,
    ) -> anyhow::Result<()> {
        log::info!("req: {req:?}");

        match req {
            UserQuestReq::AcceptQuest(q) => {
                let sess: &mut SessionIngameData = &mut self.session;
                match sess.char.try_accept_quest(q.id) {
                    Ok(()) => {
                        ctx.socket
                            .reply(UserQuestResultResp::Success(UserQuestSuccessResult {
                                id: q.id,
                                npc_tmpl_id: q.npc_tmpl_id,
                                next_quest: QuestId(0),
                            }))?;

                        /*ctx.socket.reply(QuestRecordMessageResp {
                            id: q.id,
                            state: QuestState::Accept("".to_string()),
                        })?;*/
                    }
                    Err(QuestCheckError::Field) => {
                        ctx.socket.reply(UserQuestResultResp::FailedUnknown(()))?;
                    }
                    Err(QuestCheckError::Job) => {
                        ctx.socket.reply(UserQuestResultResp::FailedUnknown(()))?;
                    }
                    Err(QuestCheckError::Level) => {
                        ctx.socket.reply(UserQuestResultResp::FailedUnknown(()))?;
                    }
                    Err(QuestCheckError::Inventory) => {
                        ctx.socket
                            .reply(UserQuestResultResp::FailedInventory(q.id))?;
                    }
                    Err(_) => todo!(),
                }
            }
            UserQuestReq::CompleteQuest(q) => {
                let mq = self.meta().get_quest(q.id).unwrap();
                self.session.char.try_complete_quest(q.id)?;

                ctx.socket
                    .reply(UserQuestResultResp::Success(UserQuestSuccessResult {
                        id: q.id,
                        npc_tmpl_id: q.npc_tmpl_id,
                        next_quest: mq.end_act.next_quest.unwrap_or(QuestId(0)),
                    }))?;

                ctx.socket.reply(QuestRecordMessageResp {
                    marker: ConstantU8,
                    id: q.id,
                    state: QuestState::Complete(ShroomTime::now()),
                })?;
            }
            _ => {
                log::info!("Quest msg: {req:?}");
            }
        }

        Ok(())
    }

    pub fn send_set_field(&mut self, sck: &mut NetSocket) -> anyhow::Result<()> {
        sck.reply(self.set_field(true))?;
        self.field_key += 1;
        Ok(())
    }

    pub fn init_char(&mut self, sck: &mut NetSocket) -> anyhow::Result<()> {
        sck.reply(FriendResultResp::Reset3(FriendList::empty()))?;
        sck.reply(FuncKeyMapInitResp::from(
            self.session.char.key_map.func().map(|map| map.to_proto()),
        ))?;
        sck.reply(QuickSlotInitResp::from(
            self.session
                .char
                .key_map
                .quick_slots()
                .map(|map| map.to_proto()),
        ))?;
        sck.reply(ClaimSvrStatusChangedResp { connected: true })?;
        sck.reply(CtxSetGenderResp {
            gender: self.session.char.gender,
        })?;

        sck.reply(BroadcastMessageResp::PinkMessage("Hello".to_string()))?;
        self.session.char.unlock_char();

        Ok(())
    }

    fn decode_msg<'de, T: DecodePacket<'de>>(
        &self,
        msg: &'de Message,
    ) -> shroom_pkt::PacketResult<T> {
        let res = msg.decode();
        if let Err(shroom_pkt::Error::EOF(ref eof)) = res {
            if let Some(ref eof_handler) = self.services.eof_handler {
                let _ = eof_handler.handle_eof(self.char_id(), msg.as_ref(), eof);
            }
        }

        res
    }

    fn handle_default(
        &mut self,
        _ctx: &mut GameContext,
        msg: shroom_pkt::pkt::Message,
    ) -> anyhow::Result<()> {
        log::info!(
            "Unhandled msg: {:?}({:2X})",
            msg.opcode::<RecvOpcodes>(),
            msg.opcode_value()
        );
        Ok(())
    }

    fn char_id(&self) -> CharacterId {
        self.session.char.id
    }

    fn run_script(&mut self, script: &mut NpcHandle, input: NpcAction) -> anyhow::Result<()> {
        script.step(&mut self.session.char, input)?;

        Ok(())
    }

    pub fn start_script(&mut self, ctx: &mut GameContext, script: NpcHandle) -> anyhow::Result<()> {
        log::info!("About to start script");
        self.current_script = Some(script);
        self.poll_npc(ctx, NpcAction::Start)?;
        log::info!("Waiting for next poll");
        Ok(())
    }

    fn poll_npc(&mut self, ctx: &mut GameContext, input: NpcAction) -> anyhow::Result<()> {
        let mut script = self
            .current_script
            .take()
            .ok_or_else(|| anyhow::format_err!("No script"))?;
        let is_end = matches!(input, NpcAction::End);
        let res = self.run_script(&mut script, input);
        if !is_end {
            res?;
        }

        if let Some(msg) = self.session.char.npc_msg.pop_front() {
            ctx.socket.reply(ScriptMessageResp {
                script_flag: 0x4, // Replace ByNpc
                speaker_id: script.npc_id().0,
                msg,
            })?;
        }

        if script.is_finished() {
            self.session.char.npc_msg.clear();
            self.session.char.unlock_char();
            if let Some(transfer) = self.session.char.do_script_transfer.take() {
                self.do_field_transfer(ctx, transfer, None)?;
            }

            return Ok(());
        }

        self.current_script = Some(script);

        Ok(())
    }

    pub fn enable_char(&mut self) {
        self.session.char.unlock_char()
    }

    pub fn meta(&self) -> &'static MetaService {
        self.services.game.meta
    }

    fn update_char_stats(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        if let Some(partial) = self.session.char.get_stats_update() {
            ctx.socket.reply(CharStatChangedResp {
                excl: true, //TODO handle this
                stats: PartialFlag {
                    hdr: (),
                    data: partial,
                },
                secondary_stat: false,
                battle_recovery: false,
            })?;
        }

        if let Some(ops) = self.session.char.get_inv_op_updates() {
            ctx.socket.reply(InventoryOperationsResp {
                reset_excl: true,
                operations: ops.into(),
                secondary_stat_changed: false,
            })?;
        }

        if let Some(skills) = self.session.char.skills.get_updates() {
            ctx.socket.reply(ChangeSkillRecordResp {
                reset_excl: true,
                skill_records: skills.into(),
                updated_secondary_stat: false,
            })?;
        }

        if let Some(skill_cd) = self.session.char.skills.get_cooldown_updates(ctx.time()) {
            for cd in skill_cd {
                ctx.socket.reply(cd)?;
            }
        }

        let removals = self.session.char.buffs.update_expirations(ctx.time());
        if !removals.is_empty() {
            log::info!("Removing buffs: {:?}", removals);
            ctx.socket.reply(LocalSecondaryStatResetResp {
                flags: removals,
                movement_affecting: true,
            })?;
        }

        let updated_stats = self.session.char.buffs.take_updated();
        if !updated_stats.is_empty() {
            log::info!("Updated stats: {:?}", updated_stats);
            let stats = CharBuffPacket {
                buffs: &self.session.char.buffs,
                flags: updated_stats,
                t: ctx.time(),
            };
            ctx.socket.reply(LocalSecondaryStatSetResp2 {
                stats,
                delay: Duration::ZERO.into(),
                movement_affecting: Some(true).into(),
            })?;
        }

        for (qid, qr) in self.session.char.quests.updates_states() {
            log::info!("Quest update: {:?} with {qr}", qid);
            ctx.socket.reply(QuestRecordMessageResp {
                marker: ConstantU8,
                id: qid,
                state: QuestState::Accept(qr),
            })?;
        }

        Ok(())
    }

    fn handle_use_skill(
        &mut self,
        ctx: &mut GameContext,
        req: UserSkillUseReq,
    ) -> anyhow::Result<()> {
        log::info!("Affected mobs: {:?}", req.affected_mobs);
        let data = UseSkillData {
            skill_id: req.skill_id,
            pos: req.pos.0,
            spirit_javelin_item: req.spirit_javelin_item.0,
            t: ctx.time(),
            buff_ix: None,
            affected_mobs: req.affected_mobs.iter().cloned().collect(),
        };
        self.session.char.use_skill(&data, ctx)?;
        /*if req.affected_mobs.iter().next().is_some() {
            if let Some(atk_data) = self.session.char.get_attack_data(req.skill_id)? {
                for mob in req.affected_mobs.iter() {
                    field!(ctx).debuff_mob(
                        *mob,
                        atk_data.as_ref(),
                        self.session.char.id,
                        req.skill_id,
                    )?;
                }
            }
        }*/

        /*
        if let Some(summon) = self.session.char.do_summon.take() {
            self.session.char.summon_skill_id = Some(req.skill_id);
            field!(ctx).summon_spawn(summon)?;
        }

        if let Some(mystic_door) = self.session.char.do_mystic_door.take() {
            let chr = &self.session.char;
            field!(ctx).add_town_portal(TownPortal {
                char_id: chr.id,
                state: 1,
                pos: chr.pos,
                target_map: mystic_door,
            })?;
        }
        */
        self.session.char.handle_update(ctx)?;

        Ok(())
    }

    fn handle_summon_use_skill(
        &mut self,
        ctx: &mut GameContext,
        req: SummonSkillReq,
    ) -> anyhow::Result<()> {
        let data = UseSkillData {
            skill_id: req.skill_id,
            pos: None,
            spirit_javelin_item: None,
            t: ctx.time(),
            buff_ix: req.buff_ix.0.map(|x| x as u8),
            affected_mobs: Vec::default(),
        };
        self.session.char.use_skill(&data, ctx)?;

        dbg!(&req);

        Ok(())
    }

    fn handle_summon_attack(
        &mut self,
        ctx: &mut GameContext,
        req: SummonAttackReq,
    ) -> anyhow::Result<()> {
        let Some(summon) = self.session.char.get_summon(req.summon_id) else {
            return Ok(());
        };

        let atk: Vec<AttackTargetInfo> = req
            .targets
            .iter()
            .map(|t| AttackTargetInfo {
                mob_id: t.mob_id,
                hit_action: t.hit_action,
                fore_action: t.fore_action.clone(),
                frame_id: t.frame_id,
                calc_damage_stat_ix: t.calc_damage_stat_ix,
                pos: t.pos,
                pos_prev: t.pos_prev,
                delay: t.delay,
                mob_crc: 0,
                hits: Hits::single(t.hit),
            })
            .collect();
        self.handle_attack(ctx, atk, summon.skill_id)?;

        Ok(())
    }

    fn handle_script_answer_req(
        &mut self,
        ctx: &mut GameContext,
        req: ScriptAnswerReq,
    ) -> anyhow::Result<()> {
        self.poll_npc(ctx, req.into())
    }

    fn handle_reactor_hit(
        &mut self,
        ctx: &mut GameContext,
        req: ReactorHitReq,
    ) -> anyhow::Result<()> {
        field!(ctx).attack_reactor(req.id, &self.session.char)?;
        Ok(())
    }

    fn handle_mob_apply_ctrl(
        &mut self,
        ctx: &mut GameContext,
        req: MobApplyCtrlReq,
    ) -> anyhow::Result<()> {
        field!(ctx).set_mob_aggro(req.mob_id, self.char_id())?;
        Ok(())
    }

    fn handle_mob_move(&mut self, ctx: &mut GameContext, req: MobMoveReq) -> anyhow::Result<()> {
        field!(ctx).update_mob_pos(req, self.char_id())?;
        Ok(())
    }

    fn handle_portal_script(
        &mut self,
        _ctx: &mut GameContext,
        _req: UserPortalScriptReq,
    ) -> anyhow::Result<()> {
        self.enable_char();
        Ok(())
    }

    fn handle_user_move(&mut self, ctx: &mut GameContext, req: UserMoveReq) -> anyhow::Result<()> {
        let chr = &mut self.session.char;
        chr.pos = req.move_path.pos;
        let last = req.move_path.get_last_pos_fh();

        if let Some((pos, fh)) = last {
            chr.pos = pos;
            chr.fh = fh.unwrap_or(chr.fh);
        }
        field!(ctx).handle_user_move(self.char_id(), req.move_path)?;
        Ok(())
    }

    fn handle_npc_move(&mut self, ctx: &mut GameContext, req: NpcMoveReq) -> anyhow::Result<()> {
        field!(ctx).handle_npc_move(self.char_id(), req)?;
        Ok(())
    }

    fn handle_drop_money(
        &mut self,
        ctx: &mut GameContext,
        req: UserDropMoneyReq,
    ) -> anyhow::Result<()> {
        let can_drop = self.session.char.update_mesos((req.money as i32).neg());
        if can_drop {
            let char = &self.session.char;
            field!(ctx).drop_money(req.money, char.pos)?;
        }

        self.enable_char();

        Ok(())
    }

    fn handle_drop_pick_up(
        &mut self,
        ctx: &mut GameContext,
        req: UserDropPickUpReq,
    ) -> anyhow::Result<()> {
        let Some(item) = field!(ctx).try_loot_drop(req.drop_id, self.char_id()) else {
            return Ok(());
        };

        let char = &mut self.session.char;
        match item.value {
            DropTypeValue::Mesos(money) => {
                char.update_mesos(money as i32);
            }
            DropTypeValue::ExistingItem(item) => {
                //TODO handle persistent equip items
                char.add_equip_item(item.item_id)?;
            }
            DropTypeValue::Item(item_id) => {
                let inv_ty = item_id.get_inv_type()?;
                if !inv_ty.is_stack() {
                    char.add_equip_item(item_id)?;
                } else {
                    char.add_stack_item(inv_ty, item_id, item.quantity)?;
                };
            }
        }
        Ok(())
    }

    fn handle_attack(
        &mut self,
        ctx: &mut GameContext,
        targets: Vec<AttackTargetInfo>,
        skill_id: SkillId,
    ) -> anyhow::Result<()> {
        let skill_id = match skill_id {
            SkillId(0) => None,
            _ => Some(skill_id),
        };

        self.session.char.handle_attack(
            &crate::life::char::class::AttackData { skill_id, targets },
            ctx,
        )?;
        Ok(())
    }

    fn handle_melee_attack(
        &mut self,
        ctx: &mut GameContext,
        req: UserMeleeAttackReq,
    ) -> anyhow::Result<()> {
        self.handle_attack(ctx, req.targets, req.info.skill_id)
    }

    fn handle_shoot_attack(
        &mut self,
        ctx: &mut GameContext,
        req: UserShotAttackReq,
    ) -> anyhow::Result<()> {
        self.handle_attack(ctx, req.targets, req.info.hdr.skill_id)
    }

    fn handle_magic_attack(
        &mut self,
        ctx: &mut GameContext,
        req: UserMagicAttackReq,
    ) -> anyhow::Result<()> {
        self.handle_attack(ctx, req.targets, req.info.skill_id)
    }

    fn handle_body_attack(
        &mut self,
        ctx: &mut GameContext,
        req: UserBodyAttackReq,
    ) -> anyhow::Result<()> {
        self.handle_attack(ctx, req.targets, req.info.skill_id)
    }

    fn handle_user_hit_req(
        &mut self,
        _ctx: &mut GameContext,
        _req: UserHitReq,
    ) -> anyhow::Result<()> {
        //dbg!(&req);
        Ok(())
    }

    fn handle_skill_up(
        &mut self,
        _ctx: &mut GameContext,
        req: UserSkillUpReq,
    ) -> anyhow::Result<()> {
        self.session.char.skill_up(req.skill_id)?;
        Ok(())
    }

    fn handle_skill_cancel(
        &mut self,
        _ctx: &mut GameContext,
        req: UserSkillCancelReq,
    ) -> anyhow::Result<()> {
        self.session.char.buffs.cancel_by_id(req.skill_id.into());
        Ok(())
    }

    fn handle_inv_change_slot(
        &mut self,
        ctx: &mut GameContext,
        req: InvChangeSlotPosReq,
    ) -> anyhow::Result<()> {
        let count = (req.count != u16::MAX).then_some(req.count as usize);
        let drop = req.to == 0;
        let from = (req.inv_type, req.from).try_into()?;
        // Check for drop
        if drop {
            let item = self.session.char.inventory.drop_item(from, count)?;
            let chr_id = self.session.char.id;
            // TODO handle persistent equip items
            let drop = match item {
                Either::Left(eq) => DropItem {
                    owner: shroom_proto95::game::drop::DropOwner::User(chr_id),
                    pos: self.session.char.pos,
                    start_pos: self.session.char.pos,
                    value: DropTypeValue::Item(eq.item_id),
                    quantity: 1,
                },
                Either::Right(stack) => DropItem {
                    owner: shroom_proto95::game::drop::DropOwner::User(chr_id),
                    pos: self.session.char.pos,
                    start_pos: self.session.char.pos,
                    value: DropTypeValue::Item(stack.0),
                    quantity: stack.1,
                },
            };

            field!(ctx).add_drop(drop)?;
        } else {
            let to = (req.inv_type, req.to).try_into()?;
            self.session.char.inventory.move_item(from, to, count)?;
        }

        self.enable_char();
        Ok(())
    }

    pub fn do_field_transfer(
        &mut self,
        ctx: &mut GameContext,
        field: FieldId,
        spawn_portal: Option<&'static str>,
    ) -> anyhow::Result<()> {
        let field_meta = self.meta().get_field(field).unwrap();
        let spawn = match spawn_portal {
            Some(tn) => field_meta.get_spawn_point_by_name(tn).unwrap(),
            _ => field_meta.get_default_spawn_point().unwrap(),
        };

        ctx.room.change_room(field)?;
        self.session.char.transfer_map(field, spawn);
        self.field_id = field;
        self.field_meta = field_meta;
        log::info!("Transfering map");
        Ok(())
    }

    fn handle_field_transfer(
        &mut self,
        ctx: &mut GameContext,
        req: UserTransferFieldReq,
    ) -> anyhow::Result<()> {
        let (field, spawn) = if self.session.char.is_dead() {
            self.session.char.respawn();
            (self.field_meta.get_return_field_id(), None)
        } else {
            let (target, portal) = self
                .field_meta
                .get_target_field(&req.portal)
                .ok_or_else(|| anyhow::format_err!("Invalid portal"))?;
            (target, portal.tn.as_deref())
        };

        self.do_field_transfer(ctx, field, spawn)?;
        Ok(())
    }

    fn handle_town_portal_enter(
        &mut self,
        ctx: &mut GameContext,
        req: TownPortalEnterReq,
    ) -> anyhow::Result<()> {
        // Get town portal
        let target =
            field!(ctx).get_town_portal_target(shroom_meta::id::ObjectId(req.char_or_party_id))?;
        self.do_field_transfer(ctx, target, None)?;
        Ok(())
    }

    fn handle_stat_change(
        &mut self,
        _ctx: &mut GameContext,
        req: UserStatChangeReq,
    ) -> anyhow::Result<()> {
        let char = &mut self.session.char;
        char.stats.update_hp(req.hp as i32);
        char.stats.update_mp(req.mp as i32);
        Ok(())
    }

    fn handle_func_key_map_change(
        &mut self,
        _ctx: &mut GameContext,
        req: FuncKeyMapChangeReq,
    ) -> anyhow::Result<()> {
        let key_map = self.session.char.key_map.func_mut();

        match req {
            FuncKeyMapChangeReq::Changed(changes) => {
                for (key, key_fn) in changes.iter() {
                    key_map.set(
                        *key as usize,
                        FuncKey {
                            ty: key_fn.ty,
                            action: key_fn.action_id,
                        },
                    )?;
                }

                log::info!("Updated func key");
            }
            _ => {
                log::info!("Unhandled func key map change: {:?}", req);
            }
        }
        Ok(())
    }

    fn handle_quick_slot_changed(
        &mut self,
        _ctx: &mut GameContext,
        req: QuickslotKeyMapChangedReq,
    ) -> anyhow::Result<()> {
        self.session.char.key_map.set_quick_slots(req.0);
        Ok(())
    }

    fn handle_chat_msg(&mut self, ctx: &mut GameContext, req: ChatMsgReq) -> anyhow::Result<()> {
        let admin = false;
        if let Some(s) = req.msg.strip_prefix('@') {
            log::info!("repl: {}", s);
            let repl_resp = self.handle_repl(ctx, s)?;
            if let Some(msg) = repl_resp {
                ctx.socket.reply(UserChatMsgResp {
                    char: self.session.char.id,
                    is_admin: admin,
                    msg,
                    only_balloon: false,
                })?;
            }
        } else {
            field!(ctx).chat(UserChatMsgResp {
                char: self.session.char.id,
                is_admin: admin,
                msg: req.msg,
                only_balloon: req.only_balloon,
            })?;
        };
        Ok(())
    }

    fn set_field(&self, char_data: bool) -> SetFieldResp {
        let field_data = if char_data {
            let char = &self.session.char;
            let inv = &char.inventory;

            let equipped: ShroomIndexListZ16<Item> = self
                .session
                .char
                .inventory
                .invs
                .equipped
                .item_slots()
                .map(|(slot, item)| (slot.0 as u16, Item::Equip(item.0.item.as_ref().into())))
                .collect();

            let equip: ShroomIndexListZ16<Item> = self
                .session
                .char
                .inventory
                .invs
                .equip
                .item_slots()
                .map(|(slot, item)| (slot as u16 + 1, Item::Equip(item.item.as_ref().into())))
                .collect();

            let char_equipped = CharDataEquipped {
                equipped,
                equip,
                ..Default::default()
            };

            let skillrecords: ShroomList16<SkillInfo> =
                self.session.char.skills.get_skill_info().into();

            let quests = self
                .session
                .char
                .quests
                .active_quest_records()
                .map(|q| QuestInfo {
                    id: q.0,
                    value: q.1,
                });

            let completed =
                self.session
                    .char
                    .quests
                    .completed_records()
                    .map(|q| QuestCompleteInfo {
                        id: q.0,
                        time: q.1.try_into().unwrap(),
                    });

            let char_data = CharDataAll {
                stat: CharDataStat {
                    stat: char.get_all_stats(),
                    friend_max: 30,
                    linked_character: None.into(),
                },
                money: char.money(),
                invsize: char.inventory.inv_size(),
                equipextslotexpiration: ShroomExpirationTime::never(),
                equipped: char_equipped,
                consumeinv: inv.get_stack_inv_list(InventoryType::Consume),
                setupinv: inv.get_stack_inv_list(InventoryType::Install),
                etcinv: inv.get_stack_inv_list(InventoryType::Etc),
                cashinv: inv.get_cash_inv_list(),
                skillrecords,
                skllcooltime: ShroomList16::default(),
                quests: quests.collect(),
                questscompleted: completed.collect(),
                minigamerecords: ShroomList16::default(),
                socialrecords: SocialRecords::default(),
                teleportrockinfo: TeleportRockInfo::default(),
                newyearcards: ShroomList16::default(),
                questrecordsexpired: ShroomList16::default(),
                questcompleteold: ShroomList16::default(),
                visitorquestloginfo: ShroomList16::default(),
            };

            Either::Left(FieldCharData {
                seed: CrcSeed {
                    s1: 1,
                    s2: 2,
                    s3: 3,
                },
                logout_gift_config: LogoutGiftConfig {
                    predict_quit: 0,
                    gift_commodity_id: [0; 3],
                },
                char_data_hdr: CharDataHeader {
                    combat_orders: 0,
                    extra_data: None.into(),
                },
                char_data,
                char_data_flags: CharDataFlags::all(),
            })
        } else {
            Either::Right(FieldTransferData {
                revive: false,
                map: self.session.char.field,
                portal: self.session.char.spawn_point.id,
                hp: self.session.char.stats.hp.value,
                chase_target_pos: None.into(),
            })
        };

        SetFieldResp {
            client_option: ShroomList16::default(),
            channel_id: self.channel_id as u32,
            has_char_data: field_data.is_left(),
            char_data: field_data.into(),
            notifications: NotificationList::default(),
            old_driver_id: 0.into(),
            field_key: self.field_key.0,
            server_time: ShroomTime::now(),
        }
    }
}
