use crate::Level;
use dcl_common::Parcel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Scene {
    #[serde(skip)]
    pub id: usize,
    pub name: String,
    pub levels: Vec<Level>,
    pub parcels: Vec<Parcel>,
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::test_utils::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<Scene, _>("scene");
    }
}
