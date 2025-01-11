use std::collections::HashMap;
use std::fs::{self, File};
use std::hash::Hash;
use std::io::{self, Read, Result};
use std::path::Path;

use crate::utils::byte_reader::{self, UnpackedValue};
use crate::wii::arc::U8;

// Sourced From Reggie-Updated
// https://github.com/NSMBW-Community/Reggie-Updated/tree/fa12de16ea8df33068ae93ec4616f8e67dbc05ca

pub fn is_nsmbw_level(filename: &str) -> io::Result<bool> {
    // Check if file exists
    if fs::metadata(filename).is_err() {
        return Ok(false);
    }

    // Read the file
    let mut file = fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Check for LZ-compressed file signature
    if data.starts_with(&[0x11]) {
        return Ok(true);
    }

    // Check for U8 data signature
    if data.starts_with(b"U\xAA8-") {
        // Perform additional sanity checks using `windows` to check for slices
        if !data.windows(b"course\0".len()).any(|window| window == b"course\0") &&
           !data.windows(b"course2.bin\0".len()).any(|window| window == b"course2.bin\0") &&
           !data.windows(b"\0\0\0\x80".len()).any(|window| window == b"\0\0\0\x80") {
            return Ok(false);
        }
        return Ok(true);
    }

    // Fallback for non-matching files
    Ok(false)
}

pub fn dump_level(archive_path: String, to: String) -> Result<()> {
    let mut archive = U8::new();

    // Read the archive file
    let data = match fs::read(&archive_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to read archive '{}': {}", archive_path, e);
            return Err(e);
        }
    };

    // Load the archive
    if let Err(e) = archive.load(&data) {
        eprintln!("Failed to load archive '{}': {}", archive_path, e);
        return Err(e);
    }

    // Dump the directory
    if let Err(e) = archive.dump_dir(&to) {
        eprintln!("Failed to dump archive to '{}': {}", to, e);
        return Err(e);
    }

    // List files in the archive
    println!("Files in archive:");
    for (path, data) in &archive.files {
        match data {
            Some(content) => println!("File: {} (size: {} bytes)", path, content.len()),
            None => println!("Directory: {}", path),
        }
    }

    // Indicate success
    println!(
        "Successfully dumped the level from '{}' to '{}'",
        archive_path, to
    );

    Ok(())
}

pub fn read_level(dump_path: String, write_to: String) {
    let dump_path = Path::new(&dump_path).join("course");
    let level_bin_path = Path::new(&dump_path).join("course2.bin");
    let mut level_info: HashMap<String, UnpackedValue> = HashMap::new();
    let level_blocks = read_level_blocks(level_bin_path.to_str().expect("Could not read path"));
    


    match &level_blocks {
        Ok(blocks) => {
            let _ = &level_info.insert(String::from("tilesets"), UnpackedValue::Vec(read_level_tilesets(&blocks[0])));
            let _ = &level_info.insert(String::from("options"), UnpackedValue::Map(read_level_options(&blocks[1])));
            let _ = &level_info.insert(String::from("entrances"), UnpackedValue::Vec(read_level_entrances(&blocks[6])));
            let _ = &level_info.insert(String::from("sprites"), UnpackedValue::Vec(read_level_sprites(&blocks[7])));
            let _ = &level_info.insert(String::from("zones"), UnpackedValue::Vec(read_level_zones(&blocks[9], &blocks[2])));
            let _ = &level_info.insert(String::from("backgrounds"), UnpackedValue::Vec(read_level_backgrounds(&blocks[4], &blocks[5])));
        }
        _ => {
            println!("COULD NOT READ BLOCKS!")
        }
    }
    
    for x in &level_info {
        println!("{:?}\n", x);
    }

    //  = read_level_metadata()
    // let level_options: HashMap<String, Values> = HashMap::new();
    // let level_entrances: HashMap<String, Values> = HashMap::new();
    // let level_sprites: HashMap<String, Values> = HashMap::new();
    // let level_zones: HashMap<String, Values> = HashMap::new();
    // let level_backgrounds: HashMap<String, Values> = HashMap::new();

    
}

fn read_level_blocks(file_path: &str) -> io::Result<Vec<Vec<u8>>> {
    const BLOCKS: usize = 14;
    const BLOCK_METADATA_SIZE: usize = 8; // 8 bytes per block for offset and size
    let mut blocks = Vec::new();

    // Read the entire file into a byte vector
    let mut file = File::open(file_path)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;

    // Ensure the file has enough bytes for all block metadata
    if file_data.len() < BLOCKS * BLOCK_METADATA_SIZE {
        eprintln!("File too small to contain metadata for all blocks!");
        return Ok(blocks);
    }

    for i in 0..BLOCKS {
        let meta_offset = i * BLOCK_METADATA_SIZE;

        // Extract block offset and size
        let block_offset = u32::from_be_bytes([
            file_data[meta_offset],
            file_data[meta_offset + 1],
            file_data[meta_offset + 2],
            file_data[meta_offset + 3],
        ]) as usize;

        let block_size = u32::from_be_bytes([
            file_data[meta_offset + 4],
            file_data[meta_offset + 5],
            file_data[meta_offset + 6],
            file_data[meta_offset + 7],
        ]) as usize;

        // Check block size and bounds
        if block_size == 0 {
            blocks.push(Vec::new());
        } else if block_offset + block_size <= file_data.len() {
            blocks.push(file_data[block_offset..block_offset + block_size].to_vec());
        } else {
            eprintln!(
                "Invalid block metadata for block {}: Offset={}, Size={}",
                i, block_offset, block_size
            );
            blocks.push(Vec::new());
        }
    }

    Ok(blocks)
}

fn read_level_tilesets(block: &[u8]) -> Vec<UnpackedValue> {
    let mut level_metadata: Vec<UnpackedValue> = vec![];

    let tilesets = byte_reader::unpack("32s32s32s32s", block);

    for tileset in &tilesets {
        match tileset {
            UnpackedValue::String(string) => {level_metadata.push(UnpackedValue::String(string.to_string()))}
            _ => todo!()
        }
    }
    level_metadata
}

fn read_level_options(block: &[u8]) -> HashMap<String, UnpackedValue> {
    let mut options: HashMap<String, UnpackedValue> = HashMap::new();

    let chunks = byte_reader::unpack("2L:x:o:xx:o:3x:B:o", block);
    let values: Vec<UnpackedValue> = chunks;

    if let Some(value) = values.first() {options.insert("events_a".to_string(), value.clone());}
    if let Some(value) = values.get(1) {options.insert("events_b".to_string(), value.clone());}
    if let Some(value) = values.get(2) {options.insert("can_wrap".to_string(), value.clone());}
    if let Some(value) = values.get(3) {options.insert("is_credits".to_string(), value.clone());}
    if let Some(value) = values.get(4) {options.insert("start_entrance".to_string(), value.clone());}
    if let Some(value) = values.get(5) {options.insert("is_ambush".to_string(), value.clone());}

    // Handle timer
    let timer_chunk = &block[10..12];
    let mut timer = (256 * timer_chunk[0] as u16) + timer_chunk[1] as u16 + 200;
    if timer_chunk[0] == 255 {
        timer = 200 - (256 - timer_chunk[1] as u16)
    }
    options.insert("time_limit".to_string(), UnpackedValue::UInt16(timer));
    
    options
}

fn read_level_entrances(block: &[u8]) -> Vec<UnpackedValue> {
    const OFFSET: usize = 20;
    let mut entrances: Vec<UnpackedValue> = vec![];
    let mut i = 0;
    let block_size = block.len();
    while i < block_size {
        let chunk = byte_reader::unpack("2H:4x:4B:x:3:B:H:o:B", &block[i..(i + OFFSET)]);
        let mut entrance: HashMap<String, UnpackedValue> = HashMap::new();
        if let Some(value) = chunk.first() {entrance.insert("pos_x".to_string(), value.clone());}
        if let Some(value) = chunk.get(1) {entrance.insert("pos_y".to_string(), value.clone());}
        if let Some(value) = chunk.get(2) {entrance.insert("id".to_string(), value.clone());}
        if let Some(value) = chunk.get(3) {entrance.insert("destination_area".to_string(), value.clone());}
        if let Some(value) = chunk.get(4) {entrance.insert("destination_entrance".to_string(), value.clone());}
        if let Some(value) = chunk.get(5) {entrance.insert("type".to_string(), value.clone());}
        if let Some(value) = chunk.get(6) {entrance.insert("zone".to_string(), value.clone());}
        if let Some(value) = chunk.get(7) {entrance.insert("layer".to_string(), value.clone());}
        if let Some(value) = chunk.get(8) {entrance.insert("path".to_string(), value.clone());}
        if let Some(value) = chunk.get(10) {entrance.insert("exit_to_map".to_string(), value.clone());}
        if let Some(value) = chunk.get(11) {entrance.insert("connected_pipe_direction".to_string(), value.clone());}
        

        // Other settings
        let mut config: Vec<bool> = vec![];
        if let Some(UnpackedValue::UInt16(byte)) = chunk.get(9) {
            config = u16_to_bits(*byte);
        }

        if let Some(enterable) = config.get(7) {entrance.insert("enterable".to_string(), UnpackedValue::Boolean(*enterable));}

        entrances.push(UnpackedValue::Map(entrance));
        i += OFFSET;
    }
    
    entrances
}

fn read_level_sprites(block: &[u8]) -> Vec<UnpackedValue> {
    const OFFSET: usize = 16;
    let mut sprites: Vec<UnpackedValue> = vec![];
    let mut i = 0;
    let block_size = block.len();
    while i + OFFSET < block_size {
        let chunk = byte_reader::unpack("3H:8B:xx", &block[i..(i + OFFSET)]);
        let mut sprite: HashMap<String, UnpackedValue> = HashMap::new();

        if let UnpackedValue::UInt16(val) = chunk[0] {
            if val == 65535 {
                break;
            }
        }

        if let Some(value) = chunk.first() {sprite.insert("type".to_string(), value.clone());}
        if let Some(value) = chunk.get(1) {sprite.insert("pos_x".to_string(), value.clone());}
        if let Some(value) = chunk.get(2) {sprite.insert("pos_y".to_string(), value.clone());}
        sprite.insert("data".to_string(), UnpackedValue::Vec(chunk[3..11].to_vec()));

        sprites.push(byte_reader::UnpackedValue::Map(sprite));

        i += OFFSET;
    }
    sprites
}

fn read_level_zones(zone_config_block: &[u8], zone_bounds_block: &[u8]) -> Vec<UnpackedValue> {
    const OFFSET: usize = 24;
    let mut zones: Vec<UnpackedValue> = vec![];
    let block_size = zone_config_block.len();
    let mut i = 0;
    while i < block_size {
        let mut zone: HashMap<String, UnpackedValue> = HashMap::new();
        let zone_config = byte_reader::unpack("6H:4B:x:4B:x:2B", zone_config_block);
        let zone_bounds = byte_reader::unpack("4L:xx:3H:x", zone_bounds_block);

        let mut is_dark: bool = false;
        let mut fg_spotlight: bool = false; 

        // SPOTLIGHT
        if let Some(UnpackedValue::UInt16(val)) = zone_config.get(10) {
            let mut spotlight_setting = val.clone();
            if spotlight_setting >= 32 {
                is_dark = true;
                spotlight_setting -= 32;
            }
            if spotlight_setting >= 16 {
                fg_spotlight = true;
                spotlight_setting -= 16;
            }
        }

        let config_keys = [
            ("pos_x", 0),
            ("pos_y", 1),
            ("size_x", 2),
            ("size_y", 3),
            ("theme", 4),
            ("lighting", 5),
            ("id", 7),
            ("music", 14),
        ];

        let bound_keys = [
            ("upper_bound", 0),
            ("lower_bound", 1),
            ("lakitu_upper_bound", 2),
            ("lakitu_lower_bound", 3),
            ("multiplayer_upper_bound", 5),
            ("multiplayer_lower_bound", 6),
            ("id", 7),
            ("multiplayer_fly_screen_adjust", 4),
        ];


        for (key, index) in config_keys.iter() {
            if let Some(value) = zone_config.get(*index) {
                zone.insert(key.to_string(), value.clone());
            }
        }

        for (key, index) in bound_keys.iter() {
            if let Some(value) = zone_bounds.get(*index) {
                zone.insert(key.to_string(), value.clone());
            }
        }


        if let Some(value) = zone_config.get(15) {
            match value {
                &UnpackedValue::UInt8(v) => {
                    zone.insert("echo".to_string(), UnpackedValue::UInt8(v / 16));
                    zone.insert("boss_room".to_string(), UnpackedValue::Boolean(v % 16 != 0));
                }
                _ => {}
            }
        }

        zones.push(byte_reader::UnpackedValue::Map(zone));
        i += OFFSET
    }

    zones
}

fn read_level_backgrounds(front_bg_block: &[u8], back_bg_block: &[u8]) -> Vec<UnpackedValue> {
    const OFFSET: usize = 24;
    let mut backgrounds: Vec<UnpackedValue> = vec![];
    let block_size = front_bg_block.len();
    let mut i = 0;
    while i < block_size {
        let mut background: HashMap<String, UnpackedValue> = HashMap::new();
        let bgf: Vec<UnpackedValue> = byte_reader::unpack("x:B:4h:3h:3x:B:4x", &front_bg_block[i..]);
        let bgb = byte_reader::unpack("x:B:4h:3h:3x:B:4x", &back_bg_block[i..]);

        // Sub-hashmaps
        let mut front: HashMap<String, UnpackedValue> = HashMap::new();
        let mut back: HashMap<String, UnpackedValue> = HashMap::new();

        if let Some(value) = bgf.first() {background.insert("id".to_string(), value.clone());}

        let bg_keys = [
            ("scroll_rate_x", 1),
            ("scroll_rate_y", 2),
            ("pos_x", 4),
            ("pos_y", 3),
            ("instance", 5),
            ("zoom", 8),
        ];

        for (key, index) in bg_keys.iter() {
            if let Some(value) = bgf.get(*index) {
                front.insert(key.to_string(), value.clone());
            }
            if let Some(value) = bgb.get(*index) {
                back.insert(key.to_string(), value.clone());
            }
        }

        background.insert("front".to_string(), UnpackedValue::Map(front));
        background.insert("back".to_string(), UnpackedValue::Map(back));

        backgrounds.push(byte_reader::UnpackedValue::Map(background));

        i += OFFSET;
    }
    backgrounds
}

fn u16_to_bits(n: u16) -> Vec<bool> {
    (0..16).map(|i| (n & (1 << i)) != 0).collect()
}
