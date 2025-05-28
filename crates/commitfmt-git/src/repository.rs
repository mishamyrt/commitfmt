use std::fs;
use std::path::{Path, PathBuf};

use strum::Display;

use crate::commit::parse_git_log;
use crate::head::branch_name_from_head;
use crate::hook::HookType;
use crate::path::{find_root, get_commit_message_file, get_head_file, hooks_dir};
use crate::{Commit, GitError, GitResult};

#[derive(Debug, Clone)]
pub struct Repository {
    root_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, Display)]
pub(crate) enum ConfigKey {
    #[strum(to_string = "trailer.separators")]
    TrailerSeparators,

    #[strum(to_string = "core.commentChar")]
    CommentChar,

    #[strum(to_string = "core.commentString")]
    CommentString,
}

impl Repository {
    /// Creates a new repository from the root directory
    pub fn from_root(root: &Path) -> Repository {
        Repository { root_dir: root.to_path_buf() }
    }

    /// Opens a repository at the given path
    pub fn open(path: &Path) -> GitResult<Repository> {
        match find_root(path) {
            Some(repo_root) => Ok(Repository::from_root(&repo_root)),
            None => Err(GitError::NotFound(path.to_string_lossy().to_string())),
        }
    }

    /// Returns the root directory of the repository
    pub fn get_root(&self) -> PathBuf {
        self.root_dir.clone()
    }

    /// Returns the name of the current branch
    pub fn get_branch_name(&self) -> Option<String> {
        let head_path = get_head_file(&self.root_dir);
        let Ok(head) = fs::read_to_string(head_path) else {
            return None;
        };

        branch_name_from_head(&head).map(std::string::ToString::to_string)
    }

    /// Returns true if a commit is in progress
    pub fn is_committing(&self) -> bool {
        let msg_path = get_commit_message_file(&self.root_dir);
        fs::metadata(msg_path).is_ok()
    }

    /// Returns the commits between two references
    pub fn get_log(&self, from: &str, to: &str) -> Result<Vec<Commit>, std::io::Error> {
        let output = std::process::Command::new("git")
            .arg("log")
            .arg("--pretty=format:%h%n%B#-eoc-#")
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
        let msg_path = get_commit_message_file(&self.root_dir);
        fs::read_to_string(msg_path)
    }

    /// Writes the commit message
    pub fn write_commit_message(&self, msg: &str) -> Result<(), std::io::Error> {
        let msg_path = get_commit_message_file(&self.root_dir);
        fs::write(msg_path, msg)
    }

    /// Returns the path to the hook
    pub fn hook_path(&self, hook: HookType) -> GitResult<PathBuf> {
        let hooks_path = hooks_dir(&self.get_root())?;
        Ok(hooks_path.join(hook.as_str()))
    }

    /// Returns the comment symbol configured for the repository
    ///
    /// Depending on git version and configuration, the comment symbol may be a string or a single character.
    /// Returns `None` if both are not set.
    pub fn comment_symbol(&self) -> Option<String> {
        match get_config(ConfigKey::CommentString) {
            Some(comment_string) => Some(comment_string),
            None => get_config(ConfigKey::CommentChar),
        }
    }

    /// Returns the trailer separators configured for the repository
    pub fn trailer_separators(&self) -> Option<String> {
        get_config(ConfigKey::TrailerSeparators)
    }
}

fn get_config(key: ConfigKey) -> Option<String> {
    let output = std::process::Command::new("git").arg("config").arg(key.to_string()).output();

    if let Ok(output) = output {
        let content = String::from_utf8_lossy(&output.stdout);
        if content.is_empty() {
            None
        } else {
            Some(content.to_string())
        }
    } else {
        None
    }
}
