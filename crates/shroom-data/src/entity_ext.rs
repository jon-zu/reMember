use either::Either;

use shroom_proto95::{
    game::key_map::{FUNC_KEYS, QUICK_SLOTS},
    shared::char::{InventorySize, SkillPointPage},
};

use crate::{blob::BinaryBlob, entities::character};

//TODO FuncKeyMap

pub const KEYS: usize = FUNC_KEYS;

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FuncKey {
    pub ty: u8,
    pub action: u32,
}

impl FuncKey {
    pub fn to_proto(&self) -> shroom_proto95::game::key_map::FuncKey {
        shroom_proto95::game::key_map::FuncKey {
            ty: self.ty,
            action_id: self.action,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FuncKeyMap {
    keys: Vec<FuncKey>,
    #[serde(skip)]
    updated: bool
}
    
impl BinaryBlob for FuncKeyMap {}

impl Default for FuncKeyMap {
    fn default() -> Self {
        let mut map = Self{
            keys: vec![FuncKey::default(); KEYS],
            updated: false
        };

        let keys = [
            2, 3, 35, 4, 36, 5, 37, 6, 38, 7, 8, 41, 43, 44, 13, 45, 46, 16, 17, 18, 21, 23, 57, 26,
        ];
        let ty = [
            6, 6, 4, 6, 4, 6, 4, 6, 4, 6, 6, 4, 4, 5, 4, 5, 5, 4, 4, 4, 5, 4, 5, 4,
        ];
        let action = [
            100, 101, 2, 102, 9, 103, 3, 104, 7, 105, 106, 23, 28, 50, 4, 51, 52, 8, 5, 0, 54, 1,
            53, 15,
        ];

        for i in 0..keys.len() {
            map.set(
                keys[i],
                FuncKey {
                    ty: ty[i],
                    action: action[i],
                },
            ).unwrap();
        }

        map
    }
}

impl FuncKeyMap {
    pub fn to_proto(&self) -> [shroom_proto95::game::key_map::FuncKey; KEYS] {
        array_init::array_init(|i| self.keys[i].to_proto())
    }

    pub fn set(&mut self, key: usize, func: FuncKey) -> anyhow::Result<()> {
        if key >= KEYS {
            return Err(anyhow::anyhow!("Invalid key index"));
        }
        self.keys[key] = func;
        self.updated = true;
        Ok(())
    }

    pub fn get(&self, key: usize) -> Option<&FuncKey> {
        self.keys.get(key)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct QuickSlotKeyMap{ 
    slots: [u32; QUICK_SLOTS],
    updated: bool
}
impl BinaryBlob for QuickSlotKeyMap {}

impl QuickSlotKeyMap {
    pub fn to_proto(&self) -> [u32; QUICK_SLOTS] {
        self.slots
    }

    pub fn set(&mut self, slots: [u32; QUICK_SLOTS]) {
        self.slots = slots;
        self.updated = true;
    }

}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct KeyMap {
    pub func: Option<FuncKeyMap>,
    pub quick: Option<QuickSlotKeyMap>,
}
impl BinaryBlob for KeyMap {}

impl KeyMap {
    pub fn func(&self) -> Option<&FuncKeyMap> {
        self.func.as_ref()
    }

    pub fn quick_slots(&self) -> Option<&QuickSlotKeyMap> {
        self.quick.as_ref()
    }


    pub fn func_mut(&mut self) -> &mut FuncKeyMap {
        let map = self.func.get_or_insert_with(FuncKeyMap::default);
        map.updated = true;
        map
    }


    pub fn set_quick_slots(&mut self, map: [u32; QUICK_SLOTS]) {
        self.quick = Some(QuickSlotKeyMap {
            slots: map,
            updated: true,
        });
    }

    pub fn is_changed(&self) -> bool {
        self.func.as_ref().map_or(false, |f| f.updated)
            || self.quick.as_ref().map_or(false, |q| q.updated)
    }
}



pub const TOTAL_SKILL_PAGES: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillPointPages([u16; TOTAL_SKILL_PAGES]);

impl SkillPointPages {
    pub fn new(pages: [u16; TOTAL_SKILL_PAGES]) -> Self {
        Self(pages)
    }

    pub fn as_data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.0)
    }

    pub fn get(&self, page: usize) -> &u16 {
        &self.0[page]
    }

    pub fn get_mut(&mut self, page: usize) -> &mut u16 {
        &mut self.0[page]
    }

    pub fn to_array(&self) -> [u16; TOTAL_SKILL_PAGES] {
        self.0
    }

    pub fn to_proto(
        &self,
        ext: bool,
    ) -> Either<shroom_proto95::shared::char::SkillPointPages, u16> {
        if ext {
            // TODO: handle overflow for u8
            Either::Left(array_init::array_init(|i| SkillPointPage {
                index: i as u8,
                value: *self.get(i) as u8,
            }))
        } else {
            Either::Right(*self.get(0))
        }
    }
}

impl character::Model {
    pub fn get_skill_pages(&self) -> SkillPointPages {
        SkillPointPages(
            *bytemuck::try_from_bytes(self.skill_points.as_slice()).expect("skill pages"),
        )
    }

    pub fn get_inventory_size(&self) -> InventorySize {
        [
            self.equip_slots as u8,
            self.use_slots as u8,
            self.setup_slots as u8,
            self.etc_slots as u8,
            self.cash_slots as u8,
        ]
    }
}
