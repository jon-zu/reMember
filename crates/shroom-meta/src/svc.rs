use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Context;
use rand::thread_rng;
use rayon::prelude::{ParallelBridge, ParallelExtend, ParallelIterator};

use crate::{
    drops::{DropPool, NpcShop, NpcShops, QuestDropFlags},
    exp_table::ExpTable,
    field::{FhTree, Field},
    id::{
        job_id::JobId, FieldId, ItemId, ItemOptionId, MobId, MobSkillId, NpcId, QuestId, ReactorId,
        SkillId,
    },
    mob::{Mob, MobSkill, MobSkills},
    quest, skill,
    srv::{GoToFields, ItemSets},
    tmpl::{
        equip::{EquipItemTmpl, WeaponItemTmpl},
        item::{ItemOption, BundleItemTmpl},
    }, FIELD_REGIONS,
};

#[derive(Debug)]
pub struct MetaItems {
    pub weapons: BTreeMap<ItemId, WeaponItemTmpl>,
    pub accessory: BTreeMap<ItemId, EquipItemTmpl>,
    pub cap: BTreeMap<ItemId, EquipItemTmpl>,
    pub cape: BTreeMap<ItemId, EquipItemTmpl>,
    pub coat: BTreeMap<ItemId, EquipItemTmpl>,
    pub dragon: BTreeMap<ItemId, EquipItemTmpl>,
    pub face: BTreeMap<ItemId, EquipItemTmpl>,
    pub glove: BTreeMap<ItemId, EquipItemTmpl>,
    pub hair: BTreeMap<ItemId, EquipItemTmpl>,
    pub long_coat: BTreeMap<ItemId, EquipItemTmpl>,
    pub mechanic: BTreeMap<ItemId, EquipItemTmpl>,
    pub pants: BTreeMap<ItemId, EquipItemTmpl>,
    pub ring: BTreeMap<ItemId, EquipItemTmpl>,
    pub shield: BTreeMap<ItemId, EquipItemTmpl>,
    pub shoes: BTreeMap<ItemId, EquipItemTmpl>,
    pub consume: BTreeMap<ItemId, BundleItemTmpl>,
    pub cash: BTreeMap<ItemId, BundleItemTmpl>,
    pub etc: BTreeMap<ItemId, BundleItemTmpl>,
    pub install: BTreeMap<ItemId, BundleItemTmpl>,
    pub options: BTreeMap<ItemOptionId, ItemOption>,
}

impl MetaItems {
    pub fn load_from_dir(dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        let dir = dir.as_ref();

        Ok(Self {
            weapons: MetaData::load_from_file(dir.join("wep.bincode"))?,
            accessory: MetaData::load_from_file(dir.join("Accessory.bincode"))?,
            cap: MetaData::load_from_file(dir.join("Cap.bincode"))?,
            cape: MetaData::load_from_file(dir.join("Cape.bincode"))?,
            coat: MetaData::load_from_file(dir.join("Coat.bincode"))?,
            dragon: BTreeMap::new(), //MetaData::load_from_file(dir.join("Dragon.bincode"))?,
            face: MetaData::load_from_file(dir.join("Face.bincode"))?,
            glove: MetaData::load_from_file(dir.join("Glove.bincode"))?,
            hair: MetaData::load_from_file(dir.join("Hair.bincode"))?,
            long_coat: MetaData::load_from_file(dir.join("Longcoat.bincode"))?,
            mechanic: MetaData::load_from_file(dir.join("Mechanic.bincode"))?,
            pants: MetaData::load_from_file(dir.join("Pants.bincode"))?,
            ring: MetaData::load_from_file(dir.join("Ring.bincode"))?,
            shield: MetaData::load_from_file(dir.join("Shield.bincode"))?,
            shoes: MetaData::load_from_file(dir.join("Shoes.bincode"))?,
            consume: MetaData::load_from_file(dir.join("Consume.bincode"))?,
            cash: MetaData::load_from_file(dir.join("Cash.bincode"))?,
            etc: MetaData::load_from_file(dir.join("Etc.bincode"))?,
            install: MetaData::load_from_file(dir.join("Install.bincode"))?,
            options: MetaData::load_from_json(dir.join("item_options.json"))?,
        })
    }

    pub fn get_equip(&self, id: ItemId) -> Option<&EquipItemTmpl> {
        self.accessory
            .get(&id)
            .or_else(|| self.cap.get(&id))
            .or_else(|| self.cape.get(&id))
            .or_else(|| self.coat.get(&id))
            .or_else(|| self.dragon.get(&id))
            .or_else(|| self.face.get(&id))
            .or_else(|| self.glove.get(&id))
            .or_else(|| self.hair.get(&id))
            .or_else(|| self.long_coat.get(&id))
            .or_else(|| self.mechanic.get(&id))
            .or_else(|| self.pants.get(&id))
            .or_else(|| self.ring.get(&id))
            .or_else(|| self.shield.get(&id))
            .or_else(|| self.shoes.get(&id))
            .or_else(|| self.weapons.get(&id).map(|v| &v.equip))
    }

    pub fn get_weapon(&self, id: ItemId) -> Option<&WeaponItemTmpl> {
        self.weapons.get(&id)
    }

    pub fn get_bundle(&self, id: ItemId) -> Option<&BundleItemTmpl> {
        self.consume
            .get(&id)
            .or_else(|| self.cash.get(&id))
            .or_else(|| self.etc.get(&id))
            .or_else(|| self.install.get(&id))
    }
}

#[derive(Debug)]
pub struct MetaData {
    pub fields: BTreeMap<FieldId, Field>,
    pub items: MetaItems,
    pub skills: BTreeMap<SkillId, skill::Skill>,
    pub quests: BTreeMap<QuestId, quest::Quest>,
    pub mob_skills: MobSkills,
    pub mobs: BTreeMap<MobId, Mob>,
    pub npc_shops: NpcShops,
    pub drop_pool: DropPool,
    pub goto_fields: GoToFields,
    pub item_sets: ItemSets,
    pub exp_table: ExpTable,
}

pub type Meta<T> = &'static T;

pub type FieldMeta = &'static Field;
pub type MobMeta = &'static Mob;
pub type MobSkillMeta = Meta<MobSkill>;
pub type DropsMeta = &'static DropPool;
pub type SkillMeta = &'static skill::Skill;

#[derive(Debug, Clone, Copy)]
pub enum MetaOption {
    Testing,
    Full,
}

impl MetaOption {
    pub fn get_regions(&self) -> impl Iterator<Item = u8> {
        match self {
            Self::Testing => FIELD_REGIONS.iter().take(1).copied(),
            Self::Full => FIELD_REGIONS.iter().take(FIELD_REGIONS.len()).copied(),
        }
    }
}

impl MetaData {
    fn load_from_file<T: serde::de::DeserializeOwned>(file: impl AsRef<Path>) -> anyhow::Result<T> {
        let filename = file.as_ref().to_str().unwrap().to_string();
        let file = BufReader::new(File::open(file).context(filename.clone())?);
        bincode::deserialize_from(file).context(filename)
    }

    fn load_from_json<T: serde::de::DeserializeOwned>(file: impl AsRef<Path>) -> anyhow::Result<T> {
        let file = BufReader::new(File::open(file)?);
        Ok(serde_json::from_reader(file)?)
    }

    pub fn load_from_dir(dir: PathBuf, opt: MetaOption) -> anyhow::Result<Self> {
        let mut fields = BTreeMap::new();
        fields.par_extend(opt.get_regions().par_bridge().flat_map(|region| {
            Self::load_from_file::<BTreeMap<u32, Field>>(
                dir.join(format!("fields/fields{region}.bincode")),
            )
            .unwrap()
            .into_iter()
            .map(|(id, field)| (FieldId(id), field))
            .par_bridge()
        }));

        let skills: BTreeMap<u32, skill::Skill> =
            Self::load_from_json(dir.join("skills.json")).context("Skill")?;
        let mob_skills: MobSkills =
            Self::load_from_json(dir.join("mob_skills.json")).context("Mob Skills")?;

        let mobs: BTreeMap<MobId, Mob> =
            Self::load_from_file(dir.join("mobs.bincode")).context("mobs")?;

        let drop_pool = DropPool::from_drop_lists(
            Self::load_from_json(dir.join("ext/mob_drops.json")).context("Mob Drops")?,
            Self::load_from_json(dir.join("ext/reactor_drops.json")).context("Reactor Drops")?,
        );

        let item_sets = Self::load_from_json(dir.join("item_sets.json")).context("Item sets")?;
        let goto_fields =
            Self::load_from_json(dir.join("fields_goto.json")).context("Fields goto")?;

        Ok(Self {
            fields,
            mobs,
            quests: Self::load_from_json(dir.join("quest.json")).context("Quests")?,
            items: MetaItems::load_from_dir(&dir)?,
            skills: skills
                .into_iter()
                .map(|(id, skill)| (SkillId(id), skill))
                .collect(),
            mob_skills,
            npc_shops: Self::load_from_json(dir.join("ext/npc_shop.json")).context("Shops")?,
            drop_pool,
            goto_fields,
            item_sets,
            exp_table: ExpTable::build(),
        })
    }
}

#[derive(Debug)]
pub struct MetaService {
    meta_data: MetaData,
}

impl MetaService {
    pub fn new(meta_data: MetaData) -> Self {
        Self { meta_data }
    }

    pub fn items(&self) -> &MetaItems {
        &self.meta_data.items
    }

    pub fn get_quest(&self, id: QuestId) -> Option<&quest::Quest> {
        self.meta_data.quests.get(&id)
    }

    pub fn get_next_level_exp(&self, level: u8) -> u32 {
        self.meta_data.exp_table.get_exp(level) as u32
    }

    pub fn get_mob_skill_data(&self, id: MobSkillId, lvl: u8) -> Option<&MobSkill> {
        self.meta_data
            .mob_skills
            .0
            .get(&id)
            .and_then(|v| v.get(&lvl))
    }

    pub fn goto(&self) -> &GoToFields {
        &self.meta_data.goto_fields
    }

    pub fn item_sets(&self) -> &ItemSets {
        &self.meta_data.item_sets
    }

    pub fn load_from_dir(dir: impl AsRef<Path>, opt: MetaOption) -> anyhow::Result<Self> {
        Ok(Self::new(MetaData::load_from_dir(
            dir.as_ref().to_path_buf(),
            opt,
        )?))
    }

    pub fn get_field(&self, field_id: FieldId) -> Option<&Field> {
        self.meta_data.fields.get(&field_id)
    }

    pub fn get_field_fh_data(&self, field_id: FieldId) -> Option<&FhTree> {
        self.meta_data.fields.get(&field_id).map(|v| &v.fh_tree)
    }

    pub fn get_mob_data(&self, mob_id: MobId) -> Option<&Mob> {
        self.meta_data.mobs.get(&mob_id)
    }

    pub fn get_reactor_drops(&self, id: ReactorId, flags: &QuestDropFlags) -> Vec<(ItemId, usize)> {
        self.meta_data
            .drop_pool
            .get_reactor_drops(id, flags, &mut thread_rng())
    }

    pub fn get_drops_for_mob(&self, id: MobId, flags: &QuestDropFlags) -> Vec<(ItemId, usize)> {
        self.meta_data
            .drop_pool
            .get_drops_for_mob(id, flags, &mut thread_rng())
    }

    pub fn get_money_drops_for_mob(&self, _id: MobId) -> u32 {
        self.meta_data
            .drop_pool
            .get_money_drop(&mut rand::thread_rng())
    }

    pub fn get_drops_and_money_for_mob(
        &self,
        id: MobId,
        flags: &QuestDropFlags,
    ) -> (Vec<(ItemId, usize)>, u32) {
        let drops = self.get_drops_for_mob(id, flags);
        let money = self.get_money_drops_for_mob(id);
        (drops, money)
    }

    pub fn get_skill(&self, id: SkillId) -> Option<&skill::Skill> {
        self.meta_data.skills.get(&id)
    }

    pub fn get_skills_for_job(&self, job: JobId) -> impl Iterator<Item = (SkillId, &skill::Skill)> {
        self.meta_data
            .skills
            .range(job.skill_range())
            .map(|(id, skill)| (*id, skill))
    }

    pub fn get_npc_shop(&self, npc_id: NpcId) -> Option<&NpcShop> {
        self.meta_data.npc_shops.get(&npc_id)
    }

    pub fn get_quest_mob_drop_flags(&self, quest_id: QuestId) -> Option<&HashSet<MobId>> {
        self.meta_data.drop_pool.mob_quest_flags.get(&quest_id)
    }

    pub fn get_quest_reactor_drop_flags(&self, quest_id: QuestId) -> Option<&HashSet<ReactorId>> {
        self.meta_data.drop_pool.reactor_quest_flags.get(&quest_id)
    }
}
