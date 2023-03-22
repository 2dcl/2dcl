use std::any::Any;
use std::{fmt::Debug, path::Path};

#[typetag::serde(tag = "type")]
pub trait Component: Debug {
    fn compile(&self, _source_path: &Path, _destination_path: &Path) -> Result<(), String> {
        Ok(())
    }

    fn check(&self, _level_id: usize, _source_path: &Path) -> Result<(), String> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any;
}
