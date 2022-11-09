use core::any::Any;
use serde::{Serialize, Deserialize};

use crate::{Component, Vec2, Vec3};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transform {
    pub location: Vec2<i32>,
    pub rotation: Vec3<f32>,
    pub scale: Vec2<f32>,
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
}
