pub mod buffs;
pub mod class;
pub mod inv;
pub mod pet;
pub mod quest;
pub mod stats;
pub mod summon;

use std::{
    collections::VecDeque,
    ops::{Add, Div},
    time::Instant,
};

use chrono::Utc;
use itertools::Itertools;
use sea_orm::Set;
use shroom_data::{
    entities::character::{self, Model},
    entity_ext::KeyMap,
    model::{
        inv::{InventorySet, NoopInvSetHandler},
        skill::{SkillData, SkillSet},
    },
    services::{character::QuestSet, item::ItemService},
};
use shroom_meta::{
    class::HealBuff,
    field::SpawnPoint,
    id::{
        item_id::InventoryType, job_id::JobId, CharacterId, FaceId, FieldId, FootholdId, HairId, ItemId, NpcId, ObjectId, QuestId, SkillId, Skin
    },
    twod::Vec2,
};
use shroom_pkt::ShroomIndexList8;

use shroom_proto95::{
    game::{
        script::ScriptMessage,
        user::{
            remote::{GuildMarkData, TamingMobData, UserRemoteInitData},
            secondary_stats::RemoteCharSecondaryStatPartial,
        },
    },
    shared::{
        char::{AvatarData, AvatarEquips, CharStat, CharStatPartial},
        inventory::InventoryOperation,
        Gender,
    },
};
use shroom_script::SessionCtx;
use shroom_srv::{act::Context, game::pool::PoolItem, util::DelayQueue, GameTime};

use crate::{
    field::AttackerContext, game::GameContext, life::char::summon::Summon,
    services::shared::SharedGameServices,
};

use self::{
    buffs::CharBuffs,
    class::{AttackData, ClassContext, ClassHandler, UseSkillData},
    inv::CharInventory,
    pet::{CharPets, Pet},
    quest::{CharQuests, QuestCheckError},
    stats::CharStats,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharEvents {
    SummonSpawned(ObjectId),
    SummonRemoved(ObjectId),
}

#[derive(Debug)]
pub struct Character {
    pub game: SharedGameServices,
    pub id: CharacterId,
    pub name: String,
    pub gender: Gender,
    pub stats: CharStats,
    pub inventory: CharInventory,
    pub field: FieldId,
    pub spawn_point: SpawnPoint,
    pub skin: Skin,
    pub hair: HairId,
    pub face: FaceId,
    pub skills: SkillSet,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub buffs: CharBuffs,
    pub npc_msg: VecDeque<ScriptMessage>,
    pub do_script_transfer: Option<FieldId>,
    pub key_map: KeyMap,
    pub pets: CharPets,
    pub summons: slab::Slab<Summon>,
    pub quests: CharQuests,
    pub last_update: GameTime,

    pub last_id: u32,

    pub pending: DelayQueue<CharEvents>,
    pub npc_id: Option<NpcId>,

    pub playtime: std::time::Duration,
    pub game_start: std::time::Instant,
}

impl<'a> AttackerContext for &'a Character {
    fn attacker(&self) -> CharacterId {
        self.id
    }

    fn get_reactor_quest_flag(
        &self,
        rid: shroom_meta::id::ReactorId,
    ) -> Option<&shroom_meta::drops::QuestDropFlags> {
        self.quests.reactor_quest_drop_flags.get(&rid)
    }

    fn get_mob_quest_flag(
        &self,
        mid: shroom_meta::id::MobId,
    ) -> Option<&shroom_meta::drops::QuestDropFlags> {
        self.quests.mob_quest_drop_flags.get(&mid)
    }
}

impl Character {
    pub fn new(
        game: SharedGameServices,
        t: GameTime,
        model: Model,
        inventory: InventorySet<NoopInvSetHandler>,
        skills: SkillSet,
        key_map: KeyMap,
        q: QuestSet,
    ) -> Self {
        let meta = game.meta;
        let field = FieldId(model.field_id as u32);
        let field_meta = game.meta.get_field(field).unwrap();
        let spawn_point = field_meta.get_spawn_point(model.spawn_point as u8).unwrap();

        Self {
            game,
            id: CharacterId(model.id as u32),
            stats: (&model).into(),
            inventory: CharInventory::from_inv_set(model.get_inventory_size(), inventory),
            gender: (&model.gender).into(),
            name: model.name.clone(),
            field,
            skin: Skin::try_from(model.skin as u8).expect("skin"),
            hair: HairId(model.hair as u32),
            face: FaceId(model.face as u32),
            pos: spawn_point.pos,
            fh: FootholdId::none(),
            spawn_point,
            skills,
            npc_msg: VecDeque::default(),
            buffs: CharBuffs::new(),
            do_script_transfer: None,
            summons: Default::default(),
            last_id: 1,
            pending: DelayQueue::new(),
            game_start: Instant::now(),
            playtime: std::time::Duration::from_secs(model.play_time as u64),
            npc_id: None,
            last_update: t,
            key_map,
            pets: CharPets::default(),
            quests: CharQuests::from_data(q, meta),
        }
    }

    pub fn try_accept_quest(&mut self, qid: QuestId) -> Result<(), QuestCheckError> {
        let meta = self.meta();
        self.quests
            .try_start_quest(qid, meta, &self.field, &self.stats, &mut self.inventory)
    }

    pub fn try_complete_quest(&mut self, qid: QuestId) -> anyhow::Result<()> {
        let meta = self.meta();

        let mq = meta.get_quest(qid).unwrap();
        if self
            .quests
            .check_complete_quest(&self.field, &self.stats, &mut self.inventory)
            .is_err()
        {
            anyhow::bail!("Quest not active");
        }

        CharQuests::reward_quest(qid, mq, self)?;
        self.quests.complete_quest(qid, meta)?;
        Ok(())
    }

    pub fn handle_update(&mut self, ctx: &mut GameContext) -> anyhow::Result<()> {
        while let Some(event) = self.pending.pop(ctx.time()) {
            match event {
                CharEvents::SummonSpawned(id) => {
                    let summ = self.summons.get(id.0 as usize).unwrap();
                    ctx.room
                        .tx
                        .broadcast_encode(summ.enter_msg(id, ctx.time()))?;
                    self.pending.push(
                        CharEvents::SummonRemoved(id),
                        ctx.time() + std::time::Duration::from_secs(10),
                    );
                }
                CharEvents::SummonRemoved(id) => {
                    let summ = self.summons.remove(id.0 as usize);
                    ctx.room.tx.broadcast_encode(summ.leave_msg(id, ()))?;
                }
            }
        }

        self.last_update = ctx.time();

        Ok(())
    }

    pub fn add_pet(&mut self, mut pet: Pet, ctx: &mut GameContext) -> anyhow::Result<()> {
        pet.assign_char(self);
        let ix = self.pets.add_pet(pet)?;

        let pet = self.pets.get(ix).unwrap();
        ctx.socket.reply(pet.local_enter_msg())?;
        //TODO either handling
        ctx.room
            .tx()
            .broadcast_filter_encode(pet.enter_msg(true).left().unwrap(), self.id)?;

        Ok(())
    }

    pub fn get_summon(&self, id: ObjectId) -> Option<&Summon> {
        self.summons.get(id.0 as usize)
    }

    pub fn get_summon_mut(&mut self, id: ObjectId) -> Option<&mut Summon> {
        self.summons.get_mut(id.0 as usize)
    }

    pub fn add_summon(
        &mut self,
        ctx: &mut GameContext,
        summon: Summon,
    ) -> anyhow::Result<ObjectId> {
        let exp = summon.expiration;
        let id = ObjectId(self.summons.insert(summon) as u32);

        let t = ctx.time();

        let summ = self.summons.get(id.0 as usize).unwrap();
        ctx.room.tx.broadcast_encode(summ.enter_msg(id, t))?;
        self.pending.push(CharEvents::SummonRemoved(id), exp);
        Ok(id)
    }

    pub fn apply_heal(&mut self, heal: HealBuff) -> anyhow::Result<()> {
        match heal {
            HealBuff::Flat(flat) => {
                self.stats.update_hp(flat as i32);
            }
            HealBuff::Ratio(perc) => {
                self.stats.heal_hp_ratio(perc.ratio());
            }
        }
        Ok(())
    }

    pub fn add_exp(&mut self, mut exp: u32) {
        while exp > 0 {
            let next_exp = self.game.meta.get_next_level_exp(self.stats.level);
            let missing = next_exp - self.stats.exp;
            if exp < missing {
                self.stats.exp_mut().force_update(|e| *e += exp);
                break;
            }

            exp -= missing;
            self.stats.exp_mut().force_update(|e| *e = 0);
            self.level_up();
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.stats.level_mut().force_update(|l| *l = level);
        self.stats.exp_mut().force_update(|e| *e = 0);
    }

    pub fn level_up(&mut self) {
        self.stats.process_level_up();
    }

    pub fn handle_attack(&mut self, atk: &AttackData, ctx: &mut GameContext) -> anyhow::Result<()> {
        ClassHandler::handle_attack(ClassContext::new(self, ctx), atk)?;

        Ok(())
    }

    pub fn add_stack_item(
        &mut self,
        inv_ty: InventoryType,
        id: ItemId,
        quantity: usize,
    ) -> anyhow::Result<()> {
        self.inventory.try_add_stack_item(id, quantity, inv_ty)?;
        Ok(())
    }

    pub fn add_equip_item(&mut self, id: ItemId) -> anyhow::Result<()> {
        let item = self.game.data.item.create_equip(id)?;
        self.inventory.try_add_equip(item)?;
        Ok(())
    }

    pub fn add_items(&mut self, id: ItemId, quantity: Option<usize>) -> anyhow::Result<()> {
        let quantity = quantity.unwrap_or(1);
        let inv_ty = id.get_inv_type().unwrap();
        if inv_ty.is_stack() {
            self.add_stack_item(inv_ty, id, quantity)?;
        } else {
            self.add_equip_item(id)?;
        }
        Ok(())
    }

    pub fn get_stats_update(&mut self) -> Option<CharStatPartial> {
        self.stats.get_stats_partial()
    }

    pub fn transfer_map(&mut self, map: FieldId, sp: SpawnPoint) {
        self.field = map;
        self.spawn_point = sp;
        // Reset the updates, since we use set field anyway
        self.stats.reset();

        self.pos = self.spawn_point.pos;
        self.fh = FootholdId::none();
    }

    pub fn unlock_char(&mut self) {
        *self.stats.action_locked_mut() = false;
    }

    pub fn decrease_exp(&mut self, town: bool) {
        if self.stats.exp <= 200 {
            return;
        }

        let reduction_rate = match town {
            true => 0.01,
            false => {
                let temp_rate = if self.stats.job.level() == 0 {
                    0.08
                } else {
                    0.2
                };
                temp_rate.div((self.stats.luk as f64).add(0.05))
            }
        };

        let delta = (self.stats.exp as f64 * reduction_rate) as u32;

        self.stats
            .exp_mut()
            .force_update(|exp| *exp = exp.saturating_sub(delta));
    }

    pub fn update_mesos(&mut self, delta: i32) -> bool {
        self.stats.money_mut().force_update(|money| {
            *money = (*money).saturating_add_signed(delta).min(i32::MAX as u32)
        });
        true
    }

    pub fn get_map_id(&self) -> FieldId {
        self.field
    }

    pub fn money(&self) -> u32 {
        self.stats.money
    }

    pub fn is_dead(&self) -> bool {
        self.stats.hp.value == 0
    }

    pub fn add_sp(&mut self, add: u32) {
        self.stats
            .skill_points_mut()
            .force_update(|sp| *sp.get_mut(0) += add as u16);
    }

    pub fn change_job(&mut self, job: JobId, prev_skills: bool) -> anyhow::Result<()> {
        *self.stats.job_mut() = job;

        // Give new skills
        self.skills.add_skills(
            self.game.meta.get_skills_for_job(job).map(SkillData::from),
            false,
        );

        if prev_skills {
            for prev_job in job.prev_jobs() {
                self.skills.add_skills(
                    self.game
                        .meta
                        .get_skills_for_job(prev_job)
                        .map(SkillData::from),
                    false,
                );
            }
        }
        Ok(())
    }

    pub fn give_test_set(&mut self, data: &ItemService) -> anyhow::Result<()> {
        for item in [
            1432040, 1432028, 1452016, 1462019, 1092030, 1382009, 1472030, 1332056,
        ] {
            self.inventory.add_equip_by_id(ItemId(item), data)?;
        }

        // Basic throwing stars
        for i in 2070000..=2070018 {
            self.inventory
                .try_add_stack_item(ItemId(i), 400, InventoryType::Consume)?;
        }
        //self.inventory.try_add_stack_item_by_id(ItemId::SUBI_THROWING_STARS, 400, InventoryType::Consume)?;
        self.inventory
            .try_add_stack_item(ItemId(4006000), 200, InventoryType::Etc)?;
        self.inventory
            .try_add_stack_item(ItemId(4006001), 200, InventoryType::Etc)?;

        Ok(())
    }

    pub fn respawn(&mut self) {
        self.stats.mp_mut().set_stat(1);
        self.stats.hp_mut().set_stat(1);
    }

    pub fn get_all_stats(&self) -> CharStat {
        let (job_id, sub_job) = (self.stats.job, 0);

        CharStat {
            char_id: self.id,
            skin_color: self.skin,
            face: self.face,
            hair: self.hair,
            level: self.stats.level,
            str: self.stats.str,
            dex: self.stats.dex,
            int: self.stats.int,
            luk: self.stats.luk,
            hp: self.stats.hp.value,
            max_hp: self.stats.hp.max,
            mp: self.stats.mp.value,
            max_mp: self.stats.mp.max,
            ap: self.stats.ap,
            sp: self
                .stats
                .skill_points
                .to_proto(job_id.has_extended_sp())
                .into(),
            exp: self.stats.exp as i32,
            fame: self.stats.fame,
            tmp_exp: 0,
            name: self.name.as_str().try_into().expect("Name"),
            gender: self.gender,
            pets: [0; 3],
            job_id,
            map_id: self.field,
            portal: self.spawn_point.id,
            playtime: 0,
            sub_job,
        }
    }

    pub fn get_avatar_data(&self) -> AvatarData {
        AvatarData {
            gender: self.gender,
            skin: self.skin,
            mega: false,
            face: self.face,
            hair: self.hair,
            equips: AvatarEquips {
                equips: self
                    .inventory
                    .get_equipped_slot_ids()
                    .map(|(slot, item)| (slot as u8, item))
                    .collect_vec()
                    .into(),
                masked_equips: ShroomIndexList8::from(vec![]),
                weapon_sticker_id: ItemId(0),
            },
            pets: [ItemId(5000008); 3],
        }
    }

    pub fn get_inv_op_updates(&mut self) -> Option<Vec<InventoryOperation>> {
        self.inventory.get_updates()
    }

    pub fn use_skill(&mut self, req: &UseSkillData, ctx: &mut GameContext) -> anyhow::Result<()> {
        let skill = self.skills.get(req.skill_id)?;
        let mp_cost = skill.mp_cost();
        //let cd = Duration::from_secs(15);
        if let Some(cost) = mp_cost {
            if !self.stats.try_update_mp(-(cost as i32)) {
                return Ok(());
            }
        }

        // Give buff
        ClassHandler::handle_skill(ClassContext::new(self, ctx), req)?;
        *self.stats.action_locked_mut() = false;
        //self.skills.set_cooldown(skill_id, cd);

        Ok(())
    }

    pub fn skill_up(&mut self, skill_id: SkillId) -> anyhow::Result<()> {
        if self.stats.try_take_sp(skill_id.page_ix()) {
            anyhow::bail!("Insufficient SP");
        }

        self.skills.skill_up(skill_id, 1)?;
        Ok(())
    }

    pub fn get_remote_init_data(&self) -> UserRemoteInitData {
        let job = self.stats.job;
        // TODO
        let secondary_stat = RemoteCharSecondaryStatPartial {
            //shadowpartner: Some(4111002).into(),
            darksight: Some(()).into(),
            curse: Some(1000).into(),
            ..Default::default()
        };

        UserRemoteInitData {
            level: self.stats.level,
            name: self.name.clone(),
            guild_name: "".to_string(),
            guild_mark: GuildMarkData::default(),
            secondary_stat: secondary_stat.into(),
            defense_att: 0,
            defense_state: 0,
            job,
            avatar: self.get_avatar_data(),
            driver_id: CharacterId(0),
            passenger_id: CharacterId(0),
            choco_count: 0,
            active_effect_item: ItemId(0),
            completed_set_item_id: ItemId(0),
            portable_chair: ItemId(0),
            pos: self.pos,
            fh: self.fh,
            show_admin_effects: false,
            pet_infos: Default::default(),
            taming_mob: TamingMobData::default(),
            mini_room: None.into(),
            ad_board: None.into(),
            couple: None.into(),
            friendship: None.into(),
            marriage: None.into(),
            load_flags: 0,
            new_year_cards: None.into(),
            phase: 0,
            move_action: 0,
        }
    }

    pub fn db_model(&self) -> character::ActiveModel {
        let playtime = (self.game_start.elapsed() + self.playtime).as_secs() as i32;
        let s = &self.stats;
        character::ActiveModel {
            id: Set(self.id.0 as i32),
            name: Set(self.name.clone()),
            last_login_at: Set(Some(Utc::now().naive_utc())),
            gender: Set(self.gender.into()),
            skill_points: Set(self.stats.skill_points.as_data().to_vec()),
            play_time: Set(playtime),
            level: Set(s.level as i32),
            exp: Set(s.exp as i32),
            //TODO gacha exp
            str: Set(s.str as i32),
            dex: Set(s.dex as i32),
            luk: Set(s.luk as i32),
            int: Set(s.int as i32),
            hp: Set(s.hp.value as i32),
            max_hp: Set(s.hp.max as i32),
            mp: Set(s.mp.value as i32),
            max_mp: Set(s.mp.max as i32),
            mesos: Set(s.money as i32),
            //TODO buddy cap
            fame: Set(s.fame as i32),
            ap: Set(s.ap as i32),
            job: Set(s.job as i32),
            face: Set(self.face.0 as i32),
            skin: Set(self.skin as i32),
            hair: Set(self.hair.0 as i32),
            field_id: Set(self.field.0 as i32),
            spawn_point: Set(self.spawn_point.id as i32),
            ..Default::default()
        }
    }
}
