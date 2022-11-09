use crate::Vec2;
use core::any::Any;
use serde::{Serialize, Deserialize};
use crate::Component;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CircleCollider {
    pub center: Vec2<i32>,
    pub radius: u32,
}

#[typetag::serde]
impl Component for CircleCollider 
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
      can_go_from_json_to_mp::<CircleCollider, _>("components/circle_collider");
    }
}
