use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SpriteMakerError {
    NoWearables,
    NoBody,
    InvalidImageFormat(PathBuf),
}

impl error::Error for SpriteMakerError {}

impl fmt::Display for SpriteMakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpriteMakerError::NoWearables => {
                write!(f, "No wearables found in the wearables folder.")
            }
            SpriteMakerError::NoBody => {
                write!(f, "No body found in the wearables folder.")
            }
            SpriteMakerError::InvalidImageFormat(s) => {
                write!(f, "File has an invalid image format : {}", s.display())
            }
        }
    }
}

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
