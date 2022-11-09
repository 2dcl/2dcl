use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Size { 
  pub width: u16,
  pub height: u16
}

impl Default for Size {
  fn default() -> Self { 
    Size {
      width: 1,
      height: 1
    }
  }
}

#[cfg(test)]
mod test {
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn can_be_serialized_from_json() {
      can_go_from_json_to_mp::<Size, _>("size");
    }
}