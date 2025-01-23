use std::io::{self, Read};
use std::collections::HashMap;
use std::fs;

use crate::utils::byte_serializer::{self, UnpackedValue};
use crate::wii::arc::U8;

// Translated From Reggie-Updated
// https://github.com/NSMBW-Community/Reggie-Updated/tree/fa12de16ea8df33068ae93ec4616f8e67dbc05ca


pub struct Level {
    pub unpacked_buffer: UnpackedValue
}

pub fn new() -> Level {
    Level { 
        unpacked_buffer: UnpackedValue::Vec(vec![])
    }
}

impl Level {
    pub fn open_archive(&mut self, archive_path: String) {
        
        let mut level_archive = U8::new();

        let level_archive_data = fs::read(archive_path)
            .expect("Could not read file...");
        level_archive.load(&level_archive_data)
            .expect("Could not load data...");
        
        let areas = level_archive
            .get_dir("course")
            .expect("Error getting arc dir");

        let mut level: Vec<UnpackedValue> = vec![];

        let mut i = 1;
        for area in areas {
            if !area.contains("bgdat") && area.contains("course") {
                println!("{}", format_args!("course/course{}.bin", i));
                let area_data = level_archive.get(&format!("course/course{}.bin", i)).expect("Could not read area!");

                let fg_data = level_archive.get(&format!("course/course{}_bgdatL0.bin", i));
                let g_data = level_archive.get(&format!("course/course{}_bgdatL1.bin", i));
                let bg_data = level_archive.get(&format!("course/course{}_bgdatL2.bin", i));

                level.push(Self::read_area(area_data.to_vec(), fg_data, g_data, bg_data));
                
                i += 1;
            }
        }
        self.unpacked_buffer = UnpackedValue::Vec(level);
    }
    
    pub fn open_binary(&self, binary_path: String) {
    }

    pub fn is_valid_level(filename: &str) -> io::Result<bool> {
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

    fn read_area(
        data: Vec<u8>,
        fg_data: Option<&Vec<u8>>,
        g_data: Option<&Vec<u8>>,
        bg_data: Option<&Vec<u8>>,
    ) -> UnpackedValue {
        let mut level_info: HashMap<String, UnpackedValue> = HashMap::new();
        
        match Self::read_blocks(data) {
            Ok(blocks) => {
                level_info.insert("options".to_string(), UnpackedValue::Map(Self::read_options(&blocks[1])));
                level_info.insert("tilesets".to_string(), UnpackedValue::Vec(Self::read_tilesets(&blocks[0])));
                level_info.insert("entrances".to_string(), UnpackedValue::Vec(Self::read_entrances(&blocks[6])));
                level_info.insert("sprites".to_string(), UnpackedValue::Vec(Self::read_sprites(&blocks[7])));
                level_info.insert("zones".to_string(), UnpackedValue::Vec(Self::read_zones(&blocks[9], &blocks[2])));
                level_info.insert("backgrounds".to_string(), UnpackedValue::Vec(Self::read_backgrounds(&blocks[4], &blocks[5])));
                level_info.insert("paths".to_string(), UnpackedValue::Vec(Self::read_paths(&blocks[12], &blocks[13])));
                level_info.insert("regions".to_string(), UnpackedValue::Vec(Self::read_regions(&blocks[10])));
                level_info.insert("cameras".to_string(), UnpackedValue::Vec(Self::read_cameras(&blocks[11])));
                level_info.insert("tiles".to_string(), Self::read_tiles(fg_data, g_data, bg_data));
            }
            Err(_) => {
                eprintln!("Error: Could not read blocks!");
            }
        }
    
        UnpackedValue::Map(level_info)
    }
    
    fn read_blocks(course_data: Vec<u8>) -> io::Result<Vec<Vec<u8>>> {
        const BLOCKS: usize = 14;
        const BLOCK_METADATA_SIZE: usize = 8;
    
        if course_data.len() < BLOCKS * BLOCK_METADATA_SIZE {
            eprintln!("Error: File too small to contain metadata for all blocks!");
            return Ok(Vec::new());
        }
    
        let mut blocks = Vec::with_capacity(BLOCKS);
    
        for i in 0..BLOCKS {
            let meta_offset = i * BLOCK_METADATA_SIZE;
    
            let block_offset = u32::from_be_bytes(course_data[meta_offset..meta_offset + 4].try_into().unwrap()) as usize;
            let block_size = u32::from_be_bytes(course_data[meta_offset + 4..meta_offset + 8].try_into().unwrap()) as usize;
    
            if block_size == 0 {
                blocks.push(Vec::new());
            } else if block_offset + block_size <= course_data.len() {
                blocks.push(course_data[block_offset..block_offset + block_size].to_vec());
            } else {
                eprintln!(
                    "Warning: Invalid metadata for block {}. Offset={}, Size={}, FileSize={}",
                    i, block_offset, block_size, course_data.len()
                );
                blocks.push(Vec::new());
            }
        }
    
        Ok(blocks)
    }
    
    fn read_options(block: &[u8]) -> HashMap<String, UnpackedValue> {
    let mut options = HashMap::new();

    let chunk_options = byte_serializer::unpack("2L:x:o:xx:o:3x:B:o", block);

    let option_keys = [
        ("events_a", 0),
        ("events_b", 1),
        ("can_wrap", 2),
        ("is_credits", 3),
        ("start_entrance", 4),
        ("is_ambush", 5),
    ];
    map_keys(&option_keys, &chunk_options, &mut options);

    // Handle the timer chunk
    let timer_chunk = &block[10..12];
    let timer = if timer_chunk[0] == 255 {
        200 - (256 - timer_chunk[1] as u16)
    } else {
        (256 * timer_chunk[0] as u16) + timer_chunk[1] as u16 + 200
    };
    options.insert("time_limit".to_string(), UnpackedValue::UInt16(timer));

    options
}

    fn read_tilesets(block: &[u8]) -> Vec<UnpackedValue> {
    let chunk_tilesets = byte_serializer::unpack("32s32s32s32s", block);

    chunk_tilesets
        .into_iter()
        .filter_map(|tileset| {
            if let UnpackedValue::String(string) = tileset {
                Some(UnpackedValue::String(string.to_string()))
            } else {
                None
            }
        })
        .collect()
}

    fn read_entrances(block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 20;
        const ENTRANCE_KEYS: [(&str, usize); 11] = [
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

        block
            .chunks_exact(OFFSET)
            .map(|chunk| {
                let chunk_entrances = byte_serializer::unpack("2H:4x:4B:x:3:B:H:o:B", chunk);
                let mut entrance: HashMap<String, UnpackedValue> = HashMap::new();

                map_keys(&ENTRANCE_KEYS, &chunk_entrances, &mut entrance);

                if let Some(UnpackedValue::UInt16(byte)) = chunk_entrances.get(9) {
                    if let Some(enterable) = u16_to_bits(*byte).get(7) {
                        entrance.insert("enterable".to_string(), UnpackedValue::Boolean(*enterable));
                    }
                }
                
                UnpackedValue::Map(entrance)
            })
            .collect()
    }

    fn read_sprites(block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 16;
        const SPRITE_KEYS: [(&str, usize); 3] = [
            ("type", 0),
            ("pos_x", 1),
            ("pos_y", 2),
        ];
    
        block
            .chunks_exact(OFFSET)
            .filter_map(|chunk_sprites| {
                let chunk = byte_serializer::unpack("3H:8B:xx", chunk_sprites);
                let mut sprite: HashMap<String, UnpackedValue> = HashMap::new();
    
                // Check for sentinel value (65535), signaling the end
                if let Some(UnpackedValue::UInt16(val)) = chunk.get(0) {
                    if *val == 65535 {
                        return None; // Skip this chunk and terminate the loop
                    }
                }
    
                map_keys(&SPRITE_KEYS, &chunk, &mut sprite);
    
                sprite.insert(
                    "data".to_string(),
                    UnpackedValue::Vec(chunk[3..11].to_vec()),
                );
    
                Some(UnpackedValue::Map(sprite))
            })
            .collect()
    }

    fn read_zones(zone_config_block: &[u8], zone_bounds_block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 24;
        const CONFIG_KEYS: [(&str, usize); 8] = [
            ("pos_x", 0),
            ("pos_y", 1),
            ("size_x", 2),
            ("size_y", 3),
            ("theme", 4),
            ("lighting", 5),
            ("id", 7),
            ("music", 14),
        ];
    
        const BOUND_KEYS: [(&str, usize); 8] = [
            ("upper_bound", 0),
            ("lower_bound", 1),
            ("lakitu_upper_bound", 2),
            ("lakitu_lower_bound", 3),
            ("multiplayer_upper_bound", 5),
            ("multiplayer_lower_bound", 6),
            ("id", 7),
            ("multiplayer_fly_screen_adjust", 4),
        ];
    
        zone_config_block
            .chunks_exact(OFFSET)
            .zip(zone_bounds_block.chunks_exact(OFFSET))
            .filter_map(|(chunk_zone_config, chunk_zone_bounds)| {
                let mut zone: HashMap<String, UnpackedValue> = HashMap::new();
                let chunk_zone_config = byte_serializer::unpack("6H:4B:x:4B:x:2B", chunk_zone_config);
                let chunk_zone_bounds = byte_serializer::unpack("4L:xx:3H:x", chunk_zone_bounds);
    
                let mut is_dark = false;
                let mut fg_spotlight = false;
                let mut spotlight_setting = 0;
    
                // SPOTLIGHT
                if let Some(UnpackedValue::UInt16(val)) = chunk_zone_config.get(10) {
                    spotlight_setting = *val;
                    if spotlight_setting >= 32 {
                        is_dark = true;
                        spotlight_setting -= 32;
                    }
                    if spotlight_setting >= 16 {
                        fg_spotlight = true;
                        spotlight_setting -= 16;
                    }
                }
    
                map_keys(&CONFIG_KEYS, &chunk_zone_config, &mut zone);
                map_keys(&BOUND_KEYS, &chunk_zone_bounds, &mut zone);
    
                // additional settings for echo, boss_room, etc.
                if let Some(UnpackedValue::UInt8(v)) = chunk_zone_config.get(15) {
                    zone.insert("echo".to_string(), UnpackedValue::UInt8(v / 16));
                    zone.insert("boss_room".to_string(), UnpackedValue::Boolean(v % 16 != 0));
                    zone.insert("is_dark".to_string(), UnpackedValue::Boolean(is_dark));
                    zone.insert("fg_spotlight".to_string(), UnpackedValue::Boolean(fg_spotlight));
                    zone.insert("spotlight_config".to_string(), UnpackedValue::UInt16(spotlight_setting));
                }
    
                Some(UnpackedValue::Map(zone))
            })
            .collect()
    }
    
    fn read_backgrounds(front_bg_block: &[u8], back_bg_block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 24;
        const BG_KEYS: [(&str, usize); 6] = [
            ("scroll_rate_x", 1),
            ("scroll_rate_y", 2),
            ("pos_x", 4),
            ("pos_y", 3),
            ("instance", 5),
            ("zoom", 8),
        ];
    
        front_bg_block
            .chunks_exact(OFFSET)
            .zip(back_bg_block.chunks_exact(OFFSET))
            .map(|(chunk_f, chunk_b)| {
                let mut background: HashMap<String, UnpackedValue> = HashMap::new();
    
                let chunk_f = byte_serializer::unpack("x:B:4h:3h:3x:B:4x", chunk_f);
                let chunk_b = byte_serializer::unpack("x:B:4h:3h:3x:B:4x", chunk_b);
    
                let mut front: HashMap<String, UnpackedValue> = HashMap::new();
                let mut back: HashMap<String, UnpackedValue> = HashMap::new();
    
                // Insert ID as separate field
                if let Some(value) = chunk_f.first() {
                    background.insert("id".to_string(), value.clone());
                }
    
                map_keys(&BG_KEYS, &chunk_f, &mut front);
                map_keys(&BG_KEYS, &chunk_b, &mut back);
    
                background.insert("front".to_string(), UnpackedValue::Map(front));
                background.insert("back".to_string(), UnpackedValue::Map(back));
    
                UnpackedValue::Map(background)
            })
            .collect()
    }
    
    fn read_paths(path_block: &[u8], path_node_block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 8; // offset for path_block
        const SUB_OFFSET: u16 = 16; // offset for path_node_block
        const PATH_KEYS: [(&str, usize); 5] = [
            ("pos_x", 0),
            ("pos_y", 1),
            ("speed", 2),
            ("acceleration", 3),
            ("delay", 4),
        ];
    
        path_block
            .chunks_exact(OFFSET)
            .enumerate()
            .map(|(_, chunk)| {
                let mut path_config: HashMap<String, UnpackedValue> = HashMap::new();
    
                let unpacked_chunk = byte_serializer::unpack("BxHHxo", chunk);
                path_config.insert("id".to_string(), unpacked_chunk[0].clone());
                path_config.insert("loops".to_string(), unpacked_chunk[3].clone());
    
                let count = unpacked_chunk[2].as_u16().expect("Expected u16 value for count");
    
                let current_path_vec: Vec<UnpackedValue> = (0..count)
                    .map(|i| {
                        let current_offset = (i * SUB_OFFSET) as usize;
                        let node_chunk = byte_serializer::unpack("HHffhxx", &path_node_block[current_offset..]);
    
                        let mut current_path_map: HashMap<String, UnpackedValue> = HashMap::new();
                        map_keys(&PATH_KEYS, &node_chunk, &mut current_path_map);
    
                        UnpackedValue::Map(current_path_map)
                    })
                    .collect();
    
                path_config.insert("points".to_owned(), UnpackedValue::Vec(current_path_vec));
    
                UnpackedValue::Map(path_config)
            })
            .collect()
    }
    
    fn read_regions(block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 12;
        const REGION_KEYS: [(&str, usize); 5] = [
            ("pos_x", 0),
            ("pos_y", 1),
            ("size_x", 2),
            ("size_y", 3),
            ("id", 4),
        ];
    
        block
            .chunks_exact(OFFSET)
            .map(|chunk| {
                let unpacked_chunk = byte_serializer::unpack("HHHHBxxx", chunk);
                let mut region_config: HashMap<String, UnpackedValue> = HashMap::new();
    
                map_keys(&REGION_KEYS, &unpacked_chunk, &mut region_config);
    
                UnpackedValue::Map(region_config)
            })
            .collect()
    }
    
    fn read_cameras(block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 20;
        const CAMERA_KEYS: [(&str, usize); 3] = [
            ("zoom_config", 1),
            ("scren_heights", 2),
            ("event_trigger_id", 3),
        ];
    
        block
            .chunks_exact(OFFSET)
            .map(|chunk| {
                let unpacked_chunk = byte_serializer::unpack("12x:BBB:xxx:B:x", chunk);
                let mut camera_config: HashMap<String, UnpackedValue> = HashMap::new();
    
                map_keys(&CAMERA_KEYS, &unpacked_chunk, &mut camera_config);
    
                UnpackedValue::Map(camera_config)
            })
            .collect()
    }
    
    fn read_tiles(fg_data: Option<&Vec<u8>>, g_data: Option<&Vec<u8>>, bg_data: Option<&Vec<u8>>) -> UnpackedValue {
        const OFFSET: usize = 10;
        let tile_keys = [
            ("object_id", 1),
            ("pos_x", 2),
            ("pos_y", 3),
            ("size_x", 4),
            ("size_y", 5),
        ];
    
        fn process_layer(data: &[u8], tile_keys: &[(&str, usize)]) -> Vec<UnpackedValue> {
            data.chunks_exact(OFFSET)
                .map(|chunk| {
                    let mut tile_config = HashMap::new();
                    let chunk_data = byte_serializer::unpack("BBHHHH", chunk);
    
                    let tileset = chunk_data[0]
                        .as_u8()
                        .map(|val| val / 16)
                        .unwrap_or(0);
    
                    tile_config.insert("tileset".to_string(), UnpackedValue::UInt8(tileset));
                    map_keys(tile_keys, &chunk_data, &mut tile_config);
    
                    UnpackedValue::Map(tile_config)
                })
                .collect()
        }
    
        let process_tile_data = |_name: &str, data: Option<&Vec<u8>>| {
            data.map(|d| process_layer(d, &tile_keys))
                .unwrap_or_else(|| vec![])
        };
    
        let mut tiles: HashMap<String, UnpackedValue> = HashMap::new();
        
        tiles.insert("foreground".to_string(), UnpackedValue::Vec(process_tile_data("foreground", fg_data)));
        tiles.insert("ground".to_string(), UnpackedValue::Vec(process_tile_data("ground", g_data)));
        tiles.insert("background".to_string(), UnpackedValue::Vec(process_tile_data("background", bg_data)));
    
        UnpackedValue::Map(tiles)
    }
    
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
