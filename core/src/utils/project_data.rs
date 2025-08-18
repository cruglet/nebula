use godot::prelude::*;

use crate::utils::{module::Module, singleton::Singleton};

#[derive(GodotClass)]
#[class(base=Object)]
struct ProjectData {
    instance_path: GString,

    base: Base<Object>
}


#[godot_api]
impl IObject for ProjectData {
    fn init(base: Base<Object>) -> Self {
        Self {
            instance_path: GString::new(),
            base,
        }
    }
}


#[godot_api]
impl ProjectData {
    #[func]
    fn get_path() -> GString {
        Singleton::singleton().bind_mut().loaded_project_path.to_godot()
    }

    #[func]
    fn set_path(path: GString) {
        Singleton::singleton().bind_mut().loaded_project_path = path;
    }
}
