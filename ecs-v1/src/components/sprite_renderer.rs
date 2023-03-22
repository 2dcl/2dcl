use crate::Vec2;
use crate::{color::RGBA, Anchor, Component};
use core::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SpriteRenderer {
    pub sprite: String,
    #[serde(default)]
    pub color: RGBA,
    #[serde(default)]
    pub layer: i32,
    #[serde(default)]
    pub flip: Vec2<bool>,
    #[serde(default)]
    pub anchor: Anchor,
}

#[typetag::serde]
impl Component for SpriteRenderer {
    // fn compile(&self, json_path:&Path, build_path: &Path)  -> Result<(),Error> {

    //     // println!("Moving {}, to {}",&self.sprite,&build_path.display());

    //     // let mut json_path = json_path.to_path_buf();
    //     // json_path.push(&self.sprite);

    //     // let mut build_path = build_path.to_path_buf();
    //     // build_path.push(&self.sprite);

    //     // println!("Test Moving {}, to {}",json_path.display(),build_path.display());
    //     // copy(json_path, build_path)?;
    //     Ok(())

    // }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<SpriteRenderer, _>("components/sprite_renderer");
    }

    #[test]
    fn supports_optional_values_with_defaults() {
        let json = load_json_fixture("components/sprite_renderer_optional").unwrap();
        let result: SpriteRenderer = serde_json::from_str(&json).unwrap();
        assert_eq!(
            result,
            SpriteRenderer {
                sprite: "a_pixel.png".to_string(),
                color: RGBA {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0
                },
                layer: 0,
                flip: Vec2 { x: false, y: false },
                anchor: Anchor::Center
            }
        )
    }
}
