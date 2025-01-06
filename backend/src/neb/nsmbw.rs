// Sourced From Reggie-Updated
// https://github.com/NSMBW-Community/Reggie-Updated/tree/fa12de16ea8df33068ae93ec4616f8e67dbc05ca
use std::fs;
use std::io::{self, Read, Result};
use crate::wii::arc::U8;

pub fn is_nsmbw_level(filename: &str) -> io::Result<bool> {
    // Check if file exists
    if fs::metadata(filename).is_err() {
        return Ok(false);
    }

    // Read the file
    let mut file = fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Check for LZ-compressed file signature
    if data.starts_with(&[0x11]) {
        return Ok(true);
    }

    // Check for U8 data signature
    if data.starts_with(b"U\xAA8-") {
        // Perform additional sanity checks using `windows` to check for slices
        if !data.windows(b"course\0".len()).any(|window| window == b"course\0") &&
           !data.windows(b"course1.bin\0".len()).any(|window| window == b"course1.bin\0") &&
           !data.windows(b"\0\0\0\x80".len()).any(|window| window == b"\0\0\0\x80") {
            return Ok(false);
        }
        return Ok(true);
    }

    // Fallback for non-matching files
    Ok(false)
}

pub fn dump_level(archive_path: String, to: String) -> Result<()> {
    let mut archive = U8::new();

    // Read the archive file
    let data = match fs::read(&archive_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to read archive '{}': {}", archive_path, e);
            return Err(e);
        }
    };

    // Load the archive
    if let Err(e) = archive.load(&data) {
        eprintln!("Failed to load archive '{}': {}", archive_path, e);
        return Err(e);
    }

    // Dump the directory
    if let Err(e) = archive.dump_dir(&to) {
        eprintln!("Failed to dump archive to '{}': {}", to, e);
        return Err(e);
    }

    // List files in the archive
    println!("Files in archive:");
    for (path, data) in &archive.files {
        match data {
            Some(content) => println!("File: {} (size: {} bytes)", path, content.len()),
            None => println!("Directory: {}", path),
        }
    }

    // Indicate success
    println!(
        "Successfully dumped the level from '{}' to '{}'",
        archive_path, to
    );

    Ok(())
}

