
use core::fmt;
use std::{fmt::Display, io, mem};

const BLOCK_SIZE: i32 = 8192;

/// Compiled caption identifier.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VCCD {
    #[default] Default = 1145258838
}

impl VCCD {
    /// Creates a [`VCCD`] from a `i32`.
    /// 
    /// Returns `None` if the provided VCCD is invalid.
    #[must_use]
    #[inline]
    pub fn new(value: i32) -> Option<Self> {
        match value {
            1145258838 => Some(Self::Default),
            _ => None
        }
    }
}

/// Compiled caption version.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Version {
    #[default] Default = 1
}

impl Version {
    /// Creates a [`Version`] from a `i32`.
    /// 
    /// Returns `None` if the provided version is invalid.
    #[must_use]
    #[inline]
    pub fn new(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Default),
            _ => None
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Header {
    pub vccd: VCCD, pub version: Version,
    pub block_count: i32, pub block_size: i32,
    pub dir_size: i32, pub data_offset: i32
}

impl Header {
    /// Reads [`Header`] from a byte buffer.
    /// 
    /// # Errors
    /// 
    /// Returns `Err` if the header format is invalid.
    #[must_use]
    pub fn from_reader(rdr: &mut impl io::Read) -> io::Result<Self> {
        let mut buf = [0u8; mem::size_of::<Self>()];
        rdr.read_exact(&mut buf)?;

        let [raw_vccd, raw_version, block_count, block_size, dir_size, data_offset] = unsafe { mem::transmute::<[u8; mem::size_of::<Self>()], [i32; 6]>(buf) };

        if None == VCCD::new(raw_vccd) {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid VCCD"))
        };

        if None == Version::new(raw_version) {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid Version"))
        };

        Ok(Self { block_count, block_size, dir_size, data_offset, ..Default::default() })
    }
}

impl Default for Header {
    fn default() -> Self {
        Self { vccd: Default::default(), version: Default::default(), block_count: Default::default(), block_size: BLOCK_SIZE, dir_size: Default::default(), data_offset: Default::default() }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VCCD: {}\nVersion: {}\nBlock count: {}\nBlock size: {}\nDir size: {}\nData offset: {}\n", self.vccd as i32, self.version as i32, self.block_count, self.block_size, self.dir_size, self.data_offset)
    }
}