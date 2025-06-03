pub(crate) mod logging;

pub use commitfmt::Commitfmt;
pub(crate) mod commitfmt;

pub use logging::setup_logger;

use thiserror::Error;

/// Application error.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Git operation failed: {0}")]
    Git(#[from] commitfmt_git::GitError),

    #[error("Failed to parse commit message: {0}")]
    Parse(#[from] commitfmt_cc::ParseError),

    #[error("Found {0} problems")]
    Lint(usize),

    #[error("Message has {0} problems")]
    Unfixable(usize),

    #[error("Failed to append footers: {0}")]
    AppendFooters(#[from] commitfmt_format::FooterError),

    #[error("Failed to open configuration file: {0}")]
    OpenConfig(#[from] commitfmt_workspace::WorkspaceError),
}

/// Application result.
pub type Result<T> = std::result::Result<T, Error>;

/// Commit range. (from..to)
pub type CommitRange<'a> = (&'a str, &'a str);
