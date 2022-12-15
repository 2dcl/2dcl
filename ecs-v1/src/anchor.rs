use crate::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum Anchor {
  #[default]
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
    Custom(Vec2<i32>),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<Anchor, _>("anchor/center");
        can_go_from_json_to_mp::<Anchor, _>("anchor/bottom_left");
        can_go_from_json_to_mp::<Anchor, _>("anchor/bottom_center");
        can_go_from_json_to_mp::<Anchor, _>("anchor/bottom_right");
        can_go_from_json_to_mp::<Anchor, _>("anchor/center_left");
        can_go_from_json_to_mp::<Anchor, _>("anchor/center_right");
        can_go_from_json_to_mp::<Anchor, _>("anchor/top_left");
        can_go_from_json_to_mp::<Anchor, _>("anchor/top_center");
        can_go_from_json_to_mp::<Anchor, _>("anchor/top_right");
    }

    #[test]
    fn can_serialize_custom_value_from_json() {
        can_go_from_json_to_mp::<Anchor, _>("anchor/custom");
    }
}
