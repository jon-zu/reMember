use std::{
    collections::{BTreeMap, HashSet},
    ops::RangeInclusive,
};

use crate::id::{ItemId, MobId, NpcId, QuestId, ReactorId};

use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReactorDrop {
    #[serde(rename = "reactorId")]
    pub reactor_id: Option<u32>,
    #[serde(rename = "reactorAction")]
    pub reactor_action: Option<String>,
    #[serde(rename = "itemId")]
    pub item_id: u32,
    #[serde(rename = "chance")]
    pub chance: f32,
    #[serde(rename = "questId")]
    pub quest_id: Option<u16>,
}

pub type ReactorDropList = Vec<ReactorDrop>;

#[derive(Debug, Deserialize)]
pub struct MobDrop {
    #[serde(rename = "mobId")]
    pub mob_id: u32,
    #[serde(rename = "itemId")]
    pub item_id: u32,
    #[serde(rename = "minQ")]
    pub min_q: u16,
    #[serde(rename = "maxQ")]
    pub max_q: u16,
    #[serde(rename = "questId")]
    pub quest_id: u16,
    pub chance: f32,
}

pub type MobDropList = Vec<MobDrop>;

#[derive(Debug, Deserialize)]
pub struct NpcShopItem {
    #[serde(rename = "itemId")]
    pub item_id: u32,
    #[serde(rename = "itemPeriod")]
    pub item_period: u16,
    pub price: u32,
    pub position: u32,
}

#[derive(Debug, Deserialize)]
pub struct NpcShop {
    pub id: u32,
    pub items: Vec<NpcShopItem>,
}

pub type NpcShops = BTreeMap<NpcId, NpcShop>;

#[derive(Debug, Default)]
pub struct QuestDropFlags(pub HashSet<QuestId>);

impl QuestDropFlags {
    pub fn insert(&mut self, quest_id: QuestId) {
        self.0.insert(quest_id);
    }

    pub fn union(&mut self, other: &Self) {
        self.0.extend(other.0.iter().copied());
    }
}

#[derive(Debug)]
pub struct DropEntry {
    pub item: ItemId,
    pub quantity: RangeInclusive<u32>,
    pub chance: f32,
    pub quest: Option<QuestId>,
}

#[derive(Debug, Default)]
pub struct DropList {
    pub entries: Vec<DropEntry>,
    pub linked_quests: HashSet<QuestId>,
}

impl DropEntry {
    pub fn get_with_rand<R: Rng + ?Sized>(
        &self,
        flags: &QuestDropFlags,
        rng: &mut R,
    ) -> Option<(ItemId, usize)> {
        // Check for flag
        if !self.quest.map(|q| flags.0.contains(&q)).unwrap_or(true) {
            return None;
        }

        // Check for chance
        if !rng.gen_bool(self.chance.into()) {
            return None;
        }

        Some((self.item, rng.gen_range(self.quantity.clone()) as usize))
    }
}

impl DropList {
    pub fn from_entries(entries: Vec<DropEntry>) -> Self {
        Self {
            linked_quests: entries.iter().filter_map(|entry| entry.quest).collect(),
            entries,
        }
    }

    pub fn add_entry(&mut self, entry: DropEntry) {
        if let Some(quest) = entry.quest {
            self.linked_quests.insert(quest);
        }
        self.entries.push(entry);
    }

    pub fn get_with_rand<'a, 'b: 'a, R: Rng + ?Sized>(
        &'a self,
        rng: &'b mut R,
        flags: &'a QuestDropFlags,
    ) -> impl Iterator<Item = (ItemId, usize)> + 'a {
        self.entries
            .iter()
            .filter_map(move |entry| entry.get_with_rand(flags, rng))
    }
}

#[derive(Debug)]
pub struct DropPool {
    pub mob_drops: BTreeMap<MobId, DropList>,
    pub reactor_drops: BTreeMap<ReactorId, DropList>,
    pub mob_quest_flags: BTreeMap<QuestId, HashSet<MobId>>,
    pub reactor_quest_flags: BTreeMap<QuestId, HashSet<ReactorId>>,
    pub money: u32,
    pub money_variance: u32,
}

impl DropPool {
    pub fn from_drop_lists(mob_list: MobDropList, reactor_list: ReactorDropList) -> Self {
        let mut mob_drops = BTreeMap::<MobId, DropList>::new();
        let map_quest = |quest_id: u16| {
            if quest_id == 0 {
                None
            } else {
                Some(QuestId(quest_id))
            }
        };

        let mut mob_quest_flags: BTreeMap<QuestId, HashSet<MobId>> = Default::default();

        for drop in mob_list {
            let mid = drop.mob_id.into();
            let qid = map_quest(drop.quest_id);

            mob_drops
                .entry(mid)
                .or_default()
                .add_entry(DropEntry {
                    item: ItemId(drop.item_id),
                    quantity: (drop.min_q as u32)..=(drop.max_q as u32),
                    chance: drop.chance,
                    quest: qid,
                });

            if let Some(qid) = qid { 
                mob_quest_flags
                    .entry(qid)
                    .or_default()
                    .insert(mid);
            }
        }

        let mut reactor_drops = BTreeMap::<ReactorId, DropList>::new();
        let mut reactor_quest_flags: BTreeMap<QuestId, HashSet<ReactorId>> = Default::default();
        for drop in reactor_list {
            let rid = drop.reactor_id.unwrap_or(0).into();
            reactor_drops.entry(rid).or_default().add_entry(DropEntry {
                item: ItemId(drop.item_id),
                quantity: 1..=1,
                chance: drop.chance,
                quest: map_quest(drop.quest_id.unwrap_or(0)),
            });

            if let Some(qid) = drop.quest_id {
                reactor_quest_flags
                    .entry(QuestId(qid))
                    .or_default()
                    .insert(rid);
            }
        }

        Self {
            mob_drops,
            reactor_drops,
            money: 50_000,
            money_variance: 49_999,
            reactor_quest_flags,
            mob_quest_flags
        }
    }

    pub fn get_drops_for_mob(
        &self,
        mob_id: MobId,
        flags: &QuestDropFlags,
        r: &mut impl Rng,
    ) -> Vec<(ItemId, usize)> {
        self.mob_drops
            .get(&mob_id)
            .map(move |drops| drops.get_with_rand(r, flags).collect())
            .unwrap_or_default()
    }

    pub fn get_reactor_drops(
        &self,
        reactor_id: ReactorId,
        flags: &QuestDropFlags,
        r: &mut impl Rng,
    ) -> Vec<(ItemId, usize)> {
        self.reactor_drops
            .get(&reactor_id)
            .map(move |drops| drops.get_with_rand(r, flags).collect())
            .unwrap_or_default()
    }

    pub fn get_money_drop<R: Rng>(&self, rng: &mut R) -> u32 {
        rng.gen_range((self.money - self.money_variance)..=self.money)
    }
}
