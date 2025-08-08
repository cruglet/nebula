use godot::prelude::*;

mod utils;

struct NebulaCore;

#[gdextension]
unsafe impl ExtensionLibrary for NebulaCore {
    fn on_level_init(init: InitLevel) {
        if init == InitLevel::Scene {
            godot_print_rich!("[color=green][NebulaCore] Initialized!");
        }
    }
}
