use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RGBA {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Channel {
  R,
  G,
  B,
  A
}


#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn channel_can_be_serialized_from_json() {
      can_go_from_json_to_mp::<Channel, _>("channel/r");
      can_go_from_json_to_mp::<Channel, _>("channel/g");
      can_go_from_json_to_mp::<Channel, _>("channel/b");
      can_go_from_json_to_mp::<Channel, _>("channel/a");
    }

    #[test]
    fn rgba_can_be_serialized_from_json() {
      can_go_from_json_to_mp::<RGBA, _>("rgba");
    }

}
