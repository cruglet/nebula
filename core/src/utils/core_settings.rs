use godot::{classes::{file_access::ModeFlags, DirAccess, DisplayServer, FileAccess, Window}, prelude::*};

use crate::utils::singleton::Singleton;

/// Manages core configuration settings for the application.
///
/// `CoreSettings` provides methods to read, write, and apply global settings such as
/// UI scale, project list, and module list. Configurations are stored in `user://core.cfg`.
#[derive(GodotClass)]
#[class(base=Object)]
pub struct CoreSettings {
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
    /// Constant representing the UI scale setting.
    #[constant] pub const SETTING_UI_SCALE: i32 = 0;

    /// Constant representing the list of projects.
    #[constant] pub const SETTING_PROJECT_LIST: i32 = 1;

    /// Constant representing the list of modules.
    #[constant] pub const SETTING_MODULE_LIST: i32 = 2;
    const MAX: i32 = 3;

    /// Returns the default configuration values as a VarDictionary.
    #[func]
    fn get_defaults() -> VarDictionary {
        let mut data: VarDictionary = VarDictionary::new();

        data.set(CoreSettings::SETTING_UI_SCALE, DisplayServer::singleton().screen_get_scale());
        data.set(CoreSettings::SETTING_PROJECT_LIST, Array::<GString>::new());
        data.set(CoreSettings::SETTING_MODULE_LIST, Array::<GString>::new());

        data
    }

    /// Applies configuration settings to the running application.
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
                CoreSettings::SETTING_MODULE_LIST => {},
                _ => {
                    godot_error!("Config loop out of range!");
                }
            }
        }

    }

    /// Checks if the configuration file exists and is valid.
    ///
    /// Returns `true` if `user://core.cfg` exists and contains a valid VarDictionary.
    #[func]
    fn exists() -> bool {
        if !FileAccess::file_exists(&CoreSettings::get_path()) {
            return false;
        }

        if let Some(config_file) = FileAccess::open(&CoreSettings::get_path(), ModeFlags::READ) && config_file.get_var().get_type() == VariantType::DICTIONARY {
            return true;
        }

        false
    }

    /// Returns the file path of the core configuration.
    #[func]
    fn get_path() -> GString {
        GString::from("user://core.cfg")
    }

    /// Gets the value of a specific setting.
    ///
    /// Parameters:
    /// - `key`: One of the `SETTING_*` constants.
    ///
    /// Returns the setting value as a `Variant`. If the key is missing, returns the default.
    #[func]
    pub fn get(key: i32) -> Variant {
        let mut val: Variant = CoreSettings::get_defaults().get_or_nil(key.to_godot());

        let data: VarDictionary = CoreSettings::_get_data();
        if data.contains_key(key.to_godot()) {
            val = data.get_or_nil(key.to_godot());
        }

        val
    }

    /// Sets the value of a specific setting and writes to disk.
    ///
    /// Parameters:
    /// - `key`: One of the `SETTING_*` constants.
    /// - `value`: The value to store.
    ///
    /// Returns `true` if the setting was stored successfully.
    #[func]
    pub fn set(key: i32, value: Variant) -> bool {
        let mut data: VarDictionary = Self::_get_data();

        data.set(key, value);

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

    /// Sets a setting value and immediately applies configuration.
    #[func]
    fn set_and_apply(key: i32, value: Variant) -> bool {
        let success: bool = CoreSettings::set(key, value);
        CoreSettings::apply_config();
        success
    }

    /// Prepends a value to an array-type setting.
    #[func]
    fn prepend(key: i32, value: Variant) -> bool {
        let mut settings_arr: Array<Variant> = CoreSettings::get(key)
            .try_to::<Array<Variant>>()
            .unwrap_or_else(|_| Array::new());

        settings_arr.insert(0, &value); 

        CoreSettings::set(key, settings_arr.to_variant());
        true
    }

    /// Appends a value to an array-type setting.
    #[func]
    fn append(key: i32, value: Variant) -> bool {
        let mut settings_arr: Array<Variant> = CoreSettings::get(key)
            .try_to::<Array<Variant>>()
            .unwrap_or_else(|_| Array::new());

        settings_arr.push(&value);
        CoreSettings::set(key, settings_arr.to_variant());
        true
    }

    pub fn _get_data() -> VarDictionary {
        let mut data: VarDictionary = VarDictionary::new();

        if CoreSettings::exists() {
            let file: Gd<FileAccess> = FileAccess::open(&CoreSettings::get_path(), ModeFlags::READ).unwrap();
            data = file.get_var().try_to().unwrap();
        }

        data
    }
}
