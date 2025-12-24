use godot::prelude::*;
use std::{path::Path, sync::Arc};
use crate::io::{file::NebulaFile, fs::NebulaFs};

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct NebulaDir {
    fs: Option<Arc<dyn NebulaFs>>,
    path: String,
    native_path: Option<String>,
    #[base] base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for NebulaDir {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            fs: None,
            path: String::new(),
            native_path: None,
            base
        }
    }
}

#[godot_api]
impl NebulaDir {
    #[func]
    /// Opens a directory from the regular filesystem at the given path.
    /// Returns a new NebulaDir instance that operates on the native filesystem.
    pub fn open(path: GString) -> Gd<Self> {
        let path_str = path.to_string();
        
        let path_obj = Path::new(&path_str);
        if !path_obj.exists() {
            godot_error!("Path '{}' does not exist", path_str);
            return NebulaDir::new_gd();
        }
        
        if !path_obj.is_dir() {
            godot_error!("Path '{}' is not a directory", path_str);
            return NebulaDir::new_gd();
        }
        
        Gd::from_init_fn(|base| Self {
            fs: None,
            path: String::new(),
            native_path: Some(path_str),
            base,
        })
    }

    #[func]
    /// Returns a [PackedStringArray] of entries; directories are denoted by a "`/`" at the end.
    pub fn get_entries(&self) -> PackedStringArray {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            return self.get_native_entries(native_path);
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => fs.get_entries(&self.path),
            None => {
                godot_warn!("NebulaDir used before initialization");
                PackedStringArray::new()
            }
        }
    }

    #[func]
    pub fn get_file(&self, rel: String) -> Gd<NebulaFile> {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            return NebulaFile::open(full_path.to_string_lossy().to_string().to_godot());
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                fs.get_file(&full)
            }
            None => NebulaFile::new_gd(),
        }
    }

    #[func]
    pub fn get_files(&self) -> PackedStringArray {
        let entries = self.get_entries();
        let mut out = PackedStringArray::new();
        for s in entries.to_vec() {
            if !s.ends_with("/") {
                out.push(&s);
            }
        }
        out
    }

    #[func]
    pub fn file_exists(&self, rel: String) -> bool {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            return full_path.exists() && full_path.is_file();
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                fs.file_exists(&full)
            }
            None => false,
        }
    }

    #[func]
    pub fn get_dir(&self, rel: String) -> Gd<NebulaDir> {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            return NebulaDir::open(full_path.to_string_lossy().to_string().to_godot());
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let mut full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                if full.ends_with('/') {
                    full.pop();
                }
                fs.get_dir(&full)
            }
            None => NebulaDir::new_gd(),
        }
    }

    #[func]
    pub fn get_dirs(&self) -> PackedStringArray {
        let entries = self.get_entries();
        let mut out = PackedStringArray::new();
        for s in entries.to_vec() {
            if s.ends_with("/") {
                out.push(&s.trim_suffix("/"));
            }
        }
        out
    }

    #[func]
    /// Returns the path of this directory.
    /// For virtual filesystems, this is the virtual path.
    /// For native filesystems, this is the native filesystem path.
    pub fn get_path(&self) -> GString {
        if let Some(native_path) = &self.native_path {
            native_path.to_godot()
        } else {
            self.path.to_godot()
        }
    }

    #[func]
    /// Creates a new directory at the given relative path.
    /// Returns true if successful, false otherwise.
    pub fn create_dir(&self, rel: String) -> bool {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            return std::fs::create_dir_all(&full_path).is_ok();
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                fs.create_dir(&full)
            }
            None => {
                godot_warn!("NebulaDir used before initialization");
                false
            }
        }
    }

    #[func]
    /// Creates a new file at the given relative path.
    /// Returns a NebulaFile instance if successful, or an invalid instance otherwise.
    pub fn create_file(&self, rel: String) -> Gd<NebulaFile> {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            let full_path_str = full_path.to_string_lossy().to_string();
            
            // Try to create an empty file
            if std::fs::File::create(&full_path).is_ok() {
                return NebulaFile::open(full_path_str.to_godot());
            } else {
                godot_error!("Failed to create file at '{}'", full_path_str);
                return NebulaFile::new_gd();
            }
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                fs.create_file(&full)
            }
            None => {
                godot_warn!("NebulaDir used before initialization");
                NebulaFile::new_gd()
            }
        }
    }

    #[func]
    /// Removes a file at the given relative path.
    /// Returns true if successful, false otherwise.
    pub fn remove_file(&self, rel: String) -> bool {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            return std::fs::remove_file(&full_path).is_ok();
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                fs.remove_file(&full)
            }
            None => {
                godot_warn!("NebulaDir used before initialization");
                false
            }
        }
    }

    #[func]
    /// Removes a directory at the given relative path.
    /// Returns true if successful, false otherwise.
    pub fn remove_dir(&self, rel: String) -> bool {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            return std::fs::remove_dir(&full_path).is_ok();
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                fs.remove_dir(&full)
            }
            None => {
                godot_warn!("NebulaDir used before initialization");
                false
            }
        }
    }

    #[func]
    /// Renames/moves a file or directory from one path to another.
    /// Both paths should be relative to this directory.
    /// Returns true if successful, false otherwise.
    pub fn rename_path(&self, from: String, to: String) -> bool {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let from_path = Path::new(native_path).join(&from);
            let to_path = Path::new(native_path).join(&to);
            return std::fs::rename(&from_path, &to_path).is_ok();
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let from_full = if self.path.is_empty() {
                    from
                } else {
                    format!("{}/{}", self.path, from)
                };
                let to_full = if self.path.is_empty() {
                    to
                } else {
                    format!("{}/{}", self.path, to)
                };
                fs.rename_path(&from_full, &to_full)
            }
            None => {
                godot_warn!("NebulaDir used before initialization");
                false
            }
        }
    }

    #[func]
    pub fn dir_exists(&self, rel: String) -> bool {
        // If using native filesystem
        if let Some(native_path) = &self.native_path {
            let full_path = Path::new(native_path).join(&rel);
            return full_path.exists() && full_path.is_dir();
        }
        
        // Otherwise use virtual filesystem
        match &self.fs {
            Some(fs) => {
                let mut full = if self.path.is_empty() {
                    rel
                } else {
                    format!("{}/{}", self.path, rel)
                };
                
                if full.ends_with('/') {
                    full.pop();
                }

                fs.dir_exists(&full)
            }
            None => false,
        }
    }

    #[func]
    /// Prints the directory tree structure to the console.
    /// 
    /// If `filesize` is `true`, displays the size of each file and the total size at the end.
    /// The `indent` parameter is used internally for recursive calls to control indentation.
    pub fn print_files(
        &self,
        #[opt(default = false)] filesize: bool,
        #[opt(default = 0)] indent: i32,
    ) {
        if let Some(native_path) = &self.native_path {
            self.print_native_files(native_path, filesize, indent, true);
        } else if let Some(fs) = &self.fs {
            self.print_files_recursive(fs.as_ref(), &self.path, filesize, indent, true);
        } else {
            godot_warn!("NebulaDir used before initialization");
        }
    }

    #[func]
    /// Estimates the memory footprint of this [NebulaDir] instance in bytes.
    pub fn get_footprint(&self) -> i64 {
        use std::mem::size_of_val;

        let mut size = 0;

        size += size_of_val(self) as u64;
        size += self.path.capacity() as u64;

        if let Some(native_path) = &self.native_path {
            size += native_path.capacity() as u64;
        }

        if let Some(fs) = &self.fs {
            size += size_of_val(fs) as u64;
        }

        size as i64
    }
}

impl NebulaDir {
    pub(crate) fn new(fs: Arc<dyn NebulaFs>, path: String) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            fs: Some(fs),
            path,
            native_path: None,
            base,
        })
    }

    /// Helper to get entries from native filesystem
    fn get_native_entries(&self, path: &str) -> PackedStringArray {
        let mut entries = PackedStringArray::new();
        
        let read_dir = match std::fs::read_dir(path) {
            Ok(rd) => rd,
            Err(e) => {
                godot_error!("Failed to read directory '{}': {}", path, e);
                return entries;
            }
        };
        
        for entry in read_dir {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            entries.push(&format!("{}/", file_name));
                        } else {
                            entries.push(&file_name);
                        }
                    }
                }
            }
        }
        
        entries
    }

    /// Helper to get file size from native filesystem
    fn get_native_file_size(&self, path: &str) -> u64 {
        std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(0)
    }

    /// Print files for native filesystem
    fn print_native_files(
        &self,
        current_path: &str,
        filesize: bool,
        indent: i32,
        is_root: bool,
    ) -> u64 {
        let prefix = "\t".repeat(indent as usize);
        let entries = self.get_native_entries(current_path);
        let mut total_size: u64 = 0;

        for i in 0..entries.len() {
            let name = entries.get(i).unwrap().to_string();
            let is_dir = name.ends_with("/");
            let clean_name = name.trim_matches('/').to_string();
            
            let full_path = Path::new(current_path).join(&clean_name);
            let full_path_str = full_path.to_string_lossy().to_string();

            if is_dir {
                godot_print!("{}[F] {}", prefix, clean_name);
                total_size += self.print_native_files(&full_path_str, filesize, indent + 1, false);
            } else {
                let size = self.get_native_file_size(&full_path_str);
                total_size += size;

                if filesize {
                    let size_str = GString::humanize_size(size as i64);
                    godot_print!("{}[f] {} <{}>", prefix, clean_name, size_str);
                } else {
                    godot_print!("{}[f] {}", prefix, clean_name);
                }
            }
        }

        if is_root && filesize {
            let total_str = GString::humanize_size(total_size as i64);
            godot_print!("TOTAL SIZE: {}", total_str);
        }

        total_size
    }

    /// Print files for virtual filesystem
    fn print_files_recursive(
        &self,
        fs: &dyn NebulaFs,
        current_path: &str,
        filesize: bool,
        indent: i32,
        is_root: bool,
    ) -> u64 {
        let prefix = "\t".repeat(indent as usize);
        let entries = fs.get_entries(current_path);
        let mut total_size: u64 = 0;

        for i in 0..entries.len() {
            let name = entries.get(i).unwrap().to_string().trim_matches('/').to_string();
            
            let full_path = if current_path.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", current_path, name)
            };

            let dir = fs.get_dir(&full_path);
            let is_dir = dir.bind().fs.is_some();

            if is_dir {
                godot_print!("{}[F] {}", prefix, name);
                total_size += self.print_files_recursive(fs, &full_path, filesize, indent + 1, false);
            } else {
                let size = fs.get_file_size(&full_path);
                total_size += size;

                if filesize {
                    let size_str = GString::humanize_size(total_size as i64);
                    godot_print!("{}[f] {} <{}>", prefix, name, size_str);
                } else {
                    godot_print!("{}[f] {}", prefix, name);
                }
            }
        }

        if is_root && filesize {
            let total_str = GString::humanize_size(total_size as i64);
            godot_print!("VIRTUAL SIZE: {}", total_str);
        }

        total_size
    }
}