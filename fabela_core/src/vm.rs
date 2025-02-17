use std::error::Error;
use boa_engine::{vm::RuntimeLimits, Context, JsValue, Source};

pub struct Vm {
    pub runtime: RuntimeLimits,
    pub context: Context
}

impl Vm {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let context = Context::default();
        let runtime = RuntimeLimits::default();

        Ok(Vm {
            runtime,
            context
        })
    }

    pub async fn run_file(&mut self, filename: impl AsRef<str>) {
        let source = [
            r#"import(""#,
            &filename.as_ref().replace('\\', "/"),
            r#"").catch((e) => {console.error(e);process.exit(1)})"#,
        ]
        .concat();
        let _ = self.run(source).await;
    }

    pub async fn run(&mut self, source: String) -> Result<JsValue, Box<dyn Error>> {
        let bytes = Source::from_bytes(&source);
        let value = self.context.eval(bytes).unwrap();
        println!("{:?}", value);
        Ok(value)
    }

}
