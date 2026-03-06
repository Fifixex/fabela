use std::path::{Path, PathBuf};
use tracing::info;

use crate::binary::Binary;
use crate::error::{FabelaError, IoContext};

pub fn compile(
    input_path: &Path,
    output_path: Option<&Path>,
) -> crate::error::Result<PathBuf> {
    let js_source = std::fs::read_to_string(input_path).io_context(format!("Reading JS file '{}'", input_path.display()))?;
    let output = resolve_output_path(input_path, output_path)?;
    let base_binary = std::env::current_exe().io_context("Resolving fabela executable path")?;

    info!("Compiling {} -> {}", input_path.display(), output.display());

    #[cfg(unix)]
    set_executable_permissions(&output)?;

    info!("Compiled successfully: {}", output.display());
    Ok(output)
}

fn resolve_output_path(input_path: &Path, output_path: Option<&Path>) -> crate::error::Result<PathBuf> {
    if let Some(p) = output_path {
        return Ok(p.to_path_buf());
    }

    let stem = input_path
        .file_stem()
        .ok_or_else(|| FabelaError::Compile(
            format!("File '{}' has no valid name", input_path.display()),
        ))?
        .to_string_lossy();

    let name = if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.into_owned()
    };

    let output = std::env::current_dir()
        .io_context("Resolving working directory")?
        .join(name);

    Ok(output)
}

// Line ref: https://github.com/denoland/deno/blob/3a4ece2ed446448e63baee8ded7d2c660a444495/cli/tools/compile.rs#L179
#[cfg(unix)]
fn set_executable_permissions(path: &Path) -> crate::error::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(path, perms)
        .io_context(format!("Setting permissions on '{}'", path.display()))
}
