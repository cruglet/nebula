use std::{fs, vec};
use byteorder::{LittleEndian, WriteBytesExt};
use crate::utils::byte_serializer::UnpackedValue;


// Godot variant types
// const TYPE_NULL: u32 = 0;
const TYPE_BOOL: u32 = 1;
const TYPE_INT: u32 = 2;
const TYPE_FLOAT: u32 = 3;
const TYPE_STRING: u32 = 4;
const TYPE_DICTIONARY: u32 = 27;
const TYPE_ARRAY: u32 = 28;


pub struct BinarySerializer {}

impl BinarySerializer {
    pub fn value_to_file(value: &UnpackedValue, path: &str) -> Result<(), std::io::Error> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(&BinarySerializer::calculate_variant_size(value).to_le_bytes());
        bytes.extend(Self::value_to_bytes(value));
        let result = std::fs::write(path, bytes);
        match &result {
            Ok(_) => println!("Successfully wrote file: {}", path),
            Err(e) => println!("Failed to write file: {}, error: {}", path, e),
        }
        result
    }

    pub fn value_from_file(path: &str) -> Option<UnpackedValue> {
        if let Ok(data) = fs::read(path) {
            let value = Self::read_variant(&data, &mut 4);
            return Some(value);
        }
        println!("Error loading file!");
        None
    }

    pub fn value_to_bytes(value: &UnpackedValue) -> Vec<u8> {
        let mut buffer = Vec::new();
        BinarySerializer::write_variant(&mut buffer, value);
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
            UnpackedValue::Float(n) => {
                buffer.write_u32::<LittleEndian>(TYPE_FLOAT).unwrap();
                buffer.write_f32::<LittleEndian>(*n).unwrap();
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

                let padding = (4 - (bytes.len() % 4)) % 4;
                buffer.extend(std::iter::repeat(0).take(padding));
            }
            UnpackedValue::Vec(vec) => {
                buffer.write_u32::<LittleEndian>(TYPE_ARRAY).unwrap(); // using dictionary type as per hexdump
                buffer.write_u32::<LittleEndian>(vec.len() as u32).unwrap();
                
                for (_, item) in vec.iter().enumerate() {
                    Self::write_variant(buffer, item);
                }
            }
            UnpackedValue::Map(map) => {
                buffer.write_u32::<LittleEndian>(TYPE_DICTIONARY).unwrap();
                buffer.write_u32::<LittleEndian>(map.len() as u32).unwrap();
                
                for (_, (key, value)) in map.iter().enumerate() {
                    // Write key as string
                    Self::write_variant(buffer, &UnpackedValue::String(key.clone()));
                    // Write value
                    Self::write_variant(buffer, value);
                }
            }
        }
    }

    fn read_variant(buffer: &[u8], pos: &mut usize) -> UnpackedValue {

        let type_id = u32::from_le_bytes(buffer[*pos..*pos + 4].try_into().unwrap());
        *pos += 4;
    
        match type_id {
            TYPE_INT => {
                let value = i32::from_le_bytes(buffer[*pos..*pos + 4].try_into().unwrap());
                *pos += 4;
                UnpackedValue::Int32(value)
            }
            TYPE_FLOAT => {
                let value = f32::from_le_bytes(buffer[*pos..*pos + 4].try_into().unwrap());
                *pos += 4;
                UnpackedValue::Float(value)
            }
            TYPE_BOOL => {
                let value = i32::from_le_bytes(buffer[*pos..*pos + 4].try_into().unwrap());
                *pos += 4;
                UnpackedValue::Boolean(value != 0)
            }
            TYPE_STRING => {
                let len = u32::from_le_bytes(buffer[*pos..*pos + 4].try_into().unwrap()) as usize;
                *pos += 4;
                let s = String::from_utf8(buffer[*pos..*pos + len].to_vec()).unwrap();
                *pos += len;
                *pos += (4 - (len % 4)) % 4;
                UnpackedValue::String(s)
            }
            TYPE_ARRAY => {
                let len = u32::from_le_bytes(buffer[*pos..*pos + 4].try_into().unwrap()) as usize;
                *pos += 4;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(Self::read_variant(buffer, pos));
                }
                UnpackedValue::Vec(vec)
            }
            TYPE_DICTIONARY => {
                let len = u32::from_le_bytes(buffer[*pos..*pos + 4].try_into().unwrap()) as usize;
                *pos += 4;
                let mut map = std::collections::HashMap::new();
                for _ in 0..len {
                    if let UnpackedValue::String(key) = Self::read_variant(buffer, pos) {
                        let value = Self::read_variant(buffer, pos);
                        map.insert(key, value);
                    }
                }
                UnpackedValue::Map(map)
            }
            _ => {
                *pos -= 4;
                let x = &buffer[*pos..*pos + 4];
                println!("pos: {}", pos);
                println!("buffer: {:?}", x);
                panic!("Unknown type ID: {}", type_id)
            },
        }
    }
    
    fn calculate_variant_size(value: &UnpackedValue) -> u32 {
        match value {
            UnpackedValue::UInt8(_) |
            UnpackedValue::UInt16(_) |
            UnpackedValue::UInt32(_) |
            UnpackedValue::Int8(_) |
            UnpackedValue::Int16(_) |
            UnpackedValue::Int32(_) |
            UnpackedValue::Float(_) |
            UnpackedValue::Boolean(_) => 8, // 4 bytes for type + 4 bytes for value
            
            UnpackedValue::String(s) => {
                let bytes = s.as_bytes();
                let padding = (4 - (bytes.len() % 4)) % 4;
                8 + bytes.len() as u32 + padding as u32 // 4 for type + 4 for length + string + padding
            }
            
            UnpackedValue::Vec(vec) => {
                let mut size = 8; // 4 for type + 4 for length
                for item in vec {
                    size += Self::calculate_variant_size(item);
                }
                size
            }
            
            UnpackedValue::Map(map) => {
                let mut size = 8; // 4 for type + 4 for length
                for (key, value) in map {
                    size += Self::calculate_variant_size(&UnpackedValue::String(key.clone()));
                    size += Self::calculate_variant_size(value);
                }
                size
            }
        }
    }
}

