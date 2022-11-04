use std::error;
use std::fmt;

#[derive(Debug)]
pub enum SceneCompileError
{
    NoParcels,
    SourceNotDirectory,
    DestinationNotDirectory,

}

impl error::Error for SceneCompileError {}

impl fmt::Display for SceneCompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self
        {
            SceneCompileError::NoParcels => write!(f, "The property 'parcels' in scene.json is empty."),
            SceneCompileError::SourceNotDirectory => write!(f, "Source is not a folder."),
            SceneCompileError::DestinationNotDirectory => write!(f, "Destination is not a folder."),
        }
        
    }
}  