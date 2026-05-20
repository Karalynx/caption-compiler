
use core::fmt;

use crate::cli::Header;

use super::parser::CaptionParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosedCaptionError{
    Format(CaptionParseError),
    CaptionLength { id: Box<str> },
    HashCollision { id: Box<str> }
}

impl<'src> fmt::Display for ClosedCaptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format(err) => write!(f, "{err}"),
            Self::CaptionLength { id } => write!(
                f,
                "\"{id}\": caption length exceeds block size of {} bytes",
                Header::BLOCK_SIZE
            ),
            Self::HashCollision { id } => write!(
                f,
                "\"{id}\": hash collision detected"
            )
        }
    }
}
impl<'src> std::error::Error for ClosedCaptionError {}

impl<'src> From<CaptionParseError> for ClosedCaptionError {
    #[inline]
    fn from(value: CaptionParseError) -> Self {
        Self::Format(value)
    }
}