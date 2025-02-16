use fabela_core::{runtime::Runtime, vm::Vm};
use std::{error::Error, time::Instant};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let now = Instant::now();

    info!("Starting runtime... ✨");

    let vm = Vm::new().await?;
    info!("Initialized VM in {}ms", now.elapsed().as_millis());

    let runtime = Runtime::new(&vm).await?;
    runtime.start().await?;

    Ok(())
}
