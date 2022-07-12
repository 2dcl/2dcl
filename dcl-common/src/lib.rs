use serde::Deserialize;

mod parcel;
pub use parcel::Parcel;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ContentId(pub String);

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct HashId(pub String);
