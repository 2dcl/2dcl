use crate::Level;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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
}
