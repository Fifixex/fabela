use rquickjs::{Context, Ctx, Runtime};
use std::path::Path;

use crate::{
    error::{FabelaError, IoContext},
    modules,
};

const MEMORY_LIMIT: usize = 32 * 1024 * 1024; // 32 MB
const STACK_SIZE: usize = 1024 * 1024; // 1 MB

pub struct Vm {
    runtime: Runtime,
    context: Context,
}

impl Vm {
    pub fn new() -> crate::error::Result<Self> {
        let runtime = Runtime::new()
            .map_err(|e| FabelaError::Vm(format!("QuickJS runtime init error: {e}")))?;

        runtime.set_memory_limit(MEMORY_LIMIT);
        runtime.set_max_stack_size(STACK_SIZE);

        let context = Context::full(&runtime)
            .map_err(|e| FabelaError::Vm(format!("QuickJS context creation error: {e}")))?;

        context.with(|ctx| {
            modules::register_all(&ctx).expect("Failed to register native modules");
        });

        Ok(Vm { runtime, context })
    }

    pub fn run_file(&self, filename: impl AsRef<str>) -> crate::error::Result<()> {
        let path = Path::new(filename.as_ref());
        let source = std::fs::read_to_string(path)
            .io_context(format!("Failed to read file '{}'", path.display()))?;
        self.run_source(&source)?;
        Ok(())
    }

    pub fn run_source(&self, source: &str) -> crate::error::Result<()> {
        self.context.with(|ctx| {
            ctx.eval::<(), _>(source)
                .map_err(|e| format_exception(&ctx, e))
        })
    }

    pub fn run_pending_jobs(&self) -> crate::error::Result<()> {
        while self.runtime.is_job_pending() {
            self.runtime
                .execute_pending_job()
                .map_err(|e| FabelaError::Vm(format!("Error executing pending job: {e}")))?;
        }
        Ok(())
    }
}

#[inline]
fn format_exception(ctx: &Ctx<'_>, err: rquickjs::Error) -> FabelaError {
    match err {
        rquickjs::Error::Exception => {
            let exception = ctx.catch();

            if let Some(exc) = exception.as_exception() {
                let name = exc
                    .as_object()
                    .get::<_, Option<String>>("name")
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| "Error".to_string());

                let message = exc.message().unwrap_or_else(|| "Unknown error".to_string());
                let stack = exc.stack().unwrap_or_default();

                if stack.is_empty() {
                    FabelaError::Vm(format!("{name}: {message}"))
                } else {
                    FabelaError::Vm(format!("{name}: {message}\n{stack}"))
                }
            } else {
                FabelaError::Vm("Unknown JavaScript exception".to_string())
            }
        }

        other => FabelaError::Vm(format!("Internal JS error: {other}")),
    }
}
