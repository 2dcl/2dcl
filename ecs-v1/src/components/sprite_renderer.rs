use crate::Vec2;
use crate::{color::RGBA, Anchor, Component};
use core::any::Any;
use imagesize::size;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MAX_SIZE_X: usize = 768;
pub const MAX_SIZE_Y: usize = 768;

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
    fn check(&self, level_id: usize, source_path: &Path) -> Result<(), String> {
        let mut source_path = source_path.to_path_buf();
        source_path.pop();
        source_path.push("assets");
        source_path.push(&self.sprite);

        let image_size = match size(&source_path) {
            Ok(v) => Vec2 {
                x: v.width,
                y: v.height,
            },
            Err(e) => {
                return Err(format!("{} won't be renderer. {}", self.sprite, e));
            }
        };

        if level_id == 0 && (image_size.x > MAX_SIZE_X || image_size.y > MAX_SIZE_Y) {
            Err(format!(
                "{} won't be rendered. Images in the overworld can't be bigger than {}x{}",
                self.sprite, MAX_SIZE_X, MAX_SIZE_Y
            ))
        } else {
            Ok(())
        }
    }

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
