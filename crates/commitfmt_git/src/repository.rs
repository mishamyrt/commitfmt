use std::fs;
use std::path::{Path, PathBuf};

use crate::commit::parse_git_log;
use crate::head::branch_name_from_head;
use crate::hook::HookType;
use crate::path::{find_root, git_directory, hooks_dir, CmdError, PathError};
use crate::Commit;

const COMMIT_MSG: &str = "COMMIT_EDITMSG";

#[derive(Debug)]
pub struct Repository {
    git_dir: PathBuf,
}

impl Repository {
    /// Creates a new repository from the root directory
    pub fn from_root(root: &Path) -> Repository {
        Repository { git_dir: git_directory(root) }
    }

    /// Opens a repository at the given path
    pub fn open(path: &Path) -> Result<Repository, PathError> {
        match find_root(path) {
            Ok(repo_root) => Ok(Repository::from_root(&repo_root)),
            Err(err) => Err(err),
        }
    }

    /// Returns the root directory of the repository
    pub fn get_root(&self) -> PathBuf {
        self.git_dir.clone()
    }

    /// Returns the name of the current branch
    pub fn get_branch_name(&self) -> Option<String> {
        let head_path = self.git_dir.join("HEAD");
        let Ok(head) = fs::read_to_string(head_path) else {
            return None;
        };

        branch_name_from_head(&head).map(std::string::ToString::to_string)
    }

    /// Returns true if a commit is in progress
    pub fn is_committing(&self) -> bool {
        let msg_path = self.git_dir.join(COMMIT_MSG);
        fs::metadata(msg_path).is_ok()
    }

    pub fn get_commits(&self, from: &str, to: &str) -> Result<Vec<Commit>, std::io::Error> {
        let output = std::process::Command::new("git")
            .arg("log")
            .arg("--pretty=format:%h%n%B#")
            .arg(format!("{from}..{to}"))
            .output()?;

        let Ok((_, commits)) = parse_git_log(&String::from_utf8_lossy(&output.stdout)) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse git log",
            ));
        };

        Ok(commits)
    }

    /// Reads the commit message
    pub fn read_commit_message(&self) -> Result<String, std::io::Error> {
        let msg_path = self.git_dir.join(COMMIT_MSG);
        fs::read_to_string(msg_path)
    }

    /// Writes the commit message
    pub fn write_commit_message(&self, msg: &str) -> Result<(), std::io::Error> {
        let msg_path = self.git_dir.join(COMMIT_MSG);
        fs::write(msg_path, msg)
    }

    /// Returns the path to the hook
    pub fn hook_path(&self, hook: HookType) -> Result<PathBuf, CmdError> {
        match hooks_dir(&self.get_root()) {
            Ok(hooks_path) => Ok(hooks_path.join(hook.as_str())),
            Err(err) => Err(err),
        }
    }
}
