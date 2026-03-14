use rquickjs::{Ctx, Function, Object, Result, Exception};

#[cfg(windows)]
use winreg::{enums::*, RegKey};

#[cfg(not(windows))]
pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let registry = Object::new(ctx.clone())?;
    globals.set("registry", registry)?;
    Ok(())
}

#[cfg(windows)]
pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let registry = Object::new(ctx.clone())?;

    registry.set("readString", Function::new(ctx.clone(), read_string)?)?;
    registry.set("readDword", Function::new(ctx.clone(), read_dword)?)?;
    registry.set("writeString", Function::new(ctx.clone(), write_string)?)?;
    registry.set("writeDword", Function::new(ctx.clone(), write_dword)?)?;
    registry.set("deleteKey", Function::new(ctx.clone(), delete_key)?)?;
    registry.set("deleteValue", Function::new(ctx.clone(), delete_value)?)?;
    registry.set("createKey", Function::new(ctx.clone(), create_key)?)?;
    registry.set("setPolicyString", Function::new(ctx.clone(), set_policy_string)?)?;
    registry.set("setPolicyDword", Function::new(ctx.clone(), set_policy_dword)?)?;

    globals.set("registry", registry)?;
    Ok(())
}

#[cfg(windows)]
fn js_err<E: std::fmt::Display>(ctx: &Ctx<'_>, msg: &str, err: E) -> rquickjs::Error {
    Exception::throw_message(ctx, &format!("{}: {}", msg, err));
    rquickjs::Error::Exception
}

#[cfg(windows)]
fn js_err_msg(ctx: &Ctx<'_>, msg: &str) -> rquickjs::Error {
    Exception::throw_message(ctx, msg);
    rquickjs::Error::Exception
}

#[cfg(windows)]
fn get_hkey(name: &str) -> Option<RegKey> {
    match name.to_uppercase().as_str() {
        "HKCU" | "HKEY_CURRENT_USER" => Some(RegKey::predef(HKEY_CURRENT_USER)),
        "HKLM" | "HKEY_LOCAL_MACHINE" => Some(RegKey::predef(HKEY_LOCAL_MACHINE)),
        "HKCR" | "HKEY_CLASSES_ROOT" => Some(RegKey::predef(HKEY_CLASSES_ROOT)),
        "HKU" | "HKEY_USERS" => Some(RegKey::predef(HKEY_USERS)),
        "HKCC" | "HKEY_CURRENT_CONFIG" => Some(RegKey::predef(HKEY_CURRENT_CONFIG)),
        _ => None,
    }
}

#[cfg(windows)]
fn read_string(ctx: Ctx<'_>, root: String, subkey: String, name: String) -> Result<String> {
    let hkey = get_hkey(&root).ok_or_else(|| js_err_msg(&ctx, "Invalid root key"))?;
    let key = hkey.open_subkey(&subkey).map_err(|e| js_err(&ctx, "Failed to open subkey", e))?;
    let value: String = key.get_value(&name).map_err(|e| js_err(&ctx, "Failed to read string", e))?;
    Ok(value)
}

#[cfg(windows)]
fn read_dword(ctx: Ctx<'_>, root: String, subkey: String, name: String) -> Result<u32> {
    let hkey = get_hkey(&root).ok_or_else(|| js_err_msg(&ctx, "Invalid root key"))?;
    let key = hkey.open_subkey(&subkey).map_err(|e| js_err(&ctx, "Failed to open subkey", e))?;
    let value: u32 = key.get_value(&name).map_err(|e| js_err(&ctx, "Failed to read dword", e))?;
    Ok(value)
}

#[cfg(windows)]
fn write_string(ctx: Ctx<'_>, root: String, subkey: String, name: String, value: String) -> Result<()> {
    let hkey = get_hkey(&root).ok_or_else(|| js_err_msg(&ctx, "Invalid root key"))?;
    let (key, _) = hkey.create_subkey(&subkey).map_err(|e| js_err(&ctx, "Failed to create/open subkey", e))?;
    key.set_value(&name, &value).map_err(|e| js_err(&ctx, "Failed to write string", e))?;
    Ok(())
}

#[cfg(windows)]
fn write_dword(ctx: Ctx<'_>, root: String, subkey: String, name: String, value: u32) -> Result<()> {
    let hkey = get_hkey(&root).ok_or_else(|| js_err_msg(&ctx, "Invalid root key"))?;
    let (key, _) = hkey.create_subkey(&subkey).map_err(|e| js_err(&ctx, "Failed to create/open subkey", e))?;
    key.set_value(&name, &value).map_err(|e| js_err(&ctx, "Failed to write dword", e))?;
    Ok(())
}

#[cfg(windows)]
fn delete_key(ctx: Ctx<'_>, root: String, subkey: String) -> Result<()> {
    let hkey = get_hkey(&root).ok_or_else(|| js_err_msg(&ctx, "Invalid root key"))?;
    hkey.delete_subkey_all(&subkey).map_err(|e| js_err(&ctx, "Failed to delete subkey", e))?;
    Ok(())
}

#[cfg(windows)]
fn delete_value(ctx: Ctx<'_>, root: String, subkey: String, name: String) -> Result<()> {
    let hkey = get_hkey(&root).ok_or_else(|| js_err_msg(&ctx, "Invalid root key"))?;
    let key = hkey.open_subkey_with_flags(&subkey, KEY_SET_VALUE).map_err(|e| js_err(&ctx, "Failed to open subkey for delete", e))?;
    key.delete_value(&name).map_err(|e| js_err(&ctx, "Failed to delete value", e))?;
    Ok(())
}

#[cfg(windows)]
fn create_key(ctx: Ctx<'_>, root: String, subkey: String) -> Result<()> {
    let hkey = get_hkey(&root).ok_or_else(|| js_err_msg(&ctx, "Invalid root key"))?;
    hkey.create_subkey(&subkey).map_err(|e| js_err(&ctx, "Failed to create subkey", e))?;
    Ok(())
}

#[cfg(windows)]
fn set_policy_string(ctx: Ctx<'_>, subkey: String, name: String, value: String) -> Result<()> {
    let path = format!("SOFTWARE\\Policies\\{}", subkey);
    write_string(ctx, "HKLM".to_string(), path, name, value)
}

#[cfg(windows)]
fn set_policy_dword(ctx: Ctx<'_>, subkey: String, name: String, value: u32) -> Result<()> {
    let path = format!("SOFTWARE\\Policies\\{}", subkey);
    write_dword(ctx, "HKLM".to_string(), path, name, value)
}
