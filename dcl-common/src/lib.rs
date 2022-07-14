use serde::Deserialize;

mod parcel;
pub use parcel::Parcel;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ContentId(pub String);

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct HashId(pub String);

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct EntityId(pub String);

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

