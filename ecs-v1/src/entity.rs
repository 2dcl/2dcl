use serde::{Serialize, Deserialize};
use crate::Component;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Entity {
    #[serde(skip)]
    pub id: usize,
    pub name: String,
    #[serde(default)]
    pub children: Vec<Entity>,
    pub components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(name: String) -> Entity {
        Entity {
            name,
            ..Default::default()
        }
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
