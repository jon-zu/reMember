use futures::future::try_join_all;

use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::helper::{stats::with_char_stats, stats::with_equip_stats, *};

#[derive(Iden)]
enum Account {
    Table,
    Id,
    Username,
    PasswordHash,
    Gender,
    AcceptedTos,
    LastLoginAt,
    CreatedAt,
    Pin,
    Pic,
    Country,
    GmLevel,
    LastSelectedWorld,
    CharacterSlots,
    NxCredit,
    NxPrepaid,
    ShroomPoints,
    Tester,
}

#[derive(Iden)]
enum Ban {
    Table,
    Id,
    Reason,
    Time,
    AccId,
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    AccId,
    Name,
    CreatedAt,
    LastLoginAt,
    Gender,
    SkillPoints,
    PlayTime,
}

#[derive(Iden)]
enum ItemStack {
    Table,
    IsCash,
    Id,
    ItemId,
    GameId,
    ExpiresAt,
    Quantity,
    Flags,
}

#[derive(Iden)]
enum EquipItem {
    Table,
    IsCash,
    Id,
    ItemId,
    GameId,
    ExpiresAt,
    Flags,
    OwnerTag,
    LevelTy,
    ItemLevel,
    ItemExp,
    Upgrades,
    HammerSlots,
    Grade,
    Option1,
    Option2,
    Option3,
    Socket1,
    Socket2,
    Durability,
    EquippedAt,
    Stars

}

#[derive(Iden)]
enum PetItem {
    Table,
    Id,
    ItemId,
    GameId,
    ExpiresAt,
    DeadAt,
    Flags,
    Name,
    Level,
    Tameness,
    Fullness,
    Skill,
    RemainingLife,
    Active,
    Attr1,
    Attr2,
}

#[derive(Iden)]
enum InventorySlot {
    Table,
    Id,
    InvType,
    Slot,
    CharId,
    EquipItemId,
    StackItemId,
    PetItemId,
}

#[derive(Iden)]
enum Skill {
    Table,
    Id,
    #[allow(clippy::enum_variant_names)]
    SkillId,
    CharId,
    Level,
    MasterLevel,
    ExpiresAt,
    Cooldown,
}

#[derive(Iden)]
enum FuncKeyMap {
    Table,
    Id,
    CharId,
    Data
}

#[derive(Iden)]
enum Quest {
    Table,
    Id,
    CharId,
    Data,
    CompletedAt,
    StartedAt,
    Status
}

#[derive(DeriveMigrationName)]
pub struct Migration {
    acc_table: ShroomTbl,
    char_table: ShroomTbl,
    ban_table: ShroomTbl,
    eq_table: ShroomTbl,
    stack_item_table: ShroomTbl,
    pet_item_table: ShroomTbl,
    inv_slot_table: ShroomTbl,
    skill_table: ShroomTbl,
    func_key_map_table: ShroomTbl,
    quest_table: ShroomTbl
}

impl Default for Migration {
    fn default() -> Self {
        let acc_table = ShroomTbl::new(
            Account::Table,
            Account::Id,
            false,
            [
                ColumnDef::new(Account::Username)
                    .string()
                    .not_null()
                    .unique_key()
                    .to_owned(),
                ColumnDef::new(Account::PasswordHash)
                    .string()
                    .not_null()
                    .to_owned(),
                shroom_bool(Account::AcceptedTos),
                shroom_gender_col(Account::Gender).null().to_owned(),
                date_time(Account::LastLoginAt),
                created_at(Account::CreatedAt),
                shroom_small_str(Account::Pin),
                shroom_small_str(Account::Pic),
                shroom_id(Account::Country),
                shroom_int(Account::GmLevel),
                shroom_id(Account::LastSelectedWorld),
                shroom_size(Account::CharacterSlots),
                shroom_size(Account::NxCredit),
                shroom_size(Account::NxPrepaid),
                shroom_size(Account::ShroomPoints),
                shroom_bool(Account::Tester),
            ],
            [],
        );

        let char_table = ShroomTbl::new(
            Character::Table,
            Character::Id,
            false,
            with_char_stats([
                shroom_name(Character::Name),
                created_at(Character::CreatedAt),
                date_time(Character::LastLoginAt),
                shroom_gender_col(Character::Gender).not_null().to_owned(),
                shroom_skill_points(Character::SkillPoints),
                shroom_int(Character::PlayTime),
            ]),
            [Ref::ownership(Character::AccId, &acc_table)],
        );

        let ban_table = ShroomTbl::new(
            Ban::Table,
            Ban::Id,
            false,
            [shroom_str(Ban::Reason), date_time(Ban::Time)],
            [Ref::ownership(Ban::AccId, &acc_table)],
        );

        let item_stack_table = ShroomTbl::new(
            ItemStack::Table,
            ItemStack::Id,
            false,
            [
                date_time(ItemStack::ExpiresAt),
                shroom_bool(ItemStack::IsCash),
                shroom_game_item_id(ItemStack::GameId),
                shroom_id(ItemStack::ItemId),
                shroom_int(ItemStack::Flags),
                shroom_size(ItemStack::Quantity),
            ],
            [],
        );

        let item_equip_table = ShroomTbl::new(
            EquipItem::Table,
            EquipItem::Id,
            false,
            with_equip_stats([
                date_time(EquipItem::ExpiresAt),
                shroom_bool(EquipItem::IsCash),
                shroom_game_item_id(EquipItem::GameId),
                shroom_id(EquipItem::ItemId),
                shroom_int(EquipItem::Flags),
                shroom_size(EquipItem::LevelTy),
                shroom_size(EquipItem::ItemLevel),
                shroom_size(EquipItem::ItemExp),
                shroom_size(EquipItem::Upgrades),
                shroom_size(EquipItem::HammerSlots),
                shroom_size(EquipItem::Stars),
                shroom_name(EquipItem::OwnerTag),
                shroom_size(EquipItem::Grade),
                shroom_int(EquipItem::Option1),
                shroom_int(EquipItem::Option2),
                shroom_int(EquipItem::Option3),
                shroom_int(EquipItem::Socket1),
                shroom_int(EquipItem::Socket2),
                shroom_int(EquipItem::Durability),
                date_time(EquipItem::EquippedAt)
            ]),
            [],
        );

        let item_pet_table = ShroomTbl::new(
            PetItem::Table,
            PetItem::Id,
            false,
            [
                date_time(PetItem::ExpiresAt),
                date_time(PetItem::DeadAt),
                shroom_game_item_id(PetItem::GameId),
                shroom_id(PetItem::ItemId),
                shroom_int(PetItem::Flags),
                shroom_name(PetItem::Name),
                shroom_stat(PetItem::Level),
                shroom_stat(PetItem::Tameness),
                shroom_stat(PetItem::Fullness),
                shroom_stat(PetItem::Skill),
                shroom_stat(PetItem::RemainingLife),
                shroom_bool(PetItem::Active),
                shroom_int(PetItem::Attr1),
                shroom_int(PetItem::Attr2),
            ],
            [],
        );

        let inv_slot_table = ShroomTbl::new(
            InventorySlot::Table,
            InventorySlot::Id,
            false,
            [
                shroom_int(InventorySlot::InvType),
                shroom_int(InventorySlot::Slot),
            ],
            [
                Ref::ownership(InventorySlot::CharId, &char_table),
                Ref::opt(InventorySlot::EquipItemId, &item_equip_table),
                Ref::opt(InventorySlot::StackItemId, &item_stack_table),
                Ref::opt(InventorySlot::PetItemId, &item_pet_table),
            ],
        );

        let skill_table = ShroomTbl::new(
            Skill::Table,
            Skill::Id,
            false,
            [
                shroom_id(Skill::SkillId),
                shroom_int(Skill::Level),
                shroom_int(Skill::MasterLevel),
                date_time(Skill::ExpiresAt),
                date_time(Skill::Cooldown),
            ],
            [Ref::ownership(Skill::CharId, &char_table)],
        );

        let func_key_map_table = ShroomTbl::new(
            FuncKeyMap::Table,
            FuncKeyMap::Id,
            false,
            [
                shroom_func_key_map(FuncKeyMap::Data)
            ],
            [Ref::ownership(FuncKeyMap::CharId, &char_table)],
        );

        let quest_table = ShroomTbl::new(
            Quest::Table,
            Quest::Id,
            true,
            [
                date_time(Quest::CompletedAt),
                date_time(Quest::StartedAt),
                shroom_int(Quest::Status),
                shroom_quest_data(Quest::Data)
            ],
            [Ref::ownership_primary(Quest::CharId, &char_table)],
        );


        Self {
            acc_table,
            char_table,
            ban_table,
            eq_table: item_equip_table,
            stack_item_table: item_stack_table,
            pet_item_table: item_pet_table,
            inv_slot_table,
            skill_table,
            func_key_map_table,
            quest_table
        }
    }
}

impl Migration {
    fn table_iter(&self) -> impl Iterator<Item = &'_ ShroomTbl> {
        [
            &self.acc_table,
            &self.char_table,
            &self.ban_table,
            &self.eq_table,
            &self.pet_item_table,
            &self.stack_item_table,
            &self.inv_slot_table,
            &self.skill_table,
            &self.func_key_map_table,
            &self.quest_table
        ]
        .into_iter()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(shroom_gender_ty()).await?;

        for tbl in self.table_iter() {
            tbl.create_table(manager).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        try_join_all(self.table_iter().map(|tbl| tbl.drop_fk(manager))).await?;
        for tbl in self.table_iter() {
            tbl.drop_table(manager).await?;
        }

        manager
            .drop_type(Type::drop().name(Gender::GenderTy).to_owned())
            .await?;

        Ok(())
    }
}
