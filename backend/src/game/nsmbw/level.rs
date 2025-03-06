use std::error::Error;
use std::{fs, collections::HashMap, path::Path};

use crate::system::archive::arc::U8Arc;
use crate::system::binary::serializer::{self, UnpackedValue};


pub struct NSMBWLevel {
	areas: Vec<NSMBWArea>,
}

impl NSMBWLevel {
	pub fn new() -> NSMBWLevel {
		NSMBWLevel {
			areas : vec![],
		}
	}

	pub fn open(&mut self, arc_path: &Path) {
		match fs::read(arc_path) {
			Ok(buffer) => { self.parse_level(&buffer); }
			Err(error) => println!("Error when reading file: {:?}\n{:?}", arc_path, error)
		}
	}

	pub fn print_info(&self) {
		println!("{:?}", self.areas);
	}

	fn parse_level(&mut self, buf: &[u8]) {
		if buf.is_empty() {
			println!("Buffer is empty");
			return;
		}

		if !buf.starts_with(b"U\xAA8-") {
			println!("Header mismatch! {:?}", buf.first());
		}

		self.areas = Self::parse_areas(buf).expect("msg");	
		println!("{:?}", self.areas.first());
	}

	fn parse_areas(buffer: &[u8]) -> Result<Vec<NSMBWArea>, Box<dyn Error>> {
		let mut archive = U8Arc::new();
		archive.load(buffer)?;
	
		let area_files = archive.get_dir("course").ok_or("Missing 'course' directory")?;
		let mut level_areas = Vec::new();
		let empty_vec: Vec<u8> = Vec::new(); // Stable empty vector
	
		for area_file in area_files {
			if !area_file.contains("course") || area_file.contains("_bgdatL") {
				continue;
			}
	
			let area_name = match area_file.strip_suffix(".bin") {
				Some(name) => name,
				None => continue,
			};
	
			let area_data = match archive.get(&format!("course/{}.bin", area_name)) {
				Some(data) => data,
				None => continue,
			};
	
			let area_tile_data = [
				archive.get(&format!("course/{}_bgdatL0.bin", area_name)).unwrap_or(&empty_vec),
				archive.get(&format!("course/{}_bgdatL1.bin", area_name)).unwrap_or(&empty_vec),
				archive.get(&format!("course/{}_bgdatL2.bin", area_name)).unwrap_or(&empty_vec),
			];
	
			if let Ok(area) = NSMBWArea::parse_area(area_data, area_tile_data) {
				level_areas.push(area);
			}
		}
	
		Ok(level_areas)
	}

	
}

#[derive(Debug)]
struct NSMBWArea {
	tilesets: [String; 4],
	options: HashMap<String, UnpackedValue>,
	backgrounds: [Vec<UnpackedValue>; 2],
	entrances: Vec<UnpackedValue>,
	sprites: Vec<UnpackedValue>,
	zones: Vec<UnpackedValue>,
	regions: Vec<UnpackedValue>,
	cameras: Vec<UnpackedValue>,
	paths: Vec<UnpackedValue>,
	tiles: [Vec<UnpackedValue>; 3]
}

impl NSMBWArea {
    pub fn parse_area(area_buf: &[u8], tile_buf: [&Vec<u8>; 3]) -> Result<NSMBWArea, Box<dyn std::error::Error>> {
		let blocks = Self::parse_blocks(area_buf)?;

		let area = NSMBWArea {
			tilesets: Self::area_parse_tilesets(&blocks[0]),
			options: Self::area_parse_options(&blocks[1]),
			backgrounds: Self::area_parse_backgrounds(&blocks[4], &blocks[5]),
			entrances: Self::area_parse_entrances(&blocks[6]),
			sprites: Self::area_parse_sprites(&blocks[7]),
			zones: Self::area_parse_zones(&blocks[9], &blocks[2]),
			regions: Self::area_parse_regions(&blocks[10]),
			cameras: Self::area_parse_cameras(&blocks[11]),
			paths: Self::area_parse_paths(&blocks[12], &blocks[13]),
			tiles: Self::area_parse_tiles(tile_buf),
		};

		Ok(area)
    }

    fn parse_blocks(course_data: &[u8]) -> Result<[Vec<u8>; 14], Box<dyn std::error::Error>> {
        const BLOCKS: usize = 14;
        const BLOCK_METADATA_SIZE: usize = 8;

        if course_data.len() < BLOCKS * BLOCK_METADATA_SIZE {
            eprintln!("Error: File too small to contain metadata for all blocks!");
            return Ok(std::array::from_fn(|_| Vec::new())); // Return an empty array
        }

        let blocks: [Vec<u8>; BLOCKS] = std::array::from_fn(|i| {
            let meta_offset = i * BLOCK_METADATA_SIZE;

            let block_offset = u32::from_be_bytes(course_data[meta_offset..meta_offset + 4].try_into().unwrap()) as usize;
            let block_size = u32::from_be_bytes(course_data[meta_offset + 4..meta_offset + 8].try_into().unwrap()) as usize;

            if block_size == 0 {
                Vec::new()
            } else if block_offset + block_size <= course_data.len() {
                course_data[block_offset..block_offset + block_size].to_vec()
            } else {
                eprintln!(
                    "Warning: Invalid metadata for block {}. Offset={}, Size={}, FileSize={}",
                    i, block_offset, block_size, course_data.len()
                 );
                Vec::new()
            }
        });
        Ok(blocks)
    }

	fn area_parse_tilesets(block: &[u8]) -> [String; 4] {
		let chunk_tilesets = serializer::unpack("32s32s32s32s", block);
	
		let tilesets: Vec<String> = chunk_tilesets
			.into_iter()
			.filter_map(|tileset| {
				if let UnpackedValue::String(string) = tileset {
					Some(string.to_string()) // Convert to owned String
				} else {
					None
				}
			})
			.collect();

		// Ensure we always return exactly 4 elements
		match tilesets.try_into() {
			Ok(array) => array,
			Err(_) => panic!("Expected exactly 4 tilesets, but got a different amount!"),
		}
	}
	
    fn area_parse_options(block: &[u8]) -> HashMap<String, UnpackedValue> {
			let mut options = HashMap::new();
		
			let chunk_options = serializer::unpack("2L:x:o:xx:o:3x:B:o", block);
		
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

	fn area_parse_backgrounds(front_bg_block: &[u8], back_bg_block: &[u8]) -> [Vec<UnpackedValue>; 2] {
		use std::collections::HashMap;
	
		const OFFSET: usize = 24;
		const BG_KEYS: [(&str, usize); 6] = [
			("scroll_rate_x", 1),
			("scroll_rate_y", 2),
			("pos_x", 4),
			("pos_y", 3),
			("instance", 5),
			("zoom", 8),
		];
	
		let mut front_backgrounds = Vec::new();
		let mut back_backgrounds = Vec::new();
	
		for (chunk_f, chunk_b) in front_bg_block.chunks_exact(OFFSET).zip(back_bg_block.chunks_exact(OFFSET)) {
			let mut background: HashMap<String, UnpackedValue> = HashMap::new();
	
			let chunk_f = serializer::unpack("x:B:4h:3h:3x:B:4x", chunk_f);
			let chunk_b = serializer::unpack("x:B:4h:3h:3x:B:4x", chunk_b);
	
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
	
			front_backgrounds.push(UnpackedValue::Map(background.clone()));
			back_backgrounds.push(UnpackedValue::Map(background));
		}
	
		[front_backgrounds, back_backgrounds]
	}
	
	fn area_parse_entrances(block: &[u8]) -> Vec<UnpackedValue> {
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
                let chunk_entrances = serializer::unpack("2H:4x:4B:x:3:B:H:o:B", chunk);
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
    
	fn area_parse_sprites(block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 16;
        const SPRITE_KEYS: [(&str, usize); 3] = [
            ("type", 0),
            ("pos_x", 1),
            ("pos_y", 2),
        ];
    
        block
            .chunks_exact(OFFSET)
            .filter_map(|chunk_sprites| {
                let chunk = serializer::unpack("3H:8B:xx", chunk_sprites);
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

	fn area_parse_zones(zone_config_block: &[u8], zone_bounds_block: &[u8]) -> Vec<UnpackedValue> {
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
                let chunk_zone_config = serializer::unpack("6H:4B:x:4B:x:2B", chunk_zone_config);
                let chunk_zone_bounds = serializer::unpack("4L:xx:3H:x", chunk_zone_bounds);
    
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

	fn area_parse_regions(block: &[u8]) -> Vec<UnpackedValue> {
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
                let unpacked_chunk = serializer::unpack("HHHHBxxx", chunk);
                let mut region_config: HashMap<String, UnpackedValue> = HashMap::new();
    
                map_keys(&REGION_KEYS, &unpacked_chunk, &mut region_config);
    
                UnpackedValue::Map(region_config)
            })
            .collect()
    }

	fn area_parse_cameras(block: &[u8]) -> Vec<UnpackedValue> {
        const OFFSET: usize = 20;
        const CAMERA_KEYS: [(&str, usize); 3] = [
            ("zoom_config", 1),
            ("scren_heights", 2),
            ("event_trigger_id", 3),
        ];
    
        block
            .chunks_exact(OFFSET)
            .map(|chunk| {
                let unpacked_chunk = serializer::unpack("12x:BBB:xxx:B:x", chunk);
                let mut camera_config: HashMap<String, UnpackedValue> = HashMap::new();
    
                map_keys(&CAMERA_KEYS, &unpacked_chunk, &mut camera_config);
    
                UnpackedValue::Map(camera_config)
            })
            .collect()
    }

	fn area_parse_paths(path_block: &[u8], path_node_block: &[u8]) -> Vec<UnpackedValue> {
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
    
                let unpacked_chunk = serializer::unpack("BxHHxo", chunk);
                path_config.insert("id".to_string(), unpacked_chunk[0].clone());
                path_config.insert("loops".to_string(), unpacked_chunk[3].clone());
    
                let count = unpacked_chunk[2].as_u16().expect("Expected u16 value for count");
    
                let current_path_vec: Vec<UnpackedValue> = (0..count)
                    .map(|i| {
                        let current_offset = (i * SUB_OFFSET) as usize;
                        let node_chunk = serializer::unpack("HHffhxx", &path_node_block[current_offset..]);
    
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

	fn area_parse_tiles(tile_blocks: [&Vec<u8>; 3]) -> [Vec<UnpackedValue>; 3] {
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
					let chunk_data = serializer::unpack("BBHHHH", chunk);
	
					let tileset = chunk_data
						.get(0)
						.and_then(|v| v.as_u8())
						.map(|val| val / 16)
						.unwrap_or(0);
	
					tile_config.insert("tileset".to_string(), UnpackedValue::UInt8(tileset));
					map_keys(tile_keys, &chunk_data, &mut tile_config);
	
					UnpackedValue::Map(tile_config)
				})
				.collect()
		}
	
		[
			process_layer(tile_blocks[0], &tile_keys), // FG
			process_layer(tile_blocks[1], &tile_keys), // G
			process_layer(tile_blocks[2], &tile_keys), // BG
		]
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
