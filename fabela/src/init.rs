/// To debug `fabela`:
/// `FABELA_LOG=fabela fabela --help`
/// # Panics
pub fn init_tracing() {
    use tracing_subscriber::{filter::Targets, prelude::*};

    // Usage without the `regex` feature.
    // <https://github.com/tokio-rs/tracing/issues/1436#issuecomment-918528013>
    tracing_subscriber::registry()
        .with(std::env::var("FABELA_LOG").map_or_else(
            |_| Targets::new(),
            |env_var| {
                use std::str::FromStr;
                Targets::from_str(&env_var).unwrap()
            },
        ))
        .with(
            tracing_subscriber::fmt::layer()
                // https://github.com/tokio-rs/tracing/issues/2492
                .with_writer(std::io::stderr),
        )
        .init();
}
