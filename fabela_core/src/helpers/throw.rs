use rquickjs::{Ctx, Result, String};

pub fn throw_error<T>(ctx: &Ctx<'_>, op: &str, err: std::io::Error) -> Result<T> {
    let msg = format!("{op}: {err}");
    Err(ctx.throw(String::from_str(ctx.clone(), &msg)?.into_value()))
}
