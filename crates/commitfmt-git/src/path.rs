use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{GitError, GitResult};

const COMMIT_MSG_FILE: &str = "COMMIT_EDITMSG";
const HEAD_FILE: &str = "HEAD";

/// Returns the path to the .git directory of a provided repo root.
#[inline]
pub(crate) fn git_directory(repo_dir: &Path) -> PathBuf {
    repo_dir.join(".git")
}

/// Returns the path to the commit message file
#[inline]
pub(crate) fn get_commit_message_file(repo_dir: &Path) -> PathBuf {
    git_directory(repo_dir).join(COMMIT_MSG_FILE)
}

/// Returns the path to the HEAD file
#[inline]
pub(crate) fn get_head_file(repo_dir: &Path) -> PathBuf {
    git_directory(repo_dir).join(HEAD_FILE)
}

/// Finds the root directory of a git repository.
/// It traverses up the tree until it finds the .git directory.
pub(crate) fn find_root(start_path: &Path) -> Option<PathBuf> {
    let Ok(mut current) = start_path.canonicalize() else {
        return None;
    };

    loop {
        let git_path = git_directory(&current);
        if git_path.is_dir() {
            return Some(current);
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

    None
}

/// Returns the path to the hooks directory
/// It uses the `git rev-parse --git-path hooks` command so it depends
/// on `git` executable at the $PATH.
pub(crate) fn hooks_dir(repo_dir: &Path) -> GitResult<PathBuf> {
    let mut cmd = Command::new("git");
    cmd.arg("rev-parse").arg("--git-path").arg("hooks").current_dir(repo_dir);

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                let output = String::from_utf8_lossy(&output.stdout);
                let hook_path = PathBuf::from(output.trim());
                return Ok(repo_dir.join(hook_path));
            }

            let code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stderr);
            Err(GitError::CommandFailed(code, stdout.to_string()))
        }
        Err(error) => Err(GitError::IOError(error)),
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::path::hooks_dir;
    use std::{fs::create_dir_all, process::Command};
    use tempfile::tempdir;

    use super::find_root;

    #[test]
    fn test_find_root() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        Command::new("git").arg("init").current_dir(dir_path).output().unwrap();

        let result = find_root(dir_path);

        assert!(result.is_some());
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

        assert!(result.is_some());
        assert_eq!(result.unwrap().to_str(), dir_path.canonicalize().unwrap().to_str());
    }

    #[test]
    fn test_find_root_not_found() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let result = find_root(dir_path);

        assert!(result.is_none());
    }

    #[test]
    fn test_hooks_dir() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        Command::new("git").arg("init").current_dir(dir_path).output().unwrap();
        Command::new("git")
            .arg("config")
            .arg("core.hooksPath")
            .arg("hooks")
            .current_dir(dir_path)
            .output()
            .unwrap();

        let result = hooks_dir(dir_path);
        assert!(result.is_ok());
        assert!(result.unwrap().ends_with("hooks"));
    }

    #[test]
    fn test_hooks_dir_not_found() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let result = hooks_dir(dir_path);
        assert!(result.is_err());
    }
}
