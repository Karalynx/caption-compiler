
use core::fmt;
use std::io;

use crate::captions::ClosedCaptionError;

#[derive(Debug)]
pub enum CompileError {
    Read(io::Error),
    Parse(ClosedCaptionError),
    Write(io::Error)
}

impl PartialEq for CompileError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Read(l), Self::Read(r)) => l.kind() == r.kind(),
            (Self::Parse(l), Self::Parse(r)) => l == r,
            (Self::Write(l), Self::Write(r)) => l.kind() == r.kind(),
            _ => false
        }
    }
}
impl Eq for CompileError {}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Read(err) => write!(f, "unexpected error while reading, {err}"),
            Self::Parse(err) => write!(f, "unexpected error while parsing, {err}"),
            Self::Write(err) => write!(f, "unexpected error while writing, {err}")
        }
    }
}
impl std::error::Error for CompileError {}

impl From<ClosedCaptionError> for CompileError {
    fn from(value: ClosedCaptionError) -> Self {
        Self::Parse(value)
    }
}