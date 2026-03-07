use rquickjs::{Ctx, Function, Result};

pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let console = rquickjs::Object::new(ctx.clone())?;

    console.set(
        "log",
        Function::new(
            ctx.clone(),
            |args: rquickjs::function::Rest<rquickjs::Value>| {
                let parts: Vec<String> = args.0.iter().map(|v| format_js_value(v)).collect();
                println!("{}", parts.join(" "));
            },
        )?,
    )?;

    globals.set("console", console)?;
    Ok(())
}

fn format_js_value(value: &rquickjs::Value) -> String {
    if let Some(s) = value.as_string() {
        return s.to_string().unwrap_or_else(|_| "[string]".into());
    }

    if let Some(n) = value.as_int() {
        return n.to_string();
    }

    if let Some(n) = value.as_float() {
        if n.fract().abs() < f64::EPSILON {
            return (n as i64).to_string();
        }
        return n.to_string();
    }

    if let Some(b) = value.as_bool() {
        return b.to_string();
    }

    if value.is_null() {
        return "null".into();
    }

    if value.is_undefined() {
        return "undefined".into();
    }

    if let Some(arr) = value.as_array() {
        let items: Vec<String> = arr
            .iter::<rquickjs::Value>()
            .filter_map(|r| r.ok())
            .map(|v| format_js_value(&v))
            .collect();

        let mut out = String::from("[ ");
        out.push_str(&items.join(", "));
        out.push_str(" ]");
        return out;
    }

    if value.is_object() {
        return "[Object]".into();
    }

    format!("{value:?}")
}
