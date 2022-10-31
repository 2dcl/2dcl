use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Channel {
  R,
  G,
  B,
  A
}
