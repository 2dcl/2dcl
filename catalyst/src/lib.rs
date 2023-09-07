//! # Catalyst
//!
//! Catalyst is a Rust client library for
//! [Decentraland's Catalyst API](https://decentraland.github.io/catalyst-api-specs/).
//!

mod content_client;
pub use content_client::*;

mod archipelago_client;
pub use archipelago_client::*;
// mod lambda_client;
// pub use lambda_client::LambdaClient;

pub mod deployment;
pub mod emote;
pub mod island;
pub mod profile;
pub mod scene;
pub mod wearable;
pub mod entity;

mod content_id;
pub use content_id::ContentId;

mod server;
pub use server::Server;

pub mod status;

pub mod entity_information;
pub mod snapshot;

// Represents an id in the form of a hash, used for content files and entities.
pub type HashId = String;

// Represents a content file using the resource name
pub type Urn = String;
