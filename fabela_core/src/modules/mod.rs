pub mod console;

use rquickjs::{Ctx, Result};

/// Register all native modules into the QuickJS context.
pub fn register_all(ctx: &Ctx<'_>) -> Result<()> {
    console::register(ctx)?;
    Ok(())
}
