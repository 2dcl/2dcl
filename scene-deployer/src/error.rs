use std::error;
use std::fmt;

use dcl_common::Parcel;

#[derive(Debug)]
pub enum SceneDeployError {
    InvalidPointers {
        expected_parcels: Vec<String>,
        parcels_found: Vec<String>,
    },
    MissingSceneEntity,
}

impl error::Error for SceneDeployError {}

impl fmt::Display for SceneDeployError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            /*  SceneCompileError::NoParcels => {
                write!(f, "The property 'parcels' in scene.json is empty.")
            } */
            SceneDeployError::InvalidPointers{parcels_found, expected_parcels} => write!(f, "The parcels doesn't match the 3D scene.\nParcels expected: {:?}\nParcels found: {:?}", expected_parcels, parcels_found),
            SceneDeployError::MissingSceneEntity => write!(f, "Missing scene entity."),
        }
    }
}
