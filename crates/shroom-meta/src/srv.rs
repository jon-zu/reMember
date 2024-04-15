use crate::{id::{FieldId, ItemId}, util::search::{SearchMap, TextIndexable}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemConsumables {
    pub id: ItemId,
    pub quantity: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemSet {
    pub equips: Vec<ItemId>,
    pub consumables: Vec<ItemConsumables>
}

impl TextIndexable for ItemSet {
    type Key = String;

    fn name<'a>(&'a self, key: &'a String) -> &'a str {
        key.as_str()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GotoFieldEntry(pub FieldId);

impl TextIndexable for GotoFieldEntry {
    type Key = String;

    fn name<'a>(&'a self, key: &'a String) -> &'a str {
        key.as_str()
    }
}

pub type GoToFields = SearchMap<GotoFieldEntry>;
pub type ItemSets = SearchMap<ItemSet>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_item_sets() {
        let data = std::fs::read_to_string(
            "/home/jonas/projects/shroom/ShroomMS/shroom-metadata/item_sets.json",
        )
        .unwrap();
        let sets: ItemSets = serde_json::from_str(&data).unwrap();
        assert!(!sets.is_empty());
    }

    #[test]
    fn load_goto_fields() {
        let data = std::fs::read_to_string(
            "/home/jonas/projects/shroom/ShroomMS/shroom-metadata/fields_goto.json",
        )
        .unwrap();
        let sets: GoToFields = serde_json::from_str(&data).unwrap();
        assert!(!sets.is_empty());
    }
}
