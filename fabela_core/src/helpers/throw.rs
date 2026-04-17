use rquickjs::{Ctx, Exception, Result};

/// Generic errors
pub fn throw<T>(ctx: &Ctx<'_>, msg: impl ToString) -> Result<T> {
    Exception::throw_message(ctx, &msg.to_string());
    Err(rquickjs::Error::Exception)
}

/// IO errors
pub fn throw_error<T>(ctx: &Ctx<'_>, op: &str, err: std::io::Error) -> Result<T> {
    throw(ctx, format!("{op}: {err}"))
}
