//! # Catalyst
//!
//! Catalyst is a Rust client library for
//! [Decentraland's Catalyst API](https://decentraland.github.io/catalyst-api-specs/).
//!

mod content_client;
pub use content_client::ContentClient;

// mod lambda_client;
// pub use lambda_client::LambdaClient;

mod entity;
pub use entity::Entity;
pub use entity::EntityId;
pub use entity::EntityType;

mod content_id;
pub use content_id::ContentId;

mod entity_files;
pub use entity_files::SceneFile;
pub use entity_files::ContentFile;

pub mod server;
pub use server::Server;


