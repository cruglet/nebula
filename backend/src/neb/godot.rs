use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::vec;
use byteorder::{LittleEndian, WriteBytesExt};
use crate::utils::byte_reader::{self, UnpackedValue};


// Godot variant types
const TYPE_NULL: u32 = 0;
const TYPE_BOOL: u32 = 1;
const TYPE_INT: u32 = 2;
const TYPE_STRING: u32 = 4;
const TYPE_DICTIONARY: u32 = 27;
const TYPE_ARRAY: u32 = 28;

pub fn value_to_bytes(value: &UnpackedValue) -> Vec<u8> {
    let mut buffer = Vec::new();
    write_variant(&mut buffer, value);
    buffer
}

fn write_variant(buffer: &mut Vec<u8>, value: &UnpackedValue) {
    match value {
        UnpackedValue::UInt8(n) => {
            buffer.write_u32::<LittleEndian>(TYPE_INT).unwrap();
            buffer.write_i32::<LittleEndian>(*n as i32).unwrap();
        }
        UnpackedValue::UInt16(n) => {
            buffer.write_u32::<LittleEndian>(TYPE_INT).unwrap();
            buffer.write_i32::<LittleEndian>(*n as i32).unwrap();
        }
        UnpackedValue::UInt32(n) => {
            buffer.write_u32::<LittleEndian>(TYPE_INT).unwrap();
            buffer.write_i32::<LittleEndian>(*n as i32).unwrap();
        }
        UnpackedValue::Int8(n) => {
            buffer.write_u32::<LittleEndian>(TYPE_INT).unwrap();
            buffer.write_i32::<LittleEndian>(*n as i32).unwrap();
        }
        UnpackedValue::Int16(n) => {
            buffer.write_u32::<LittleEndian>(TYPE_INT).unwrap();
            buffer.write_i32::<LittleEndian>(*n as i32).unwrap();
        }
        UnpackedValue::Int32(n) => {
            buffer.write_u32::<LittleEndian>(TYPE_INT).unwrap();
            buffer.write_i32::<LittleEndian>(*n).unwrap();
        }
        UnpackedValue::Boolean(b) => {
            buffer.write_u32::<LittleEndian>(TYPE_BOOL).unwrap();
            buffer.write_i32::<LittleEndian>(*b as i32).unwrap();
        }
        UnpackedValue::String(s) => {
            buffer.write_u32::<LittleEndian>(TYPE_STRING).unwrap();
            let bytes = s.as_bytes();
            buffer.write_u32::<LittleEndian>(bytes.len() as u32).unwrap();
            buffer.extend_from_slice(bytes);
            // Pad to 4 bytes alignment
            let padding = (4 - (bytes.len() % 4)) % 4;
            buffer.extend(std::iter::repeat(0).take(padding));
        }
        UnpackedValue::Vec(vec) => {
            buffer.write_u32::<LittleEndian>(TYPE_ARRAY).unwrap(); // Using dictionary type as per hexdump
            buffer.write_u32::<LittleEndian>(vec.len() as u32).unwrap();
            
            for (i, item) in vec.iter().enumerate() {
                write_variant(buffer, item);
            }
        }
        UnpackedValue::Map(map) => {
            buffer.write_u32::<LittleEndian>(TYPE_DICTIONARY).unwrap();
            buffer.write_u32::<LittleEndian>(map.len() as u32).unwrap();
            
            for (i, (key, value)) in map.iter().enumerate() {
                // Write key as string
                write_variant(buffer, &UnpackedValue::String(key.clone()));
                // Write value
                write_variant(buffer, value);
            }
        }
    }
}

pub fn value_to_file(value: &UnpackedValue, path: &str) -> std::io::Result<()> {
    let mut bytes: Vec<u8> = vec![];
    bytes.extend_from_slice(&calculate_variant_size(value).to_le_bytes());
    bytes.extend(value_to_bytes(value));
    let result = std::fs::write(path, bytes);
    match &result {
        Ok(_) => println!("Successfully wrote file: {}", path),
        Err(e) => println!("Failed to write file: {}, error: {}", path, e),
    }
    result
}

fn calculate_variant_size(value: &UnpackedValue) -> u32 {
    match value {
        UnpackedValue::UInt8(_) |
        UnpackedValue::UInt16(_) |
        UnpackedValue::UInt32(_) |
        UnpackedValue::Int8(_) |
        UnpackedValue::Int16(_) |
        UnpackedValue::Int32(_) |
        UnpackedValue::Boolean(_) => 8, // 4 bytes for type + 4 bytes for value
        
        UnpackedValue::String(s) => {
            let bytes = s.as_bytes();
            let padding = (4 - (bytes.len() % 4)) % 4;
            8 + bytes.len() as u32 + padding as u32 // 4 for type + 4 for length + string + padding
        }
        
        UnpackedValue::Vec(vec) => {
            let mut size = 8; // 4 for type + 4 for length
            for item in vec {
                size += calculate_variant_size(item);
            }
            size
        }
        
        UnpackedValue::Map(map) => {
            let mut size = 8; // 4 for type + 4 for length
            for (key, value) in map {
                size += calculate_variant_size(&UnpackedValue::String(key.clone()));
                size += calculate_variant_size(value);
            }
            size
        }
    }
}