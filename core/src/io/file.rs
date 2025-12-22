use std::sync::Arc;

use godot::prelude::*;
use crate::io::buffer::NebulaBuffer;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct NebulaFile {
    buffer: Gd<NebulaBuffer>,

    #[base]
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for NebulaFile {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            buffer: NebulaBuffer::new_gd(), // must be initialized later
        }
    }
}

#[godot_api]
impl NebulaFile {
    /// Internal constructor used by FS implementations (ARC, WBFS, etc.)
    pub fn from_buffer(buffer: Gd<NebulaBuffer>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            buffer,
        })
    }

    #[func]
    /// Get the underlying [NebulaBuffer]
    pub fn get_buffer(&self) -> Gd<NebulaBuffer> {
        self.buffer.clone()
    }

    // /// File size in bytes
    // #[func]
    // pub fn size(&self) -> i64 {
    //     self.buffer.bind().len() as i64
    // }

    #[func]
    /// Reads a byte range, similar to [NebulaBuffer].
    pub fn read_range(&self, offset: i32, size: i32) -> PackedByteArray {
        self.buffer
            .bind()
            .read_bytes(offset, size)
    }
}

impl NebulaFile {
    pub fn new(buffer: Arc<NebulaBuffer>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            buffer: buffer.to_gd(),
        })
    }
}