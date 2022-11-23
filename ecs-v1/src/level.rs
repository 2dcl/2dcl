use crate::{Entity, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Level {
    #[serde(skip)]
    pub id: usize,
    pub name: String,
    #[serde(default)]
    pub dimensions: Vec2<u16>,
    #[serde(default)]
    pub player_layer: i16,
    pub entities: Vec<Entity>,
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::test_utils::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<Level, _>("level");
    }
}
