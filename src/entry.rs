
use core::fmt;
use std::{fmt::Display, io, mem};

/// Description of a caption.
/// - `crc32` holds the calculated CRC32 checksum the caption's lowercase key.
/// - `block` stores the index of a block the caption is in.
/// - `offset` stores the offset from the beginning of the block.
/// - `length` stores the size of the caption.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct CaptionEntry {
    pub crc32: u32,
    pub block: i32, pub offset: u16,
    pub length: u16
}

impl CaptionEntry {
    /// Reads [`CaptionEntry`] from a byte buffer.
    /// 
    /// # Errors
    /// 
    /// Returns `Err` if the buffer contains less than 12 bytes.
    #[must_use]
    #[inline]
    pub fn from_reader(rdr: &mut impl io::Read) -> io::Result<Self> {
        let mut entry = Self::default();
        rdr.read_exact(unsafe {mem::transmute::<&mut u32, &mut[u8; mem::size_of::<CaptionEntry>()]>(&mut entry.crc32)})?;

        Ok(entry)
    }
}

impl Display for CaptionEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash: {}\nBlock: {}\nOffset: {}\nLength: {}\n", self.crc32, self.block, self.offset, self.length)
    }
}