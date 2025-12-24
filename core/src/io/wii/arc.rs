use std::sync::Arc;
use godot::{classes::ProjectSettings, prelude::*};
use crate::io::{
    buffer::NebulaBuffer, 
    bytesource::{ByteSource, DiskFileSource, SubrangeSource}, 
    dir::NebulaDir, 
    file::NebulaFile, 
    fs::NebulaFs
};

const U8_HEADER: [u8; 4] = [0x55, 0xAA, 0x38, 0x2D];

#[derive(Clone)]
struct ArcEntry {
    path: String,
    offset: u64,
    size: u64,
    is_dir: bool,
}

pub struct ArcFs {
    source: Arc<dyn ByteSource>,
    entries: Vec<ArcEntry>,
}

impl ArcFs {
    pub fn new(source: Arc<dyn ByteSource>) -> Result<Self, String> {
        let entries = parse_arc_index(&source)?;
        Ok(Self { source, entries })
    }
}

fn read_u16_be(data: &[u8], offset: usize) -> u16 {
    if offset + 2 > data.len() {
        return 0;
    }

    u16::from_be_bytes([data[offset], data[offset + 1]])
}

fn read_u32_be(data: &[u8], offset: usize) -> u32 {
    if offset + 4 > data.len() {
        return 0;
    }
    u32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn parse_arc_index(source: &Arc<dyn ByteSource>) -> Result<Vec<ArcEntry>, String> {
    let file_size = source.len();
    if file_size < 0x20 {
        return Err("Invalid or empty ARC file".to_string());
    }

    let raw_data = source
        .read_range(0, file_size as usize)
        .map_err(|e| format!("Failed to read ARC file: {}", e))?;

    let mut offset = 0;
    let mut found = false;
    while offset <= raw_data.len().saturating_sub(4) {
        if &raw_data[offset..offset + 4] == &U8_HEADER {
            found = true;
            break;
        }
        offset += 1;
    }

    if !found || offset + 16 > raw_data.len() {
        return Err("U8 header incomplete or corrupted".to_string());
    }

    let rootnode_offset = read_u32_be(&raw_data, offset + 4) as usize;
    let data_offset = read_u32_be(&raw_data, offset + 12) as usize;
    let node_base = offset + rootnode_offset;

    if node_base + 12 > raw_data.len() {
        return Err("Invalid node base offset".to_string());
    }

    let root_node_size = read_u32_be(&raw_data, node_base + 8) as usize;

    #[derive(Debug)]
    struct Node {
        node_type: u16,
        name_offset: u16,
        data_offset: u32,
        size: u32,
    }

    let mut nodes = Vec::new();
    let mut node_offset = node_base + 12;

    for _ in 0..(root_node_size - 1) {
        if node_offset + 12 > raw_data.len() {
            break;
        }

        let node = Node {
            node_type: read_u16_be(&raw_data, node_offset),
            name_offset: read_u16_be(&raw_data, node_offset + 2),
            data_offset: read_u32_be(&raw_data, node_offset + 4),
            size: read_u32_be(&raw_data, node_offset + 8),
        };

        nodes.push(node);
        node_offset += 12;
    }

    let string_table_offset = node_offset;
    let string_table_size = data_offset.saturating_sub(string_table_offset - offset);
    let string_table = &raw_data[string_table_offset..string_table_offset + string_table_size];

    let mut entries = Vec::new();
    let mut path_stack: Vec<String> = Vec::new();
    let mut count_stack: Vec<usize> = vec![root_node_size];
    let mut current_index = 0;

    entries.push(ArcEntry {
        path: String::new(),
        offset: 0,
        size: 0,
        is_dir: true,
    });

    for node in nodes {
        current_index += 1;

        let mut name = String::new();
        let mut name_pos = node.name_offset as usize;
        while name_pos < string_table.len() {
            let byte = string_table[name_pos];
            if byte == 0 {
                break;
            }
            name.push(byte as char);
            name_pos += 1;
        }

        let full_path = if path_stack.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", path_stack.join("/"), name)
        };

        if node.node_type == 0x0100 {
            entries.push(ArcEntry {
                path: full_path.clone(),
                offset: 0,
                size: 0,
                is_dir: true,
            });
            path_stack.push(name);
            count_stack.push(node.size as usize);
        } else if node.node_type == 0x0000 {
            entries.push(ArcEntry {
                path: full_path,
                offset: (offset + node.data_offset as usize) as u64,
                size: node.size as u64,
                is_dir: false,
            });
        }

        while !count_stack.is_empty() && current_index + 1 == *count_stack.last().unwrap() {
            count_stack.pop();
            if !path_stack.is_empty() {
                path_stack.pop();
            }
        }
    }

    Ok(entries)
}

impl NebulaFs for ArcFs {
    fn get_entries(&self, path: &str) -> PackedStringArray {
        let mut out = PackedStringArray::new();
        let prefix = if path.is_empty() { String::new() } else { format!("{}/", path) };

        for entry in &self.entries {
            if entry.path.starts_with(&prefix) && entry.path != path {
                let rest = &entry.path[prefix.len()..];
                if !rest.is_empty() && !rest.contains('/') {
                    let name = if entry.is_dir { format!("{}/", rest) } else { rest.to_string() };
                    out.push(&name);
                }
            }
        }

        out
    }

    // fn get_files(&self) -> PackedStringArray {
    //     let entries = self.get_entries("");
    //     let mut out = PackedStringArray::new();
    //     for s in entries.to_vec() {
    //         if !s.ends_with("/") {
    //             out.push(&s);
    //         }
    //     }
    //     out
    // }

    // fn get_dirs(&self) -> PackedStringArray {
    //     let entries = self.get_entries("");
    //     let mut out = PackedStringArray::new();
    //     for s in entries.to_vec() {
    //         if s.ends_with("/") {
    //             out.push(&s);
    //         }
    //     }
    //     out
    // }

    fn file_exists(&self, path: &str) -> bool {
        self.entries.iter().any(|e| !e.is_dir && e.path == path)
    }

    fn dir_exists(&self, path: &str) -> bool {
        self.entries.iter().any(|e| e.is_dir && e.path == path)
    }

    fn get_file(&self, path: &str) -> Gd<NebulaFile> {
        let entry = self.entries.iter().find(|e| !e.is_dir && e.path == path);
        match entry {
            Some(entry) => {
                let subrange = Arc::new(SubrangeSource::new(self.source.clone(), entry.offset, entry.size));
                let mut buffer = NebulaBuffer::new_gd();
                buffer.bind_mut().set_source(subrange);
                NebulaFile::from_buffer(buffer)
            }
            None => NebulaFile::from_buffer(NebulaBuffer::new_gd()),
        }
    }

    fn get_dir(&self, path: &str) -> Gd<NebulaDir> {
        if !self.dir_exists(path) {
            return NebulaDir::new_gd();
        }
        NebulaDir::new(Arc::new(self.clone()), path.to_string())
    }
    
    fn get_file_size(&self, path: &str) -> u64 {
        self.entries
            .iter()
            .find(|e| !e.is_dir && e.path == path)
            .map(|e| e.size)
            .unwrap_or(0)
    }
}


impl Clone for ArcFs {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            entries: self.entries.clone(),
        }
    }
}


#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct ARC {
    #[base]
    base: Base<RefCounted>,
    fs: Option<Arc<dyn NebulaFs>>,
}

#[godot_api]
impl IRefCounted for ARC {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base, fs: None }
    }
}

#[godot_api]
impl ARC {
    #[func]
    pub fn open(path: GString) -> Option<Gd<ARC>> {
        let path = ProjectSettings::singleton().globalize_path(&path).to_string();
        let disk = match DiskFileSource::new(&path) {
            Ok(f) => f,
            Err(err) => {
                godot_error!("ARC.open: failed to open '{}': {}", path, err);
                return None;
            }
        };
        let source: Arc<dyn ByteSource> = Arc::new(disk);
        let fs = match ArcFs::new(source) {
            Ok(fs) => fs,
            Err(err) => {
                godot_error!("ARC.open: invalid ARC '{}': {}", path, err);
                return None;
            }
        };

        let mut arc_instance = ARC::new_gd();
        arc_instance.bind_mut().fs = Some(Arc::new(fs));
        Some(arc_instance)
    }

    #[func]
    pub fn to_dir(&self) -> Gd<NebulaDir> {
        match &self.fs {
            Some(fs) => NebulaDir::new(fs.clone(), String::new()),
            None => NebulaDir::new_gd(),
        }
    }

    #[func]
    pub fn from_buffer(buffer: Gd<NebulaBuffer>) -> Gd<NebulaDir> {
        let src = {
            let buf = buffer.bind();
            match &buf.source {
                Some(src) => src.clone(),
                None => {
                    godot_error!("ARC.from_buffer: buffer has no source");
                    return NebulaDir::new_gd();
                }
            }
        };

        let fs = match ArcFs::new(src) {
            Ok(fs) => Arc::new(fs) as Arc<dyn NebulaFs>,
            Err(err) => {
                godot_error!("ARC.from_buffer: invalid ARC buffer: {}", err);
                return NebulaDir::new_gd();
            }
        };

        NebulaDir::new(fs, String::new())
    }

}