use std::ops::{Deref, DerefMut};

use chrono::NaiveDateTime;
use derive_more::Not;

use crate::{
    id::{item_id::ItemGrade, ItemId, ItemOptionId},
    item::EquipBaseStats, tmpl::item::{ScrollItem, BundleItemValue},
};

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemFlags : u16 {
        const Protected = 0x01;
        const PreventSlipping = 0x02;
        const PreventColdness = 0x04;
        const Untradeable = 0x08;
        const ScissorsApplied = 0x10;
        const Sandbox = 0x40;
        const PetCome = 0x80;
        const AccountSharing = 0x100;
        const MergeUntradeable = 0x200;
    }
}

#[derive(Debug, Clone)]
pub struct ItemLevelInfo {
    pub item_level_ty: u8,
    pub level: u8,
    pub exp: u32,
}

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub db_id: Option<i32>,
    pub item_id: ItemId,
    pub is_cash: bool,
    pub game_id: i64,
    pub expiration: Option<NaiveDateTime>,
    pub owner: Option<String>,
    pub flags: ItemFlags,
    pub last_update: u32,
}

impl ItemInfo {
    pub fn cash_id(&self) -> Option<u64> {
        self.is_cash.then_some(self.game_id as u64)
    }

    pub fn sn(&self) -> Option<u64> {
        self.is_cash.not().then_some(self.game_id as u64)
    }
}

impl ItemInfo {
    pub fn from_id(item_id: ItemId, game_id: i64, is_cash: bool) -> Self {
        Self {
            db_id: None,
            is_cash,
            item_id,
            game_id,
            expiration: None,
            owner: None,
            flags: ItemFlags::empty(),
            last_update: 0,
        }
    }

    pub fn is_expired(&self, now: NaiveDateTime) -> bool {
        match self.expiration {
            Some(t_exp) => t_exp <= now,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Durability {
    #[default]
    Infinite,
    Finite(u32),
}

impl From<i32> for Durability {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self::Infinite
        } else {
            Self::Finite(value as u32)
        }
    }
}

impl From<Durability> for i32 {
    fn from(value: Durability) -> Self {
        match value {
            Durability::Infinite => -1,
            Durability::Finite(v) => v as i32,
        }
    }
}

#[derive(Debug, Default)]
pub struct EquipOptions(pub [ItemOptionId; 3]);


#[derive(Debug, Default)]
pub struct EquipSockets(pub [u16; 2]);

#[derive(Debug)]
pub struct EquipItem {
    pub info: ItemInfo,
    pub stats: EquipBaseStats,
    pub upgrades: u8,
    pub upgrade_slots: u8,
    pub hammers_used: u8,
    pub stars: u8,
    pub level_info: Option<ItemLevelInfo>,
    pub sockets: EquipSockets,
    pub options: EquipOptions,
    pub equipped_at: Option<NaiveDateTime>,
    pub grade: ItemGrade,
    pub durability: Durability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeResult {
    Success,
    Failed,
    InvalidUpgrade,
    NoSlots,
    Destroyed,
}

impl UpgradeResult {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    pub fn is_destroyed(&self) -> bool {
        matches!(self, Self::Destroyed)
    }
}

impl EquipItem {
    pub fn mark_updated(&mut self) {
        self.info.last_update += 1;
    }

    fn try_apply_upgrade(&self, upgrade: &ScrollItem, rng: &mut impl rand::Rng) -> UpgradeResult {
        // Proc scroll
        if upgrade.success.proc(self.upgrades as usize, rng) {
            return UpgradeResult::Success;
        }

        if upgrade.destroy.proc(self.upgrades as usize, rng) {
            return UpgradeResult::Destroyed;
        }

        UpgradeResult::Failed
    }

    pub fn apply_upgrade(
        &mut self,
        upgrade: &crate::tmpl::item::BundleItemTmpl,
        protected: bool,
        rng: &mut impl rand::Rng,
    ) -> UpgradeResult {
        let upgrade_id = upgrade.id;
        let BundleItemValue::Scroll(ref upgrade) = upgrade.value else {
            return UpgradeResult::InvalidUpgrade;
        };

        if !self.item_id.can_upgrade_with(upgrade_id) {
            return UpgradeResult::InvalidUpgrade;
        }

        // Handle recover scroll
        if upgrade.recover_slots > 0 {
            return match self.try_apply_upgrade(upgrade, rng) {
                UpgradeResult::Success => {
                    //TODO(!) check max slots
                    self.upgrade_slots += 1;
                    self.mark_updated();
                    UpgradeResult::Success
                }
                res => res,
            };
        }

        if self.upgrade_slots == 0 {
            return UpgradeResult::NoSlots;
        }

        // Apply scroll
        let res = self.try_apply_upgrade(upgrade, rng);
        if res != UpgradeResult::Success {
            // Only take a slot if item is not protected
            if !protected {
                self.upgrade_slots -= 1;
                self.mark_updated();
            }
            return res;
        }

        // Apply the upgrade
        if let Some(ref range) = upgrade.rand_stats {
            self.stats.apply_chaos_scroll(rng, range.clone());
            self.upgrades += 1;
        } else if upgrade.prevent_slip {
            // Slipping flag
            self.flags.insert(ItemFlags::PreventSlipping);
        } else if upgrade.warm_support {
            self.flags.insert(ItemFlags::PreventColdness);
        } else {
            // Stats
            self.stats += &upgrade.inc_stats;
            self.upgrades += 1;
        }
        self.upgrade_slots -= 1;
        self.mark_updated();

        UpgradeResult::Success
    }

    pub fn apply_enhancement(
        &mut self,
        enhance_id: ItemId,
        rng: &mut impl rand::Rng,
    ) -> UpgradeResult {
        // TODO verify item and enhancement

        let success = if enhance_id.0 % 2 == 0 { 1.0 } else { 0.0 };
        let success = (1_f64).min(success - 0.1 * self.stars as f64);
        if !rng.gen_bool(success) {
            return UpgradeResult::Failed;
        }

        for (stat, value) in self.stats.iter_mut() {
            if value.0 == 0 {
                // Skip
                if !stat.add_on_enhance() {
                    continue;
                }

                // Check for add chance
                /*if !rng.gen_bool(0.01) {
                    continue;
                }*/
            }

            let range = 0..=stat.max_enhance_stat().0;
            value.0 = value.0.saturating_add(rng.gen_range(range));
        }

        self.stars += 1;
        self.mark_updated();

        UpgradeResult::Success
    }
    
}

impl Deref for EquipItem {
    type Target = ItemInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for EquipItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

#[derive(Debug, Clone)]
pub struct StackItem {
    pub info: ItemInfo,
    pub quantity: u16,
}

impl Deref for StackItem {
    type Target = ItemInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for StackItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}



/*
impl InvItemId for ItemId {
    fn is_unique(&self) -> bool {
        false
    }
}

impl InvItem for StackItem {
    type Id = ItemId;
    type SlotIndex = usize;

    fn id(&self) -> Self::Id {
        self.info.item_id.into()
    }
}

impl InvStackItem for StackItem {
    fn max_stack_size(&self) -> usize {
        1024
    }

    fn quantity(&self) -> usize {
        self.quantity as usize
    }

    fn set_quantity(&mut self, count: usize) {
        self.quantity = count as u16;
        self.last_update += 1;
    }
}*/

#[derive(Debug, Clone)]
pub struct PetItem {
    pub info: ItemInfo,
    pub dead_at: Option<NaiveDateTime>,
    pub name: String,
    pub level: u8,
    pub tameness: u16,
    pub fullness: u8,
    pub attr1: u16,
    pub attr2: u16,
    pub remaining_life: u32,
    pub skill: u16,
}

impl Deref for PetItem {
    type Target = ItemInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for PetItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

/*impl From<pet_item::Model> for PetItem {
    fn from(value: pet_item::Model) -> Self {
        Self {
            info: ItemInfo {
                is_cash: true,
                db_id: Some(value.id),
                item_id: ItemId(value.item_id as u32),
                game_id: value.game_id,
                expiration: value.expires_at,
                owner: None,
                flags: proto_item::ItemFlags::from_bits(value.flags as u16).unwrap(), //TODO ::from(value.flags as u16),
                last_update: 0,
            },
            dead_at: value.dead_at,
            name: value.name,
            level: value.level as u8,
            tameness: value.tameness as u16,
            fullness: value.fullness as u8,
            attr1: value.attr1 as u16,
            attr2: value.attr2 as u16,
            remaining_life: value.remaining_life as u32,
            skill: value.skill as u16,
        }
    }
}

impl From<&PetItem> for proto_item::ItemPetData {
    fn from(value: &PetItem) -> Self {
        Self {
            info: proto_item::ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id().into(),
                expiration: value.expiration.map(db_to_shroom_time).into(),
            },
            name: value.name.as_str().try_into().unwrap(),
            level: value.level,
            tameness: value.tameness,
            fullness: value.fullness,
            dead_at: value.dead_at.map(db_to_shroom_time).into(), //TODO
            skill: value.skill,
            remain_life: value.remaining_life,
            attribute1: value.attr1,
            attribute2: value.attr2,
        }
    }
}*/
