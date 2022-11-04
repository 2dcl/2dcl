use core::any::Any;
use std::fs::copy;
use std::path::Path;
use std::io::Error;

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
    fn compile(&self, json_path:&Path, build_path: &Path)  -> Result<(),Error> {
        
        // println!("Moving {}, to {}",&self.sprite,&build_path.display());

        // let mut json_path = json_path.to_path_buf();
        // json_path.push(&self.sprite);
    
        // let mut build_path = build_path.to_path_buf();
        // build_path.push(&self.sprite);

        // println!("Test Moving {}, to {}",json_path.display(),build_path.display());        
        // copy(json_path, build_path)?;
        Ok(())
 
    }

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
        let json = include_str!("../../fixtures/components/sprite_renderer.json");
        let result = json_to_mp::<&str, SpriteRenderer>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/components/sprite_renderer.mp").unwrap();
       
        assert_eq!(result, expected);
    }
}
