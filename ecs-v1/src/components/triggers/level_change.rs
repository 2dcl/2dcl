use crate::{Vec2, Component};
use serde::{Serialize, Deserialize};
use crate::components::Trigger;
use core::any::Any;
use std::io::Error;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LevelChange {
    pub level: String,
    #[serde(default)]
    pub spawn_point: Vec2<i32>,
}

#[typetag::serde]
impl Component for LevelChange 
{
  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[typetag::serde]
impl Trigger for LevelChange 
{
  fn on_trigger(&self) -> Result<(),Error>
  {
    Ok(())
  }
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;
    
    #[test]
    fn can_be_serialized_from_json() {
      can_go_from_json_to_mp::<LevelChange, _>("components/level_change");
    }

}
