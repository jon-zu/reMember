#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub mod blob;
pub mod entities;
pub mod entity_ext;
pub mod proto_mapper;
pub mod services;
pub mod util;
pub mod model;
//pub mod inventory;

use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use entities::{account, ban, character, equip_item, func_key_map, inventory_slot, item_stack, pet_item, quest, skill};

use sea_orm::{
    ActiveValue, ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr,
    Schema,
};
pub const SQL_OPT_MEMORY: &str = "sqlite::memory:";
pub const SQL_OPT_TEST_FILE: &str = "sqlite://test.db?mode=rwc";

pub fn created_at(db: &DatabaseConnection) -> ActiveValue<NaiveDateTime> {
    match db {
        DatabaseConnection::SqlxSqlitePoolConnection(_) => ActiveValue::Set(Utc::now().naive_utc()),
        _ => ActiveValue::NotSet,
    }
}

pub async fn gen_psql(opt: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(opt.to_owned());
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    if log_level != "debug" {
        opt.sqlx_logging(false);
    }
    let db = Database::connect(opt).await?;
    Ok(db)
}

pub async fn gen_sqlite(opt: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(opt.to_owned());
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    if log_level != "debug" {
        opt.sqlx_logging(false);
    }
    //TODO fix this later, but required for now else it drops
    let three_hrs = Duration::from_secs(60 * 60 * 3);
    opt.idle_timeout(three_hrs).max_lifetime(three_hrs);
    let db = Database::connect(opt).await?;

    let schema = Schema::new(DbBackend::Sqlite);

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(account::Entity)),
    )
    .await?;
    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(character::Entity)),
    )
    .await?;
    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(skill::Entity)),
    )
    .await?;
    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(ban::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(equip_item::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(pet_item::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(item_stack::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(inventory_slot::Entity)),
    )
    .await?;



    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(func_key_map::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(quest::Entity)),
    )
    .await?;

    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sqlite_build() {
        gen_sqlite(SQL_OPT_MEMORY).await.unwrap();
    }
}
