extern crate image;

// Sourced form NSMBLib-Updated
// https://github.com/NSMBW-Community/NSMBLib-Updated/tree/master

use image::{ImageBuffer, Rgba};
use std::vec::Vec;

fn prepare_rgb4a3_luts() -> (Vec<u32>, Vec<u32>) {
    let mut rgb4a3lut = vec![0u32; 0x10000];
    let rgb4a3lut_no_alpha = vec![0u32; 0x10000];

    // RGB4A3
    for d in 0..0x8000 {
        let (mut alpha, mut red, mut green, mut blue);
        if true { // Use alpha
            alpha = d >> 12;
            alpha = alpha << 5 | alpha << 2 | alpha >> 1;
        } else {
            alpha = 0xFF;
        }
        red = ((d >> 8) & 0xF) * 17;
        green = ((d >> 4) & 0xF) * 17;
        blue = (d & 0xF) * 17;
        rgb4a3lut[d as usize] = blue | (green << 8) | (red << 16) | (alpha << 24);
    }

    // RGB555
    for d in 0..0x8000 {
        let red = (d >> 10) << 3 | (d >> 2) & 0x7;
        let green = ((d >> 5) & 0x1F) << 3 | ((d >> 5) & 0x1F) >> 2;
        let blue = (d & 0x1F) << 3 | (d & 0x1F) >> 2;
        rgb4a3lut[(d + 0x8000) as usize] = blue | (green << 8) | (red << 16) | 0xFF000000;
    }

    (rgb4a3lut, rgb4a3lut_no_alpha)
}


pub fn rgb4a3_decode(tex: &[u8], use_alpha: bool) -> image::ImageResult<()> {
    let (rgb4a3lut, rgb4a3lut_no_alpha) = prepare_rgb4a3_luts();
    let lut = if use_alpha { &rgb4a3lut } else { &rgb4a3lut_no_alpha };

    let mut tx = 0;
    let mut ty = 0;
    let mut iter = tex.iter();
    let mut dest = vec![0u32; 262144]; // 1024 * 256 = 262144 pixels

    // Loop over all texels (16384 texels total)
    for i in 0..16384 {
        let temp1 = (i / 256) % 8;
        if temp1 == 0 || temp1 == 7 {
            // Skip rows
            for _ in 0..32 {
                iter.next(); // Skip texels
            }
        } else {
            let temp2 = i % 8;
            if temp2 == 0 || temp2 == 7 {
                // Skip columns
                for _ in 0..32 {
                    iter.next(); // Skip texels
                }
            } else {
                // Render the texel
                for y in ty..ty + 4 {
                    for x in tx..tx + 4 {
                        if let Some(val1) = iter.next() {
                            if let Some(val2) = iter.next() {
                                let pixel_value = ((*val1 as u32) << 8) | (*val2 as u32);
                                if let Some(color) = lut.get(pixel_value as usize) {
                                    dest[(x + y * 1024) as usize] = *color;
                                } else {
                                    eprintln!("Warning: Invalid LUT index: {}", pixel_value);
                                }
                            } else {
                                eprintln!("Warning: Not enough data for texel at ({}, {})", x, y);
                            }
                        } else {
                            eprintln!("Warning: Not enough data for texel at ({}, {})", x, y);
                        }
                    }
                }
            }
        }

        // Move on to the next texel
        tx += 4;
        if tx >= 1024 {
            tx = 0;
            ty += 4;
        }
    }

    // Convert the dest vector into an ImageBuffer
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(1024, 256);
    for (i, &pixel) in dest.iter().enumerate() {
        let a = ((pixel >> 24) & 0xFF) as u8;
        let r = ((pixel >> 16) & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = (pixel & 0xFF) as u8;
        let x = (i % 1024) as u32;
        let y = (i / 1024) as u32;
        img.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    // Save the image as PNG
    img.save("../test/t.png")
}

