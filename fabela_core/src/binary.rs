use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct BinaryOptions {
    pub file: File,
    pub output_path: PathBuf,
}
pub struct Binary {}

impl Binary {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn load_and_write_binary(
        &self,
        options: &BinaryOptions,
    ) -> Result<(), Box<dyn Error>> {
        let original_binary = self.get_base_binary(&options.output_path)?;
        self.write_standalone_binary(original_binary).await?;
        Ok(())
    }

    fn get_base_binary(&self, target: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
        let base_binary = std::fs::read(target)?;
        Ok(base_binary)
    }

    async fn write_standalone_binary(
        &self,
        original_binary: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        println!("Compiling binary...");
        Ok(())
    }
}
