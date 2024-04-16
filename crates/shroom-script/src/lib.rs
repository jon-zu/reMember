use npc::NpcPlugin;
use shroom_meta::{
    id::{job_id::JobId, FieldId, ItemId, Money, NpcId, QuestId},
    MetaService, QuestDataId,
};
use shroom_proto95::game::script::ScriptMessage;

pub mod npc;
pub mod poll_state;

pub type PluginId = usize;
pub trait SessionCtx {
    fn set_npc_id(&mut self, id: Option<NpcId>);
    fn current_npc_id(&self) -> Option<NpcId>;
    fn send_msg(&mut self, msg: ScriptMessage);

    fn level(&self) -> u8;
    fn set_level(&mut self, level: u8);

    fn job(&self) -> JobId;
    fn set_job(&mut self, job: JobId);

    fn has_item(&self, id: ItemId) -> bool;
    fn has_item_quantity(&self, id: ItemId, count: usize) -> bool;
    fn try_take_item(&mut self, item: ItemId, count: usize) -> anyhow::Result<bool>;
    fn try_take_items(&mut self, items: &[(ItemId, usize)]) -> anyhow::Result<bool>;
    fn try_take_all_items(&mut self, id: ItemId) -> anyhow::Result<usize>;
    fn try_give_item(&mut self, item: ItemId, count: usize) -> anyhow::Result<bool>;
    fn try_give_items(&mut self, items: &[(ItemId, usize)]) -> anyhow::Result<bool>;

    fn money(&self) -> Money;
    fn set_money(&mut self, money: Money);
    fn update_money(&mut self, delta: i32) -> bool;

    fn get_quest_state_data(&self, id: QuestDataId) -> Option<Vec<u8>>;
    fn set_quest_state_data(&mut self, id: QuestDataId, data: Vec<u8>) -> anyhow::Result<()>;

    fn has_completed_quest(&self, id: QuestId) -> bool;
    fn is_active_quest(&self, id: QuestId) -> bool;

    fn transfer_field(&mut self, field_id: FieldId);

    fn say(&self, msg: &str);

    fn meta(&self) -> &'static MetaService;
    fn search_fields(&self, query: &str) -> Result<FieldId, Vec<(FieldId, String)>>;
}

pub type BoxedSessionCtx = Box<dyn SessionCtx + Send>;
pub type BoxedNpcPlugin = Box<dyn NpcPlugin + Send>;
pub trait PluginBundle {
    fn get_id_by_name(&self, name: &str) -> Option<PluginId>;
    fn get_npc_plugin(&self, id: PluginId) -> Option<BoxedNpcPlugin>;
    fn get_fallback_npc_plugin(&self) -> BoxedNpcPlugin;
}
