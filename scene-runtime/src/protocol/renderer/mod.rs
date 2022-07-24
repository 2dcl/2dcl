use crate::protocol::protobuf::{CrdtManyMessages, CrdtResponse, PullCrdtRequest};

pub fn SendCrdt(message: CrdtManyMessages) -> CrdtResponse {
  CrdtResponse {

  }
}

pub fn PullCrdt(request: PullCrdtRequest) -> CrdtManyMessages {
  CrdtManyMessages {
    scene_id: "asdad".to_string(),
    payload: Vec::new()
  }
}
