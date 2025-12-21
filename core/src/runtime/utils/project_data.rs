use godot::prelude::*;

use crate::runtime::utils::singleton::Singleton;

/// Helper class for managing a project's settings & data.
#[allow(dead_code)]
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
    /// Returns the path to the currently loaded project file.
    #[func]
    fn get_path() -> GString {
        Singleton::singleton().bind_mut().loaded_project_path.to_godot_owned()
    }

    /// Assigns the currently loaded project file to [param path].
    #[func]
    fn set_path(path: GString) {
        Singleton::singleton().bind_mut().loaded_project_path = path;
    }
}
