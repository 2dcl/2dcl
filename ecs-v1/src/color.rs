use serde::{Serialize, Deserialize};

#[derive(Deserialize, PartialEq, Serialize, Debug, Clone)]
pub struct RGBA {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32
}

impl Default for RGBA {
  fn default() -> Self { RGBA { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum Channel {
  R,
  G,
  B,
  #[default]
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

    #[test]
    fn rgba_defaults_to_white() {
      assert_eq!(RGBA::default(), RGBA { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
    }

}
