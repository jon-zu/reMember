//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "equip_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub expires_at: Option<DateTime>,
    pub is_cash: bool,
    #[sea_orm(unique)]
    pub game_id: i64,
    pub item_id: i32,
    pub flags: i32,
    pub level_ty: i32,
    pub item_level: i32,
    pub item_exp: i32,
    pub upgrades: i32,
    pub hammer_slots: i32,
    pub stars: i32,
    pub owner_tag: String,
    pub grade: i32,
    pub option1: i32,
    pub option2: i32,
    pub option3: i32,
    pub socket1: i32,
    pub socket2: i32,
    pub durability: i32,
    pub equipped_at: Option<DateTime>,
    pub level: i32,
    pub upgrade_slots: i32,
    pub str: i32,
    pub dex: i32,
    pub luk: i32,
    pub int: i32,
    pub hp: i32,
    pub mp: i32,
    pub weapon_atk: i32,
    pub magic_atk: i32,
    pub weapon_def: i32,
    pub magic_def: i32,
    pub accuracy: i32,
    pub avoid: i32,
    pub craft: i32,
    pub speed: i32,
    pub jump: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::inventory_slot::Entity")]
    InventorySlot,
}

impl Related<super::inventory_slot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InventorySlot.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
