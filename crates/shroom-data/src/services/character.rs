use std::fmt::Write;

use chrono::Utc;
use derive_more::Debug;
use either::Either;
use itertools::Itertools;
use sea_orm::{
    prelude::DateTimeUtc, ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set
};
use serde::{Deserialize, Serialize};
use shroom_meta::{
    id::{
        job_id::{JobGroup, JobId},
        CharacterId, FaceId, HairId, ItemId, MobId, QuestId, SkillId, Skin,
    }, CharLevel, MetaService, QuestDataId
};
use shroom_proto95::{
    login::char::{DeleteCharResult, SelectCharResultCode},
    shared::Gender,
};
use shroom_srv::GameTime;

use crate::{
    blob::BinaryBlob,
    created_at,
    entities::{
        account,
        character::{self, ActiveModel, Column, Entity, Model},
        func_key_map, quest, skill,
    },
    entity_ext::KeyMap,
    model::skill::{SkillData, SkillSet},
};

use super::{
    account::{AccountId, AccountService},
    item::{CharacterEquippedItemIds, ItemService},
    DbConn,
};

#[derive(Debug, Clone)]
pub struct ItemStarterSet {
    pub bottom: ItemId,
    pub shoes: ItemId,
    pub top: ItemId,
    pub weapon: ItemId,
    pub guide: ItemId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobQuestRecord {
    pub mob_id: MobId,
    pub cur: u16,
    pub target: u16,
}

impl MobQuestRecord {
    pub fn update(&mut self, n: usize) {
        let v = self.cur as usize + n;
        self.cur = v.min(self.target as usize) as u16
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemQuestRecord {
    pub item_id: MobId,
    pub cur: u16,
    pub target: u16,
}

impl ItemQuestRecord {
    pub fn update(&mut self, n: usize) {
        let v = self.cur as usize + n;
        self.cur = v.min(self.target as usize) as u16
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestRecord {
    Mob(Vec<MobQuestRecord>),
    Ex(Vec<(String, String)>)
}

impl QuestRecord {
    pub fn to_qr_string(&self) -> String {
        match self {
            QuestRecord::Mob(mobs) => {
                mobs.iter().fold(String::new(), |mut acc, m| {
                    write!(&mut acc, "{:03}", m.cur).unwrap();
                    acc
                })
            }
            QuestRecord::Ex(ex) => {
                ex.iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .join(";")
            }
        }
    }

    pub fn is_ex(&self) -> bool {
        matches!(self, QuestRecord::Ex(_))
    }

    pub fn update_mobs(&mut self, mob_id: MobId, n: usize) -> bool {
        let mut updated = false;
        if let QuestRecord::Mob(mobs) = self {
            if let Some(mqr) = mobs.iter_mut().find(|qr| qr.mob_id == mob_id) {
                mqr.update(n);
                updated = true;
            }
        };

        updated
    }
}

impl BinaryBlob for QuestRecord {}

#[derive(Debug)]
pub struct ActiveQuest {
    pub quest: QuestId,
    pub data: QuestRecord,
    pub started_at: DateTimeUtc,
}

#[derive(Debug)]
pub struct FinishedQuest {
    pub quest: QuestId,
    pub started_at: DateTimeUtc,
    pub finished_at: DateTimeUtc,
}

#[derive(Debug)]
pub struct QuestData {
    pub quest: QuestDataId,
    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct QuestSet {
    pub active: Vec<ActiveQuest>,
    pub finished: Vec<FinishedQuest>,
    pub data: Vec<QuestData>
}

impl ItemStarterSet {
    pub fn validate(&self, job: JobGroup) -> anyhow::Result<()> {
        //TODO: update to v95
        let _bottom = check_contains(job.get_starter_bottoms(), self.bottom, "Bottom ID")?;
        let _shoes = check_contains(job.get_starter_shoes(), self.shoes, "Shoes ID")?;
        let _top = check_contains(job.get_starter_tops(), self.top, "Top ID")?;
        let _weapon = check_contains(job.get_starter_weapons(), self.weapon, "Weapon ID")?;
        if self.guide != job.get_guide_item() {
            anyhow::bail!("Invalid starter guide");
        }

        Ok(())
    }

    pub fn default_starter_set(job: JobGroup) -> Self {
        Self {
            shoes: ItemId::LEATHER_SANDALS,
            bottom: ItemId::BLUE_JEAN_SHORTS,
            top: ItemId::WHITE_UNDERSHIRT,
            weapon: ItemId::SWORD,
            guide: job.get_guide_item(),
        }
    }

    pub fn from_job_group(job: JobGroup) -> Self {
        Self {
            shoes: job.get_starter_shoes().next().unwrap(),
            bottom: job.get_starter_bottoms().next().unwrap(),
            top: job.get_starter_tops().next().unwrap(),
            weapon: job.get_starter_weapons().next().unwrap(),
            guide: job.get_guide_item(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterCreateDTO {
    pub name: String,
    pub job: Either<JobGroup, JobId>,
    pub face: FaceId,
    pub skin: Skin,
    pub hair: HairId,
    pub starter_set: ItemStarterSet,
    pub gender: Gender,
    pub max_skills: bool,
    pub level: Option<CharLevel>
}

impl CharacterCreateDTO {
    pub fn get_starter_set(&self) -> ItemStarterSet {
        self.starter_set.clone()
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(())
        /*  de-uglify and test this
        let job = self.job_group;
        let _face = check_contains(job.get_starter_face(), self.face, "Face ID")?;
        let _hair = check_contains(job.get_starter_hair(), self.hair, "Hair")?;
        self.starter_set.validate(job)?;

        Ok(())*/
    }
}

fn is_valid_char_name(name: &str) -> bool {
    //TODO error messages
    if !(3..13).contains(&name.len()) {
        return false;
    }

    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }

    true
}

pub fn check_contains<T: PartialEq + std::fmt::Debug>(
    mut iter: impl Iterator<Item = T>,
    check_id: T,
    name: &str,
) -> anyhow::Result<T> {
    if !iter.any(|id| id == check_id) {
        anyhow::bail!("Invalid {name} ({check_id:?}) for char creation ")
    }

    Ok(check_id)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharWithEquips {
    pub char: Model,
    pub equips: CharacterEquippedItemIds,
}

#[derive(Debug)]
pub struct CharacterService<'svc> {
    db: DbConn,
    meta: &'static MetaService,
    account: &'svc AccountService,
    item: &'svc ItemService,
}

impl<'svc> CharacterService<'svc> {
    pub fn new(
        db: DbConn,
        meta: &'static MetaService,
        account: &'svc AccountService,
        item: &'svc ItemService,
    ) -> Self {
        Self {
            db,
            account,
            meta,
            item,
        }
    }

    pub async fn check_name(&self, name: &str) -> anyhow::Result<bool> {
        if !is_valid_char_name(name) {
            return Ok(false);
        }

        let other_id = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(Column::Name.eq(name))
            .count(&self.db.0)
            .await?;

        Ok(other_id == 0)
    }

    pub async fn get_characters_for_account(&self, acc_id: i32) -> anyhow::Result<Vec<Model>> {
        Ok(Entity::find()
            .filter(Column::AccId.eq(acc_id))
            .all(&self.db.0)
            .await?)
    }

    pub async fn get_characters_with_equips(
        &self,
        acc_id: AccountId,
    ) -> anyhow::Result<Vec<CharWithEquips>> {
        // TODO should be a single query + caching
        let chars = self.get_characters_for_account(acc_id).await?;
        let mut res = Vec::with_capacity(chars.len());
        for char in chars {
            let equips = self
                .item
                .load_equipped_items(CharacterId(char.id as u32))
                .await?;
            res.push(CharWithEquips { char, equips });
        }
        Ok(res)
    }

    pub async fn get(&self, char_id: CharacterId) -> anyhow::Result<Option<Model>> {
        Ok(Entity::find_by_id(char_id.0 as i32).one(&self.db.0).await?)
    }

    pub async fn must_get(&self, char_id: CharacterId) -> anyhow::Result<Model> {
        self.get(char_id)
            .await?
            .ok_or_else(|| anyhow::format_err!("No char for id: {char_id}"))
    }

    pub async fn char_ids_for_account(
        &self,
        acc_id: AccountId,
    ) -> anyhow::Result<Vec<CharacterId>> {
        Ok(Entity::find()
            .select_only()
            .column_as(Column::Id, "id")
            .filter(Column::AccId.eq(acc_id))
            .into_tuple::<i32>()
            .all(&self.db.0)
            .await?
            .into_iter()
            .map(|c: i32| CharacterId(c as u32))
            .collect())
    }

    pub async fn create_character(
        &self,
        acc_id: i32,
        create: CharacterCreateDTO,
        item_svc: &ItemService,
    ) -> anyhow::Result<CharacterId> {
        create.validate()?;

        if !self.check_name(&create.name).await? {
            anyhow::bail!("Name is not valid");
        }

        let job_group = match create.job {
            Either::Left(g) => g,
            Either::Right(j) => j.job_group(),
        };

        let job_id = match create.job {
            Either::Left(g) => g.get_noob_job_id(),
            Either::Right(j) => j,
        };

        let field_id = job_group.get_start_field().0 as i32;
        let char = ActiveModel {
            acc_id: Set(acc_id),
            created_at: created_at(&self.db.0),
            gender: Set((create.gender).into()),
            name: Set(create.name),
            field_id: Set(field_id),
            job: Set(job_id as i32),
            level: Set(create.level.map(|l| l.0 as i32).unwrap_or(1)),
            str: Set(13 * 100),
            dex: Set(4 * 100),
            int: Set(4 * 100),
            luk: Set(4 * 100),
            hp: Set(50 * 100),
            max_hp: Set(50 * 100),
            mp: Set(50 * 100),
            max_mp: Set(50 * 100),
            equip_slots: Set(24),
            use_slots: Set(24),
            setup_slots: Set(24),
            etc_slots: Set(24),
            cash_slots: Set(24),
            storage_slots: Set(16),
            buddy_capacity: Set(20),
            skin: Set(create.skin as u8 as i32),
            face: Set(create.face.0 as i32),
            hair: Set(create.hair.0 as i32),
            exp: Set(0),
            gacha_exp: Set(0),
            mesos: Set(50_000),
            fame: Set(0),
            ap: Set(5),
            sp: Set(10),
            spawn_point: Set(0),
            skill_points: Set(vec![0; 20]),
            play_time: Set(0),
            ..Default::default()
        };

        let char_id = Entity::insert(char).exec(&self.db.0).await?.last_insert_id;
        let char_id = CharacterId(char_id as u32);
        Box::pin(item_svc.create_starter_set(char_id, create.starter_set)).await?;

        let mut skills: Vec<_> = self
            .meta
            .get_skills_for_job(job_id)
            .map(SkillData::from)
            .collect();
        for prev_job in job_id.prev_jobs() {
            for skill in self.meta.get_skills_for_job(prev_job) {
                skills.push(SkillData::from(skill));
            }
        }
        let mut skill_set = SkillSet::from_skills(skills.into_iter())?;
        if create.max_skills {
            skill_set.max_all();
        }
        self.save_skills(GameTime::default(), char_id, &skill_set)
            .await?;

        Ok(char_id)
    }

    pub async fn delete_character(
        &self,
        acc: &account::Model,
        char_id: CharacterId,
        pic: &str,
    ) -> anyhow::Result<DeleteCharResult> {
        if !self.account.check_pic(acc, pic)? {
            return Ok(DeleteCharResult::InvalidPic);
        }

        let char = self.must_get(char_id).await?;
        if char.acc_id != acc.id {
            return Ok(DeleteCharResult::UnknownErr);
        }

        /* Check:
        - world transfer
        - family
        - guild
        */

        Ok(DeleteCharResult::Success)
    }

    pub async fn select_char_with_pic(
        &self,
        acc: &account::Model,
        char_id: CharacterId,
        pic: &str,
    ) -> anyhow::Result<SelectCharResultCode> {
        if !self.account.check_pic(acc, pic)? {
            return Ok(SelectCharResultCode::InvalidPic);
        }

        self.select_char(acc, char_id).await
    }

    pub async fn select_char(
        &self,
        acc: &account::Model,
        char_id: CharacterId,
    ) -> anyhow::Result<SelectCharResultCode> {
        let char = self.must_get(char_id).await?;
        if char.acc_id != acc.id {
            return Ok(SelectCharResultCode::UnknownErr);
        }
        Ok(SelectCharResultCode::Success)
    }

    pub async fn load_skills(&self, t: GameTime, id: CharacterId) -> anyhow::Result<SkillSet> {
        let skills = skill::Entity::find()
            .filter(skill::Column::CharId.eq(id.0 as i32))
            .all(&self.db.0)
            .await?;

        let now = Utc::now();

        SkillSet::from_skills(skills.iter().map(|skill| {
            let id = SkillId(skill.skill_id as u32);
            let meta = self.meta.get_skill(id).unwrap();
            SkillData {
                id,
                level: skill.level as usize,
                mastery_level: (skill.master_level != 0).then_some(skill.master_level as usize),
                expires_at: skill
                    .expires_at
                    .map(|exp_at| t + (now - exp_at.and_utc()).to_std().unwrap()),
                cooldown: skill
                    .expires_at
                    .map(|exp_at| t + (now - exp_at.and_utc()).to_std().unwrap()),
                meta,
            }
        }))
    }

    pub async fn save_skills(
        &self,
        t: GameTime,
        char_id: CharacterId,
        skills: &SkillSet,
    ) -> anyhow::Result<()> {
        // Remove all skills
        skill::Entity::delete_many()
            .filter(skill::Column::CharId.eq(char_id.0 as i32))
            .exec(&self.db.0)
            .await?;

        let now = Utc::now();
        let get_utc_time =
            |tt: GameTime| now + chrono::Duration::from_std(tt.delta_ticks(t).into()).unwrap();

        // Insert new skills
        let skills: Vec<_> = skills
            .skills()
            .map(|skill| skill::ActiveModel {
                id: NotSet,
                skill_id: Set(skill.id.0 as i32),
                level: Set(skill.level as i32),
                master_level: Set(skill.mastery_level.unwrap_or(0) as i32),
                expires_at: Set(skill.expires_at.map(|t| get_utc_time(t).naive_utc())),
                cooldown: Set(skill.cooldown.map(|t| get_utc_time(t).naive_utc())),
                char_id: Set(char_id.0 as i32),
            })
            .collect();
        skill::Entity::insert_many(skills).exec(&self.db.0).await?;

        Ok(())
    }

    pub async fn save_char(&self, char: character::ActiveModel) -> anyhow::Result<()> {
        char.save(&self.db.0).await?;
        Ok(())
    }

    pub async fn load_key_map(&self, char_id: CharacterId) -> anyhow::Result<KeyMap> {
        let map = func_key_map::Entity::find()
            .filter(func_key_map::Column::CharId.eq(char_id.0 as i32))
            .one(&self.db.0)
            .await?;

        Ok(KeyMap::from_option_blob(map.as_ref().map(|m| m.data.as_slice()))?.unwrap_or_default())
    }

    pub async fn save_key_map(&self, char_id: CharacterId, map: &KeyMap) -> anyhow::Result<()> {
        if !map.is_changed() {
            return Ok(());
        }

        func_key_map::ActiveModel {
            id: NotSet,
            char_id: Set(char_id.0 as i32),
            data: Set(map.to_blob()?),
        }
        .save(&self.db.0)
        .await?;
        Ok(())
    }

    pub async fn load_quests(&self, char_id: CharacterId) -> anyhow::Result<QuestSet> {
        let q = quest::Entity::find()
            .filter(quest::Column::CharId.eq(char_id.0 as i32))
            .all(&self.db.0)
            .await?;

        let mut active_quests = Vec::new();
        let mut finished_quests = Vec::new();
        let mut quest_data = Vec::new();
        for quest in q {
            if quest.status == 0 {
                quest_data.push(QuestData {
                    quest: QuestDataId(quest.id as u32),
                    data: quest.data,
                });
            } else if quest.status == 1 {
                active_quests.push(ActiveQuest {
                    quest: QuestId(quest.id as u16),
                    data: QuestRecord::from_blob(&quest.data)?,
                    started_at: quest.started_at.unwrap().and_utc(),
                });
            } else {
                finished_quests.push(FinishedQuest {
                    quest: QuestId(quest.id as u16),
                    finished_at: quest.completed_at.unwrap().and_utc(),
                    started_at: quest.started_at.unwrap().and_utc(),
                });
            }
        }

        Ok(QuestSet {
            active: active_quests,
            finished: finished_quests,
            data: quest_data,
        })
    }

    pub async fn save_quest(
        &self,
        char_id: CharacterId,
        q: QuestSet
    ) -> anyhow::Result<()> {
        let mut quests = Vec::new();
        for quest in q.active {
            quests.push(quest::ActiveModel {
                char_id: Set(char_id.0 as i32),
                id: Set(quest.quest.0 as i32),
                data: Set(quest.data.to_blob()?),
                started_at: Set(Some(quest.started_at.naive_utc())),
                completed_at: Set(None),
                status: Set(1),
            });
        }
        for quest in q.finished {
            quests.push(quest::ActiveModel {
                char_id: Set(char_id.0 as i32),
                id: Set(quest.quest.0 as i32),
                data: Set(Vec::new()),
                started_at: Set(Some(quest.finished_at.naive_utc())),
                completed_at: Set(Some(quest.finished_at.naive_utc())),
                status: Set(2),
            });
        }

        for quest in q.data {
            quests.push(quest::ActiveModel {
                char_id: Set(char_id.0 as i32),
                id: Set(quest.quest.0 as i32),
                data: Set(quest.data.clone()),
                started_at: Set(None),
                completed_at: Set(None),
                status: Set(0),
            });
        }

        if quests.is_empty() {
            return Ok(());
        }

        // TODO use upserts
        quest::Entity::delete_many()
            .filter(quest::Column::CharId.eq(char_id.0 as i32))
            .exec(&self.db.0)
            .await?;

        quest::Entity::insert_many(quests).exec(&self.db.0).await?;

        Ok(())
    }
}
