use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CircleCollider {
    pub center: [i16; 2],
    pub radius: i32,
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
        let json = include_str!("../../fixtures/components/circle_collider.json");
        let result = json_to_mp::<&str, CircleCollider>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/components/circle_collider.mp").unwrap();
       
        assert_eq!(result, expected);
    }
}
