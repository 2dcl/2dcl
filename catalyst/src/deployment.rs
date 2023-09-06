use dcl_crypto::AuthChain;
use serde::Deserialize;
use serde::Serialize;

use crate::entity::Metadata;
use crate::EntityId;
use crate::EntityType;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub entity_id: EntityId,
    pub entity_type: EntityType,
    pub pointers: Vec<String>,
    pub auth_chain: AuthChain,
    pub entity_timestamp: f32,
    pub deployment_id: f32,
    pub local_timestamp: f32,
    pub metadata: Metadata,
    pub deployer_address: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwritten_by: Option<String>,
}
