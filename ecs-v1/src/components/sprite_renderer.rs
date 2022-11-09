use crate::Vec2;
use core::any::Any;
use serde::{Serialize, Deserialize};
use crate::{Anchor, Component, color::RGBA};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpriteRenderer {
    pub sprite: String,
    pub color: RGBA,
    pub layer: i32,
    pub flip: Vec2<bool>,
    pub anchor: Anchor
}

#[typetag::serde]
impl Component for SpriteRenderer
 {
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
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
      can_go_from_json_to_mp::<SpriteRenderer, _>("components/sprite_renderer");
    }
}
