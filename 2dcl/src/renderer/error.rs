use std::error;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SpriteMakerError {
    NoWearables,
    InvalidImageFormat(PathBuf),
}

impl error::Error for SpriteMakerError {}

impl fmt::Display for SpriteMakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpriteMakerError::NoWearables => {
                write!(f, "No wearables found in the wearables folder.")
            }
            SpriteMakerError::InvalidImageFormat(s) => {
                write!(f, "File has an invalid image format : {}", s.display())
            }
        }
    }
}