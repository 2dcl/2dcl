use serde::Deserialize;

mod parcel;
mod scene;

pub use parcel::Parcel;
pub use scene::Scene;
pub use scene::Component;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// TODO(fran): test this can be deserialized from a json string
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EthNetwork {
    Mainnet,
    Ropsten,
}
