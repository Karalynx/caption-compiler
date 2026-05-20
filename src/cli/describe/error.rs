
use core::fmt;
use std::io;

use crate::cli::{Header, HeaderError};

#[derive(Debug)]
pub enum DescribeError {
    InvalidVCCD { found: i32 },
    InvalidVersion { found: i32 },
    Read(io::Error)
}

impl PartialEq for DescribeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidVCCD { found: l_found }, Self::InvalidVCCD { found: r_found }) => l_found == r_found,
            (Self::InvalidVersion { found: l_found }, Self::InvalidVersion { found: r_found }) => l_found == r_found,
            (Self::Read(l), Self::Read(r)) => l.kind() == r.kind(),
            _ => false
        }
    }
}
impl Eq for DescribeError {}

impl fmt::Display for DescribeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidVCCD { found} => write!(
                f,
                "expected VCCD to be {}, found {found}",
                Header::VCCD
            ),
            Self::InvalidVersion { found} => write!(
                f,
                "expected version to be {}, found {found}",
                Header::VERSION
            ),
            Self::Read(err) => write!(f, "{err}")
        }
    }
}
impl std::error::Error for DescribeError {}

impl From<HeaderError> for DescribeError {
    #[inline]
    fn from(value: HeaderError) -> Self {
        match value {
            HeaderError::InvalidVCCD { found } => Self::InvalidVCCD  { found },
            HeaderError::InvalidVersion { found } => Self::InvalidVersion { found },
            HeaderError::Read(err) => Self::Read(err)
        }
    }
}
impl From<io::Error> for DescribeError {
    #[inline]
    fn from(value: io::Error) -> Self {
        Self::Read(value)
    }
}