use std::{fs::File, io::{Read, Seek, SeekFrom, Write}, sync::Mutex};

pub trait ByteSource: Send + Sync {
    fn len(&self) -> u64;
    fn read_range(&self, offset: u64, size: usize) -> std::io::Result<Vec<u8>>;
    fn write_range(&self, offset: u64, data: &[u8]) -> std::io::Result<()>;
}




pub struct DiskFileSource {
    file: Mutex<File>,
    size: u64,
}

impl DiskFileSource {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
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
        Ok(())
    }
}
