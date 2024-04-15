use plugins::npc::CharCtx;
use shroom_proto95::{game::script::ScriptMessage, id::ItemId};

use crate::services::character::Character;

impl CharCtx for Character {
    fn send_msg(&mut self, msg: ScriptMessage) {
        self.npc_msg.push_back(msg);
    }

    fn set_money(&mut self, money:shroom_proto95::shared::char::Money) {
        *self.stats.money_mut() = money;
    }

    fn update_money(&mut self, delta: i32) -> bool {
        self.update_mesos(delta)
    }

    fn get_money(&self) ->shroom_proto95::shared::char::Money {
        self.stats.money
    }

    fn get_level(&self) -> u8 {
        self.stats.level
    }

    fn set_level(&mut self, level: u8) {
        self.stats.level_mut().set(level);
    }

    fn has_item(&self, id: ItemId) -> bool {
        self.inventory.contains_id(&id).unwrap_or(false)
    }

    fn try_add_item(&mut self, id: ItemId, quantity: usize) -> bool {
        self.add_stack_item(id.get_inv_type().unwrap(), id, quantity)
            .is_ok()
    }

    fn try_add_items(&mut self, items: &[(ItemId, usize)]) -> bool {
        for (id, quantity) in items.iter().cloned() {
            if !self.try_add_item(id, quantity) {
                return false;
            }
        }

        true
    }
}

pub type NpcScriptHandle = plugins::npc::NpcScriptHandle<Character>;
