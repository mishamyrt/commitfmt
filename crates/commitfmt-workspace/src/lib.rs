mod configuration;
mod rules;
mod settings;

pub use settings::AdditionalFooter;

use commitfmt_linter::rules::LinterGroup;
use thiserror::Error;

pub use settings::{open_settings, CommitSettings, OnConflictAction};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Config file not found in {0}")]
    ConfigNotFound(String),

    #[error("Unable to parse config: {0}")]
    ParseError(String),

    #[error("File error")]
    FileError(String),

    #[error("IO error: {0}")]
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

    #[error("Additional footer value has problem: {0}")]
    BadFooterValue(String),

    #[error("Footer '{0}' value is not found")]
    FooterValueNotFound(String),

    #[error("Footer '{0}' has multiple values")]
    MultipleFooterValues(String),

    #[error("Footer '{0}' has Invalid on conflict action: {1}")]
    InvalidOnConflictAction(String, String),

    #[error("Nested extend is not supported")]
    NestedExtend,

    #[error("Unable to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),

    #[error("Unknown on conflict action: {0}")]
    UnknownOnConflictAction(String),

    #[error("Invalid word case: {0}")]
    InvalidWordCase(String),

    #[error("Invalid text case: {0}")]
    InvalidTextCase(String),

    #[error("Invalid pattern: {0}")]
    InvalidPattern(#[from] regex::Error),

    #[error("Template parsing failed: {0}")]
    TemplateParseError(#[from] commitfmt_tpl::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
