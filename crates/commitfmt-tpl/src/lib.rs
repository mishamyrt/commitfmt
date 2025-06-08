use thiserror::Error;

mod template;

pub use template::{Segment, Template};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Command produced non-UTF8 output")]
    OutputNotUtf8(#[from] std::str::Utf8Error),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Command execution failed with exit code {0}: {1}")]
    CommandExecutionFailed(i32, String),

    #[error("Template parsing failed: {0}")]
    ParseError(String),

    #[error("Parsing failed due to unclosed tag")]
    UnclosedTag,

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
}

pub type Result<T> = std::result::Result<T, Error>;
