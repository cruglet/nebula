use std::fs;

use crate::utils::image_serializer;
use crate::wii::arc::U8;
use crate::wii::lzss;


// Translated from Puzzle-Next
// https://github.com/Developers-Collective/Puzzle-Next


pub struct Tileset {
    pub image_data: Vec<u8>
}

pub fn new() -> Tileset {
    Tileset {
        image_data: vec![],
    }
}

impl Tileset {
    pub fn open_archive(&mut self, archive_path: String) {
        let mut tileset_archive = U8::new();
    
        let tileset_archive_data = fs::read(&archive_path)
            .expect("Could not read archive data!");
    
        tileset_archive
            .load(&tileset_archive_data)
            .expect("Could not load archive data!");
    
        if let Some(bg_tex_files) = tileset_archive.get_dir("BG_tex") {

            if let Some(texture_name) = bg_tex_files.first() {

                let texture_path = format!("BG_tex/{}", texture_name);
                if let Some(compressed_texture) = tileset_archive.get(&texture_path) {

                    if let Some(decompressed_texture) = lzss::decompress_raw(compressed_texture.to_vec()) {
                        self.image_data = decompressed_texture;
                    }

                }
            }
        }
    }

    pub fn extract(&self, path: String) -> Option<&Vec<u8>> {
        if self.image_data.len() == 0 { return None }

        let image = image_serializer::rgb4a3_decode(&self.image_data, true);
        let image = image_serializer::minify_image(image);

        if image.is_empty() { return None }

        image.save(path.clone()).expect(&format!("Could not write to path: {}", path));
        println!("Tileset extraction successful!");

        Some(&self.image_data)
    }
}
