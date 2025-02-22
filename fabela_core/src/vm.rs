use boa_engine::{Context, JsValue, Source, vm::RuntimeLimits};
use std::{error::Error, path::Path};

pub struct Vm {
    pub runtime: RuntimeLimits,
    pub context: Context,
}

impl Vm {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let context = Context::default();
        let runtime = RuntimeLimits::default();

        Ok(Vm { runtime, context })
    }

    pub async fn run_file(
        &mut self,
        filename: impl AsRef<str>,
    ) -> Result<JsValue, Box<dyn Error + Send + Sync>> {
        let source = Source::from_filepath(Path::new(filename.as_ref()))?;
        let value = self.context.eval(source).unwrap();
        Ok(value)
    }

    pub async fn run(&mut self, source: String) -> Result<JsValue, Box<dyn Error>> {
        let bytes = Source::from_bytes(&source);
        let value = self.context.eval(bytes).unwrap();
        Ok(value)
    }
}
