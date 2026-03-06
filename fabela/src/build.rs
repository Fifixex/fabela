use std::{fs, path::PathBuf, time::Instant};

use fabela_core::{compile::compile, error::{FabelaError, Result}};
use tracing::info;


pub fn build_file(
    file: PathBuf,
    output: Option<PathBuf>,
) -> Result<()> {
    let now = Instant::now();

    let output_path = compile(&file, output.as_deref())?;

    let size = fs::metadata(&output_path)
        .map_err(|e| FabelaError::Io {
            context: format!("Leyendo metadata de '{}'", output_path.display()),
            source: e,
        })?
        .len();


    // Line ref: https://github.com/farm-fe/farm/blob/main/crates/plugin_file_size/src/lib.rs#L39
    println!(
        "Built: {} ({:.2} MB)",
        output_path.display(),
        size as f64 / (1024.0 * 1024.0)
    );

    info!("Builded in {}ms", now.elapsed().as_millis());

    Ok(())
}
