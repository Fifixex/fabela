use crate::vm::Vm;
pub use std::error::Error;

pub struct Runtime {}

impl Runtime {
    pub async fn new(vm: &Vm) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Runtime {})
    }

    pub async fn start(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}
