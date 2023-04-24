use serde::Deserialize;

mod parcel;

pub use parcel::Parcel;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// TODO(fran): test this can be deserialized from a json string
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EthNetwork {
    Mainnet,
    Ropsten,
}
