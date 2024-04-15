use shroom_meta::{
    id::{item_id::ItemGrade, ItemId, ItemOptionId},
    item::{
        it::{EquipItem, EquipOptions, EquipSockets, ItemFlags, ItemInfo, ItemLevelInfo, PetItem, StackItem},
        EquipBaseStats, EquipStat, ItemStat,
    },
};


use crate::entities::{equip_item, item_stack, pet_item};

use super::inv::{PetItemSlot, StackItemSlot};

impl From<equip_item::Model> for EquipItem {
    fn from(value: equip_item::Model) -> Self {
        let stats = EquipBaseStats::from_fn(|stat| match stat {
            EquipStat::Acc => ItemStat(value.accuracy as u16),
            EquipStat::Eva => ItemStat(value.avoid as u16),
            EquipStat::Dex => ItemStat(value.dex as u16),
            EquipStat::Int => ItemStat(value.int as u16),
            EquipStat::Luk => ItemStat(value.luk as u16),
            EquipStat::Str => ItemStat(value.str as u16),
            EquipStat::Hp => ItemStat(value.hp as u16),
            EquipStat::Mp => ItemStat(value.mp as u16),
            EquipStat::Pad => ItemStat(value.weapon_atk as u16),
            EquipStat::Mad => ItemStat(value.magic_atk as u16),
            EquipStat::Pdd => ItemStat(value.weapon_def as u16),
            EquipStat::Mdd => ItemStat(value.magic_def as u16),
            EquipStat::Craft => ItemStat(value.craft as u16),
            EquipStat::Speed => ItemStat(value.speed as u16),
            EquipStat::Jump => ItemStat(value.jump as u16),
        });
        let owner = if value.owner_tag.is_empty() {
            None
        } else {
            Some(value.owner_tag)
        };
        Self {
            info: ItemInfo {
                db_id: Some(value.id),
                is_cash: value.is_cash,
                item_id: ItemId(value.item_id as u32),
                game_id: value.game_id,
                expiration: value.expires_at,
                owner,
                flags: ItemFlags::from_bits(value.flags as u16).unwrap(),
                last_update: 0,
            },
            hammers_used: value.hammer_slots as u8,
            level_info: Some(ItemLevelInfo {
                item_level_ty: value.level_ty as u8, //TODO
                level: value.item_level as u8,       //TODO
                exp: value.item_exp as u32,
            }),
            upgrade_slots: value.upgrade_slots as u8,
            upgrades: value.upgrades as u8,
            stats,
            sockets: EquipSockets([value.socket1 as u16, value.socket2 as u16]),
            options: EquipOptions([
                ItemOptionId(value.option1 as u16),
                ItemOptionId(value.option2 as u16),
                ItemOptionId(value.option3 as u16),
            ]),
            equipped_at: value.equipped_at,
            durability: value.durability.into(),
            grade: ItemGrade::try_from(value.grade as u8).unwrap(),
            stars: value.stars as u8,
        }
    }
}

impl From<item_stack::Model> for StackItem {
    fn from(value: item_stack::Model) -> Self {
        Self {
            info: ItemInfo {
                db_id: Some(value.id),
                is_cash: value.is_cash,
                item_id: ItemId(value.item_id as u32),
                game_id: value.game_id,
                expiration: value.expires_at,
                owner: None,
                flags: ItemFlags::from_bits(value.flags as u16).unwrap(), //TODO ::from(value.flags as u16),
                last_update: 0,
            },
            quantity: value.quantity as u16,
        }
    }
}


impl From<item_stack::Model> for StackItemSlot {
    fn from(value: item_stack::Model) -> Self {
        Self {
            item: Box::new(value.into()),
        }
    }
}

impl From<pet_item::Model> for PetItem {
    fn from(value: pet_item::Model) -> Self {
        Self {
            info: ItemInfo {
                is_cash: true,
                db_id: Some(value.id),
                item_id: ItemId(value.item_id as u32),
                game_id: value.game_id,
                expiration: value.expires_at,
                owner: None,
                flags: ItemFlags::from_bits(value.flags as u16).unwrap(),
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

impl From<pet_item::Model> for PetItemSlot {
    fn from(value: pet_item::Model) -> Self {
        Self {
            item: Box::new(value.into()),
        }
    }
}