pub use std::error::Error;

pub struct Vm {}

impl Vm {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Vm {})
    }
}
