use std::{path::PathBuf, time::Instant};

use fabela_core::{binary::Binary, error::Result, runtime::Runtime};
use tracing::info;

pub fn run_embedded() -> Result<bool> {
    match Binary::extract_embedded_source().into_diagnostic()? {
        Some(source) => {
            info!("Running embedded script ⚡");
            let now = Instant::now();

            let runtime = Runtime::new().into_diagnostic()?;
            runtime.execute_source(&source).into_diagnostic()?;

            info!("Executed in {}ms", now.elapsed().as_millis());
            Ok(true)
        }
        None => Ok(false),
    }
}

pub fn run_file(file: PathBuf) -> Result<()> {
    let runtime = Runtime::new()?;
    runtime.execute_file(&file.to_string_lossy())?;
    Ok(())
}
