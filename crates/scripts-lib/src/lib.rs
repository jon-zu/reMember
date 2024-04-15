use std::{
    sync::{Arc, RwLock, RwLockReadGuard},
    time::Duration,
};

use shroom_meta::{id::{job_id::JobId, FieldId, ItemId, Money, NpcId}, QuestDataId};
use shroom_proto95::game::script::ScriptMessage;
use shroom_script::{npc::NpcAction, BoxedNpcPlugin, BoxedSessionCtx, PluginBundle, SessionCtx};

//TODO block reloading as long handles are active

//#[derive(Debug)]
pub struct Shared;

pub struct NpcHandle {
    plugin: BoxedNpcPlugin,
    id: NpcId,
    _shared: Arc<Shared>,
}

struct RefCtx<T>(*mut T);

impl<T> RefCtx<T> {
    fn get_ref(&self) -> &T {
        unsafe { &*self.0 }
    }

    fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}

impl<T: SessionCtx> SessionCtx for RefCtx<T> {
    fn search_fields(&self, query: &str) -> Result<FieldId, Vec<(FieldId, String)>> {
        self.get_ref().search_fields(query)
    }
    
    fn meta(&self) -> &'static shroom_meta::MetaService {
        self.get_ref().meta()
    }

    fn send_msg(&mut self, msg: ScriptMessage) {
        self.get_mut().send_msg(msg);
    }

    fn level(&self) -> u8 {
        self.get_ref().level()
    }

    fn set_level(&mut self, level: u8) {
        self.get_mut().set_level(level);
    }

    fn job(&self) -> JobId {
        self.get_ref().job()
    }

    fn set_job(&mut self, job: JobId) {
        self.get_mut().set_job(job);
    }

    fn has_item(&self, id: ItemId) -> bool {
        self.get_ref().has_item(id)
    }

    fn try_give_item(&mut self, item: ItemId, count: usize) -> anyhow::Result<bool> {
        self.get_mut().try_give_item(item, count)
    }

    fn try_give_items(&mut self, items: &[(ItemId, usize)]) -> anyhow::Result<bool> {
        self.get_mut().try_give_items(items)
    }

    fn money(&self) -> Money {
        self.get_ref().money()
    }

    fn set_money(&mut self, money: Money) {
        self.get_mut().set_money(money);
    }

    fn update_money(&mut self, delta: i32) -> bool {
        self.get_mut().update_money(delta)
    }

    fn transfer_field(&mut self, field_id: FieldId) {
        self.get_mut().transfer_field(field_id);
    }

    fn say(&self, msg: &str) {
        self.get_ref().say(msg);
    }
    
    fn has_item_quantity(&self, id: ItemId, count: usize) -> bool {
        self.get_ref().has_item_quantity(id, count)
    }
    
    fn try_take_item(&mut self, item: ItemId, count: usize) -> anyhow::Result<bool> {
        self.get_mut().try_take_item(item, count)
    }
    
    fn try_take_items(&mut self, items: &[(ItemId, usize)]) -> anyhow::Result<bool> {
        self.get_mut().try_take_items(items)
    }
    
    fn try_take_all_items(&mut self, id: ItemId) -> anyhow::Result<usize> {
        self.get_mut().try_take_all_items(id)
    }
    
    fn get_quest_state_data(&self, id: QuestDataId) -> Option<Vec<u8>> {
        self.get_ref().get_quest_state_data(id)
    }
    
    fn set_quest_state_data(&mut self, id: QuestDataId, data: Vec<u8>) -> anyhow::Result<()> {
        self.get_mut().set_quest_state_data(id, data)
    }
}

unsafe impl<T: Send> Send for RefCtx<T> {}

impl NpcHandle {
    pub fn step<Ctx: SessionCtx + Send + 'static>(
        &mut self,
        ctx: &mut Ctx,
        action: NpcAction,
    ) -> anyhow::Result<()> {
        let ctx = RefCtx(ctx as *mut Ctx);
        let mut ctx: BoxedSessionCtx = Box::new(ctx);
        self.plugin.step(&mut ctx, action)
    }

    pub fn is_finished(&self) -> bool {
        self.plugin.is_finished()
    }

    pub fn npc_id(&self) -> NpcId {
        self.id
    }
}

#[hot_lib_reloader::hot_module(
    dylib = "scripts",
    lib_dir = if cfg!(debug_assertions) { "target/debug" } else { "target/release" })
]
mod hot_lib {
    hot_functions_from_file!("crates/scripts-lib/scripts/src/lib.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

pub struct ScriptService {
    bundle: RwLock<Option<Box<dyn PluginBundle>>>,
    observer: hot_lib_reloader::LibReloadObserver,
    shared: Arc<Shared>,
}

impl std::fmt::Debug for ScriptService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptService").finish()
    }
}

unsafe impl Send for ScriptService {}
unsafe impl Sync for ScriptService {}

impl Default for ScriptService {
    fn default() -> Self {
        Self {
            bundle: RwLock::new(Some(hot_lib::get_plugin_bundle())),
            observer: hot_lib::subscribe(),
            shared: Arc::new(Shared),
        }
    }
}

impl ScriptService {
    fn get_bundle(&self) -> RwLockReadGuard<Option<Box<dyn PluginBundle>>> {
        // TODO!: this has to be reworked, right now this can and will fail sometimes
        // it's just a somewhat effective placebo check
        // to prevent the plugin lib from being reloaded when an active handle is created
        // it has sync issues because a new handle could be created at any time
        if Arc::strong_count(&self.shared) == 1 {
            if let Some(block) = self
                .observer
                .wait_for_about_to_reload_timeout(Duration::ZERO)
            {
                log::info!("About to reload scripts");
                let mut bundle = self.bundle.write().unwrap();
                *bundle = None;
                std::mem::drop(block);
                self.observer.wait_for_reload();
                std::thread::sleep(Duration::from_millis(150));
                *bundle = Some(hot_lib::get_plugin_bundle());
                log::info!("Reloaded new scripts");
            }
        }

        self.bundle.read().unwrap()
    }

    pub fn get_npc_script(&self, name: &str, npc: NpcId) -> Option<NpcHandle> {
        let bundle = self.get_bundle();
        let id = bundle.as_ref().unwrap().get_id_by_name(name)?;
        let plugin = bundle.as_ref().unwrap().get_npc_plugin(id)?;
        Some(NpcHandle {
            plugin,
            id: npc,
            _shared: self.shared.clone(),
        })
    }
}
