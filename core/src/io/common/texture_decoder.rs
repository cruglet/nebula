use godot::{
    classes::{Image, ImageTexture},
    prelude::*,
};

use crate::io::buffer::NebulaBuffer;

/// Handles decoding of custom texture formats.
///
/// `TextureDecoder` provides methods for converting proprietary or compressed
/// texture formats into standard Godot Image and ImageTexture objects.
#[derive(GodotClass)]
#[class(base=Object)]
pub struct TextureDecoder {
    base: Base<Object>,
}

#[godot_api]
impl IObject for TextureDecoder {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl TextureDecoder {
    #[func]
    pub fn rgb4a3_decode(mut tex: Gd<NebulaBuffer>, #[opt(default = true)] use_alpha: bool) -> Gd<ImageTexture> {
        let luts = Self::prepare_rgb4a3_luts();
        let lut = if use_alpha { &luts.0 } else { &luts.1 };
        let lut_size = lut.len();
        
        let mut tex_buffer = tex.bind_mut();
        let tex_size = tex_buffer.size();
        
        let width = 1024;
        let height = 256;
        let mut img_data = vec![0u8; width * height * 4];
        let mut tex_index = 0;
        
        for block_y in 0..64 {
            let base_y = block_y * 4;
            
            for block_x in 0..256 {
                let base_x = block_x * 4;
                
                let temp1 = block_x % 8;
                let temp2 = block_y % 8;
                let should_skip = (temp1 == 0 || temp1 == 7) || (temp2 == 0 || temp2 == 7);
                
                if should_skip {
                    tex_index += 32;
                } else {
                    for y in 0..4 {
                        let pixel_y = base_y + y;
                        let row_offset = pixel_y * width * 4;
                        
                        for x in 0..4 {
                            if tex_index + 1 < tex_size {
                                tex_buffer.goto(tex_index as i32);
                                let byte1 = tex_buffer.read_u8();
                                let byte2 = tex_buffer.read_u8();
                                let pixel_value = ((byte1 as usize) << 8) | (byte2 as usize);
                                tex_index += 2;
                                
                                if pixel_value < lut_size {
                                    let pixel = lut[pixel_value];
                                    let byte_offset = row_offset + (base_x + x) * 4;
                                    
                                    img_data[byte_offset] = ((pixel >> 16) & 0xFF) as u8; // R
                                    img_data[byte_offset + 1] = ((pixel >> 8) & 0xFF) as u8; // G
                                    img_data[byte_offset + 2] = (pixel & 0xFF) as u8; // B
                                    img_data[byte_offset + 3] = ((pixel >> 24) & 0xFF) as u8; // A
                                }
                            } else {
                                tex_index += 2;
                            }
                        }
                    }
                }
            }
        }
        
        let img_data_packed = PackedByteArray::from(img_data.as_slice());
        let image = Image::create_from_data(
            width as i32,
            height as i32,
            false,
            godot::classes::image::Format::RGBA8,
            &img_data_packed,
        ).expect("Failed to create image");
        
        let mut texture = ImageTexture::new_gd();
        texture.set_image(&image);
        texture
    }

    fn prepare_rgb4a3_luts() -> (Vec<u32>, Vec<u32>) {
        let mut lut_alpha = vec![0u32; 65536];
        let mut lut_no_alpha = vec![0u32; 65536];
        
        // RGB4A3 format (0x0000 - 0x7FFF)
        for d in 0..0x8000 {
            let alpha_raw = (d >> 12) & 0xF;
            let alpha = ((alpha_raw << 5) | (alpha_raw << 2) | (alpha_raw >> 1)) as u32;
            let red = (((d >> 8) & 0xF) * 17) as u32;
            let green = (((d >> 4) & 0xF) * 17) as u32;
            let blue = ((d & 0xF) * 17) as u32;
            
            let pixel_alpha = (alpha << 24) | (red << 16) | (green << 8) | blue;
            let pixel_no_alpha = 0xFF000000 | (red << 16) | (green << 8) | blue;
            
            lut_alpha[d] = pixel_alpha;
            lut_no_alpha[d] = pixel_no_alpha;
        }
        
        // RGB555 format (0x8000 - 0xFFFF)
        for d in 0..0x8000 {
            let d_shifted = d + 0x8000;
            
            let red_5bit = d >> 10;
            let green_5bit = (d >> 5) & 0x1F;
            let blue_5bit = d & 0x1F;
            
            let red = ((red_5bit << 3) | (red_5bit >> 2)) as u32;
            let green = ((green_5bit << 3) | (green_5bit >> 2)) as u32;
            let blue = ((blue_5bit << 3) | (blue_5bit >> 2)) as u32;
            
            let pixel = 0xFF000000 | (red << 16) | (green << 8) | blue;
            
            lut_alpha[d_shifted] = pixel;
            lut_no_alpha[d_shifted] = pixel;
        }
        
        (lut_alpha, lut_no_alpha)
    }
}