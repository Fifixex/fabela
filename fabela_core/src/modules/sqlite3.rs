use rquickjs::{Ctx, Exception, Function, JsLifetime, Object, Result};
use rusqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::throw::throw;

type Db = Rc<RefCell<Option<Connection>>>;

#[derive(rquickjs::class::Trace, JsLifetime)]
#[rquickjs::class]
pub struct Database {
    #[qjs(skip_trace)]
    db: Db,
}

fn get_conn<'a>(ctx: &Ctx<'_>, db: &'a Db) -> Result<std::cell::Ref<'a, Option<Connection>>> {
    let conn = db.try_borrow().map_err(|_| {
        Exception::throw_message(ctx, "database busy");
        rquickjs::Error::Exception
    })?;

    if conn.is_none() {
        return throw(ctx, "database closed");
    }

    Ok(conn)
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
        let conn_ref = get_conn(&ctx, &self.db)?;
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
