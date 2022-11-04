use std::{fmt::Debug, path::Path};
use std::io::Error;
use std::any::Any;

#[typetag::serde(tag = "type")]
pub trait Component: Debug {

    fn compile(&self, _source_path:&Path, _destination_path: &Path) -> Result<(),Error>
    {
        Ok(())
    }

    
    fn check(&self) -> Result<(),Error>
    {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any;
}
