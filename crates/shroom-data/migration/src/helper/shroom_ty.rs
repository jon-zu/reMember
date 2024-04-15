use sea_orm_migration::{
    prelude::*,
    sea_query::extension::postgres::{Type, TypeCreateStatement},
};

#[derive(Iden)]
pub enum Gender {
    GenderTy,
    Female,
    Male,
}

pub fn shroom_gender_ty() -> TypeCreateStatement {
    Type::create()
        .as_enum(Gender::GenderTy)
        .values([Gender::Male, Gender::Female])
        .to_owned()
}

pub fn shroom_quest_data(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .binary()
        .binary_len(32)
        .not_null()
        .to_owned()
}

pub fn shroom_skill_points(id: impl IntoIden) -> ColumnDef {
    const PAGES: u32 = 10;
    ColumnDef::new(id)
        .binary()
        .binary_len(PAGES * 2)
        .not_null()
        .to_owned()
}

pub fn shroom_func_key_map(id: impl IntoIden) -> ColumnDef {
    const KEYS: u32 = 89;
    ColumnDef::new(id)
        .binary()
        .binary_len(KEYS * 5)
        .not_null()
        .to_owned()
}

pub fn shroom_gender_col(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .enumeration(Gender::GenderTy, [Gender::Male, Gender::Female])
        .to_owned()
}

pub fn shroom_id(name: impl IntoIden) -> ColumnDef {
    ColumnDef::new(name).integer().not_null().to_owned()
}

pub fn shroom_opt_id(name: impl IntoIden) -> ColumnDef {
    ColumnDef::new(name).integer().null().to_owned()
}

pub fn shroom_id_pkey(name: impl IntoIden, auto_inc: bool) -> ColumnDef {
    let mut id = shroom_id(name);
    if auto_inc {
        id.auto_increment().primary_key().to_owned()
    } else {
        id.to_owned()
    }
}

pub fn shroom_int(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .integer()
        .default(0)
        .not_null()
        .to_owned()
}

pub fn shroom_size(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .integer()
        .default(0)
        .not_null()
        .to_owned()
}

pub fn shroom_bool(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .boolean()
        .not_null()
        .default(false)
        .to_owned()
}

pub fn shroom_str(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).string().to_owned()
}

pub fn shroom_small_str(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).string().string_len(16).to_owned()
}

pub fn date_time(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).timestamp().to_owned()
}

pub fn shroom_game_item_id(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .big_integer()
        .not_null()
        .unique_key()
        .to_owned()
}

pub fn created_at(id: impl IntoIden) -> ColumnDef {
    date_time(id)
        .default(Expr::current_timestamp())
        .not_null()
        .to_owned()
}

pub fn shroom_name(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).string_len(13).not_null().to_owned()
}

pub fn shroom_stat(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .integer()
        .not_null()
        .default(0)
        .to_owned()
}

pub fn char_stat(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .integer()
        .not_null()
        .default(0)
        .to_owned()
}
