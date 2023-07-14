use rmp_serde::Deserializer;
use crate::Level;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use dcl_common::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    #[serde(skip)]
    pub id: usize,
    #[serde(default = "timestamp_default")]
    pub timestamp: SystemTime,
    pub name: String,
    pub levels: Vec<Level>,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            id: usize::default(),
            timestamp: timestamp_default(),
            name: String::default(),
            levels: Vec::default(),
        }
    }
}

impl Scene {
    pub fn from_mp(data: &Vec<u8>) -> Result<Scene> { 
        let mut de = Deserializer::from_read_ref(data);
        let deserialized_scene: Scene = 
            Deserialize::deserialize(&mut de)?;
        Ok(deserialized_scene)
    }

    pub fn from_json(data: String) -> Result<Scene> { 
        Ok(serde_json::from_str(&data)?)
    }
}

fn timestamp_default() -> SystemTime {
    SystemTime::now()
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::test_utils::*;

    #[test]
    fn can_be_serialized_from_json() {
        can_go_from_json_to_mp::<Scene, _>("scene");
    }

    #[test]
    fn deserialize_from_mp() {
        let scene_mp = load_mp_fixture("scene").unwrap();
        let scene_json = load_json_fixture("scene").unwrap();

        let scene_from_mp = Scene::from_mp(&scene_mp).unwrap();
        let scene_from_json = Scene::from_json(scene_json).unwrap();

        assert_eq!(scene_from_mp.name, scene_from_json.name)
    }
}
