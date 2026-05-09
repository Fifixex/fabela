use rquickjs::{Ctx, Exception, Function, JsLifetime, Object, Result, Value};
use rusqlite::{Connection, ToSql, params_from_iter};
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

#[derive(rquickjs::class::Trace, JsLifetime)]
#[rquickjs::class]
pub struct Statement {
    #[qjs(skip_trace)]
    db: Db,
    sql: String,
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

fn prepare_cached<'a>(
    ctx: &Ctx<'_>,
    conn: &'a Connection,
    sql: &str,
) -> Result<rusqlite::CachedStatement<'a>> {
    conn.prepare_cached(sql).map_err(|e| {
        Exception::throw_message(ctx, &e.to_string());
        rquickjs::Error::Exception
    })
}

#[inline]
fn row_to_object<'js>(
    ctx: &Ctx<'js>,
    row: &rusqlite::Row,
    column_names: &[String],
) -> Result<Object<'js>> {
    let obj = Object::new(ctx.clone())?;

    for (i, name) in column_names.iter().enumerate() {
        let val = row.get_ref(i).map_err(|e| {
            Exception::throw_message(ctx, &e.to_string());
            rquickjs::Error::Exception
        })?;

        obj.set(name, sqlite_to_js(ctx.clone(), val)?)?;
    }

    Ok(obj)
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

    pub fn prepare(&self, ctx: Ctx<'_>, sql: String) -> Result<Statement> {
        let _ = get_conn(&ctx, &self.db)?;
        Ok(Statement {
            db: self.db.clone(),
            sql,
        })
    }

    pub fn close(&self, ctx: Ctx<'_>) -> Result<()> {
        let mut conn_ref = self.db.borrow_mut();

        if let Some(conn) = conn_ref.take() {
            conn.close().ok();
        } else {
            return throw(&ctx, "database already closed");
        }

        Ok(())
    }
}

#[rquickjs::methods]
impl Statement {
    pub fn run<'js>(
        &self,
        ctx: Ctx<'js>,
        args: rquickjs::function::Rest<Value<'js>>,
    ) -> Result<Object<'js>> {
        let conn_ref = get_conn(&ctx, &self.db)?;
        let conn = conn_ref.as_ref().unwrap();

        let mut stmt = prepare_cached(&ctx, conn, &self.sql)?;

        let params = js_values_to_params(&ctx, &args.0)?;
        let changes = stmt.execute(params_from_iter(params)).map_err(|e| {
            Exception::throw_message(&ctx, &e.to_string());
            rquickjs::Error::Exception
        })?;

        let obj = Object::new(ctx.clone())?;
        obj.set("changes", changes)?;
        obj.set("lastInsertRowid", conn.last_insert_rowid())?;

        Ok(obj)
    }

    pub fn get<'js>(
        &self,
        ctx: Ctx<'js>,
        args: rquickjs::function::Rest<Value<'js>>,
    ) -> Result<Value<'js>> {
        let conn_ref = get_conn(&ctx, &self.db)?;
        let conn = conn_ref.as_ref().unwrap();

        let mut stmt = prepare_cached(&ctx, conn, &self.sql)?;

        let column_names: Vec<String> = stmt.column_names().iter().map(|c| c.to_string()).collect();

        let params = js_values_to_params(&ctx, &args.0)?;
        let mut rows = stmt.query(params_from_iter(params)).map_err(|e| {
            Exception::throw_message(&ctx, &e.to_string());
            rquickjs::Error::Exception
        })?;

        if let Some(row) = rows.next().map_err(|e| {
            Exception::throw_message(&ctx, &e.to_string());
            rquickjs::Error::Exception
        })? {
            let obj = row_to_object(&ctx, row, &column_names)?;
            Ok(obj.into_value())
        } else {
            Ok(Value::new_null(ctx))
        }
    }

    pub fn all<'js>(
        &self,
        ctx: Ctx<'js>,
        args: rquickjs::function::Rest<Value<'js>>,
    ) -> Result<Value<'js>> {
        let conn_ref = get_conn(&ctx, &self.db)?;
        let conn = conn_ref.as_ref().unwrap();

        let mut stmt = prepare_cached(&ctx, conn, &self.sql)?;

        let column_names: Vec<String> = stmt.column_names().iter().map(|c| c.to_string()).collect();

        let params = js_values_to_params(&ctx, &args.0)?;
        let mut rows = stmt.query(params_from_iter(params)).map_err(|e| {
            Exception::throw_message(&ctx, &e.to_string());
            rquickjs::Error::Exception
        })?;

        let result = rquickjs::Array::new(ctx.clone())?;
        let mut i = 0;

        while let Some(row) = rows.next().map_err(|e| {
            Exception::throw_message(&ctx, &e.to_string());
            rquickjs::Error::Exception
        })? {
            let obj = row_to_object(&ctx, row, &column_names)?;
            result.set(i, obj)?;
            i += 1;
        }

        Ok(result.into_value())
    }
}

fn sqlite_to_js<'js>(ctx: Ctx<'js>, val: rusqlite::types::ValueRef<'_>) -> Result<Value<'js>> {
    match val {
        rusqlite::types::ValueRef::Null => Ok(Value::new_null(ctx)),
        rusqlite::types::ValueRef::Integer(i) => Ok(Value::new_float(ctx, i as f64)),
        rusqlite::types::ValueRef::Real(f) => Ok(Value::new_float(ctx, f)),
        rusqlite::types::ValueRef::Text(s) => {
            let s = std::str::from_utf8(s).unwrap_or("");
            Ok(rquickjs::String::from_str(ctx, s)?.into_value())
        }
        rusqlite::types::ValueRef::Blob(_) => throw(&ctx, "blob not supported"),
    }
}

fn js_values_to_params(ctx: &Ctx<'_>, values: &[Value]) -> Result<Vec<Box<dyn ToSql>>> {
    let mut params: Vec<Box<dyn ToSql>> = Vec::with_capacity(values.len());

    for val in values {
        if val.is_null() || val.is_undefined() {
            params.push(Box::new(rusqlite::types::Null));
        } else if let Some(s) = val.as_string() {
            params.push(Box::new(s.to_string()?));
        } else if let Some(i) = val.as_int() {
            params.push(Box::new(i as i64));
        } else if let Some(f) = val.as_float() {
            params.push(Box::new(f));
        } else if let Some(b) = val.as_bool() {
            params.push(Box::new(b));
        } else {
            return throw(ctx, "unsupported parameter type");
        }
    }

    Ok(params)
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
