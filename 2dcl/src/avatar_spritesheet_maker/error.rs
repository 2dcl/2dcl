use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AvatarMakerError {
    MissingEntityId,
}

impl error::Error for AvatarMakerError {}

impl fmt::Display for AvatarMakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AvatarMakerError::MissingEntityId => {
                write!(f, "Missing entity ID")
            }
        }
    }
}
