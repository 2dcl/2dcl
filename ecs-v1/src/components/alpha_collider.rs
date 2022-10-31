use serde::{Serialize, Deserialize};
use crate::Anchor;
use crate::color::Channel;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlphaCollider {
    pub sprite: String,
    pub channel: Channel,
    pub anchor: Anchor
}


#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    use std::fs::File;
    use std::io::Write;


    #[test]
    fn can_be_serialized_from_json() {
        let json = include_str!("../../fixtures/components/alpha_collider.json");
        let expected = load_mp_fixture("fixtures/components/alpha_collider.mp").unwrap();
        let result = json_to_mp::<&str, AlphaCollider>(json).expect("json to mp failed");
        assert_eq!(result, expected);
    }
}
