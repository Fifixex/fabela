use std::process::Command;

use rquickjs::{Ctx, Function, Object, Result};

use crate::helpers::throw::throw_error;

fn build_command(command: &str) -> Command {
    #[cfg(windows)]
    {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", command]);
        cmd
    }

    #[cfg(not(windows))]
    {
        let mut cmd = Command::new("sh");
        cmd.args(["-c", command]);
        cmd
    }
}

pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let child_process = Object::new(ctx.clone())?;

    let ctx_clone = ctx.clone();
    child_process.set(
        "execSync",
        Function::new(ctx.clone(), move |command: String| -> Result<Object> {
            let output = build_command(&command)
                .output()
                .or_else(|e| throw_error(&ctx_clone, "execSync", e))?;

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status = output.status.code().unwrap_or(-1);

            let result = Object::new(ctx_clone.clone())?;
            result.set("stdout", stdout)?;
            result.set("stderr", stderr)?;
            result.set("status", status)?;

            Ok(result)
        })?,
    )?;

    globals.set("child_process", child_process)?;
    Ok(())
}
