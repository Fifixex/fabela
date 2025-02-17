use fabela_core::vm::Vm;
use std::{error::Error, process, time::Instant};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let now = Instant::now();

    info!("Starting runtime... ✨");

    let mut vm = Vm::new().await?;
    info!("Initialized VM in {}ms", now.elapsed().as_millis());

    run(&mut vm).await.unwrap();
    Ok(())
}

async fn run(vm: &mut Vm) -> Result<i32, Box<dyn Error>> {
    vm.run_file("./index.js").await?;
    process::exit(1);
}
