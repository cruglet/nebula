use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use godot::{classes::ProjectSettings, prelude::*};
use aes::Aes128;
use cbc::{Decryptor, cipher::{BlockDecryptMut, KeyIvInit}};
use crate::io::{
    buffer::NebulaBuffer,
    bytesource::{ByteSource, DiskFileSource, SubrangeSource, MemoryByteSource},
    dir::NebulaDir,
    file::NebulaFile,
    fs::NebulaFs
};
use crate::runtime::utils::singleton::Singleton;

const WBFS_MAGIC: [u8; 4] = [0x57, 0x42, 0x46, 0x53];
const WII_SECTOR_COUNT: u32 = 0x46090;
const WII_SEC_SZ_S: u32 = 15;
const CLUSTER_SIZE: usize = 0x8000;
const SHA1_BLOCK_SIZE: usize = 0x400;
const DATA_BLOCK_SIZE: usize = CLUSTER_SIZE - SHA1_BLOCK_SIZE;

#[derive(Clone)]
struct FsEntry {
    offset: u64,
    size: u64,
}

pub struct WbfsFs {
    source: Arc<dyn ByteSource>,
    wlba_map: std::collections::HashMap<u16, u16>,
    sector_size: u32,
    partition_offset: u64,
    partition_data_offset: u64,
    partition_data_size: u64,
    decryption_key: Vec<u8>,
    filesystem: std::collections::HashMap<String, FsEntry>,
    game_name: String,
    game_id: String,
    cluster_cache: RwLock<HashMap<usize, Vec<u8>>>,
}

impl WbfsFs {
    pub fn new(source: Arc<dyn ByteSource>) -> Result<Self, String> {
        let file_size = source.len();
        if file_size < 0x200 {
            return Err("File too small to be WBFS".to_string());
        }

        let header = source.read_range(0, 0x200)
            .map_err(|e| format!("Failed to read WBFS header: {}", e))?;

        if &header[0..4] != WBFS_MAGIC {
            return Err("Invalid WBFS magic".to_string());
        }

        let hd_sector_size = 1u32 << header[8];
        let wbfs_sector_size = 1u32 << header[9];
        
        let wbfs_sec_sz_s = header[9] as u32;
        
        if wbfs_sec_sz_s < WII_SEC_SZ_S {
            return Err(format!("WBFS sector size too small: 2^{}", wbfs_sec_sz_s));
        }
        
        let wii_sec_per_wbfs_sect = 1u32 << (wbfs_sec_sz_s - WII_SEC_SZ_S);
        let blocks_per_disc = ((WII_SECTOR_COUNT + wii_sec_per_wbfs_sect - 1) 
                              / wii_sec_per_wbfs_sect) as usize;

        let disc_header = source.read_range(hd_sector_size as u64, 0x100)
            .map_err(|e| format!("Failed to read disc header: {}", e))?;

        let game_id = String::from_utf8_lossy(&disc_header[0..6]).to_string();
        let game_name = String::from_utf8_lossy(&disc_header[0x20..0x60])
            .trim_end_matches('\0')
            .to_string();

        let wlba_offset = hd_sector_size as u64 + 0x100;
        let wlba_count = blocks_per_disc;
        let wlba_size = wlba_count * 2;
        let wlba_data = source.read_range(wlba_offset, wlba_size)
            .map_err(|e| format!("Failed to read WLBA table: {}", e))?;

        let mut wlba_map = std::collections::HashMap::new();
        for i in 0..wlba_count {
            let wlba = u16::from_be_bytes([wlba_data[i * 2], wlba_data[i * 2 + 1]]);
            if wlba > 0 {
                wlba_map.insert(i as u16, wlba);
            }
        }

        let partition_table_data = Self::get_iso_data(&source, &wlba_map, wbfs_sector_size, 0x40000, 8)?;
        let partition_table_offset = (u32::from_be_bytes([
            partition_table_data[4], partition_table_data[5],
            partition_table_data[6], partition_table_data[7]
        ]) << 2) as u64;

        let partition_entry = Self::get_iso_data(&source, &wlba_map, wbfs_sector_size, partition_table_offset, 8)?;
        let partition_offset = (u32::from_be_bytes([
            partition_entry[0], partition_entry[1],
            partition_entry[2], partition_entry[3]
        ]) << 2) as u64;

        let ticket_data = Self::get_iso_data(&source, &wlba_map, wbfs_sector_size, partition_offset, 0x2A4)?;
        let encrypted_title_key = &ticket_data[0x1BF..0x1CF];
        let mut title_key_iv = ticket_data[0x1DC..0x1E4].to_vec();
        title_key_iv.extend_from_slice(&[0u8; 8]);

        let common_key_hex = Singleton::get_key("WII_COMMON".to_godot()).get_string_from_ascii().to_string();
        let common_key = hex::decode(&common_key_hex)
            .map_err(|e| format!("Failed to decode common key from hex!: {}", e))?;

        if common_key.len() != 16 {
            return Err(format!("Common key has invalid length: {} (expected 16)", common_key.len()));
        }

        if encrypted_title_key.len() != 16 {
            return Err(format!("Encrypted title key has invalid length: {} (expected 16)", encrypted_title_key.len()));
        }

        if title_key_iv.len() != 16 {
            return Err(format!("Title key IV has invalid length: {} (expected 16)", title_key_iv.len()));
        }

        let decryption_key = aes_cbc_decrypt(encrypted_title_key, &common_key, &title_key_iv)?;

        let partition_info = Self::get_iso_data(&source, &wlba_map, wbfs_sector_size, partition_offset + 0x2B8, 8)?;
        let partition_data_offset = (u32::from_be_bytes([
            partition_info[0], partition_info[1],
            partition_info[2], partition_info[3]
        ]) << 2) as u64;
        let partition_data_size = (u32::from_be_bytes([
            partition_info[4], partition_info[5],
            partition_info[6], partition_info[7]
        ]) << 2) as u64;

        let mut wbfs = Self {
            source,
            wlba_map,
            sector_size: wbfs_sector_size,
            partition_offset,
            partition_data_offset,
            partition_data_size,
            decryption_key,
            filesystem: std::collections::HashMap::new(),
            game_name,
            game_id,
            cluster_cache: RwLock::new(HashMap::new()),
        };

        let fs_info = wbfs.get_decrypted_data(0x424, 12)?;
        let filesystem_offset = (u32::from_be_bytes([fs_info[0], fs_info[1], fs_info[2], fs_info[3]]) << 2) as u64;
        
        wbfs.filesystem = wbfs.parse_filesystem(filesystem_offset)?;

        Ok(wbfs)
    }

    fn get_iso_data(
        source: &Arc<dyn ByteSource>,
        wlba_map: &std::collections::HashMap<u16, u16>,
        sector_size: u32,
        address: u64,
        size: usize
    ) -> Result<Vec<u8>, String> {
        let sector_index = address / sector_size as u64;
        let sector_offset = address % sector_size as u64;
        let iso_block_index = sector_index as u16;

        let wbfs_block = wlba_map.get(&iso_block_index)
            .ok_or_else(|| format!("ISO block {} not found in WLBA map", iso_block_index))?;

        let wbfs_address = (*wbfs_block as u64)
            .checked_mul(sector_size as u64)
            .and_then(|base| base.checked_add(sector_offset))
            .ok_or_else(|| format!("Address calculation overflow for block {}", wbfs_block))?;

        source.read_range(wbfs_address, size)
            .map_err(|e| format!("Failed to read ISO data: {}", e))
    }

    fn get_decrypted_data(&self, offset: u64, size: usize) -> Result<Vec<u8>, String> {
        let mut result = Vec::with_capacity(size);
        let mut current_cluster = (offset / DATA_BLOCK_SIZE as u64) as usize;
        let mut cluster_data_offset = (offset % DATA_BLOCK_SIZE as u64) as usize;
        let mut bytes_remaining = size;

        let mut iteration = 0;
        while bytes_remaining > 0 {
            iteration += 1;
            if iteration > 1000 {
                return Err(format!("Too many iterations in get_decrypted_data: iteration={}, bytes_remaining={}", iteration, bytes_remaining));
            }
            
            let cluster_data = self.decrypt_cluster(current_cluster)?;
            let bytes_to_take = bytes_remaining.min(cluster_data.len() - cluster_data_offset);
            result.extend_from_slice(&cluster_data[cluster_data_offset..cluster_data_offset + bytes_to_take]);
            
            cluster_data_offset = 0;
            bytes_remaining -= bytes_to_take;
            current_cluster += 1;
        }

        Ok(result)
    }

    fn decrypt_cluster(&self, cluster_index: usize) -> Result<Vec<u8>, String> {
        if let Ok(cache) = self.cluster_cache.read() {
            if let Some(cached) = cache.get(&cluster_index) {
                return Ok(cached.clone());
            }
        }
        
        let cluster_start = cluster_index as u64 * CLUSTER_SIZE as u64;
        let iv_offset = self.partition_offset + self.partition_data_offset + cluster_start + 0x3D0;
        let data_offset = self.partition_offset + self.partition_data_offset + cluster_start + SHA1_BLOCK_SIZE as u64;

        let iv = Self::get_iso_data(&self.source, &self.wlba_map, self.sector_size, iv_offset, 16)?;
        let encrypted = Self::get_iso_data(&self.source, &self.wlba_map, self.sector_size, data_offset, DATA_BLOCK_SIZE)?;

        let decrypted = aes_cbc_decrypt(&encrypted, &self.decryption_key, &iv)?;
        
        if let Ok(mut cache) = self.cluster_cache.write() {
            cache.insert(cluster_index, decrypted.clone());
        }
        
        Ok(decrypted)
    }

    fn parse_filesystem(&self, fs_offset: u64) -> Result<std::collections::HashMap<String, FsEntry>, String> {
        let header = self.get_decrypted_data(fs_offset, 12)?;
        let total_entries = u32::from_be_bytes([header[8], header[9], header[10], header[11]]) as usize;

        if total_entries == 0 || total_entries > 100000 {
            return Err(format!("Invalid total_entries: {}", total_entries));
        }

        let table = self.get_decrypted_data(fs_offset, total_entries * 12)?;
        let string_table_offset = fs_offset + (total_entries * 12) as u64;

        let mut filesystem = std::collections::HashMap::new();
        let mut dir_stack = vec![("".to_string(), total_entries)];

        for i in 1..total_entries {
            if i % 100 == 0 {
            }
            
            while i >= dir_stack.last().unwrap().1 {
                dir_stack.pop();
            }

            let entry_offset = i * 12;
            let type_name = u32::from_be_bytes([
                table[entry_offset], table[entry_offset + 1],
                table[entry_offset + 2], table[entry_offset + 3]
            ]);
            let entry_type = (type_name >> 24) as u8;
            let name_offset = type_name & 0x00FFFFFF;

            let name = self.read_null_string(string_table_offset + name_offset as u64)?;

            if entry_type == 1 {
                let next_sibling = u32::from_be_bytes([
                    table[entry_offset + 8], table[entry_offset + 9],
                    table[entry_offset + 10], table[entry_offset + 11]
                ]) as usize;
                let dir_path = format!("{}{}/", dir_stack.last().unwrap().0, name);
                dir_stack.push((dir_path, next_sibling));
            } else {
                let file_offset = (u32::from_be_bytes([
                    table[entry_offset + 4], table[entry_offset + 5],
                    table[entry_offset + 6], table[entry_offset + 7]
                ]) * 4) as u64;
                let file_size = u32::from_be_bytes([
                    table[entry_offset + 8], table[entry_offset + 9],
                    table[entry_offset + 10], table[entry_offset + 11]
                ]) as u64;
                let file_path = format!("{}{}", dir_stack.last().unwrap().0, name);

                filesystem.insert(file_path, FsEntry { offset: file_offset, size: file_size });
            }
        }

        Ok(filesystem)
    }

    fn read_null_string(&self, offset: u64) -> Result<String, String> {
        const CHUNK_SIZE: usize = 256;
        let mut result = String::new();
        let mut current = offset;
        
        loop {
            let chunk = self.get_decrypted_data(current, CHUNK_SIZE)?;
            
            for (_, &byte) in chunk.iter().enumerate() {
                if byte == 0 {
                    return Ok(result);
                }
                if byte >= 32 && byte < 127 || byte >= 160 {
                    result.push(byte as char);
                } else if byte != 0 {
                    return Err(format!("Invalid character in string at offset {:x}: byte={}", offset, byte));
                }
                
                if result.len() > 255 {
                    return Err(format!("String too long at offset {:x}", offset));
                }
            }
            
            current += CHUNK_SIZE as u64;
            
            if current - offset > 1024 {
                return Err(format!("String exceeds 1KB at offset {:x}", offset));
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.game_name
    }

    pub fn get_id(&self) -> &str {
        &self.game_id
    }
}

fn aes_cbc_decrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    type Aes128CbcDec = Decryptor<Aes128>;
    
    let cipher = Aes128CbcDec::new_from_slices(key, iv)
        .map_err(|e| format!("Invalid key/IV length: {:?}", e))?;
    
    let mut buffer = data.to_vec();
    cipher.decrypt_padded_mut::<cbc::cipher::block_padding::NoPadding>(&mut buffer)
        .map_err(|e| format!("Decryption failed: {:?}", e))?;
    
    Ok(buffer)
}

impl NebulaFs for WbfsFs {
    fn get_entries(&self, path: &str) -> PackedStringArray {
        let mut out = PackedStringArray::new();
        let prefix = if path.is_empty() { 
            String::new() 
        } else { 
            format!("{}/", path.trim_end_matches('/'))
        };

        let mut seen = std::collections::HashSet::new();
        for entry_path in self.filesystem.keys() {
            if let Some(rest) = entry_path.strip_prefix(&prefix) {
                if rest.is_empty() {
                    continue;
                }
                let first_component = rest.split('/').next().unwrap();
                if !first_component.is_empty() && seen.insert(first_component.to_string()) {
                    let name = if rest.contains('/') {
                        format!("{}/", first_component)
                    } else {
                        first_component.to_string()
                    };
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
        self.filesystem.contains_key(path)
    }

    fn dir_exists(&self, path: &str) -> bool {
        let normalized = path.trim_end_matches('/');
        if normalized.is_empty() {
            return true;
        }
        let prefix = format!("{}/", normalized);
        self.filesystem.keys().any(|p| p.starts_with(&prefix))
    }

    fn get_file(&self, path: &str) -> Gd<NebulaFile> {
        match self.filesystem.get(path) {
            Some(entry) => {
                let data = self.get_decrypted_data(entry.offset, entry.size as usize)
                    .unwrap_or_default();
                let memory_source = MemoryByteSource::with_capacity(data.len());
                let _ = memory_source.write_range(0, &data);
                let subrange = Arc::new(SubrangeSource::new(
                    Arc::new(memory_source),
                    0,
                    entry.size
                ));
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
        self.filesystem.get(path)
            .map(|entry| entry.size)
            .unwrap_or(0)
    }
}

impl Clone for WbfsFs {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            wlba_map: self.wlba_map.clone(),
            sector_size: self.sector_size,
            partition_offset: self.partition_offset,
            partition_data_offset: self.partition_data_offset,
            partition_data_size: self.partition_data_size,
            decryption_key: self.decryption_key.clone(),
            filesystem: self.filesystem.clone(),
            game_name: self.game_name.clone(),
            game_id: self.game_id.clone(),
            cluster_cache: RwLock::new(HashMap::new()),
        }
    }
}


#[derive(GodotClass)]
/// Class used to instantiate and work with files using the Wii Backup Filesystem format (wbfs).
#[class(base=RefCounted)]
pub struct WBFS {
    #[base]
    base: Base<RefCounted>,
    fs: Option<Arc<WbfsFs>>,
}

#[godot_api]
impl IRefCounted for WBFS {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base, fs: None }
    }
}

#[godot_api]
impl WBFS {
    #[func]
    /// Opens a WBFS file from the given path and returns a `WBFS` instance.
    /// Logs an error and returns `null` if the file cannot be opened or is invalid.
    pub fn open(path: GString) -> Option<Gd<WBFS>> {
        let path = ProjectSettings::singleton().globalize_path(&path).to_string();
        let disk = match DiskFileSource::new(&path) {
            Ok(f) => f,
            Err(err) => {
                godot_error!("WBFS.open: failed to open '{}': {}", path, err);
                return None;
            }
        };

        let source: Arc<dyn ByteSource> = Arc::new(disk);
        let fs = match WbfsFs::new(source) {
            Ok(fs) => fs,
            Err(err) => {
                godot_error!("WBFS.open: invalid WBFS '{}': {}", path, err);
                return None;
            }
        };

        let mut wbfs_instance = WBFS::new_gd();
        wbfs_instance.bind_mut().fs = Some(Arc::new(fs));
        Some(wbfs_instance)
    }

    #[func]
    /// Returns `true` if this WBFS instance contains a valid disc and filesystem data.
    /// Returns `false` if the WBFS instance was not successfully loaded or is empty.
    pub fn is_valid(&self) -> bool {
        self.fs.is_some()
    }

    #[func]
    /// Returns the root directory of the WBFS file as a [NebulaDir].
    pub fn to_dir(&self) -> Gd<NebulaDir> {
        match &self.fs {
            Some(fs) => NebulaDir::new(fs.clone(), String::new()),
            None => NebulaDir::new_gd(),
        }
    }

    #[func]
    /// Returns the full name of the game contained in this WBFS file.
    pub fn get_name(&self) -> GString {
        match &self.fs {
            Some(fs) => fs.get_name().to_godot(),
            None => GString::new(),
        }
    }

    #[func]
    /// Returns the disc ID of the game (e.g., `RM8E01`).
    pub fn get_id(&self) -> GString {
        match &self.fs {
            Some(fs) => fs.get_id().to_godot(),
            None => GString::new(),
        }
    }

    #[func]
    /// Returns the universal ID of the disc, where the region character is replaced with `x`.
    pub fn get_universal_id(&self) -> GString {
        let Some(fs) = &self.fs else {
            return GString::new();
        };

        let mut id = fs.get_id().to_string();
        if id.len() >= 4 {
            id.replace_range(3..4, "x");
        }

        id.to_godot()
    }

    #[func]
    /// Returns the region code of the disc as a single character string.
    ///
    /// The region codes are as follows:
    /// - `D` => German
    /// - `E` => USA
    /// - `F` => France
    /// - `I` => Italy
    /// - `J` => Japan
    /// - `K` => Korea
    /// - `P` => PAL
    /// - `R` => Russia
    /// - `S` => Spanish
    /// - `T` => Taiwan
    /// - `U` => Australia
    /// - `X` => Unknown or invalid
    ///
    /// Returns `X` if the WBFS instance is invalid or the disc ID is too short.
    pub fn get_region_code(&self) -> GString {
        let Some(fs) = &self.fs else {
            return "X".to_godot();
        };

        let id = fs.get_id();
        if id.len() < 4 {
            return "X".to_godot();
        }

        let region_char = id.chars().nth(3).unwrap_or('X');
        region_char.to_string().to_godot()
    }


    #[func]
    /// Returns the full name of the disc region (e.g., "USA").
    /// Returns "Unknown" if the region is invalid.
    pub fn get_region_string(&self) -> GString {
        match self.get_region_code().to_string().as_str() {
            "D" => "German",
            "E" => "USA",
            "F" => "France",
            "I" => "Italy",
            "J" => "Japan",
            "K" => "Korea",
            "P" => "PAL",
            "R" => "Russia",
            "S" => "Spanish",
            "T" => "Taiwan",
            "U" => "Australia",
            _ => "Unknown",
        }
        .to_godot()
    }

    #[func]
    /// Returns the disc number (for multi-disc games) as an integer.
    /// Returns 0 if invalid or unavailable.
    pub fn get_disc_number(&self) -> i32 {
        let Some(fs) = &self.fs else {
            return 0;
        };

        let id = fs.get_id();
        if id.len() < 6 {
            return 0;
        }

        match id.chars().nth(5) {
            Some(c) if c.is_ascii_digit() => c.to_digit(10).unwrap_or(0) as i32,
            _ => 0,
        }
    }

    #[func]
    /// Returns the total size of all files in the WBFS disc in bytes.
    pub fn get_used_size(&self) -> i64 {
        let Some(fs) = &self.fs else {
            return 0;
        };

        fs.filesystem.values()
            .map(|entry| entry.size as i64)
            .sum()
    }

    #[func]
    /// Returns the 2-character publisher code from the disc ID (e.g., "RM").
    /// Returns an empty string if the ID is too short.
    pub fn get_publisher_code(&self) -> GString {
        let Some(fs) = &self.fs else {
            return GString::new();
        };

        let id = fs.get_id();
        if id.len() < 2 {
            return GString::new();
        }

        id.chars().take(2).collect::<String>().to_godot()
    }
}
