use core::any::Any;
use serde::{Serialize, Deserialize};
use crate::Component;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transform {
    pub location: [f32; 2],
    pub rotation: [f32; 3],
    pub scale: [f32; 2],
}

#[typetag::serde]
impl Component for Transform 
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
        let json = include_str!("../../fixtures/components/transform.json");
        let result = json_to_mp::<&str, Transform>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/components/transform.mp").unwrap();
        
        assert_eq!(result, expected);
    }
}
