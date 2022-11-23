use crate::Component;
use std::io::Error;

#[typetag::serde(tag = "type")]
pub trait Trigger: Component {
    fn on_trigger(&self) -> Result<(), Error>;
}
