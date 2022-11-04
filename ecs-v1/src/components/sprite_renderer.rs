use serde::{Serialize, Deserialize};
use crate::{Anchor,Component};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpriteRenderer {
    pub sprite: String,
    pub color: [f32; 4],
    pub layer: i32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub anchor: Anchor
}

#[typetag::serde]
impl Component for SpriteRenderer 
{
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
        let json = include_str!("../../fixtures/components/sprite_renderer.json");
        let result = json_to_mp::<&str, SpriteRenderer>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/components/sprite_renderer.mp").unwrap();
       
        assert_eq!(result, expected);
    }
}
