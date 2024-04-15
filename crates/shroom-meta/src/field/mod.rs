pub mod fh_tree;
use std::{collections::BTreeMap, ops::RangeInclusive, time::Duration};

pub use fh_tree::FhTree;
use serde::{Deserialize, Serialize};

use crate::{
    id::{FieldId, FootholdId, MobId, NpcId, ReactorId},
    twod::{Rect2D, Vec2},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldNpc {
    pub id: NpcId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub hide: bool,
    pub range_x: RangeInclusive<i16>,
    pub flip: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldMob {
    pub id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub hide: bool,
    pub respawn_time: Option<Duration>,
    pub range_x: RangeInclusive<i16>,
    pub flip: bool,
    pub cy: Option<i16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldLife {
    Mob(FieldMob),
    Npc(FieldNpc),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldReactor {
    pub pos: Vec2,
    pub name: Option<String>,
    pub id: ReactorId,
    pub time: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldPortal {
    pub pos: Vec2,
    pub only_once: bool,
    pub hide_tooltip: bool,
    pub has_delay: bool,
    pub teleport: bool,
    pub reactor_name: Option<String>,
    pub script: Option<String>,
    pub session_value: Option<String>,
    pub session_value_key: Option<String>,
    pub tm: Option<FieldId>,
    pub tn: Option<String>,
    pub pn: Option<String>,
    pub pt: Option<FieldId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Foothold {
    pub pt1: Vec2,
    pub pt2: Vec2,
    pub next: FootholdId,
    pub prev: FootholdId,
    pub forbid_falldown: bool,
    pub cant_through: bool,
    pub force: Option<i32>,
    pub piece: Option<i32>,
}

#[derive(Debug)]
pub struct SpawnPoint {
    pub id: u8,
    pub pos: Vec2,
}

impl From<(u8, &FieldPortal)> for SpawnPoint {
    fn from(portal: (u8, &FieldPortal)) -> Self {
        Self {
            id: portal.0,
            pos: portal.1.pos,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: FieldId,
    pub cloud: bool,
    pub scroll_disable: bool,
    pub no_regen: bool,
    pub fly: bool,
    pub zakum_hack: bool,
    pub rect: Rect2D,
    pub return_field: Option<FieldId>,
    pub forced_return_field: Option<FieldId>,
    pub portals: BTreeMap<u8, FieldPortal>,
    pub life: BTreeMap<u32, FieldLife>,
    pub reactors: BTreeMap<u32, FieldReactor>,
    pub footholds: BTreeMap<FootholdId, BTreeMap<FootholdId, BTreeMap<FootholdId, Foothold>>>,
    pub fh_tree: FhTree,
}

impl Field {
    pub fn get_return_field_id(&self) -> FieldId {
        self.return_field
            .or(self.forced_return_field)
            .unwrap_or(self.id)
    }

    pub fn get_first_portal_id(&self) -> Option<u8> {
        self.portals.keys().next().cloned()
    }

    pub fn get_portal_by_name(&self, tn: &str) -> Option<(u8, &FieldPortal)> {
        self.portals
            .iter()
            .find(|(_, p)| p.pn.as_deref() == Some(tn))
            .map(|(k, v)| (*k, v))
    }

    pub fn get_spawn_point_by_name(&self, tn: &str) -> Option<SpawnPoint> {
        self.get_portal_by_name(tn)
            .map(|(id, portal)| (id, portal).into())
    }

    pub fn get_default_spawn_point(&self) -> Option<SpawnPoint> {
        self.get_first_portal_id()
            .and_then(|id| self.get_spawn_point(id))
    }

    pub fn get_spawn_point(&self, sp: u8) -> Option<SpawnPoint> {
        self.portals.get(&sp).map(|portal| (sp, portal).into())
    }

    pub fn get_target_field(&self, portal_name: &str) -> Option<(FieldId, &FieldPortal)> {
        let (_, portal) = self.get_portal_by_name(portal_name)?;
        let map_id = if portal.tm == Some(FieldId(999999)) {
            self.id
        } else {
            //TODO unwrap
            portal.tm.unwrap()
        };
        Some((map_id, portal))
    }
}
