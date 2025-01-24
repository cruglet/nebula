use std::error::Error;

pub fn decompress(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // Validate magic number
    if data.is_empty() || data[0] != 0x11 {
        return Err("Invalid compression format".into());
    }

    // Read decompressed size
    let mut decompsize: usize = 0;
    let mut source_idx = 1;

    // First try 3-byte size
    for i in 0..3 {
        decompsize |= (data[source_idx] as usize) << (i * 8);
        source_idx += 1;
    }

    // If size is 0, use 4-byte extended size
    if decompsize == 0 {
        decompsize = 0;
        for i in 0..4 {
            decompsize |= (data[source_idx] as usize) << (i * 8);
            source_idx += 1;
        }
    }

    // Size validation
    if decompsize > 0x800000 {
        return Err("Decompressed size too large".into());
    }

    // Debug: Output the decompressed size
    println!("Decompressed size: {}", decompsize);

    let mut decoded = Vec::with_capacity(decompsize);
    let mut curr_size = 0;

    while curr_size < decompsize {
        if source_idx == data.len() {
            break;
        }
        let flags = data[source_idx];
        source_idx += 1;

        // Debug: Output the current flags
        println!("Flags: {:#x}", flags);

        for i in 0..8 {
            if curr_size >= decompsize {
                break;
            }

            let flag = flags & (0x80 >> i);

            // Debug: Output the current flag and the flag bit position
            println!("Flag bit {}: {:#x}", i, flag);

            if flag > 0 {
                let b1 = data[source_idx];
                source_idx += 1;

                // Debug: Output the value of b1
                println!("b1: {:#x}", b1);

                let (len, disp) = match b1 >> 4 {
                    0 => {
                        let bt = data[source_idx];
                        source_idx += 1;
                        let len = ((b1 as u16) << 4) | ((bt >> 4) as u16) + 0x11;
                        let disp = ((bt & 0x0F) as u16) << 8 | data[source_idx] as u16;
                        source_idx += 1;
                        (len, disp)
                    },
                    1 => {
                        let bt = data[source_idx];
                        let b2 = data[source_idx + 1];
                        let b3 = data[source_idx + 2];
                        source_idx += 3;
                        let len = (((b1 & 0xF) as u16) << 12) | ((bt as u16) << 4) | ((b2 >> 4) as u16) + 0x111;
                        let disp = ((b2 & 0xF) as u16) << 8 | b3 as u16;
                        (len, disp)
                    },
                    _ => {
                        let len = (b1 >> 4) + 1;
                        let b2 = data[source_idx];
                        source_idx += 1;
                        let disp = ((b1 & 0x0F) as u16) << 8 | b2 as u16;
                        (len as u16, disp)
                    }
                };

                // Debug: Output the length and displacement
                println!("Length: {}, Displacement: {}", len, disp);

                // Ensure we have enough space for the copy
                if curr_size + len as usize > decompsize {
                    return Err("Copy would exceed decompressed size".into());
                }

                // Perform copy from previously decoded data
                for j in 0..len as usize {
                    // Ensure base_idx is within the current buffer
                    let base_idx = curr_size.saturating_sub(disp as usize);
                    
                    // Ensure src_idx is within the current buffer length
                    let src_idx = base_idx + j;
                    if src_idx >= curr_size {
                        // Handle out-of-bounds case, potentially by padding or breaking
                        break;
                    }
                
                    // Safely access and copy byte
                    let byte = decoded[src_idx];
                    decoded.push(byte);
                    curr_size += 1;
                
                    // Optional: Add a safety check to prevent exceeding total expected size
                    if curr_size > decompsize {
                        return Err("Decompression exceeded expected size".into());
                    }
                }
            } else {
                // Literal byte
                if source_idx < data.len() {
                decoded.push(data[source_idx]);
                source_idx += 1;
                curr_size += 1;
                }

                // Debug: Output the literal byte
                println!("Literal byte: {:#x}", data[source_idx - 1]);
            }
        }
    }

    // Debug: Output the final decompressed data size
    println!("Decompression completed. Final size: {}", decoded.len());

    Ok(decoded)
}
