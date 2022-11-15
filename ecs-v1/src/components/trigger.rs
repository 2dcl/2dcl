
use std::io::Error;
use crate::{Component, Vec2};

#[typetag::serde(tag = "type")]
pub trait Trigger: Component {

    fn on_trigger(&self)-> Result<(),Error>;
}
