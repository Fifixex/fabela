use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Hello, world!");
    Ok(())
}
