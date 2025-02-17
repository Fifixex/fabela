use fabela_core::vm::Vm;
use std::{env, error::Error, time::Instant};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let now = Instant::now();

    info!("Starting runtime... ✨");

    let mut vm = Vm::new().await?;
    info!("Initialized VM in {}ms", now.elapsed().as_millis());

    vm.run_file("./index.js").await;
    Ok(())
}
