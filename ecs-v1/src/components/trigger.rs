
use std::io::Error;
use crate::Component;

#[typetag::serde(tag = "type")]
pub trait Trigger: Component {

    fn on_trigger(&self)-> Result<(),Error>;
}
