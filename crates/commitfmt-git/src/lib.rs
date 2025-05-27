use thiserror::Error;

mod commit;
mod head;
mod hook;
mod path;
mod repository;

pub use commit::Commit;
pub use hook::HookType;
pub use repository::Repository;

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
}

type GitResult<T> = std::result::Result<T, GitError>;
