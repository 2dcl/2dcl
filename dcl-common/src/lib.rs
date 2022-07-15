mod parcel;
pub use parcel::Parcel;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


