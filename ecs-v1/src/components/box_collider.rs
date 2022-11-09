use crate::Vec2;
use crate::Size;
use core::any::Any;
use serde::{Serialize, Deserialize};

use crate::Component;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxCollider {
    pub center: Vec2<i32>,
    pub size: Size,
}

#[typetag::serde]
impl Component for BoxCollider 
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
      can_go_from_json_to_mp::<BoxCollider, _>("components/box_collider");
    }
}
