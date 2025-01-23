use std::env;
use std::io::Result;

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
            let mut level = nsmbw::Level::new();
            level.open_archive(input_path.to_string());
            godot::BinarySerializer::value_to_file(&level.unpacked_buffer, output_path)?;
        }

    } else {
        eprintln!("Unknown command or module. Usage: {} <module> --dump <path-to-file.arc> <output-directory>", args[0]);
        std::process::exit(1);
    }

    Ok(())
}
