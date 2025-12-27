use godot::prelude::*;
use crate::io::buffer::NebulaBuffer;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct LZSS {
    #[base] 
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for LZSS {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl LZSS {
    #[func]
    fn decompress(mut buffer: Gd<NebulaBuffer>) -> Gd<NebulaBuffer> {
        let mut buf = buffer.bind_mut();
        
        let compression_type = buf.read_u8();
        
        match compression_type {
            0x11 => Self::decompress_lz11(&mut buf),
            _ => {
                godot_error!("LZSS::decompress: unsupported compression type 0x{:02X}", compression_type);
                NebulaBuffer::new_gd()
            }
        }
    }

    fn decompress_lz11(buf: &mut NebulaBuffer) -> Gd<NebulaBuffer> {
        let data_size = buf.size() as usize;
        let mut source_index = buf.get_offset() as usize;

        if source_index + 2 >= data_size {
            godot_error!("LZSS::decompress_lz11: insufficient data for size header");
            return NebulaBuffer::new_gd();
        }

        let mut decompressed_size = buf.read_u8() as usize
            | ((buf.read_u8() as usize) << 8)
            | ((buf.read_u8() as usize) << 16);
        source_index += 3;

        if decompressed_size == 0 {
            if source_index + 3 >= data_size {
                godot_error!("LZSS::decompress_lz11: insufficient data for extended size");
                return NebulaBuffer::new_gd();
            }

            decompressed_size = buf.read_u8() as usize
                | ((buf.read_u8() as usize) << 8)
                | ((buf.read_u8() as usize) << 16)
                | ((buf.read_u8() as usize) << 24);
            source_index += 4;
        }

        if decompressed_size > 0x800000 || decompressed_size == 0 {
            godot_error!("LZSS::decompress_lz11: invalid decompressed size {}", decompressed_size);
            return NebulaBuffer::new_gd();
        }

        let mut decompressed_data = vec![0u8; decompressed_size];
        let mut current_size = 0;

        while current_size < decompressed_size && source_index < data_size {
            buf.goto(source_index as i32);
            let flags = buf.read_u8();
            source_index += 1;

            let mut bit_mask = 0x80u8;
            for _ in 0..8 {
                if current_size >= decompressed_size {
                    break;
                }

                if (flags & bit_mask) != 0 {
                    if source_index >= data_size {
                        godot_error!("LZSS::decompress_lz11: unexpected end of data");
                        return NebulaBuffer::new_gd();
                    }

                    buf.goto(source_index as i32);
                    let byte_one = buf.read_u8();
                    source_index += 1;
                    let first_nibble = byte_one >> 4;

                    let (length, displacement) = if first_nibble == 0 {
                        if source_index + 1 >= data_size {
                            godot_error!("LZSS::decompress_lz11: insufficient data for type 0");
                            return NebulaBuffer::new_gd();
                        }

                        let byte_temp = buf.read_u8();
                        let byte_two = buf.read_u8();
                        source_index += 2;

                        let length = (((byte_one as usize) << 4) | ((byte_temp as usize) >> 4)) + 0x11;
                        let displacement = (((byte_temp & 0x0F) as usize) << 8) | (byte_two as usize);

                        (length, displacement)
                    } else if first_nibble == 1 {
                        if source_index + 2 >= data_size {
                            godot_error!("LZSS::decompress_lz11: insufficient data for type 1");
                            return NebulaBuffer::new_gd();
                        }

                        let byte_temp = buf.read_u8();
                        let byte_two = buf.read_u8();
                        let byte_three = buf.read_u8();
                        source_index += 3;

                        let length = ((((byte_one & 0x0F) as usize) << 12)
                            | ((byte_temp as usize) << 4)
                            | ((byte_two as usize) >> 4)) + 0x111;
                        let displacement = (((byte_two & 0x0F) as usize) << 8) | (byte_three as usize);

                        (length, displacement)
                    } else {
                        if source_index >= data_size {
                            godot_error!("LZSS::decompress_lz11: insufficient data for type 2");
                            return NebulaBuffer::new_gd();
                        }

                        let byte_two = buf.read_u8();
                        source_index += 1;

                        let length = (first_nibble as usize) + 1;
                        let displacement = (((byte_one & 0x0F) as usize) << 8) | (byte_two as usize);

                        (length, displacement)
                    };

                    if displacement >= current_size {
                        godot_error!("LZSS::decompress_lz11: invalid displacement");
                        return NebulaBuffer::new_gd();
                    }

                    let copy_source = current_size - displacement - 1;
                    let copy_end = (current_size + length).min(decompressed_size);

                    if displacement < length {
                        for i in 0..(copy_end - current_size) {
                            decompressed_data[current_size + i] = 
                                decompressed_data[copy_source + (i % (displacement + 1))];
                        }
                    } else {
                        for i in 0..(copy_end - current_size) {
                            decompressed_data[current_size + i] = decompressed_data[copy_source + i];
                        }
                    }

                    current_size = copy_end;
                } else {
                    if source_index >= data_size {
                        godot_error!("LZSS::decompress_lz11: unexpected end of data (literal)");
                        return NebulaBuffer::new_gd();
                    }

                    buf.goto(source_index as i32);
                    decompressed_data[current_size] = buf.read_u8();
                    source_index += 1;
                    current_size += 1;
                }

                bit_mask >>= 1;

                if current_size >= decompressed_size {
                    break;
                }
            }
        }

        let packed_bytes = PackedByteArray::from(decompressed_data.as_slice());
        NebulaBuffer::from_bytes(packed_bytes)
    }
}