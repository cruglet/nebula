use image::{ImageBuffer, ImageResult, Rgba};

use std::fs;
use std::io::Write;

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

        let tileset_data = fs::read(archive_path)
            .expect("Could not read file...");
        tileset_archive.load(&tileset_data)
            .expect("Could not load data...");

        println!("{:?}", tileset_archive.get_dir("BG_tex"));
        let texture = tileset_archive.get("BG_tex/real_tileset_tex.bin.LZ").unwrap();

        let x: Vec<u8> = lz11::decompress(texture).unwrap();

        let mut decompressed_file = fs::File::create("../test/dec_file.bin").unwrap();
        let _ = decompressed_file.write(&x);

        image_serializer::rgb4a3_decode(&x, true).expect("err");
    }

    pub fn decode(texture: &[u8], use_alpha: bool, premultiply: bool) -> Vec<u8> {
        let mut decoded = vec![0u8; 1048576];
        let output = unsafe { 
            std::slice::from_raw_parts_mut(
                decoded.as_mut_ptr() as *mut u32, 
                decoded.len() / 4
            )
        };
    
        let alpha_ormask = if use_alpha { 0 } else { 7 };
        let mut pointer = 0;
        let mut tx = 0;
        let mut ty = 0;
    
        for _ in 0..16384 {
            for y in ty..ty+4 {
                let sourcey = y << 10;
                for x in tx..tx+4 {
                    // Bounds check added here
                    if pointer + 1 >= texture.len() {
                        break;
                    }
    
                    let pos = sourcey | x;
                    let a = texture[pointer];
                    let b = texture[pointer + 1];
                    pointer += 2;
                    let ab = (a as u16) << 8 | b as u16;
    
                    if ab & 0x8000 == 0 {
                        // RGB4A3 format
                        let alpha = (ab >> 12) | alpha_ormask;
                        let alpha255 = (alpha << 5) | (alpha << 2) | (alpha >> 1);
                        let red = (ab >> 8) & 0xF;
                        let green = (ab >> 4) & 0xF;
                        let blue = ab & 0xF;
    
                        let mut pixel = (alpha255 as u32) << 24 | 
                            ((red as u32) << 20) | ((red as u32) << 16) | 
                            ((green as u32) << 12) | ((green as u32) << 8) | 
                            ((blue as u32) << 4) | blue as u32;
    
                        if premultiply {
                            let al = pixel >> 24;
                            let mut t = ((pixel & 0xFF00FF) * al + 0x800080) >> 8;
                            t &= 0xFF00FF;
                            let x = (((pixel >> 8) & 0xFF) * al + 0x80) & 0xFF00;
                            pixel = x | t | (al << 24);
                        }
    
                        output[pos] = pixel;
                    } else {
                        // RGB555 format
                        let red = ((ab >> 10) & 0x1F) as u8;
                        let green = ((ab >> 5) & 0x1F) as u8;
                        let blue = (ab & 0x1F) as u8;
    
                        let red8 = (red << 3) | (red >> 2);
                        let green8 = (green << 3) | (green >> 2);
                        let blue8 = (blue << 3) | (blue >> 2);
    
                        output[pos] = 0xFF000000 | 
                            ((red8 as u32) << 16) | 
                            ((green8 as u32) << 8) | 
                            blue8 as u32;
                    }
                }
            }
    
            tx += 4;
            if tx >= 1024 {
                tx = 0;
                ty += 4;
            }
        }
    
        decoded
    }

}
