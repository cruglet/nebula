use std::fs;

// gonna add filetype checking later
pub struct LZFile {
    header: u8,
    data: Vec<u8>,
}

impl LZFile {
    pub fn decompress(&self) -> Option<Vec<u8>> {
        match self.header {
            0x11 => self.decompress_lz11(),
            _ => {
                println!("Unsupported filetype!: {}", self.header);
                None
            }
        }
    }
    
    fn decompress_lz11(&self) -> Option<Vec<u8>> {
        if self.data.is_empty() || self.data[0] != 0x11 {
            return None;
        }
    
        println!("Data length: {}", self.data.len());
    
        let mut length: u32;
        let mut displacement: u32;
        let mut copy_dest: u32;
    
        let mut byte_temp: u8;
        let mut byte_one: u8;
        let mut byte_two: u8;
        let mut byte_three: u8;
    
        let mut decompressed_size = 0;
        let mut source_iter = self.data.iter().copied().skip(1);
    
        for i in 0..3 {
            decompressed_size |= (source_iter.next()? as usize) << (i * 8);
        }
    
        if decompressed_size == 0 {
            for i in 0..4 {
                decompressed_size |= (source_iter.next()? as usize) << (i * 8);
            }
        }
    
        if decompressed_size > 0x800000 {
            return None;
        }
    
        println!("Decompressed size: {}", decompressed_size);
    
        let mut decompressed_data = Vec::with_capacity(decompressed_size);
        let mut current_size = 0;
    
        while current_size < decompressed_size {
            let flags = source_iter.next()?;
    
            for bit_position in 0..8 {
                if current_size >= decompressed_size {
                    break;
                }
    
                let flag = flags & (0x80 >> bit_position);
    
                if flag > 0 {
                    byte_one = source_iter.next()?;
    
                    match byte_one >> 4 {
                        0 => {
                            length = (byte_one as u32) << 4;
                            byte_temp = source_iter.next()?;
                            length |= (byte_temp as u32) >> 4;
                            length += 0x11;
    
                            displacement = ((byte_temp & 0x0F) as u32) << 8;
                            byte_two = source_iter.next()?;
                            displacement |= byte_two as u32;
                        }
                        1 => {
                            byte_temp = source_iter.next()?;
                            byte_two = source_iter.next()?;
                            byte_three = source_iter.next()?;
    
                            length = ((byte_one & 0x0F) as u32) << 12;
                            length |= (byte_temp as u32) << 4;
                            length |= (byte_two >> 4) as u32;
                            length += 0x111;
    
                            displacement = ((byte_two & 0x0F) as u32) << 8;
                            displacement |= byte_three as u32;
                        }
                        _ => {
                            length = ((byte_one >> 4) + 1) as u32;
                            displacement = ((byte_one & 0x0F) as u32) << 8;
                            byte_two = source_iter.next()?;
                            displacement |= byte_two as u32;
                        }
                    };
    
                    if displacement as usize > current_size {
                        return None;
                    }
    
                    copy_dest = current_size as u32;
    
                    for offset in 0..length {
                        if current_size < decompressed_size {
                            let source_index = (copy_dest - displacement - 1 + offset) as usize;
    
                            if let Some(&value) = decompressed_data.get(source_index) {
                                decompressed_data.push(value);
                                current_size += 1;
                            }
                        }
                    }
    
                    if current_size > decompressed_size {
                        break;
                    }
                }

					  if let Some(value) = source_iter.next() {
							decompressed_data.push(value);
							current_size += 1;
 
							if current_size > decompressed_size {
								 break;
							}
					  } else {
							return None;
					  }
                
    
                if decompressed_data.len() == decompressed_size {
                    return Some(decompressed_data);
                }
            }
        }
        None
    }
}

pub fn decompress_raw(data: Vec<u8>) -> Option<Vec<u8>> {
    LZFile::decompress(&LZFile {
        header: *data.first()?,
        data,
    })
}

/* This should preferably be outside */
pub fn open(path: &str) -> Option<LZFile> {
    let data = fs::read(path).ok()?;
    let header = *data.first()?;

    Some(LZFile {
        header,
        data,
    })
}
