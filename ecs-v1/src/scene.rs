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
        let json = include_str!("../fixtures/scene.json");
        let result = json_to_mp::<&str, Scene>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/scene.mp").unwrap();
       
        assert_eq!(result, expected);
    }
}
