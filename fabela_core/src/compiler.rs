use rand::Rng;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::binary::{Binary, BinaryOptions};

pub async fn compile<P>(path: P) -> Result<(), Box<dyn Error + Send + Sync>>
where
    P: AsRef<Path>,
{
    let binary = Binary::new();
    let path = path.as_ref();
    let temp_path = get_temp_path(&path);
    let file = std::fs::File::create(&temp_path)?;
    let output_path = Path::new("foo.txt").to_path_buf();
    let writer = binary
        .load_and_write_binary(&BinaryOptions { file, output_path })
        .await;
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
