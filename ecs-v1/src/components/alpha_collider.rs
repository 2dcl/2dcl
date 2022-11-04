use std::fs::copy;
use std::path::Path;
use std::io::Error;

use serde::{Serialize, Deserialize};
use crate::{Anchor, Component};
use crate::color::Channel;

#[derive(Deserialize, Serialize, Debug)]
pub struct AlphaCollider {
    pub sprite: String,
    pub channel: Channel,
    pub anchor: Anchor
}

#[typetag::serde]
impl Component for AlphaCollider 
{
    fn compile(&self, json_path:&Path, build_path: &Path) -> Result<(),Error> {
        
       

        let mut json_path = json_path.to_path_buf();
        json_path.push(&self.sprite);
    
        let mut build_path = build_path.to_path_buf();
        build_path.push(&self.sprite);
        println!("Moving {}, to {}",&json_path.display(),&build_path.display());
        copy(json_path, build_path)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
        let json = include_str!("../../fixtures/components/alpha_collider.json");
        let result = json_to_mp::<&str, AlphaCollider>(json).expect("json to mp failed");
        let expected = load_mp_fixture("fixtures/components/alpha_collider.mp").unwrap();

        assert_eq!(result, expected);
    }
}
