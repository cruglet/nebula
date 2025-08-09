use godot::{classes::{file_access::ModeFlags, DirAccess, DisplayServer, FileAccess, Window}, prelude::*};

use crate::utils::singleton::Singleton;

#[derive(GodotClass)]
#[class(base=Object)]
struct CoreSettings {
    base: Base<Object>
}

#[godot_api]
impl IObject for CoreSettings {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl CoreSettings {
    #[constant] const SETTING_UI_SCALE: i32 = 0;
    #[constant] const SETTING_PROJECT_LIST: i32 = 1;
    const MAX: i32 = 2;

    #[func]
    fn get_defaults() -> Dictionary {
        let mut data: Dictionary = Dictionary::new();

        data.set(CoreSettings::SETTING_UI_SCALE, DisplayServer::singleton().screen_get_scale());
        data.set(CoreSettings::SETTING_PROJECT_LIST, Array::<GString>::new());

        data
    }

    #[func]
    fn apply_config() {
        let mut window: Gd<Window> = Singleton::get_tree().get_root().unwrap();

        for i in 0..CoreSettings::MAX {
            match i {
                CoreSettings::SETTING_PROJECT_LIST => {}
                CoreSettings::SETTING_UI_SCALE => {
                    let ui_scale: f32 = CoreSettings::get(CoreSettings::SETTING_UI_SCALE).try_to().unwrap();
                    window.set_content_scale_factor(ui_scale);
                    DisplayServer::singleton().window_set_min_size(Vector2i { x: (960.0 * ui_scale).round() as i32, y: (540.0 * ui_scale).round() as i32 });
                }
                _ => {
                    godot_error!("Config loop out of range!");
                }
            }
        }

    }


    #[func]
    fn exists() -> bool {
        if !FileAccess::file_exists(&CoreSettings::get_path()) {
            return false;
        }

        let config_file_op: Option<Gd<FileAccess>> = FileAccess::open(&CoreSettings::get_path(), ModeFlags::READ);

        if config_file_op.is_some() {
            let config_file = config_file_op.unwrap();
            if config_file.get_var().get_type() == VariantType::DICTIONARY {
                return true;
            };
        }

        false
    }

    #[func]
    fn get_path() -> GString {
        GString::from("user://core.cfg")
    }

    #[func]
    fn get(key: i32) -> Variant {
        let mut val: Variant = CoreSettings::get_defaults().get_or_nil(key.to_godot());

        let data: Dictionary = CoreSettings::_get_data();
        if data.contains_key(key.to_godot()) {
            val = data.get_or_nil(key.to_godot());
        }

        val
    }

    #[func]
    fn set(key: i32, value: Variant) -> bool {
        let mut data: Dictionary = Dictionary::new();

        data.set(key, value);

        // Create dir if it does not exist
        if !FileAccess::file_exists(&CoreSettings::get_path()) {
            DirAccess::make_dir_recursive_absolute(&CoreSettings::get_path().get_base_dir());
        }

        if let Some(mut file) = FileAccess::open(&CoreSettings::get_path(), ModeFlags::WRITE) {
            file.store_var(&data.to_variant());
        } else {
            godot_print!("Failed to open file: {}", CoreSettings::get_path());
        }

        true
    }

    #[func]
    fn set_and_apply(key: i32, value: Variant) -> bool {
        let success: bool = CoreSettings::set(key, value);
        CoreSettings::apply_config();
        success
    }

    pub fn _get_data() -> Dictionary {
        let mut data: Dictionary = Dictionary::new();

        if CoreSettings::exists() {
            let file: Gd<FileAccess> = FileAccess::open(&CoreSettings::get_path(), ModeFlags::READ).unwrap();
            data = file.get_var().try_to().unwrap();
        }

        data
    }
}
