//! # Catalyst
//!
//! Catalyst is a Rust client library for
//! [Decentraland's Catalyst API](https://decentraland.github.io/catalyst-api-specs/).
//!

mod content_client;
mod lambda_client;
pub mod server;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use server::Server;

pub use content_client::ContentClient;
pub use lambda_client::LambdaClient;
