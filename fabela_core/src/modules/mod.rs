pub mod child_process;
pub mod console;
pub mod fs;
pub mod os;
pub mod registry;

use rquickjs::{Ctx, Result};

/// Register all native modules into the QuickJS context.
pub fn register_all(ctx: &Ctx<'_>) -> Result<()> {
    console::register(ctx)?;
    registry::register(ctx)?;
    fs::register(ctx)?;
    os::register(ctx)?;
    child_process::register(ctx)?;
    Ok(())
}
