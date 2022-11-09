use core::any::Any;
use std::path::Path;
use std::io::Error;

use serde::{Serialize, Deserialize};
use crate::{Anchor, Component};
use crate::color::Channel;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct MaskCollider {
    pub sprite: String,
    #[serde(default)]
    pub channel: Channel,
    #[serde(default)]
    pub anchor: Anchor
}

#[typetag::serde]
impl Component for MaskCollider 
{
    fn compile(&self, _json_path:&Path, _build_path: &Path) -> Result<(),Error> {
        // let mut json_path = json_path.to_path_buf();
        // json_path.push(&self.sprite);
    
        // let mut build_path = build_path.to_path_buf();
        // build_path.push(&self.sprite);
        // println!("Moving {}, to {}",&json_path.display(),&build_path.display());
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
      can_go_from_json_to_mp::<MaskCollider, _>("components/mask_collider");
    }

    #[test]
    fn supports_optional_values_with_defaults() {
        let json = load_json_fixture("components/mask_collider_optional").unwrap();
        let result : MaskCollider = serde_json::from_str(&json).unwrap();
        assert_eq!(result, MaskCollider {
            sprite: "a_pixel.png".to_string(),
            channel: Channel::A,
            anchor: Anchor::Center
        })
    }

}
