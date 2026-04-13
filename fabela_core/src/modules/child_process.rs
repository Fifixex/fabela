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

    child_process.set(
        "execSync",
        Function::new(
            ctx.clone(),
            |ctx: Ctx<'_>, command: String| -> Result<Vec<String>> {
                let output = build_command(&command)
                    .output()
                    .or_else(|e| throw_error(&ctx, "execSync", e))?;

                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let status = output.status.code().unwrap_or(-1).to_string();

                Ok(vec![stdout, stderr, status])
            },
        )?,
    )?;

    globals.set("child_process", child_process)?;
    Ok(())
}
