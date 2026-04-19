use rquickjs::{Ctx, Exception, Function, JsLifetime, Object, Result};
use rusqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(rquickjs::class::Trace, JsLifetime)]
#[rquickjs::class]
pub struct Database {
    #[qjs(skip_trace)]
    db: Rc<RefCell<Option<Connection>>>,
}

#[rquickjs::methods]
impl Database {
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'_>, path: String) -> Result<Self> {
        let conn = Connection::open(path).map_err(|e| {
            Exception::throw_message(&ctx, &e.to_string());
            rquickjs::Error::Exception
        })?;

        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")
            .ok();

        Ok(Self {
            db: Rc::new(RefCell::new(Some(conn))),
        })
    }

    pub fn exec(&self, ctx: Ctx<'_>, sql: String) -> Result<()> {
        let conn_ref = self.db.borrow();
        let conn = conn_ref.as_ref().unwrap();

        conn.execute_batch(&sql).map_err(|e| {
            Exception::throw_message(&ctx, &e.to_string());
            rquickjs::Error::Exception
        })?;

        Ok(())
    }
}

pub fn register(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();
    let sqlite = Object::new(ctx.clone())?;

    sqlite.set(
        "open",
        Function::new(
            ctx.clone(),
            |ctx: Ctx<'_>, path: String| -> Result<Database> { Database::new(ctx, path) },
        )?,
    )?;

    globals.set("sqlite", sqlite)?;
    Ok(())
}
