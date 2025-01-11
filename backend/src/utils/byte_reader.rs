use std::collections::HashMap;
use byteorder::{BigEndian, ByteOrder};
use std::str;

#[derive(Debug, Clone)]
pub enum UnpackedValue {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Boolean(bool),
    String(String),
    Map(HashMap<String, UnpackedValue>),
    Vec(Vec<UnpackedValue>),
}

pub fn unpack(format: &str, data: &[u8]) -> Vec<UnpackedValue> {
    let expanded_format = expand_format(format);
    let mut result = Vec::new();
    let mut offset = 0;
    let mut chars = expanded_format.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if offset >= data.len() {
            break;
        }

        match ch {
            'l' => {
                if offset + 4 <= data.len() {
                    let value = BigEndian::read_i32(&data[offset..]);
                    result.push(UnpackedValue::Int32(value));
                    offset += 4;
                }
            }
            'L' => {
                if offset + 4 <= data.len() {
                    let value = BigEndian::read_u32(&data[offset..]);
                    result.push(UnpackedValue::UInt32(value));
                    offset += 4;
                }
            }
            'h' => {
                if offset + 2 <= data.len() {
                    let value = BigEndian::read_i16(&data[offset..]);
                    result.push(UnpackedValue::Int16(value));
                    offset += 2;
                }
            }
            'H' => {
                if offset + 2 <= data.len() {
                    let value = BigEndian::read_u16(&data[offset..]);
                    result.push(UnpackedValue::UInt16(value));
                    offset += 2;
                }
            }
            'b' => {
                if offset + 1 <= data.len() {
                    let value = data[offset] as i8;
                    result.push(UnpackedValue::Int8(value));
                    offset += 1;
                }
            }
            'B' => {
                if offset + 1 <= data.len() {
                    let value = data[offset];
                    result.push(UnpackedValue::UInt8(value));
                    offset += 1;
                }
            }
            'o' => {
                if offset + 1 <= data.len() {
                    let value = data[offset];
                    result.push(UnpackedValue::Boolean(value != 0));
                    offset += 1;
                }
            }
            'x' => {
                if offset + 1 <= data.len() {
                    offset += 1;
                }
            }
            _ if ch.is_digit(10) => {
                let mut length_str = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_digit(10) {
                        length_str.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(length) = length_str.parse::<usize>() {
                    if let Some('s') = chars.peek() {
                        chars.next();
                        if offset + length <= data.len() {
                            if let Ok(string) = str::from_utf8(&data[offset..offset + length]) {
                                result.push(UnpackedValue::String(string.to_string().trim_matches('\0').to_string()));
                            }
                            offset += length;
                        }
                    }
                }
                continue;
            }
            _ => {}
        }
        chars.next();
    }

    result
}

fn expand_format(format: &str) -> String {
    let mut expanded = String::new();
    let cleaned_text = format.replace(":", "");
    let mut chars = cleaned_text.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_digit(10) {
            let mut count_str = String::new();
            while let Some(&digit) = chars.peek() {
                if digit.is_digit(10) {
                    count_str.push(digit);
                    chars.next();
                } else {
                    break;
                }
            }
            if let Ok(count) = count_str.parse::<usize>() {
                if let Some(&next_char) = chars.peek() {
                    if next_char != 's' {
                        expanded.push_str(&next_char.to_string().repeat(count));
                        chars.next();
                    } else {
                        expanded.push_str(&count_str);
                    }
                }
            }
        } else {
            expanded.push(ch);
            chars.next();
        }
    }

    expanded
}
