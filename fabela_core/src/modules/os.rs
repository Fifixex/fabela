use rquickjs::{Ctx, Function, Object, Result};
use std::env;

pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let os = Object::new(ctx.clone())?;

    os.set("platform", Function::new(ctx.clone(), || env::consts::OS))?;
    os.set("arch", Function::new(ctx.clone(), || env::consts::ARCH))?;

    os.set("EOL", if cfg!(windows) { "\r\n" } else { "\n" })?;

    os.set("homedir", Function::new(ctx.clone(), get_home_dir))?;
    os.set("tmpdir", Function::new(ctx.clone(), get_tmp_dir))?;
    os.set("type", Function::new(ctx.clone(), get_os_type))?;
    os.set("hostname", Function::new(ctx.clone(), get_hostname))?;

    globals.set("os", os)?;
    Ok(())
}

fn get_home_dir() -> String {
    #[cfg(windows)]
    {
        env::var("USERPROFILE")
            .or_else(|_| env::var("HOME"))
            .unwrap_or_default()
    }
    #[cfg(not(windows))]
    {
        env::var("HOME").unwrap_or_default()
    }
}

fn get_tmp_dir() -> String {
    env::temp_dir().to_string_lossy().into_owned()
}

fn get_os_type() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "Windows_NT"
    }
    #[cfg(target_os = "linux")]
    {
        "Linux"
    }
    #[cfg(target_os = "macos")]
    {
        "Darwin"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        env::consts::OS
    }
}

fn get_hostname() -> String {
    #[cfg(windows)]
    {
        env::var("COMPUTERNAME").unwrap_or_default()
    }
    #[cfg(not(windows))]
    {
        hostname::get()
            .map(|h| h.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}
