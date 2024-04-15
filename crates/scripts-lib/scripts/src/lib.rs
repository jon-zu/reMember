use shroom_script::{npc::FutureNpcPlugin, BoxedNpcPlugin, PluginBundle, PluginId};

pub mod samples;
pub mod job_adv;

macro_rules! plugin_bundle {
    ($name:ident, $(($id:expr, $pname:ident, $pfn:path)),*) => {
        pub struct $name;

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }

        impl PluginBundle for $name {
            fn get_npc_plugin(&self, id: PluginId) -> Option<BoxedNpcPlugin> {
                match id {
                    $($id => Some(FutureNpcPlugin::launch($pfn)),)*
                    _ => None,
                }
            }

            fn get_id_by_name(&self, name: &str) -> Option<PluginId> {
                Some(match name {
                    $(stringify!($pname) => $id,)*
                    _ => return None,
                })
            }
        }
    };
}

plugin_bundle!(
    BasicPluginBundle,
    (0, npc_1000, samples::npc_script_1000),
    (1, npc_minesweeper, samples::npc_script_minesweeper),
    (2, npc_memory, samples::npc_script_memory),
    (3, npc_taxi, samples::npc_script_taxi),
    (4, npc_warrior, samples::npc_script_warrior),
    (5, npc_guess, samples::npc_guess_game),
    (6, npc_boss_spawner, samples::npc_boss_spawner),
    (7, field_search, samples::npc_field_finder),
    (8, npc_job_adv, job_adv::npc_script_warrior),
    (9, npc_job2_adv, job_adv::npc_script_warrior2),
    (10, npc_job2_inside, job_adv::npc_script_warrior2_inside),
    (11, npc_chief_warrior, job_adv::npc_script_warrior_chief),
    (12, npc_mirror, job_adv::npc_script_mirror),
    (13, npc_mirror_inside, job_adv::npc_script_mirror_inside),
    (14, npc_holy_stone, job_adv::npc_script_holy_stone)
);

#[no_mangle]
pub fn get_plugin_bundle() -> Box<dyn shroom_script::PluginBundle> {
    println!("Loading Plugin bundle!");
    Box::<BasicPluginBundle>::default()
}
