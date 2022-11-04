use std::fmt::Debug;


#[typetag::serde(tag = "type")]

pub trait Component: Debug {}