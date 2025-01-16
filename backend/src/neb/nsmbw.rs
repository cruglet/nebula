use std::io::{self, Read, Result};
use std::collections::HashMap;
use std::fs;

use crate::utils::byte_reader::{self, UnpackedValue};
use crate::wii::arc::U8;

use super::godot;

// Translated From Reggie-Updated
// https://github.com/NSMBW-Community/Reggie-Updated/tree/fa12de16ea8df33068ae93ec4616f8e67dbc05ca

// TODO:
// - read tiles (bgdat files)
// - read paths 
// - read regions 
// - read cameras 

pub fn is_nsmbw_level(filename: &str) -> io::Result<bool> {
    if fs::metadata(filename).is_err() {
        return Ok(false);
    }

    let mut file = fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.starts_with(&[0x11]) {
        return Ok(true);
    }

    if data.starts_with(b"U\xAA8-") {
        // Perform additional sanity checks using windows to check for slices
        if !data.windows(b"course\0".len()).any(|window| window == b"course\0") &&
           !data.windows(b"course2.bin\0".len()).any(|window| window == b"course2.bin\0") &&
           !data.windows(b"\0\0\0\x80".len()).any(|window| window == b"\0\0\0\x80") {
            return Ok(false);
        }
        return Ok(true);
    }

    // fallback for non-matching files
    Ok(false)
}

pub fn dump_level(path: String, to: String) -> Result<()> { 
    let mut level_archive = U8::new();
    let level_archive_data = fs::read(path).expect("Could not read file...");
    level_archive.load(&level_archive_data)?;
    let areas = level_archive.get_dir("course").expect("Error getting arc dir");

    let mut level: Vec<UnpackedValue> = vec![];

    let mut i = 0;
    for area in areas {
        if !area.contains("bgdat") && area.contains("course") {
            i += 1;
            println!("{}", format!("course/course{}.bin", i));
            let area_data = level_archive.get(&format!("course/course{}.bin", i)).expect("Could not read area!");
            level.push(read_level(area_data.to_vec()));
        }
    }

    let success = godot::value_to_file(&UnpackedValue::Vec(level), &to);

    success
}

pub fn read_level(data: Vec<u8>) -> UnpackedValue {
    let mut level_info: HashMap<String, UnpackedValue> = HashMap::new();
    let level_blocks = read_level_blocks(data);
    
    match &level_blocks {
        Ok(blocks) => {
            let _ = &level_info.insert(String::from("options"), UnpackedValue::Map(read_level_options(&blocks[1])));
            let _ = &level_info.insert(String::from("tilesets"), UnpackedValue::Vec(read_level_tilesets(&blocks[0])));
            let _ = &level_info.insert(String::from("entrances"), UnpackedValue::Vec(read_level_entrances(&blocks[6])));
            let _ = &level_info.insert(String::from("sprites"), UnpackedValue::Vec(read_level_sprites(&blocks[7])));
            let _ = &level_info.insert(String::from("zones"), UnpackedValue::Vec(read_level_zones(&blocks[9], &blocks[2])));
            let _ = &level_info.insert(String::from("backgrounds"), UnpackedValue::Vec(read_level_backgrounds(&blocks[4], &blocks[5])));
        }
        _ => {
            println!("COULD NOT READ BLOCKS!")
        }
    }
    UnpackedValue::Map(level_info)
}

fn read_level_blocks(course_data: Vec<u8>) -> io::Result<Vec<Vec<u8>>> {
    const BLOCKS: usize = 14;
    const BLOCK_METADATA_SIZE: usize = 8; 
    let mut blocks = Vec::new();

    let data = course_data;

    if data.len() < BLOCKS * BLOCK_METADATA_SIZE {
        eprintln!("File too small to contain metadata for all blocks!");
        return Ok(blocks);
    }

    for i in 0..BLOCKS {
        let meta_offset = i * BLOCK_METADATA_SIZE;

        let block_offset = u32::from_be_bytes([
            data[meta_offset],
            data[meta_offset + 1],
            data[meta_offset + 2],
            data[meta_offset + 3],
        ]) as usize;

        let block_size = u32::from_be_bytes([
            data[meta_offset + 4],
            data[meta_offset + 5],
            data[meta_offset + 6],
            data[meta_offset + 7],
        ]) as usize;

        if block_size == 0 {
            blocks.push(Vec::new());
        } else if block_offset + block_size <= data.len() {
            blocks.push(data[block_offset..block_offset + block_size].to_vec());
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

fn read_level_options(block: &[u8]) -> HashMap<String, UnpackedValue> {
    let mut options: HashMap<String, UnpackedValue> = HashMap::new();

    let chunk_options = byte_reader::unpack("2L:x:o:xx:o:3x:B:o", block);

    let option_keys = [
        ("events_a", 0),
        ("events_b", 1),
        ("can_wrap", 2),
        ("is_credits", 3),
        ("start_entrance", 4),
        ("is_ambush", 5),
    ];

    map_keys(&option_keys, &chunk_options, &mut options);

    // Handle timer
    let timer_chunk = &block[10..12];
    let mut timer = (256 * timer_chunk[0] as u16) + timer_chunk[1] as u16 + 200;
    if timer_chunk[0] == 255 {
        timer = 200 - (256 - timer_chunk[1] as u16)
    }
    options.insert("time_limit".to_string(), UnpackedValue::UInt16(timer));
    
    options
}

fn read_level_tilesets(block: &[u8]) -> Vec<UnpackedValue> {
    let mut level_tilesets: Vec<UnpackedValue> = vec![];

    let chunk_tilesets = byte_reader::unpack("32s32s32s32s", block);

    for tileset in &chunk_tilesets {
        match tileset {
            UnpackedValue::String(string) => {level_tilesets.push(UnpackedValue::String(string.to_string()))}
            _ => todo!()
        }
    }
    level_tilesets
}

fn read_level_entrances(block: &[u8]) -> Vec<UnpackedValue> {
    const OFFSET: usize = 20;
    let mut entrances: Vec<UnpackedValue> = vec![];
    let mut i = 0;
    let block_size = block.len();
    while i < block_size {
        let chunk_entrances = byte_reader::unpack("2H:4x:4B:x:3:B:H:o:B", &block[i..(i + OFFSET)]);
        let mut entrance: HashMap<String, UnpackedValue> = HashMap::new();

        let entrance_keys = [
            ("pos_x", 0),
            ("pos_y", 1),
            ("id", 2),
            ("destination_area", 3),
            ("destination_entrance", 4),
            ("type", 5),
            ("zone", 6),
            ("layer", 7),
            ("path", 8),
            ("exit_to_map", 10),
            ("connected_pipe_direction", 11),
        ];

        map_keys(&entrance_keys, &chunk_entrances, &mut entrance);

        // Other settings
        let mut config: Vec<bool> = vec![];
        if let Some(UnpackedValue::UInt16(byte)) = chunk_entrances.get(9) {
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
        let chunk_sprites = byte_reader::unpack("3H:8B:xx", &block[i..(i + OFFSET)]);
        let mut sprite: HashMap<String, UnpackedValue> = HashMap::new();

        if let UnpackedValue::UInt16(val) = chunk_sprites[0] {
            if val == 65535 {
                break;
            }
        }

        if let Some(value) = chunk_sprites.first() {sprite.insert("type".to_string(), value.clone());}
        if let Some(value) = chunk_sprites.get(1) {sprite.insert("pos_x".to_string(), value.clone());}
        if let Some(value) = chunk_sprites.get(2) {sprite.insert("pos_y".to_string(), value.clone());}
        
        sprite.insert("data".to_string(), UnpackedValue::Vec(chunk_sprites[3..11].to_vec()));
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
        let chunk_zone_config = byte_reader::unpack("6H:4B:x:4B:x:2B", zone_config_block);
        let chunk_zone_bounds = byte_reader::unpack("4L:xx:3H:x", zone_bounds_block);

        let mut is_dark: bool = false;
        let mut fg_spotlight: bool = false; 

        // SPOTLIGHT
        let mut spotlight_setting: u16 = 0;
        if let Some(UnpackedValue::UInt16(val)) = chunk_zone_config.get(10) {
        spotlight_setting = val.clone();
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

        map_keys(&config_keys, &chunk_zone_config, &mut zone);
        map_keys(&bound_keys, &chunk_zone_bounds, &mut zone);

        if let Some(value) = chunk_zone_config.get(15) {
            match value {
                &UnpackedValue::UInt8(v) => {
                    zone.insert("echo".to_string(), UnpackedValue::UInt8(v / 16));
                    zone.insert("boss_room".to_string(), UnpackedValue::Boolean(v % 16 != 0));
                    zone.insert("is_dark".to_string(), UnpackedValue::Boolean(is_dark));
                    zone.insert("fg_spotlight".to_string(), UnpackedValue::Boolean(fg_spotlight));
                    zone.insert("spotlight_config".to_string(), UnpackedValue::UInt16(spotlight_setting));
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
        let chunk_f: Vec<UnpackedValue> = byte_reader::unpack("x:B:4h:3h:3x:B:4x", &front_bg_block[i..]);
        let chunk_b = byte_reader::unpack("x:B:4h:3h:3x:B:4x", &back_bg_block[i..]);

        // Sub-hashmaps
        let mut front: HashMap<String, UnpackedValue> = HashMap::new();
        let mut back: HashMap<String, UnpackedValue> = HashMap::new();

        // Keep ID separate
        if let Some(value) = chunk_f.first() {background.insert("id".to_string(), value.clone());}

        let bg_keys = [
            ("scroll_rate_x", 1),
            ("scroll_rate_y", 2),
            ("pos_x", 4),
            ("pos_y", 3),
            ("instance", 5),
            ("zoom", 8),
        ];

        map_keys(&bg_keys, &chunk_f, &mut front);
        map_keys(&bg_keys, &chunk_b, &mut back);

        background.insert("front".to_string(), UnpackedValue::Map(front));
        background.insert("back".to_string(), UnpackedValue::Map(back));
        backgrounds.push(byte_reader::UnpackedValue::Map(background));

        i += OFFSET;
    }
    backgrounds
}

fn map_keys(key_list: &[(&str, usize)], chunk: &Vec<UnpackedValue>, map: &mut HashMap<String, UnpackedValue>) {
    for (key, index) in key_list.iter() {
        if let Some(value) = chunk.get(*index) {
            map.insert(key.to_string(), value.clone());
        }
    }
}

fn u16_to_bits(n: u16) -> Vec<bool> {
    (0..16).map(|i| (n & (1 << i)) != 0).collect()
}
