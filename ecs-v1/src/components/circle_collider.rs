use crate::collision_type::CollisionType;
use crate::Vec2;
use core::any::Any;
use serde::{Serialize, Deserialize};
use crate::Component;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CircleCollider {
    #[serde(default)]
    pub collision_type: CollisionType,
    #[serde(default)]
    pub center: Vec2<i32>,
    #[serde(default = "default_radius")]
    pub radius: u32,
}

#[typetag::serde]
impl Component for CircleCollider 
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn default_radius() -> u32 { 1 }


#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
      can_go_from_json_to_mp::<CircleCollider, _>("components/circle_collider");
    }

    #[test]
    fn supports_optional_values_with_defaults() {
        let json = load_json_fixture("components/circle_collider_optional").unwrap();
        let result : CircleCollider = serde_json::from_str(&json).unwrap();
        assert_eq!(result, CircleCollider {
            collision_type: CollisionType::Solid,
            center: Vec2 { x: 0, y: 0 },
            radius: 1
        })
    }

}
