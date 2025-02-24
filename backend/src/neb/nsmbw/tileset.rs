use std::collections::HashMap;
use std::fs;

use crate::neb::godot;
use crate::utils::byte_serializer::UnpackedValue;
use crate::utils::image_serializer;
use crate::wii::arc::U8;
use crate::wii::lzss;

pub struct Tileset {
    pub image_data: Vec<u8>,
    pub behavior_data: Vec<u8>,
    pub metadata: Vec<u8>,
    pub objects: Vec<u8>,
}

pub fn new() -> Tileset {
    Tileset {
        image_data: vec![],
        behavior_data: vec![],
        metadata: vec![],
        objects: vec![],
    }
}

impl Tileset {
    pub fn load(&mut self, archive_path: String) {
        let mut tileset_archive = U8::new();
    
        let tileset_archive_data = fs::read(&archive_path)
            .expect("Could not read archive data!");
    
        tileset_archive
            .load(&tileset_archive_data)
            .expect("Could not load archive data!");
    
        if let Some(bg_tex_files) = tileset_archive.get_dir("BG_tex") {

            if let Some(tileset) = self.fetch_tileset_name(bg_tex_files) {
                let texture_path = format!("BG_tex/{}_tex.bin.LZ", tileset); // Re-add .bin if needed
                let behavior_path = format!("BG_chk/d_bgchk_{}.bin", tileset);
                let metadata_path = format!("BG_unt/{}_hd.bin", tileset);
                let obj_path = format!("BG_unt/{}.bin", tileset);  

                if let Some(compressed_texture) = tileset_archive.get(&texture_path) {
                    if let Some(decompressed_texture) = lzss::decompress_raw(compressed_texture.to_vec()) {
                        self.image_data = decompressed_texture;
                    }
                }

                if let Some(behavior ) = tileset_archive.get(&behavior_path) {
                    self.behavior_data = behavior.to_vec();
                }

                if let Some(metadata ) = tileset_archive.get(&metadata_path) {
                    self.metadata = metadata.to_vec();
                }

                if let Some(objects ) = tileset_archive.get(&obj_path) {
                    self.objects = objects.to_vec();
                }
            }
        }
    }

    pub fn extract(&self, path: String) -> Option<&Vec<u8>> {
        let mut tileset_data: HashMap<String, UnpackedValue> = HashMap::new();

        tileset_data.insert(String::from("objects"), self.read_object_data());

        if self.image_data.len() == 0 { return None }
        let image = image_serializer::rgb4a3_decode(&self.image_data, true);
        let image = image_serializer::minify_image(image);
        if image.is_empty() { return None }
        image.save(path.clone() + ".png").expect(&format!("Could not write to path: {}", path));

        godot::binary_api::BinarySerializer::value_to_file(&UnpackedValue::Map(tileset_data), &(path.clone() + ".tls")).expect("Could not write to GODOT file!");
        
        println!("Tileset extraction successful!");
        Some(&self.image_data)

    }

    /// Returns a 2D Vector of the tile data
    fn read_object_data(&self) -> UnpackedValue {
        let mut unpacked_rows: Vec<UnpackedValue> = vec![];
        let mut current_row: Vec<UnpackedValue> = vec![];
        let mut offset = 0;
        let objects = &self.objects;
        
        while offset < objects.len() {
            let mut object_data: HashMap<String, UnpackedValue> = HashMap::new();
            
            if let Some(&first_byte) = objects.get(offset) {
                object_data.insert("scale_behavior".to_string(), UnpackedValue::UInt16(first_byte as u16));
                offset += 1;
                
                // if first_byte >= 144 {
                //     if let Some(&extra_byte) = objects.get(offset) {
                //         object_data.insert("extra".to_string(), UnpackedValue::UInt16(extra_byte as u16));
                //         offset += 1;
                //     }
                // }
            }
            
            if let Some(&second_byte) = objects.get(offset) {
                object_data.insert("index".to_string(), UnpackedValue::UInt16(second_byte as u16));
                offset += 1;
            }
            
            if let Some(&third_byte) = objects.get(offset) {
                object_data.insert("object_type".to_string(), UnpackedValue::UInt16(third_byte as u16));
                offset += 1;
            }
            
            current_row.push(UnpackedValue::Map(object_data));
            
            if let Some(&next_byte) = objects.get(offset) {
                if next_byte == 254 {
                    unpacked_rows.push(UnpackedValue::Vec(current_row));
                    current_row = vec![];
                    offset += 1;
                } else if next_byte == 255 {
                    unpacked_rows.push(UnpackedValue::Vec(current_row));
                    break;
                }
            }
        }
        UnpackedValue::Vec(unpacked_rows)
    }

    fn fetch_tileset_name(&self, bg_tex_files: Vec<String>) -> Option<String> {
        for tex_file in bg_tex_files {
            if tex_file.starts_with("Pa") {
                if let Some(name) = tex_file.strip_suffix("_tex.bin.LZ").or_else(|| tex_file.strip_suffix("_tex.bin")) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
}
