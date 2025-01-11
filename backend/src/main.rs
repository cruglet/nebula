use std::env;
use std::io::Result;

pub mod wii;
pub mod utils;
pub mod neb;

fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least 4 arguments are passed
    if args.len() < 4 {
        eprintln!("Usage: {} <module> --dump <path-to-file.arc> <output-directory>", args[0]);
        std::process::exit(1);
    }

    // Extract arguments
    let module = &args[1];
    let command = &args[2];
    let input_path = &args[3];
    let output_path = if args.len() > 4 {
        &args[4]
    } else {
        "output" // Default output directory if none is provided
    };

    // Validate command
    if module == "nsmbw" {

        if command == "--dump" {
            neb::nsmbw::dump_level(input_path.to_string(), output_path.to_string())?;
            println!("Dumping completed successfully.");
        }

        if command == "--read" {
            neb::nsmbw::read_level(input_path.to_string(), output_path.to_owned());
        }
// 

    } else {
        eprintln!("Unknown command or module. Usage: {} <module> --dump <path-to-file.arc> <output-directory>", args[0]);
        std::process::exit(1);
    }

    Ok(())
}
