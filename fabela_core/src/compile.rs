use rand::Rng;
use std::error::Error;
use std::path::{Path, PathBuf};
use tracing::info;

use crate::binary::{Binary, BinaryOptions};

pub async fn compile<P>(path: P) -> Result<(), Box<dyn Error + Send + Sync>>
where
    P: AsRef<Path>,
{
    let binary = Binary::new();
    let entrypoint = path.as_ref();
    let output_path = std::env::current_dir()?.join("fabela");
    let temp_path = get_temp_path(&output_path);
    let file = std::fs::File::create(&temp_path)?;
    binary
        .load_and_write_binary(BinaryOptions { file, entrypoint })
        .await
        .unwrap();

    info!(
        "Compile {} to {}",
        entrypoint.to_string_lossy(),
        output_path.to_string_lossy(),
    );

    Ok(())
}

fn get_temp_path(path: &Path) -> PathBuf {
    let mut temp_filename = path.file_name().unwrap().to_owned();
    temp_filename.push(format!(
        ".tmp-{}",
        faster_hex::hex_encode(&rand::rng().random::<[u8; 8]>(), &mut [0u8; 16]).unwrap()
    ));
    let temp_path = path.with_file_name(temp_filename);
    temp_path
}
