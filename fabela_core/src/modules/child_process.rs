use rquickjs::{Ctx, Function, Object, Result};

use crate::helpers::throw::throw_error;

pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let child_process = Object::new(ctx.clone())?;

    child_process.set(
        "exec",
        Function::new(ctx.clone(), |ctx: Ctx<'_>, command: String| -> Result<()> {
            let mut cmd = std::process::Command::new("sh");
            cmd.arg("-c").arg(&command);
            let output = cmd.output().or_else(|e| throw_error(&ctx, "exec", e))?;
            Ok(())
        })?,
    )?;

    globals.set("child_process", child_process)?;
    Ok(())
}
