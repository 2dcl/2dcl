use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub enum  CollisionType {
  #[default]
  Solid,
  Trigger
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<CollisionType, _>("collision_type/solid");
        can_go_from_json_to_mp::<CollisionType, _>("collision_type/trigger");
    }
}
