use thiserror::Error;

mod command;
mod commit;
mod head;
mod path;
mod repository;
#[cfg(any(test, feature = "testing"))]
pub mod testing;

pub use commit::{Commit, CommitLog};
pub use repository::{Repository, RepositoryConfig};

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Unable to find: {0}")]
    NotFound(String),

    #[error("Unable to resolve: {0}")]
    NotResolvable(String),

    #[error("Command failed with exit code {0}: {1}")]
    CommandFailed(i32, String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid git output: {0}")]
    InvalidOutput(String),
}

type GitResult<T> = std::result::Result<T, GitError>;
