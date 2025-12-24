use godot::prelude::*;
use std::sync::Arc;
use crate::io::{file::NebulaFile, fs::NebulaFs};

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct NebulaDir {
    fs: Option<Arc<dyn NebulaFs>>,
    path: String,
    #[base] base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for NebulaDir {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            fs: None,
            path: String::new(),
            base
        }
    }
}

#[godot_api]
impl NebulaDir {
     #[func]
     /// Returns a [PackedByteArray] of entries; directoreis are denoted by a "`/`" at the end.
    pub fn get_entries(&self) -> PackedStringArray {
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
        match &self.fs {
            Some(fs) => {
                let entries = fs.get_entries(&self.path);
                let mut out = PackedStringArray::new();
                for s in entries.to_vec() {
                    if !s.ends_with("/") {
                        out.push(&s);
                    }
                }
                out
            }
            None => PackedStringArray::new(),
        }
    }

    #[func]
    pub fn file_exists(&self, rel: String) -> bool {
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
        match &self.fs {
            Some(fs) => {
                let entries = fs.get_entries(&self.path);
                let mut out = PackedStringArray::new();
                for s in entries.to_vec() {
                    if s.ends_with("/") {
                        out.push(&s);
                    }
                }
                out
            }
            None => PackedStringArray::new(),
        }
    }

    #[func]
    pub fn dir_exists(&self, rel: String) -> bool {
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
        let Some(fs) = &self.fs else {
            godot_warn!("NebulaDir used before initialization");
            return;
        };

        self.print_files_recursive(fs.as_ref(), &self.path, filesize, indent, true);
    }

    #[func]
    /// Estimates the memory footprint of this NebulaDir instance in bytes.
    pub fn get_footprint(&self) -> i64 {
        use std::mem::size_of_val;

        let mut size = 0;

        size += size_of_val(self) as u64;

        size += self.path.capacity() as u64;

        if let Some(fs) = &self.fs {
            size += size_of_val(fs) as u64;
        }

        size as i64
    }
}

impl NebulaDir {
    pub(crate) fn new(fs: Arc<dyn NebulaFs>, path: String) -> Gd<Self> {
        let dir = Gd::from_init_fn(|base| Self {
            fs: Some(fs),
            path,
            base,
        });
        dir
    }

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
                    let size_str = Self::humanize_size(size);
                    godot_print!("{}[f] {} <{}>", prefix, name, size_str);
                } else {
                    godot_print!("{}[f] {}", prefix, name);
                }
            }
        }

        if is_root && filesize {
            let total_str = Self::humanize_size(total_size);
            godot_print!("VIRTUAL SIZE: {}", total_str);
        }

        total_size
    }

    fn humanize_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
        
        if bytes == 0 {
            return "0 B".to_string();
        }

        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[0])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
}