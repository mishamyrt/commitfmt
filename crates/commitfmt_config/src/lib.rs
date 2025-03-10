pub mod settings;
pub mod parse;

pub(crate) mod parse_toml;
pub(crate) mod config;

use commitfmt_linter::rules::LinterGroup;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Unable to parse config: {0}")]
    ParseError(String),

    #[error("File error")]
    FileError(String),

    #[error("IO error")]
    IOError(#[from] std::io::Error),

    #[error("Unexpected field type for {0}. Expected {1}")]
    UnexpectedFieldType(String, String),

    #[error("Unexpected value type. Expected: {0}")]
    UnexpectedValueType(String),

    #[error("Unknown rule: {0} → {1}")]
    UnknownRule(LinterGroup, String),

    #[error("Invalid TOML file: {0}")]
    TomlError(String),

    #[error("Too large config file")]
    FileTooLarge,

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("Config file is missing")]
    MissingConfigFile,
}
