pub mod console;
pub mod registry;

use rquickjs::{Ctx, Result};

/// Register all native modules into the QuickJS context.
pub fn register_all(ctx: &Ctx<'_>) -> Result<()> {
    console::register(ctx)?;
    registry::register(ctx)?;
    Ok(())
}
