use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Unable to find repository root")]
    RootNotFound(PathBuf),

    #[error("Unable to resolve path")]
    NotResolvable(),
}

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("Command exited with error: {0}")]
    InvalidResult(String),

    #[error("Command execution failed")]
    ExecutionFailed(std::io::Error),
}

/// Returns the path to the .git directory of a provided repo root.
pub fn git_directory(path: &Path) -> PathBuf {
    path.join(".git")
}


/// Finds the root directory of a git repository.
/// It traverses up the tree until it finds the .git directory.
pub fn find_root(start_path: &Path) -> Result<PathBuf, PathError> {
    let Ok(mut current) = start_path.canonicalize() else {
        return Err(PathError::NotResolvable());
    };

    loop {
        let git_path = git_directory(&current);
        if git_path.is_dir() {
            return Ok(current);
        }

        match current.parent() {
            Some(parent) => {
                if parent == current {
                    break;
                }
                current = parent.to_path_buf();
            }
            None => break,
        }
    }

    Err(PathError::RootNotFound(current))
}

/// Returns the path to the hooks directory
/// It uses the `git rev-parse --git-path hooks` command so it depends
/// on `git` executable at the $PATH.
pub fn hooks_dir(repo_path: &Path) -> Result<PathBuf, CmdError> {
    let mut cmd = Command::new("git");
    cmd.arg("rev-parse").arg("--git-path").arg("hooks").current_dir(repo_path);

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                let output = String::from_utf8_lossy(&output.stdout);
                let hook_path = PathBuf::from(output.trim());
                Ok(hook_path)
            } else {
                let output = String::from_utf8_lossy(&output.stderr);
                Err(CmdError::InvalidResult(output.to_string()))
            }
        }
        Err(error) => Err(CmdError::ExecutionFailed(error)),
    }
}

#[allow(unused_imports)]
mod tests {
    use std::{fs::create_dir_all, process::Command};
    use tempfile::tempdir;
    use crate::path::hooks_dir;

    use super::find_root;

    #[test]
    fn test_find_root() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        Command::new("git").arg("init").current_dir(dir_path).output().unwrap();

        let result = find_root(dir_path);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str(), dir_path.canonicalize().unwrap().to_str());
    }

    #[test]
    fn test_find_root_from_subdir() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        Command::new("git").arg("init").current_dir(dir_path).output().unwrap();

        let subdir = dir_path.join("a").join("b");

        create_dir_all(&subdir).expect("Unable to create subdirectory");

        let result = find_root(subdir.as_path());

        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str(), dir_path.canonicalize().unwrap().to_str());
    }

    #[test]
    fn test_find_root_not_found() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let result = find_root(dir_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_hooks_dir() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        Command::new("git").arg("init").current_dir(dir_path).output().unwrap();
        Command::new("git").arg("config").arg("core.hooksPath").arg("hooks").current_dir(dir_path).output().unwrap();

        let result = hooks_dir(dir_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str().unwrap(), "hooks".to_string());
    }

    #[test]
    fn test_hooks_dir_not_found() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let result = hooks_dir(dir_path);
        assert!(result.is_err());
    }
}
