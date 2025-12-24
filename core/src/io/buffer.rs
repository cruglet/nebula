use godot::{classes::ProjectSettings, prelude::*};
use std::{ops::Range, sync::Arc};
use crate::io::bytesource::{ByteSource, DiskFileSource, MemoryByteSource};

type BoxedByteSource = Arc<dyn ByteSource + Send + Sync + 'static>;

#[derive(GodotClass)]
#[class(base = RefCounted)]
/// A buffer for reading binary data from files or memory sources.
/// 
/// Provides methods for reading integers, floats, and strings of various encodings,
/// supports both little-endian and big-endian formats, and allows peeking or seeking
/// without advancing the cursor. Also supports Python-style format unpacking with [method unpack].
pub struct NebulaBuffer {
    #[var] 
    /// Determines the endianness used for multi-byte reads.
    /// `false` = little-endian (Default), `true` = big-endian.
    big_endian: bool,

    offset: usize,

    pub source: Option<BoxedByteSource>,
    range: Range<u64>,

    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for NebulaBuffer {
    fn init(base: Base<RefCounted>) -> Self {
        let memory_source = Arc::new(MemoryByteSource::new()) as BoxedByteSource;

        Self {
            big_endian: false,
            offset: 0,
            source: Some(memory_source.clone()),
            range: 0..memory_source.len(),
            base,
        }
    }
}

impl NebulaBuffer {
    #[inline(always)]
    fn source(&self) -> &BoxedByteSource {
        self.source
            .as_ref()
            .expect("NebulaBuffer used without a source")
    }

    #[inline(always)]
    fn abs_pos(&self) -> u64 {
        self.range.start + self.offset as u64
    }

    #[inline(always)]
    fn read_u8_impl(&mut self, advance: bool) -> u8 {
        let pos = self.abs_pos();

        let mut out = 0u8;
        if let Ok(bytes) = self.source().read_range(pos, 1) {
            if !bytes.is_empty() {
                out = bytes[0];
            }
        }

        if advance {
            self.offset += 1;
        }

        out
    }

    #[inline(always)]
    fn write_u8_impl(&mut self, value: u8) {
        let pos = self.abs_pos();
        if let Some(src) = &self.source {
            let _ = src.write_range(pos, &[value]);
        }
        self.offset += 1;
    }

    #[inline(always)]
    fn write_bytes_impl(&mut self, bytes: &[u8]) {
        let pos = self.abs_pos();
        if let Some(src) = &self.source {
            let _ = src.write_range(pos, bytes);
        }
        self.offset += bytes.len();
    }

    #[inline(always)]
    fn read_bytes_impl<const N: usize>(&mut self, advance: bool) -> [u8; N] {
        let pos = self.abs_pos();
        let mut buf = [0u8; N];

        if let Ok(bytes) = self.source().read_range(pos, N) {
            let len = bytes.len().min(N);
            buf[..len].copy_from_slice(&bytes[..len]);
        }

        if advance {
            self.offset += N;
        }

        buf
    }
}

#[godot_api]
impl NebulaBuffer {
    #[func]
    pub fn from_bytes(bytes: PackedByteArray) -> Gd<NebulaBuffer> {
        let vec = bytes.to_vec();
        let source = Arc::new(MemoryByteSource::from_vec(vec)) as BoxedByteSource;
        
        let mut buf = NebulaBuffer::new_gd();
        buf.bind_mut().set_source(source);
        buf
    }

    #[func]
    /// Creates a [NebulaBuffer] from a file path.
    /// Returns a new buffer instance. Logs an error if the file cannot be opened.
    pub fn from_file(path: GString) -> Gd<NebulaBuffer> {
        let path_str = ProjectSettings::singleton()
            .globalize_path(&path)
            .to_string();

        let source = match DiskFileSource::new(&path_str) {
            Ok(src) => Some(Arc::new(src) as BoxedByteSource),
            Err(err) => {
                godot_error!(
                    "NebulaBuffer::from_file: failed to open {}: {:?}",
                    path_str,
                    err
                );
                None
            }
        };

        let mut buf = NebulaBuffer::new_gd();
        if let Some(src) = source {
            buf.bind_mut().set_source(src);
        }

        buf
    }

    #[func]
    /// Returns the current buffer offset.
    pub fn get_offset(&self) -> i32 {
        self.offset as i32
    }

    #[func]
    /// Returns `true` if the cursor has reached the end of the buffer.
    pub fn at_end(&self) -> bool {
        self.abs_pos() >= self.range.end
    }

    #[func]
    /// Moves the buffer cursor forward or backward by the given amount.
    pub fn seek(&mut self, amount: i32) {
        if amount >= 0 {
            self.offset = self.offset.saturating_add(amount as usize);
        } else {
            self.offset = self.offset.saturating_sub((-amount) as usize);
        }
    }

    #[func]
    /// Sets the buffer cursor to the given absolute position.
    pub fn goto(&mut self, position: i32) {
        self.offset = position.max(0) as usize;
    }

    #[func]
    /// Reads a single unsigned byte and advances the cursor.
    pub fn read_u8(&mut self) -> u8 {
        self.read_u8_impl(true)
    }

    #[func]
    /// Reads a single unsigned byte without advancing the cursor.
    pub fn peek_u8(&mut self) -> u8 {
        self.read_u8_impl(false)
    }

    #[func]
    /// Reads a single signed byte and advances the cursor.
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8_impl(true) as i8
    }

    #[func]
    /// Reads a single signed byte without advancing the cursor.
    pub fn peek_i8(&mut self) -> i8 {
        self.read_u8_impl(false) as i8
    }

    #[func]
    /// Reads a 16-bit unsigned integer with the buffer's endianness and advances the cursor.
    pub fn read_u16(&mut self) -> u16 {
        let b = self.read_bytes_impl::<2>(true);
        if self.big_endian {
            u16::from_be_bytes(b)
        } else {
            u16::from_le_bytes(b)
        }
    }

    #[func]
    /// Reads a 16-bit unsigned integer without advancing the cursor.
    pub fn peek_u16(&mut self) -> u16 {
        let b = self.read_bytes_impl::<2>(false);
        if self.big_endian {
            u16::from_be_bytes(b)
        } else {
            u16::from_le_bytes(b)
        }
    }

    #[func]
    /// Reads a 16-bit signed integer and advances the cursor.
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }

    #[func]
    /// Reads a 16-bit signed integer without advancing the cursor.
    pub fn peek_i16(&mut self) -> i16 {
        self.peek_u16() as i16
    }

    #[func]
    /// Reads a 32-bit unsigned integer and advances the cursor.
    pub fn read_u32(&mut self) -> u32 {
        let b = self.read_bytes_impl::<4>(true);
        if self.big_endian {
            u32::from_be_bytes(b)
        } else {
            u32::from_le_bytes(b)
        }
    }

    #[func]
    /// Reads a 32-bit unsigned integer without advancing the cursor.
    pub fn peek_u32(&mut self) -> u32 {
        let b = self.read_bytes_impl::<4>(false);
        if self.big_endian {
            u32::from_be_bytes(b)
        } else {
            u32::from_le_bytes(b)
        }
    }

    #[func]
    /// Reads a 32-bit signed integer and advances the cursor.
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }

    #[func]
    /// Reads a 32-bit signed integer without advancing the cursor.
    pub fn peek_i32(&mut self) -> i32 {
        self.peek_u32() as i32
    }

    #[func]
    /// Reads a 64-bit unsigned integer and advances the cursor.
    pub fn read_u64(&mut self) -> u64 {
        let b = self.read_bytes_impl::<8>(true);
        if self.big_endian {
            u64::from_be_bytes(b)
        } else {
            u64::from_le_bytes(b)
        }
    }

    #[func]
    /// Reads a 64-bit unsigned integer without advancing the cursor.
    pub fn peek_u64(&mut self) -> u64 {
        let b = self.read_bytes_impl::<8>(false);
        if self.big_endian {
            u64::from_be_bytes(b)
        } else {
            u64::from_le_bytes(b)
        }
    }

    #[func]
    /// Reads a 64-bit signed integer and advances the cursor.
    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }

    #[func]
    /// Reads a 64-bit signed integer without advancing the cursor.
    pub fn peek_i64(&mut self) -> i64 {
        self.peek_u64() as i64
    }

    #[func]
    /// Reads a 32-bit floating-point number and advances the cursor.
    pub fn read_f32(&mut self) -> f32 {
        let bits = self.read_u32();
        f32::from_bits(bits)
    }

    #[func]
    /// Reads a 32-bit floating-point number without advancing the cursor.
    pub fn peek_f32(&mut self) -> f32 {
        let saved = self.offset;
        let val = self.read_f32();
        self.offset = saved;
        val
    }

    #[func]
    /// Reads a 64-bit floating-point number and advances the cursor.
    pub fn read_f64(&mut self) -> f64 {
        let bits = self.read_u64();
        f64::from_bits(bits)
    }

    #[func]
    /// Reads a 64-bit floating-point number without advancing the cursor.
    pub fn peek_f64(&mut self) -> f64 {
        let saved = self.offset;
        let val = self.read_f64();
        self.offset = saved;
        val
    }

    #[func]
    /// Reads an ASCII string up to a null terminator or the specified length and advances the cursor.
    pub fn read_string_ascii(&mut self, #[opt(default = -1)] amount: i32) -> GString {
        let mut bytes = Vec::new();

        if amount < 0 {
            loop {
                let b = self.read_u8();
                if b == 0 { break; }
                bytes.push(b);
            }
        } else {
            for _ in 0..amount {
                let b = self.read_u8();
                if b == 0 { break; }
                bytes.push(b);
            }
        }

        GString::from(String::from_utf8_lossy(&bytes).as_ref())
    }

    #[func]
    /// Reads an ASCII string without advancing the cursor.
    pub fn peek_string_ascii(&mut self, #[opt(default = -1)] amount: i32) -> GString {
        let saved = self.offset;
        let s = self.read_string_ascii(amount);
        self.offset = saved;
        s
    }

    #[func]
    /// Reads a UTF-8 string up to a null terminator or the specified length and advances the cursor.
    pub fn read_string_utf8(&mut self, #[opt(default = -1)] amount: i32) -> GString {
        let mut bytes = Vec::new();

        if amount < 0 {
            loop {
                let b = self.read_u8();
                if b == 0 { break; }
                bytes.push(b);
            }
        } else {
            for _ in 0..amount {
                let b = self.read_u8();
                if b == 0 { break; }
                bytes.push(b);
            }
        }

        GString::from(String::from_utf8_lossy(&bytes).as_ref())
    }

    #[func]
    /// Reads a UTF-8 string without advancing the cursor.
    pub fn peek_string_utf8(&mut self, #[opt(default = -1)] amount: i32) -> GString {
        let saved = self.offset;
        let s = self.read_string_utf8(amount);
        self.offset = saved;
        s
    }

    #[func]
    /// Reads a UTF-16 string up to a null terminator or the specified number of units and advances the cursor.
    pub fn read_string_utf16(&mut self, #[opt(default = -1)] amount: i32) -> GString {
        let mut units = Vec::new();

        if amount < 0 {
            loop {
                let u = self.read_u16();
                if u == 0 { break; }
                units.push(u);
            }
        } else {
            for _ in 0..amount {
                let u = self.read_u16();
                if u == 0 { break; }
                units.push(u);
            }
        }

        GString::from(&String::from_utf16_lossy(&units))
    }

    #[func]
    /// Reads a UTF-16 string without advancing the cursor.
    pub fn peek_string_utf16(&mut self, #[opt(default = -1)] amount: i32) -> GString {
        let saved = self.offset;
        let s = self.read_string_utf16(amount);
        self.offset = saved;
        s
    }

    #[func]
    /// Reads a sequence of bytes from a given offset and length, returning a [PackedByteArray].
    pub fn read_bytes(&self, offset: i32, size: i32) -> PackedByteArray {
        if let Some(src) = &self.source {
            src.read_range(offset as u64, size as usize)
                .unwrap_or_default()
                .into()
        } else {
            PackedByteArray::new()
        }
    }

    #[func]
    /// Unpacks multiple values from the buffer according to a format string.
    ///
    /// The format string can contain:
    /// - **Endianness prefix (optional)**:
    ///   - `<` -> little endian
    ///   - `>` -> big endian
    ///   - If no prefix is provided, default endianness as set by [param big_endian] is used.
    /// [br]
    /// - **Type codes**:
    ///   - `b` -> signed 8-bit integer (`i8`)
    ///   - `B` -> unsigned 8-bit integer (`u8`)
    ///   - `h` -> signed 16-bit integer (`i16`)
    ///   - `H` -> unsigned 16-bit integer (`u16`)
    ///   - `i` -> signed 32-bit integer (`i32`)
    ///   - `I` -> unsigned 32-bit integer (`u32`)
    ///   - `q` -> signed 64-bit integer (`i64`)
    ///   - `Q` -> unsigned 64-bit integer (`u64`)
    ///   - `f` -> 32-bit floating point (`f32`)
    ///   - `d` -> 64-bit floating point (`f64`)
    ///   - `x` -> pad byte (skip, does not produce output)
    /// [br]
    /// - **Repeat counts**:
    ///   - A number before a type repeats that type multiple times, e.g.:
    ///     - `5B` -> 5 unsigned bytes
    ///     - `2b` -> 2 signed bytes
    ///     - `3f` -> 3 floats
    ///   - Padding can also be repeated: `4x` skips 4 bytes.
    /// [br][br]
    /// **Example**:
    /// ```gdscript
    /// # Binary layout (little-endian):
    /// # offset 0: u32   magic
    /// # offset 4: u16   version
    /// # offset 6: u8    flags
    /// # offset 7: pad   (1 byte)
    /// # offset 8: f32   position_x
    /// # offset 12: f32  position_y
    /// 
    /// var buf = NebulaBuffer.from_file("res://data.bin")
    /// 
    /// var values = buf.unpack("<IHBx2f")
    /// 
    /// var magic = values[0] # u32
    /// var version = values[1] # u16
    /// var flags = values[2] # u8
    /// var position_x = values[3] # f32
    /// var position_y = values[4] # f32
    /// ```
    /// The buffer cursor is advanced by 16 bytes total:
    /// `4 + 2 + 1 + 1 (padding) + 4 + 4`.
    /// - Advances the buffer cursor automatically for all types, including padding.
    /// - Returns an `Array` containing the unpacked values in order.
    /// - Any unknown format character will log an error and stop unpacking.
    ///
    /// This method is useful for reading structured binary data from files or memory buffers.
    pub fn unpack(&mut self, format: GString) -> VarArray {
        let fmt = format.to_string();
        let mut chars = fmt.chars().peekable();
        let saved_endian = self.big_endian;

        if let Some(&c) = chars.peek() {
            match c {
                '<' => { self.big_endian = false; chars.next(); }
                '>' => { self.big_endian = true; chars.next(); }
                _ => {}
            }
        }

        let mut out = VarArray::new();
        let mut repeat: usize = 0;

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                repeat = repeat * 10 + (c as u8 - b'0') as usize;
                continue;
            }

            let count = if repeat == 0 { 1 } else { repeat };
            repeat = 0;

            match c {
                'x' => self.offset += count,

                'b' => for _ in 0..count { out.push(&self.read_i8().to_variant()); },
                'B' => for _ in 0..count { out.push(&self.read_u8().to_variant()); },
                'h' => for _ in 0..count { out.push(&self.read_i16().to_variant()); },
                'H' => for _ in 0..count { out.push(&self.read_u16().to_variant()); },
                'i' => for _ in 0..count { out.push(&self.read_i32().to_variant()); },
                'I' => for _ in 0..count { out.push(&self.read_u32().to_variant()); },
                'q' => for _ in 0..count { out.push(&self.read_i64().to_variant()); },
                'Q' => for _ in 0..count { out.push(&self.read_u64().to_variant()); },
                'f' => for _ in 0..count { out.push(&self.read_f32().to_variant()); },
                'd' => for _ in 0..count { out.push(&self.read_f64().to_variant()); },

                _ => {
                    godot_error!("NebulaBuffer::unpack: unknown format char '{}'", c);
                    break;
                }
            }
        }

        self.big_endian = saved_endian;
        out
    }

    #[func] 
    /// Writes an unsigned byte at the current cursor position
    /// and advances the cursor.
    pub fn store_u8(&mut self, value: u8) { self.write_u8_impl(value); }
    
    #[func]
    /// Writes a signed byte at the current cursor position
    /// and advances the cursor.
    pub fn store_i8(&mut self, value: i8) { self.write_u8_impl(value as u8); }

    #[func]
    /// Writes a 16-bit unsigned integer using the current endianness
    /// and advances the cursor.
    pub fn store_u16(&mut self, value: u16) {
        let bytes = if self.big_endian { value.to_be_bytes() } else { value.to_le_bytes() };
        self.write_bytes_impl(&bytes);
    }

    #[func]
    /// Writes a 16-bit signed integer using the current endianness
    /// and advances the cursor.
    pub fn store_i16(&mut self, value: i16) { self.store_u16(value as u16); }

    #[func]
    /// Writes a 32-bit unsigned integer using the current endianness
    /// and advances the cursor.
    pub fn store_u32(&mut self, value: u32) {
        let bytes = if self.big_endian { value.to_be_bytes() } else { value.to_le_bytes() };
        self.write_bytes_impl(&bytes);
    }

    #[func]
    /// Writes a 32-bit signed integer using the current endianness
    /// and advances the cursor.
    pub fn store_i32(&mut self, value: i32) { self.store_u32(value as u32); }

    #[func]
    /// Writes a 64-bit unsigned integer using the current endianness
    /// and advances the cursor.
    pub fn store_u64(&mut self, value: u64) {
        let bytes = if self.big_endian { value.to_be_bytes() } else { value.to_le_bytes() };
        self.write_bytes_impl(&bytes);
    }

    #[func]
    /// Writes a 64-bit signed integer using the current endianness
    /// and advances the cursor.
    pub fn store_i64(&mut self, value: i64) { self.store_u64(value as u64); }

    #[func]
    /// Writes a 32-bit floating-point number and advances the cursor.
    pub fn store_f32(&mut self, value: f32) { self.store_u32(value.to_bits()); }

    #[func]
    /// Writes a 64-bit floating-point number and advances the cursor.
    pub fn store_f64(&mut self, value: f64) { self.store_u64(value.to_bits()); }

    #[func]
    /// Writes an ASCII string to the buffer and advances the cursor.
    ///
    /// If `escape` is `true`, a null terminator (`\0`) is written after
    /// the string.  
    /// If `escape` is `false` (default), no terminator is written.
    ///
    /// This method does not perform any character validation; bytes are
    /// written exactly as provided by the string.
    pub fn store_string_ascii(
        &mut self,
        s: GString,
        #[opt(default = false)] escape: bool,
    ) {
        let s_str = s.to_string();
        let bytes = s_str.as_bytes();
        self.write_bytes_impl(bytes);
        if escape {
            self.write_u8_impl(0);
        }
    }


    #[func]
    /// Writes a UTF-8 encoded string to the buffer and advances the cursor.
    ///
    /// If `escape` is `true`, a null terminator (`\0`) is written after
    /// the string.  
    /// If `escape` is `false` (default), no terminator is written.
    ///
    /// The string is written as raw UTF-8 bytes.
    pub fn store_string_utf8(
        &mut self,
        s: GString,
        #[opt(default = false)] escape: bool,
    ) {
        let s_str = s.to_string();
        let bytes = s_str.as_bytes();
        self.write_bytes_impl(bytes);
        if escape {
            self.write_u8_impl(0);
        }
    }


   #[func]
    /// Writes a UTF-16 encoded string to the buffer and advances the cursor.
    ///
    /// Each UTF-16 code unit is written using the current endianness.
    ///
    /// If `escape` is `true`, a UTF-16 null terminator (`0x0000`) is written
    /// after the string.  
    /// If `escape` is `false` (default), no terminator is written.
    pub fn store_string_utf16(
        &mut self,
        s: GString,
        #[opt(default = false)] escape: bool,
    ) {
        let s_str = s.to_string();
        let units: Vec<u16> = s_str.encode_utf16().collect();
        for u in units {
            self.store_u16(u);
        }
        if escape {
            self.store_u16(0);
        }
    }



    #[func]
    /// Stores a sequence of bytes at the current offset
    pub fn store_bytes(&mut self, bytes: PackedByteArray) {
        self.write_bytes_impl(&bytes.to_vec());
    }

    #[func]
    /// Packs multiple values into the buffer according to a format string.
    /// Format codes are the same as [method unpack], but values are taken from the input [Array].
    pub fn pack(&mut self, format: GString, values: VarArray) {
        let fmt = format.to_string();
        let mut chars = fmt.chars().peekable();
        let saved_endian = self.big_endian;

        if let Some(&c) = chars.peek() {
            match c {
                '<' => { self.big_endian = false; chars.next(); }
                '>' => { self.big_endian = true; chars.next(); }
                _ => {}
            }
        }

        let mut repeat: usize = 0;
        let mut value_index = 0;

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                repeat = repeat * 10 + (c as u8 - b'0') as usize;
                continue;
            }

            let count = if repeat == 0 { 1 } else { repeat };
            repeat = 0;

            for _ in 0..count {
                if c != 'x' && value_index >= values.len() {
                    godot_error!("NebulaBuffer::pack: not enough values for format");
                    return;
                }

                match c {
                    'x' => self.offset += 1, // skip

                    'b' => self.store_i8(values.get(value_index).map_or(0, |v| v.try_to::<i8>().unwrap_or(0))),
                    'B' => self.store_u8(values.get(value_index).map_or(0, |v| v.try_to::<u8>().unwrap_or(0))),
                    'h' => self.store_i16(values.get(value_index).map_or(0, |v| v.try_to::<i16>().unwrap_or(0))),
                    'H' => self.store_u16(values.get(value_index).map_or(0, |v| v.try_to::<u16>().unwrap_or(0))),
                    'i' => self.store_i32(values.get(value_index).map_or(0, |v| v.try_to::<i32>().unwrap_or(0))),
                    'I' => self.store_u32(values.get(value_index).map_or(0, |v| v.try_to::<u32>().unwrap_or(0))),
                    'q' => self.store_i64(values.get(value_index).map_or(0, |v| v.try_to::<i64>().unwrap_or(0))),
                    'Q' => self.store_u64(values.get(value_index).map_or(0, |v| v.try_to::<u64>().unwrap_or(0))),
                    'f' => self.store_f32(values.get(value_index).map_or(0.0, |v| v.try_to::<f32>().unwrap_or(0.0))),
                    'd' => self.store_f64(values.get(value_index).map_or(0.0, |v| v.try_to::<f64>().unwrap_or(0.0))),

                    _ => {
                        godot_error!("NebulaBuffer::pack: unknown format char '{}'", c);
                        return;
                    }
                }


                if c != 'x' {
                    value_index += 1;
                }
            }
        }

        self.big_endian = saved_endian;
    }


    pub fn set_source(&mut self, source: BoxedByteSource) {
        self.range = 0..source.len();
        self.source = Some(source);
        self.offset = 0;
    }
}
