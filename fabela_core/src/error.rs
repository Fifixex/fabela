use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum FabelaError {
    #[error("{context}: {source}")]
    #[diagnostic(code(fabela::io))]
    Io {
        context: String,
        source: std::io::Error,
    },

    #[error("Runtime Error: {0}")]
    #[diagnostic(
        code(fabela::vm),
        help("Check JS Syntaxis")
    )]
    Vm(String),

    #[error("UTF-8 payload invalid")]
    #[diagnostic(code(fabela::encoding))]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    #[diagnostic(
        code(fabela::compile),
        help("Invalid .js file")
    )]
    Compile(String),

    /// Generic runtime error
    #[error("{0}")]
    #[diagnostic(code(fabela::runtime))]
    Runtime(String),
}

/// Trait custom context error
///
/// # Example
/// ```ignore
/// std::fs::read("file.txt").io_context("Reading file!")?;
/// ```
pub trait IoContext<T> {
    fn io_context(self, context: impl Into<String>) -> std::result::Result<T, FabelaError>;
}

impl<T> IoContext<T> for std::result::Result<T, std::io::Error> {
    fn io_context(self, context: impl Into<String>) -> std::result::Result<T, FabelaError> {
        self.map_err(|source| FabelaError::Io {
            context: context.into(),
            source,
        })
    }
}

pub type Result<T> = std::result::Result<T, FabelaError>;
