use std::fs;
use std::path::{Path, PathBuf};

use strum::Display;

use crate::command::run_git;
use crate::commit::parse_git_log;
use crate::head::branch_name_from_head;
use crate::path::{find_root, get_commit_message_file, get_head_file, hooks_dir};
use crate::{Commit, GitError, GitResult};

const MESSAGE_CONFIG_PATTERN: &str = r"^(core\.comment(char|string)|trailer\.separators)$";

#[derive(Debug, Clone)]
pub struct Repository {
    root_dir: PathBuf,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RepositoryConfig {
    pub comment_symbol: Option<String>,
    pub trailer_separators: Option<String>,
}

#[derive(Debug, Clone, Copy, Display)]
pub enum HookType {
    #[strum(to_string = "prepare-commit-msg")]
    PrepareCommitMsg,

    #[strum(to_string = "applypatch-msg")]
    ApplyPatchMsg,
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
    pub fn get_log(&self, from: &str, to: &str) -> GitResult<Vec<Commit>> {
        let output =
            self.run(&["log", "--pretty=format:%h%n%B#-eoc-#", &format!("{from}..{to}")])?;

        let Ok((_, commits)) = parse_git_log(&output) else {
            return Err(GitError::CommandFailed(-1, "Failed to parse git log".to_string()));
        };

        Ok(commits)
    }

    pub fn commit(&self, message: &str) -> GitResult<()> {
        self.run(&["commit", "--allow-empty", "--message", message])?;
        Ok(())
    }

    /// Reads the commit message
    pub fn read_commit_message(&self) -> GitResult<String> {
        let msg_path = get_commit_message_file(&self.root_dir);
        fs::read_to_string(msg_path).map_err(GitError::IOError)
    }

    /// Writes the commit message
    pub fn write_commit_message(&self, msg: &str) -> GitResult<()> {
        let msg_path = get_commit_message_file(&self.root_dir);
        fs::write(msg_path, msg).map_err(GitError::IOError)
    }

    /// Returns the path to the hook
    pub fn hook_path(&self, hook: HookType) -> GitResult<PathBuf> {
        let hooks_path = hooks_dir(&self.get_root())?;
        Ok(hooks_path.join(hook.to_string()))
    }

    /// Returns commit-message settings configured for the repository.
    pub fn message_config(&self) -> RepositoryConfig {
        let Ok(output) = self.run(&["config", "--get-regexp", MESSAGE_CONFIG_PATTERN]) else {
            return RepositoryConfig::default();
        };

        parse_message_config(&output)
    }

    /// Runs a git command in the repository
    pub(crate) fn run(&self, args: &[&str]) -> GitResult<String> {
        run_git(args, &self.root_dir)
    }
}

fn parse_message_config(output: &str) -> RepositoryConfig {
    let mut comment_char = None;
    let mut comment_string = None;
    let mut trailer_separators = None;

    for line in output.lines() {
        let Some((key, value)) = line.split_once(' ') else {
            continue;
        };

        if key.eq_ignore_ascii_case("core.commentChar") {
            comment_char = Some(value.to_string());
        } else if key.eq_ignore_ascii_case("core.commentString") {
            comment_string = Some(value.to_string());
        } else if key.eq_ignore_ascii_case("trailer.separators") {
            trailer_separators = Some(value.to_string());
        }
    }

    RepositoryConfig { comment_symbol: comment_string.or(comment_char), trailer_separators }
}

#[cfg(test)]
mod tests {
    use crate::testing::TestBed;

    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn test_open_nested() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        Command::new("git").arg("init").current_dir(dir_path).output().unwrap();

        let subdir = dir_path.join("subdir").join("subsubdir");
        fs::create_dir_all(&subdir).unwrap();

        let result = Repository::open(&subdir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get_root(), dir_path.canonicalize().unwrap());
    }

    #[test]
    fn test_open_not_found() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let result = Repository::open(dir_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitError::NotFound(_)));
    }

    #[test]
    fn test_get_branch_name() {
        let test_bed = TestBed::empty().unwrap();

        let branch_name = test_bed.repo.get_branch_name();

        assert_eq!(branch_name, Some(TestBed::DEFAULT_BRANCH_NAME.to_string()));
    }

    #[test]
    fn test_read_write_commit_message() {
        let test_bed = TestBed::empty().unwrap();

        let test_message = "feat: test commit message\n\nThis is a test commit.";

        // Write message
        let write_result = test_bed.repo.write_commit_message(test_message);
        assert!(write_result.is_ok());

        // Read message
        let read_result = test_bed.repo.read_commit_message();
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), test_message);
    }

    #[test]
    fn test_read_commit_message_not_found() {
        let test_bed = TestBed::empty().unwrap();

        let result = test_bed.repo.read_commit_message();
        assert!(result.is_err());
    }

    #[test]
    fn test_is_committing() {
        let test_bed = TestBed::empty().unwrap();

        assert!(!test_bed.repo.is_committing());

        test_bed.repo.write_commit_message("test commit message").unwrap();

        assert!(test_bed.repo.is_committing());
    }

    #[test]
    fn test_hook_path() {
        let test_bed = TestBed::empty().unwrap();

        let hook_path = test_bed.repo.hook_path(HookType::PrepareCommitMsg);
        assert!(hook_path.is_ok());
        assert!(hook_path.unwrap().to_string_lossy().contains("prepare-commit-msg"));

        let hook_path = test_bed.repo.hook_path(HookType::ApplyPatchMsg);
        assert!(hook_path.is_ok());
        assert!(hook_path.unwrap().to_string_lossy().contains("applypatch-msg"));
    }

    #[test]
    fn test_get_log() {
        let test_bed = TestBed::with_default_history().unwrap();

        let log = test_bed.repo.get_log("HEAD~5", "HEAD").unwrap();
        assert_eq!(log.len(), 5);
    }

    #[test]
    fn test_message_config() {
        let test_bed = TestBed::empty().unwrap();
        test_bed.repo.run(&["config", "--local", "core.commentChar", "#"]).unwrap();
        test_bed.repo.run(&["config", "--local", "core.commentString", "//"]).unwrap();
        test_bed.repo.run(&["config", "--local", "trailer.separators", ":#"]).unwrap();

        assert_eq!(
            test_bed.repo.message_config(),
            RepositoryConfig {
                comment_symbol: Some("//".to_string()),
                trailer_separators: Some(":#".to_string()),
            }
        );
    }

    #[test]
    fn test_parse_message_config() {
        assert_eq!(parse_message_config(""), RepositoryConfig::default());
        assert_eq!(
            parse_message_config("core.commentchar #\ntrailer.separators :#"),
            RepositoryConfig {
                comment_symbol: Some("#".to_string()),
                trailer_separators: Some(":#".to_string()),
            }
        );
        assert_eq!(
            parse_message_config(
                "core.commentchar #\ncore.commentstring // comment\ntrailer.separators ="
            ),
            RepositoryConfig {
                comment_symbol: Some("// comment".to_string()),
                trailer_separators: Some("=".to_string()),
            }
        );
    }

    #[test]
    fn test_hook_type_display() {
        assert_eq!(HookType::PrepareCommitMsg.to_string(), "prepare-commit-msg");
        assert_eq!(HookType::ApplyPatchMsg.to_string(), "applypatch-msg");
    }
}
