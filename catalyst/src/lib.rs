//! # Catalyst
//!
//! Catalyst is a Rust client library for
//! [Decentraland's Catalyst API](https://decentraland.github.io/catalyst-api-specs/).
//!

mod content_client;
pub use content_client::*;

mod archipelago_client;
pub use archipelago_client::*;

mod global_client;
pub use global_client::*;

mod lambda_client;
pub use lambda_client::*;

mod deployment;
pub use deployment::*;
mod emote;
pub use emote::*;
mod island;
pub use island::*;
mod profile;
pub use profile::*;
mod scene;
pub use scene::*;
mod wearable;
pub use wearable::*;
mod entity;
pub use entity::*;
mod outfits;
pub use outfits::*;

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
