use serde::{Serialize, Deserialize};
use crate::Component;
use core::any::Any;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Entity {
    #[serde(skip)]
    pub id: usize,
    pub name: String,
    pub components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(name: String) -> Entity {
        Entity {
            name: name,
            ..Default::default()
        }
    }
}

#[typetag::serde]
impl Component for Entity 
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
      can_go_from_json_to_mp::<Entity, _>("entity");
    }
}
