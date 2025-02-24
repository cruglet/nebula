// DEPRECATED
// Translated from Dolphin Emu Source code

use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

const WII_SECTOR_SIZE: u64 = 0x8000;  // size of each sector on a Wii disc ~32KB
const WII_TOTAL_SECTORS: u64 = 143432 * 2;  // total number of sectors for Wii disc
const WII_DISC_HEADER_SIZE: u64 = 256;  // size of the Wii disc header
const WBFS_MAGIC: u32 = 0x53464257;  // "WBFS" in ASCII (little-endian)

const BUFFER_SIZE: usize = 0x400000;  // 4MB buffer for reading/writing
const GAME_ID_LENGTH: usize = 6;  // length of Wii game ID
const GAME_NAME_OFFSET: usize = 0x20;  // offset of game name in header
const GAME_NAME_LENGTH: usize = 64;  // length of game name in header

pub type ProgressCallback = fn(f32);

#[derive(Debug)]
struct GameInfo {
    id: String,
    name: String,
    size: u64,
}


/// Represents a file entry within the WBFS archive
struct WbfsFileEntry {
    file_handle: File,
    starting_address: u64,
    total_size: u64,
}

/// Main structure for handling WBFS files
struct WbfsReader {
    // Collection of file parts (WBFS can span multiple files)
    file_entries: Vec<WbfsFileEntry>,
    // Total size of all WBFS parts combined
    total_archive_size: u64,
    // Size of each hardware sector (usually 512 bytes)
    hardware_sector_size: u64,
    // Size of each WBFS sector (usually larger than hardware sectors)
    wbfs_sector_size: u64,
    // Total number of WBFS sectors
    total_wbfs_sectors: u64,
    // Size of disc information block
    disc_info_block_size: u64,
    // Table mapping logical block addresses
    block_table: Vec<u16>,
    // Number of blocks per disc
    blocks_per_disc: u64,
}

#[repr(C, packed)]
struct WbfsHeader {
    magic_number: u32,
    total_hw_sectors: u32,
    hw_sector_size_shift: u8,
    wbfs_sector_size_shift: u8,
    padding: [u8; 2],
    disc_table: [u8; 500],
}

impl WbfsReader {
    /// Attempts to create a new WBFS reader from a file path
    fn try_new(primary_file_path: &Path) -> io::Result<Self> {
        let primary_file = File::open(primary_file_path)?;
        let mut reader = WbfsReader {
            file_entries: Vec::new(),
            total_archive_size: 0,
            hardware_sector_size: 0,
            wbfs_sector_size: 0,
            total_wbfs_sectors: 0,
            disc_info_block_size: 0,
            block_table: Vec::new(),
            blocks_per_disc: 0,
        };

        reader.add_file(primary_file)?;
        reader.find_and_add_split_files(primary_file_path)?;
        reader.read_and_validate_header()?;

        println!("WBFS Info:");
        println!("Total archive size: {}", reader.total_archive_size);
        println!("Hardware sector size: {}", reader.hardware_sector_size);
        println!("WBFS sector size: {}", reader.wbfs_sector_size);
        println!("Total WBFS sectors: {}", reader.total_wbfs_sectors);
        println!("Blocks per disc: {}", reader.blocks_per_disc);

        Ok(reader)
    }


    /// Adds a file to the WBFS reader's file list
    fn add_file(&mut self, file: File) -> io::Result<()> {
        let file_size = file.metadata()?.len();
        self.file_entries.push(WbfsFileEntry {
            file_handle: file,
            starting_address: self.total_archive_size,
            total_size: file_size,
        });
        self.total_archive_size += file_size;
        Ok(())
    }

    /// Finds and adds any split WBFS files (*.wbf1, *.wbf2, etc.)
    fn find_and_add_split_files(&mut self, primary_path: &Path) -> io::Result<()> {
        if let Some(base_path) = primary_path.to_str() {
            if base_path.len() >= 4 {
                for index in 1..10 {
                    let split_path = format!("{}{}", &base_path[..base_path.len() - 1], index);
                    if let Ok(split_file) = File::open(&split_path) {
                        self.add_file(split_file)?;
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    /// Reads and validates the WBFS header
    fn read_and_validate_header(&mut self) -> io::Result<()> {
        let mut header_bytes = [0u8; std::mem::size_of::<WbfsHeader>()];
        self.file_entries[0].file_handle.seek(SeekFrom::Start(0))?;
        self.file_entries[0].file_handle.read_exact(&mut header_bytes)?;

        // Parse header (assuming little-endian)
        let header = unsafe { std::ptr::read_unaligned(header_bytes.as_ptr() as *const WbfsHeader) };

        // Validate magic number
        if header.magic_number != WBFS_MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid WBFS magic number"));
        }

        // Calculate sizes
        self.hardware_sector_size = 1u64 << header.hw_sector_size_shift;
        self.wbfs_sector_size = 1u64 << header.wbfs_sector_size_shift;
        self.total_wbfs_sectors = self.total_archive_size / self.wbfs_sector_size;

        // Validate sector size
        if self.wbfs_sector_size < WII_SECTOR_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid WBFS sector size"));
        }

        // Calculate blocks
        self.blocks_per_disc = (WII_TOTAL_SECTORS * WII_SECTOR_SIZE + self.wbfs_sector_size - 1) 
            / self.wbfs_sector_size;

        // Read block table
        self.read_block_table()?;

        Ok(())
    }

    /// Reads the block allocation table
    fn read_block_table(&mut self) -> io::Result<()> {
        // Calculate the block table offset - it should be right after the disc table
        let block_table_offset = 0x100;  // WBFS header size is 0x100
        
        self.file_entries[0].file_handle.seek(SeekFrom::Start(block_table_offset))?;
        
        // Read block table
        self.block_table = vec![0u16; self.blocks_per_disc as usize];
        let mut bytes = vec![0u8; self.blocks_per_disc as usize * 2];
        self.file_entries[0].file_handle.read_exact(&mut bytes)?;
        
        // Convert bytes to u16 (big-endian, as WBFS uses big-endian)
        for i in 0..self.blocks_per_disc as usize {
            self.block_table[i] = u16::from_be_bytes([bytes[i*2], bytes[i*2+1]]);
        }

        Ok(())
    }

    /// Reads data from the WBFS file at the specified offset
    fn read_at(&mut self, offset: u64, buffer: &mut [u8]) -> io::Result<usize> {
        if offset >= WII_TOTAL_SECTORS * WII_SECTOR_SIZE {
            return Ok(0);
        }
    
        let wbfs_sector = offset / self.wbfs_sector_size;
        let sector_offset = offset % self.wbfs_sector_size;
    
        if wbfs_sector >= self.blocks_per_disc {
            return Ok(0);
        }


    
        let block = self.block_table.get(wbfs_sector as usize)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Block table out of bounds"))?;
    
        let mut available = std::cmp::min(
            self.wbfs_sector_size - sector_offset,
            buffer.len() as u64
        ) as usize;
    
        if *block == 0 {
            buffer[..available].fill(0);
            return Ok(available);
        }
    
        let absolute_offset = (*block as u64 * self.wbfs_sector_size) + sector_offset;
    
        // Validate absolute_offset
        if absolute_offset >= self.total_archive_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid absolute offset {:#X}", absolute_offset),
            ));
        }
    
        for entry in &mut self.file_entries {
            if absolute_offset >= entry.starting_address
                && absolute_offset < entry.starting_address + entry.total_size {
                let relative_offset = absolute_offset - entry.starting_address;
                entry.file_handle.seek(SeekFrom::Start(relative_offset))?;
    
                available = std::cmp::min(
                    available,
                    (entry.total_size - relative_offset) as usize
                );
                return entry.file_handle.read(&mut buffer[..available]);
            }
        }
    
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Offset {:#X} not within any file entry", absolute_offset),
        ))
    }
    

    /// Reads the game information from the WBFS header
    fn read_game_info(&mut self) -> io::Result<GameInfo> {
        let mut header_data = vec![0u8; WII_DISC_HEADER_SIZE as usize];
        self.read_at(0, &mut header_data)?;

        // Read game ID (should be ASCII/UTF-8 compatible)
        let game_id = match std::str::from_utf8(&header_data[0..GAME_ID_LENGTH]) {
            Ok(id) => id.trim_end_matches('\0').to_string(),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid game ID encoding"))
        };

        // Read game name (handling potential null bytes and shift-jis encoding)
        let name_bytes = &header_data[GAME_NAME_OFFSET..GAME_NAME_OFFSET + GAME_NAME_LENGTH];
        
        // Find the real end of the string (first null byte or end of buffer)
        let name_end = name_bytes.iter()
            .position(|&x| x == 0)
            .unwrap_or(name_bytes.len());

        // Convert the name bytes to a string, replacing invalid UTF-8 sequences
        let game_name = String::from_utf8_lossy(&name_bytes[..name_end])
            .trim()
            .to_string();

        // If we got an empty name, use a default with the game ID
        let game_name = if game_name.is_empty() {
            format!("Unknown Game ({})", game_id)
        } else {
            game_name
        };

        Ok(GameInfo {
            id: game_id,
            name: game_name,
            size: WII_TOTAL_SECTORS * WII_SECTOR_SIZE,
        })
    }

    /// Extracts the WBFS content to a file
    fn extract_to_file(&mut self, output_path: &Path, progress_callback: ProgressCallback) -> io::Result<()> {
        let game_info = self.read_game_info()?;
        println!("Extracting game: {} ({})", game_info.name, game_info.id);

        // Create output file
        let mut output_file = File::create(output_path)?;
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let mut bytes_written = 0u64;
        let total_size = game_info.size;

        // Extract data in chunks
        while bytes_written < total_size {
            let to_read = std::cmp::min(
                BUFFER_SIZE as u64,
                total_size - bytes_written
            ) as usize;

            let read = self.read_at(bytes_written, &mut buffer[..to_read])?;
            if read == 0 {
                if bytes_written < total_size {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Incomplete read during extraction"
                    ));
                }
                break;
            }

            output_file.write_all(&buffer[..read])?;
            bytes_written += read as u64;

            // Report progress
            let progress = (bytes_written as f32 / total_size as f32) * 100.0;
            progress_callback(progress);
        }

        // Verify final size
        if bytes_written != total_size {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("Expected {} bytes but got {}", total_size, bytes_written)
            ));
        }

        Ok(())
    }
}


/// Extracts a WBFS file to the specified output directory
/// Returns the extracted game's information on success
pub fn extract_wbfs(
    input_path: &Path,
    output_dir: &Path,
    progress_callback: Option<ProgressCallback>
) -> io::Result<(String, String)> {
    let mut reader = WbfsReader::try_new(input_path)?;
    let game_info = reader.read_game_info()?;
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Create output path with game ID and name
    let output_filename = format!("{}.iso", game_info.id);
    let output_path = output_dir.join(output_filename);

    // Use provided progress callback or default to printing
    let callback = progress_callback.unwrap_or(|progress| {
        println!("Extraction progress: {:.1}%", progress);
    });

    // Extract the content
    reader.extract_to_file(&output_path, callback)?;

    Ok((game_info.id, game_info.name))
}


pub fn is_valid_wbfs(path: &Path) -> io::Result<bool> {
    WbfsReader::try_new(path).map(|_| true)
}
