use std::{fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, sync::{Mutex, RwLock}};

pub trait ByteSource: Send + Sync {
    fn len(&self) -> u64;
    fn read_range(&self, offset: u64, size: usize) -> std::io::Result<Vec<u8>>;
    fn write_range(&self, offset: u64, data: &[u8]) -> std::io::Result<()>;
}




pub struct MemoryByteSource {
    data: RwLock<Vec<u8>>,
}


impl MemoryByteSource {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(Vec::new()),
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: RwLock::new(Vec::with_capacity(capacity)),
        }
    }
}

impl ByteSource for MemoryByteSource {
    fn read_range(&self, offset: u64, size: usize) -> std::io::Result<Vec<u8>> {
        let data = self.data.read().unwrap();
        let start = offset as usize;
        let end = (start + size).min(data.len());
        
        if start >= data.len() {
            return Ok(Vec::new());
        }
        
        Ok(data[start..end].to_vec())
    }
    
    fn write_range(&self, offset: u64, bytes: &[u8]) -> std::io::Result<()> {
        let mut data = self.data.write().unwrap();
        let start = offset as usize;
        let end = start + bytes.len();
        
        if end > data.len() {
            data.resize(end, 0);
        }
        
        data[start..end].copy_from_slice(bytes);
        Ok(())
    }
    
    fn len(&self) -> u64 {
        let data = self.data.read().unwrap();
        data.len() as u64
    }
}




pub struct DiskFileSource {
    file: Mutex<File>,
    size: u64,
}

impl DiskFileSource {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;
        
        let size = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;
        
        Ok(DiskFileSource {
            file: Mutex::new(file),
            size,
        })
    }
}

impl ByteSource for DiskFileSource {
    fn len(&self) -> u64 { self.size }
    
    fn read_range(&self, offset: u64, size: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0; size];
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(offset))?;
        let n = file.read(&mut buf)?;
        buf.truncate(n);
        Ok(buf)
    }
    
    fn write_range(&self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(offset))?;
        file.write_all(data)?;
        file.flush()?;
        Ok(())
    }
}
