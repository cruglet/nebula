use std::{env, fs};
use std::io::{Read, Result, Write};

pub mod wii;
pub mod utils;
pub mod neb;

use neb::{godot, nsmbw};

fn main() -> Result<()> {
    // Fetch command line arguments
    let args: Vec<String> = env::args().collect();

    // 4 args must be passed
    if args.len() < 4 {
        eprintln!("Usage: {} <module> --dump <path-to-file.arc> <path-to-output-file>", args[0]);
        std::process::exit(1);
    }

    // Map arguments
    let module = &args[1];
    let command = &args[2];
    let input_path = &args[3];
    let output_path = if args.len() > 4 {
        &args[4]
    } else {
        "output" // Fallback output dir
    };

    if module == "nsmbw" {

        if command == "--dump" {
            let mut level = nsmbw::level::new();
            level.open_archive(input_path.to_string());
            godot::binary_api::BinarySerializer::value_to_file(&level.unpacked_buffer, output_path)?;

            let mut tileset = nsmbw::tileset::new();
            tileset.open_archive("../test/real_tileset.arc".to_owned());
        }

    } else {
        eprintln!("Unknown command or module. Usage: {} <module> --dump <path-to-file.arc> <output-directory>", args[0]);
        std::process::exit(1);
    }

    Ok(())
}
