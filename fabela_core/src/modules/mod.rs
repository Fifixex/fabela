pub mod child_process;
pub mod console;
pub mod fs;
pub mod os;
pub mod registry;
pub mod sqlite3;

use rquickjs::{Ctx, Result};

/// Register all native modules into the QuickJS context.
pub fn register_all(ctx: &Ctx<'_>) -> Result<()> {
    console::register(ctx)?;
    registry::register(ctx)?;
    fs::register(ctx)?;
    os::register(ctx)?;
    child_process::register(ctx)?;
    sqlite3::register(ctx)?;
    Ok(())
}
