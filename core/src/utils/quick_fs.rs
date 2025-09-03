use godot::prelude::*;
use godot::classes::{Object, IObject};
use std::fs;


#[derive(GodotClass)]
#[class(base=Object)]
struct QuickFS {
    base: Base<Object>,
}

#[godot_api]
impl IObject for QuickFS {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl QuickFS {
    #[func]
    fn delete_folder_recursively(path: GString) {
        let path_str = path.get_base_dir().to_string();
        match fs::remove_dir_all(path_str) {
            Ok(_) => {}
            Err(err) => {
                godot_print!("Error removing directory contents!\nError: {}", err);
                godot_print!("{}", path);
            }
        }
    }
}
