
use core::{fmt, mem};
use std::io;

use packbytes::{FromBytes, ToBytes};

/// Description of a caption.
/// - `crc32` holds the calculated CRC32 checksum the caption's lowercase key.
/// - `block` stores the index of a block the caption is in.
/// - `offset` stores the offset from the beginning of the block.
/// - `length` stores the size of the caption.
#[derive(Debug, Clone, PartialEq, Eq, FromBytes, ToBytes)]
#[packbytes(le)]
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
    /// Returns `Err` if fails to read 12 bytes.
    pub fn from_reader<R: io::Read>(rdr: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; mem::size_of::<Self>()];
        rdr.read_exact(&mut buf)?;

        let entry = Self::from_bytes(buf);
        Ok(entry)
    }
}

impl fmt::Display for CaptionEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hash: {}\nBlock: {}\nOffset: {}\nLength: {}\n",
            self.crc32,
            self.block,
            self.offset,
            self.length
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn from_reader() {
        // Valid entry
        assert!(CaptionEntry::from_reader(
            &mut Cursor::new([50, 120, 21, 13,  0, 0, 0, 0,  105, 0,  91, 0])
        ).is_ok_and(|x| x == CaptionEntry { crc32: 219510834, block: 0, offset: 105, length: 91 }));

        // 2 bytes short
        assert!(CaptionEntry::from_reader(
            &mut Cursor::new([0; mem::size_of::<CaptionEntry>() - 2])
        ).is_err());

        // Buffer empty
        assert!(CaptionEntry::from_reader(
            &mut Cursor::new([])
        ).is_err());
    }
}
