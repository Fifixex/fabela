use rquickjs::{Ctx, Function, Object, Result};
use std::fs;

use crate::helpers::throw::throw_error;

pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let fs_obj = Object::new(ctx.clone())?;

    fs_obj.set(
        "readFileSync",
        Function::new(
            ctx.clone(),
            |ctx: Ctx<'_>, path: String| -> Result<String> {
                fs::read_to_string(&path).or_else(|e| throw_error(&ctx, "readFileSync", e))
            },
        )?,
    )?;

    fs_obj.set(
        "writeFileSync",
        Function::new(
            ctx.clone(),
            |ctx: Ctx<'_>, path: String, data: String| -> Result<()> {
                fs::write(&path, &data).or_else(|e| throw_error(&ctx, "writeFileSync", e))
            },
        )?,
    )?;

    fs_obj.set(
        "existsSync",
        Function::new(ctx.clone(), |path: String| -> bool {
            std::path::Path::new(&path).exists()
        })?,
    )?;

    fs_obj.set(
        "mkdirSync",
        Function::new(ctx.clone(), |ctx: Ctx<'_>, path: String| -> Result<()> {
            fs::create_dir_all(&path).or_else(|e| throw_error(&ctx, "mkdirSync", e))
        })?,
    )?;

    fs_obj.set(
        "rmdirSync",
        Function::new(ctx.clone(), |ctx: Ctx<'_>, path: String| -> Result<()> {
            fs::remove_dir_all(&path).or_else(|e| throw_error(&ctx, "rmdirSync", e))
        })?,
    )?;

    fs_obj.set(
        "readdirSync",
        Function::new(
            ctx.clone(),
            |ctx: Ctx<'_>, path: String| -> Result<Vec<String>> {
                let entries =
                    fs::read_dir(&path).or_else(|e| throw_error(&ctx, "readdirSync", e))?;
                let mut result = Vec::new();
                for entry in entries {
                    let entry = entry.or_else(|e| throw_error(&ctx, "readdirSync", e))?;
                    result.push(entry.file_name().to_string_lossy().into_owned());
                }
                Ok(result)
            },
        )?,
    )?;

    fs_obj.set(
        "renameSync",
        Function::new(
            ctx.clone(),
            |ctx: Ctx<'_>, old_path: String, new_path: String| -> Result<()> {
                fs::rename(&old_path, &new_path).or_else(|e| throw_error(&ctx, "renameSync", e))
            },
        )?,
    )?;

    fs_obj.set(
        "copyFileSync",
        Function::new(
            ctx.clone(),
            |ctx: Ctx<'_>, src: String, dest: String| -> Result<()> {
                fs::copy(&src, &dest)
                    .map(|_| ())
                    .or_else(|e| throw_error(&ctx, "copyFileSync", e))
            },
        )?,
    )?;

    globals.set("fs", fs_obj)?;

    Ok(())
}
