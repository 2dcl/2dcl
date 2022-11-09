use serde::{Serialize, Deserialize};
use dcl_common::Parcel;
use crate::Entity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
   #[serde(skip)]
   pub id: usize,
   pub name: String,
   pub entities: Vec<Entity>,
   pub parcels: Vec<Parcel>,
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
      can_go_from_json_to_mp::<Scene, _>("scene");
    }
}
