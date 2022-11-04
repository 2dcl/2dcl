use serde::{Serialize, Deserialize};
use crate::Component;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    #[serde(skip)]
    pub id: usize,
    pub name: String,
    pub components: Vec<Box<dyn Component>>,
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
        let json = include_str!("../fixtures/entity.json");
        let result = json_to_mp::<&str, Entity>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/entity.mp").unwrap();
        
        assert_eq!(result, expected);
    }
}
