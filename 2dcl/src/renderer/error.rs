use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ScenesIOError {
    InvalidPath(PathBuf),
}

impl error::Error for ScenesIOError {}

impl fmt::Display for ScenesIOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScenesIOError::InvalidPath(s) => {
                write!(f, "Invalid path : {}", s.display())
            }
        }
    }
}
