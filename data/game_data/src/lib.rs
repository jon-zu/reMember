pub mod gen;
pub mod ha_xml;
pub mod schema;

use gen::map::Portal;

pub use crate::gen::map;
pub use crate::gen::mob;
pub use crate::gen::wz2;

// Map ext

impl map::Map {
    pub fn get_return_map(&self) -> Option<u32> {
        let ret_map = self.info.return_map.as_ref();
        ret_map.map(|m| *m as u32)
    }

    pub fn get_first_portal_id(&self) -> Option<u8> {
        self.portal.keys().next().map(|k| *k as u8)
    }

    pub fn get_portal_by_name(&self, tn: &str) -> Option<(u8, &Portal)> {
        self.portal
            .iter()
            .find(|(_, p)| p.pn == tn)
            .map(|(k, v)| (*k as u8, v))
    }
}
