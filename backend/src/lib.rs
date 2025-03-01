use std::env;
use std::process;
use std::io::Result;

pub mod wii;
pub mod utils;
pub mod neb;

use neb::{nebula, nsmbw};

const DEFAULT_OUTPUT_DIR: &str = "output";

fn main() -> Result<()> {
    let args = parse_arguments()?;
    match args.module.as_str() {
        "nsmbw" => handle_nsmbw_commands(&args),
        _ => {
            eprintln!("Unknown module: '{}'.", args.module);
            print_usage(&args.program_name);
            process::exit(1);
        }
    }
}

struct Args {
    program_name: String,
    module: String,
    command: String,
    input_path: String,
    output_path: String,
}

fn parse_arguments() -> Result<Args> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Error: Insufficient arguments.");
        print_usage(&args[0]);
        process::exit(1);
    }

    Ok(Args {
        program_name: args[0].clone(),
        module: args[1].clone(),
        command: args[2].clone(),
        input_path: args[3].clone(),
        output_path: if args.len() > 4 { args[4].clone() } else { DEFAULT_OUTPUT_DIR.to_string() },
    })
}

fn handle_nsmbw_commands(args: &Args) -> Result<()> {
    match args.command.as_str() {
        "--dump-level" => dump_level(args),
        "--dump-tileset" => dump_tileset(args),

        _ => {
            eprintln!("Unknown command '{}'.", args.command);
            print_usage(&args.program_name);
            process::exit(1);
        }
    }
}

fn dump_level(args: &Args) -> Result<()> {
    let mut level = nsmbw::level::new();
    level.open_archive(args.input_path.clone());
    nebula::binary_api::BinarySerializer::value_to_file(&level.unpacked_buffer, &args.output_path)?;
    Ok(())
}

fn dump_tileset(args: &Args) -> Result<()> {
    let mut tileset = nsmbw::tileset::new();
    tileset.load(args.input_path.clone());
    tileset.extract(args.output_path.clone());
    Ok(())
}

fn print_usage(program_name: &str) {
    eprintln!(
        "Usage: {} <module> <command> <path-to-file.arc> [output-directory]",
        program_name
    );
    eprintln!("\nAvailable Modules:");
    eprintln!("  nsmbw");
    eprintln!("\nAvailable Commands for nsmbw:");
    eprintln!("  --dump-level     Dump level data from an archive");
    eprintln!("  --dump-tileset   Extract tileset data from an archive");
}
