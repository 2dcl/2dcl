use core::any::Any;
use serde::{Serialize, Deserialize};

use crate::Component;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxCollider {
    pub center: [i16; 2],
    pub size: [i16; 2],
}

#[typetag::serde]
impl Component for BoxCollider 
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
        let json = include_str!("../../fixtures/components/box_collider.json");
        let result = json_to_mp::<&str, BoxCollider>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/components/box_collider.mp").unwrap();
      
        assert_eq!(result, expected);


    }
}
