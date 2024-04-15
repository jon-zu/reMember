use sea_orm_migration::prelude::*;

use super::char_stat;

pub const CHAR_STATS: [&str; 28] = [
    "level",
    "exp",
    "gacha_exp",
    "str",
    "dex",
    "luk",
    "int",
    "hp",
    "max_hp",
    "mp",
    "max_mp",
    "mesos",
    "buddy_capacity",
    "fame",
    "sp",
    "ap",
    "job",
    "equip_slots",
    "use_slots",
    "setup_slots",
    "etc_slots",
    "cash_slots",
    "storage_slots",
    "face",
    "skin",
    "hair",
    "field_id",
    "spawn_point"
];

pub fn with_char_stats(columns: impl IntoIterator<Item = ColumnDef>) -> Vec<ColumnDef> {
    columns
        .into_iter()
        .chain(CHAR_STATS.iter().map(|stat| char_stat(Alias::new(stat.to_string()))))
        .collect()
}

pub const ITEM_STATS: [&str; 17] = [
    "level",
    "upgrade_slots",
    "str",
    "dex",
    "luk",
    "int",
    "hp",
    "mp",
    "pad",
    "mad",
    "pdd",
    "mdd",
    "accuracy",
    "eva",
    "craft",
    "speed",
    "jump",
];


pub fn with_equip_stats(columns: impl IntoIterator<Item = ColumnDef>) -> Vec<ColumnDef> {
    columns
        .into_iter()
        .chain(ITEM_STATS.iter().map(|stat| char_stat(Alias::new(stat.to_string()))))
        .collect()
}
