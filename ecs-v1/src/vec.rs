use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn vec2_can_be_serialized_from_json() {
        can_go_from_json_to_mp::<Vec2<i32>, _>("vec2");
    }

    #[test]
    fn vec3_can_be_serialized_from_json() {
        can_go_from_json_to_mp::<Vec3<i32>, _>("vec3");
    }
}
