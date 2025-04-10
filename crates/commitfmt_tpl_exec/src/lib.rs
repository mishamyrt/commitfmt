use nom::error::Error as NomError;
use thiserror::Error;

pub(crate) mod parse;
mod render;

pub use render::render;

#[derive(Error, Debug)]
pub enum TplError {
    #[error("IO error during command execution")]
    CommandIOError(#[from] std::io::Error),

    #[error("Command failed with output: {0}")]
    CommandFailed(String),

    #[error("Command produced non-UTF8 output")]
    OutputNotUtf8(#[from] std::str::Utf8Error),

    #[error("Template parsing failed: {0}")]
    ParseError(String),
}

// Helper to convert Nom's error to our custom error type
// Note: This loses some specific Nom context but avoids lifetime complexity.
impl<'a> From<NomError<&'a str>> for TplError {
    fn from(err: NomError<&'a str>) -> Self {
        TplError::ParseError(format!(
            "Parsing failed near: '{}', error code: {:?}",
            err.input, err.code
        ))
    }
}
