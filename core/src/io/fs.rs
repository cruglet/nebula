use godot::global::godot_warn;
use godot::obj::{Gd, NewGd};
use godot::builtin::PackedStringArray;

use crate::io::file::NebulaFile;
use crate::io::dir::NebulaDir;

pub trait NebulaFs: Send + Sync {
    fn get_entries(&self, path: &str) -> PackedStringArray;
    fn file_exists(&self, path: &str) -> bool;
    fn dir_exists(&self, path: &str) -> bool;
    fn get_file(&self, path: &str) -> Gd<NebulaFile>;
    fn get_dir(&self, path: &str) -> Gd<NebulaDir>;
    fn get_file_size(&self, path: &str) -> u64;

    fn create_dir(&self, _path: &str) -> bool {
        godot_warn!("This filesystem is read-only!");
        false
    }

    fn create_file(&self, _path: &str) -> Gd<NebulaFile> {
        godot_warn!("This filesystem is read-only!");
        NebulaFile::new_gd()
    }

    fn remove_file(&self, _path: &str) -> bool {
        godot_warn!("This filesystem is read-only!");
        false
    }

    fn remove_dir(&self, _path: &str) -> bool {
        godot_warn!("This filesystem is read-only!");
        false
    }

    fn rename_path(&self, _from: &str, _to: &str) -> bool {
        godot_warn!("This filesystem is read-only!");
        false
    }
}
