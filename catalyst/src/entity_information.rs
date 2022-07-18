use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EntityInformation {
  pub version: String,
  pub local_timestamp: u64, // TODO(fran): use chrono?
  pub auth_chain: Vec<AuthChain>
}

#[derive(Debug, Deserialize,  PartialEq)]
pub struct AuthChain {
  #[serde(rename="type")]
  pub kind: AuthChainType,
  pub payload: String,
  #[serde(skip)]
  pub signature: String
}

#[derive(Debug, Deserialize,  PartialEq)]
/// From https://github.com/decentraland/kernel-interface/blob/3816b9124d6e45f67146c4f586bbba0d977ddbae/src/dcl-crypto.ts#L37
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthChainType {
  Signer,
  EcdsaEphemeral,
  EcdsaSignedEntity,
  #[serde(rename="ECDSA_EIP_1654_EPHEMERAL")]
  EcdsaEip1654Ephemeral,
  EcdsaEip1654SignedEntity
}


#[cfg(test)]
mod test {
  use crate::EntityInformation;

  #[test]
  fn it_deserializes_from_json() {
    let json = include_str!("../fixtures/audit_scene_result.json");
    let result : EntityInformation = serde_json::from_str(json).unwrap();
    assert_eq!(result.version, "v3");
    assert_eq!(result.local_timestamp, 1657830110701);
  }
}
