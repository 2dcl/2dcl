//! # Catalyst
//!
//! Catalyst is a Rust client library for 
//! [Decentraland's Catalyst API](https://decentraland.github.io/catalyst-api-specs/).
//! 

use serde::{Deserialize};

pub mod server;
mod content_client;
mod lambda_client;
mod parcel;


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use server::Server;

pub use content_client::ContentClient;
pub use lambda_client::LambdaClient;

pub use parcel::Parcel;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ContentId(pub String);

#[derive(Debug, PartialEq, Deserialize)]
pub struct HashId(pub String);
