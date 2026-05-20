
use core::{fmt, mem};
use std::io;

use packbytes::{FromBytes, ToBytes};

#[derive(Debug)]
pub enum HeaderError {
    InvalidVCCD { found: i32 },
    InvalidVersion { found: i32 },
    Read(io::Error)
}

impl PartialEq for HeaderError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidVCCD { found: l_found }, Self::InvalidVCCD { found: r_found }) => l_found == r_found,
            (Self::InvalidVersion { found: l_found }, Self::InvalidVersion { found: r_found }) => l_found == r_found,
            (Self::Read(l), Self::Read(r)) => l.kind() == r.kind(),
            _ => false
        }
    }
}
impl Eq for HeaderError {}

impl std::error::Error for HeaderError {}
impl From<io::Error> for HeaderError {
    #[inline]
    fn from(value: io::Error) -> Self {
        Self::Read(value)
    }
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidVCCD { found } => write!(
                f,
                "expected VCCD to be {}, found {found}",
                Header::VCCD
            ),
            Self::InvalidVersion { found } => write!(
                f,
                "expected version to be {}, found {found}",
                Header::VERSION
            ),
            Self::Read(err) => write!(f, "{err}")
        }
    }
}

/// Compiled caption header.
/// - `vccd` should always be 1145258838.
/// - `version` should always be 1.
/// - `block_count` stores the number of blocks of size `block_size`.
/// - `block_size` stores the size of a single block.
/// - `dir_size` stores the number of captions.
/// - `data_offset` stores the beginning position of caption data.
#[derive(Debug, Clone, PartialEq, Eq, FromBytes, ToBytes)]
#[packbytes(le)]
#[repr(C)]
pub struct Header {
    pub vccd: i32, pub version: i32,
    pub block_count: i32, pub block_size: i32,
    pub dir_size: i32, pub data_offset: i32
}

impl Header {
    pub const VCCD: i32 = i32::from_le_bytes([b'V', b'C', b'C', b'D']);
    pub const VERSION: i32 = 1;

    pub(crate) const BLOCK_SIZE: i32 = 8192;

    /// Reads [`Header`] from a byte buffer.
    /// 
    /// # Errors
    /// 
    /// Returns `Err` if the header format is invalid.
    pub fn from_reader<R: io::Read>(rdr: &mut R) -> Result<Self, HeaderError> {
        let mut buf = [0u8; mem::size_of::<Self>()];
        rdr.read_exact(&mut buf)?;

        let header = Header::from_bytes(buf);
        if Self::VCCD != header.vccd {
            return Err(HeaderError::InvalidVCCD { found: header.vccd });
        }
        if Self::VERSION != header.version {
            return Err(HeaderError::InvalidVersion { found: header.version });
        }

        Ok(header)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VCCD: {}\nVersion: {}\nBlock count: {}\nBlock size: {}\nDir size: {}\nData offset: {}\n",
            self.vccd, self.version,
            self.block_count, self.block_size,
            self.dir_size, self.data_offset
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn from_reader() {
        // Valid header
        assert_eq!(
            Header::from_reader(
                &mut Cursor::new([86, 67, 67, 68,  1, 0, 0, 0,  0, 0, 0, 0,  0, 32, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0])
            ),
            Ok(Header { vccd: Header::VCCD, version: Header::VERSION, block_count: 0, block_size: Header::BLOCK_SIZE, dir_size: 0, data_offset: 0 })
        );

        // Invalid version
        assert_eq!(
            Header::from_reader(
                &mut Cursor::new([86, 67, 67, 68,  0, 0, 0, 0,  0, 0, 0, 0,  0, 32, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0])
            ),
            Err(HeaderError::InvalidVersion { found: 0 })
        );

        // Invalid VCCD
        assert_eq!(
            Header::from_reader(
                &mut Cursor::new([0, 0, 0, 0,  1, 0, 0, 0,  0, 0, 0, 0,  0, 32, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0])
            ),
            Err(HeaderError::InvalidVCCD { found: 0 })
        );

        // 4 bytes short
        assert!(Header::from_reader(
            &mut Cursor::new([86, 67, 67, 68,  1, 0, 0, 0,  0, 0, 0, 0,  0, 32, 0, 0,  0, 0, 0, 0])
        ).is_err());

        // Buffer empty
        assert!(Header::from_reader(
            &mut Cursor::new([])
        ).is_err());
    }
}