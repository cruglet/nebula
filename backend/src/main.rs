use std::env;
use std::io::Result;

pub mod wii;
pub mod utils;
pub mod neb;

use neb::nsmbw;

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
            let _ = nsmbw::dump_level(input_path.to_owned(), output_path.to_owned());
        }

        // if command == "--read" {
        //   let level = neb::nsmbw::read_level(input_path.to_string(), output_path.to_owned());
        //   neb::godot::value_to_file(&level, "../test/test.bin");
        // }
    } else {
        eprintln!("Unknown command or module. Usage: {} <module> --dump <path-to-file.arc> <output-directory>", args[0]);
        std::process::exit(1);
    }

    Ok(())
}
