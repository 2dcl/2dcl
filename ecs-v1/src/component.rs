use std::{fmt::Debug, path::Path};
use std::io::Error;

#[typetag::serde(tag = "type")]

pub trait Component: Debug {

    fn compile(&self, source_path:&Path, destination_path: &Path) -> Result<(),Error>
    {
        Ok(())
    }

    
    fn check(&self) -> Result<(),Error>
    {
        Ok(())
    }
}