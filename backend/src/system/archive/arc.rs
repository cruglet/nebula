// Sourced from Reggie! Updated:
// https://github.com/NSMBW-Community/Reggie-Updated/tree/fa12de16ea8df33068ae93ec4616f8e67dbc05ca

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const U8_MAGIC: &[u8] = b"U\xAA8-"; // like what in the actual fuck

#[derive(Debug, Clone)]
struct U8Header {
    tag: [u8; 4],
    rootnode_offset: u32,
    header_size: u32,
    data_offset: u32,
    zeroes: [u8; 16],
}

impl U8Header {
    fn new() -> Self {
        Self {
            tag: [b'U', 0xAA, b'8', b'-'],
            rootnode_offset: 0x20,
            header_size: 0,
            data_offset: 0,
            zeroes: [0; 16],
        }
    }

    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(0x20);
        result.extend_from_slice(&self.tag);
        result.extend_from_slice(&self.rootnode_offset.to_be_bytes());
        result.extend_from_slice(&self.header_size.to_be_bytes());
        result.extend_from_slice(&self.data_offset.to_be_bytes());
        result.extend_from_slice(&self.zeroes);
        result
    }

    fn unpack(data: &[u8]) -> io::Result<Self> {
        if data.len() < 0x20 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "header too short"));
        }

        let mut header = Self::new();
        header.tag.copy_from_slice(&data[0..4]);
        header.rootnode_offset = u32::from_be_bytes(data[4..8].try_into().unwrap());
        header.header_size = u32::from_be_bytes(data[8..12].try_into().unwrap());
        header.data_offset = u32::from_be_bytes(data[12..16].try_into().unwrap());
        header.zeroes.copy_from_slice(&data[16..32]);
        Ok(header)
    }
}

#[derive(Debug, Clone)]
struct U8Node {
    node_type: u16,
    name_offset: u16,
    data_offset: u32,
    size: u32,
}

impl U8Node {
    fn new() -> Self {
        Self {
            node_type: 0,
            name_offset: 0,
            data_offset: 0,
            size: 0,
        }
    }

    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(12);
        result.extend_from_slice(&self.node_type.to_be_bytes());
        result.extend_from_slice(&self.name_offset.to_be_bytes());
        result.extend_from_slice(&self.data_offset.to_be_bytes());
        result.extend_from_slice(&self.size.to_be_bytes());
        result
    }

    fn unpack(data: &[u8]) -> io::Result<Self> {
        if data.len() < 12 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "node too short"));
        }

        let mut node = Self::new();
        node.node_type = u16::from_be_bytes(data[0..2].try_into().unwrap());
        node.name_offset = u16::from_be_bytes(data[2..4].try_into().unwrap());
        node.data_offset = u32::from_be_bytes(data[4..8].try_into().unwrap());
        node.size = u32::from_be_bytes(data[8..12].try_into().unwrap());
        Ok(node)
    }
}

#[derive(Debug)]
pub struct U8Arc {
    pub files: Vec<(String, Option<Vec<u8>>)>,
}

impl U8Arc {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    fn align(x: usize, boundary: usize) -> usize {
        (x + boundary - 1) & !(boundary - 1)
    }

    pub fn dump(&self) -> Vec<u8> {
        let mut header = U8Header::new();
        let mut root_node = U8Node::new();
        root_node.node_type = 0x0100;

        let mut nodes = Vec::new();
        let mut strings = vec![0u8];
        let mut data = Vec::new();

        for (item, value) in &self.files {
            let mut node = U8Node::new();
            
            let recursion = item.matches('/').count();
            let name = item.split('/').last().unwrap();

            node.name_offset = strings.len() as u16;
            strings.extend_from_slice(name.as_bytes());
            strings.push(0);

            match value {
                None => {
                    // directory
                    node.node_type = 0x0100;
                    node.data_offset = recursion as u32;
                    
                    let mut size = nodes.len() + 1;
                    for (other_item, _) in &self.files {
                        if other_item.starts_with(item) {
                            size += 1;
                        }
                    }
                    node.size = size as u32;
                }
                Some(file_data) => {
                    // file
                    node.node_type = 0x0000;
                    node.data_offset = data.len() as u32;
                    data.extend_from_slice(file_data);
                    
                    // align data to 32 bytes
                    let padding = Self::align(data.len(), 32) - data.len();
                    data.extend(std::iter::repeat(0).take(padding));
                    
                    node.size = file_data.len() as u32;
                }
            }
            nodes.push(node);
        }

        header.header_size = ((nodes.len() + 1) * 12 + strings.len()) as u32;
        header.data_offset = Self::align(
            (header.header_size + header.rootnode_offset) as usize,
            64
        ) as u32;
        root_node.size = (nodes.len() + 1) as u32;

        // adjust file offsets
        for node in &mut nodes {
            if node.node_type == 0x0000 {
                node.data_offset += header.data_offset;
            }
        }

        // build final file
        let mut result = Vec::new();
        result.extend(header.pack());
        result.extend(root_node.pack());
        for node in &nodes {
            result.extend(node.pack());
        }
        result.extend_from_slice(&strings);
        
        // add padding before data section
        let padding = header.data_offset as usize - result.len();
        result.extend(std::iter::repeat(0).take(padding));
        result.extend_from_slice(&data);

        result
    }

    pub fn dump_dir<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let dir = dir.as_ref();
        fs::create_dir_all(dir)?;
        
        for (item, data) in &self.files {
            let path = dir.join(item);
            
            if let Some(file_data) = data {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut file = File::create(path)?;
                file.write_all(file_data)?;
            } else {
                fs::create_dir_all(path)?;
            }
        }
        Ok(())
    }

    pub fn load_dir<P: AsRef<Path>>(&mut self, dir: P) -> io::Result<()> {
        self.load_dir_recursive(dir.as_ref(), PathBuf::new())
    }

    fn load_dir_recursive(&mut self, dir: &Path, relative_path: PathBuf) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative = relative_path.join(entry.file_name());
            
            if path.is_dir() {
                self.files.push((relative.to_string_lossy().into_owned(), None));
                self.load_dir_recursive(&path, relative)?;
            } else {
                let mut file = File::open(&path)?;
                let mut data = Vec::new();
                file.read_to_end(&mut data)?;
                self.files.push((relative.to_string_lossy().into_owned(), Some(data)));
            }
        }
        Ok(())
    }

    pub fn load(&mut self, data: &[u8]) -> io::Result<()> {
        let mut offset = 0;
        
        // Find U8 magic
        while offset + 4 <= data.len() {
            if &data[offset..offset + 4] == U8_MAGIC {
                break;
            }
            offset += 1;
        }
        
        let header = U8Header::unpack(&data[offset..])?;
        offset = header.rootnode_offset as usize;

        let root_node = U8Node::unpack(&data[offset..])?;
        offset += 12;

        let mut nodes = Vec::new();
        for _ in 0..root_node.size - 1 {
            let node = U8Node::unpack(&data[offset..])?;
            offset += 12;
            nodes.push(node);
        }

        let strings_end = header.data_offset as usize - header.rootnode_offset as usize - (12 * root_node.size as usize);
        let strings = &data[offset..offset + strings_end];

        let mut recursion = vec![root_node.size]; // im having a stroke
        let mut recursion_dir = Vec::new();
        let mut counter = 0;

        for node in nodes {
            counter += 1;
            let name = strings[node.name_offset as usize..]
                .split(|&b| b == 0)
                .next()
                .unwrap();
            let name = String::from_utf8_lossy(name).into_owned();

            if node.node_type == 0x0100 {
                recursion.push(node.size);
                recursion_dir.push(name);
                let path = recursion_dir.join("/");
                self.files.push((path, None));
                // i wanna kms
            } else if node.node_type == 0 {
                let path = if recursion_dir.is_empty() {
                    name
                } else {
                    format!("{}/{}", recursion_dir.join("/"), name)
                };
                
                let file_data = data[node.data_offset as usize..][..node.size as usize].to_vec();
                self.files.push((path, Some(file_data)));
            }

            let sz = recursion.pop().unwrap();
            if sz != counter + 1 {
                recursion.push(sz);
            } else if !recursion_dir.is_empty() {
                recursion_dir.pop();
            }
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        for (item, value) in &self.files {
            if item == key {
                return value.as_ref();
            }
        }
        None
    }

    pub fn get_dir(&self, key: &str) -> Option<Vec<String>> {
        for (item, value) in &self.files {
            if item == key && value.is_none() {
                let mut ret = Vec::new();
                for (item2, _) in &self.files {
                    if item2.starts_with(key) && item2 != key {
                        if let Some(subpath) = item2.strip_prefix(&format!("{}/", key)) {
                            ret.push(subpath.to_string());
                        }
                    }
                }
                return Some(ret);
            }
        }
        None
    }

    pub fn set(&mut self, key: String, value: Vec<u8>) {
        for item in &mut self.files {
            if item.0 == key {
                item.1 = Some(value);
                return;
            }
        }
        self.files.push((key, Some(value)));
    }
}
