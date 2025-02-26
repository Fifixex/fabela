use std::error::Error;
use std::fs::File;
use std::path::Path;

use tracing::info;

pub struct BinaryOptions<'a> {
    pub file: File,
    pub entrypoint: &'a Path,
}
pub struct Binary {}

impl Binary {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn load_and_write_binary(
        &self,
        options: BinaryOptions<'_>,
    ) -> Result<(), Box<dyn Error>> {
        let original_binary = self.get_base_binary().await?;
        self.write_standalone_binary(options, original_binary)
            .await?;
        Ok(())
    }

    async fn get_base_binary(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let current_exe = std::env::current_exe()?;
        Ok(std::fs::read(current_exe)?)
    }

    async fn write_standalone_binary(
        &self,
        options: BinaryOptions<'_>,
        original_binary: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        info!("Compiling binary...");

        let BinaryOptions { file, entrypoint } = options;
        let data_section_bytes = b"Hello, World!".to_vec();
        write_binary_bytes(file, original_binary, data_section_bytes).unwrap();
        Ok(())
    }
}

fn write_binary_bytes(
    mut file_writer: File,
    original_binary: Vec<u8>,
    data_section_bytes: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    if cfg!(windows) {
        let pe = libsui::PortableExecutable::from(&original_binary)?;
        pe.write_resource("hello.txt", data_section_bytes)?
            .build(&mut file_writer)?;
    }
    Ok(())
}
