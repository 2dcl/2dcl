use crate::components::Trigger;
use crate::{Component, Vec2};
use core::any::Any;
use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct LevelChange {
    pub level: String,
    #[serde(default)]
    pub spawn_point: Vec2<i32>,
}

#[typetag::serde]
impl Component for LevelChange {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[typetag::serde]
impl Trigger for LevelChange {
    fn on_trigger(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<LevelChange, _>("components/level_change");
    }
}
