use serde::{Serialize, Deserialize};
use dcl_common::Parcel;
use crate::{Entity, Vec2};



#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
   #[serde(skip)]
   pub id: usize,
   pub name: String,
   #[serde(default)]
   pub dimensions: Vec2<u16>,
   #[serde(default)]
   pub player_layer: i16,
   pub entities: Vec<Entity>,
}



#[cfg(test)]
mod test {
  
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
      can_go_from_json_to_mp::<Level, _>("level");
    }
}
