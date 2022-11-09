use core::any::Any;
use serde::{Serialize, Deserialize};

use crate::{Component, Vec2, Vec3};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Transform {
    pub location: Vec2<i32>,
    #[serde(default)]
    pub rotation: Vec3<f32>,
    #[serde(default = "default_scale")]
    pub scale: Vec2<f32>,
}

fn default_scale() -> Vec2<f32> {
    Vec2 { x: 1.0, y: 1.0 }
}

#[typetag::serde]
impl Component for Transform 
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
      can_go_from_json_to_mp::<Transform, _>("components/transform");
    }

    #[test]
    fn supports_optional_values_with_defaults() {
        let json = load_json_fixture("components/transform_optional").unwrap();
        let result : Transform = serde_json::from_str(&json).unwrap();
        assert_eq!(result, Transform {
            location: Vec2 { x: 1, y: 1 },
            rotation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            scale: Vec2 { x: 1.0, y: 1.0 }
        })
    }

}
