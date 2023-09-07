use serde::Deserialize;
use serde::Serialize;

pub type IslandId = String;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Island {
    pub id: IslandId,
    pub peers: Vec<Peer>,
    pub max_peers: f32,
    pub center: [f32; 3],
    pub radius: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub last_ping: f32,
    pub parcel: [f32; 2],
    pub position: [f32; 3],
}

#[cfg(test)]
mod test {
    use crate::island::{Island, Peer};

    #[test]
    fn profile_deserializes_correctly() {
        let response = include_str!("../fixtures/island.json");
        let island: Island = serde_json::from_str(response).unwrap();
        let expected = Island {
            id: "id".to_string(),
            peers: vec![Peer {
                id: "id".to_string(),
                address: "address".to_string(),
                last_ping: 1694118057696.,
                parcel: [0., 0.],
                position: [0., 0., 0.],
            }],
            max_peers: 100.,
            center: [0., 0., 0.],
            radius: 0.,
        };
        assert_eq!(island, expected);
    }
}
