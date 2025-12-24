use godot::obj::Gd;
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
}
