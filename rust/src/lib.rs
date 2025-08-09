use godot::prelude::*;
use crate::utils::singleton::Singleton;

mod utils;

struct NebulaCore;

#[gdextension]
unsafe impl ExtensionLibrary for NebulaCore {
    fn on_level_init(init: InitLevel) {
        if init == InitLevel::Scene {
            let singleton = Singleton::new_alloc();
            singleton.bind().register();

            godot_print_rich!("[color=green][NebulaCore] Initialized!");
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut singleton = Singleton::singleton();
            singleton.bind_mut().unregister();
            singleton.free();
        }
    }
}
