use std::path::PathBuf;

use tempfile::TempDir;

use crate::command::run_git;
use crate::repository::Repository;
use crate::GitResult;

pub const COMMIT_HISTORY: &[&str] = &[
    "chore: initial commit",
    "feat(linter): remove description leading space rule",
    "feat(cc): trim description on parse",
    "fix: resolve parsing issue with special characters",
    "docs: update README with usage examples",
    "feat(cli): add new --verbose flag for detailed output",
    "test: add unit tests for commit message validation",
    "refactor: simplify configuration loading logic",
    "feat(format): implement auto-formatting for commit headers",
    "fix(git): handle empty repository edge case",
    "chore: update dependencies to latest versions",
    "feat: add support for custom commit templates",
    "fix(parser): correctly handle multiline commit bodies",
    "docs(api): add comprehensive API documentation",
    "feat(workspace): implement configuration inheritance",
    "test(integration): add end-to-end testing suite",
    "fix: prevent duplicate footers in formatted messages",
    "perf: optimize commit log parsing performance",
    "feat(hooks): add support for commit-msg hook",
    "ci: setup automated testing workflow",
];

pub struct TestBed {
    dir: TempDir,
    pub repo: Repository,
}

impl TestBed {
    pub const DEFAULT_BRANCH_NAME: &str = "main";
    pub const DEFAULT_USER_NAME: &str = "Test User";
    pub const DEFAULT_USER_EMAIL: &str = "test@example.com";

    /// Creates a new test bed with a fresh git repository
    pub fn empty() -> GitResult<Self> {
        let dir = TempDir::with_prefix("commitfmt-git-test-")?;
        let dir_path = dir.path();

        run_git(&["init"], dir_path)?;
        run_git(&["config", "user.name", Self::DEFAULT_USER_NAME], dir_path)?;
        run_git(&["config", "user.email", Self::DEFAULT_USER_EMAIL], dir_path)?;
        run_git(&["switch", "-c", Self::DEFAULT_BRANCH_NAME], dir_path)?;

        let repo = Repository::from_root(dir_path);

        Ok(Self { dir, repo })
    }

    /// Returns the path to the test bed
    pub fn path(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }

    /// Switches to a new branch
    pub fn switch_to_new(&self, branch_name: &str) -> GitResult<()> {
        run_git(&["switch", "-c", branch_name], self.dir.path())?;
        Ok(())
    }

    pub fn with_default_history() -> GitResult<Self> {
        let test_bed = Self::empty()?;
        for message in COMMIT_HISTORY {
            test_bed.repo.commit(message)?;
        }
        Ok(test_bed)
    }

    pub fn with_history(commits: &[&str]) -> GitResult<Self> {
        let test_bed = Self::empty()?;
        for message in commits {
            test_bed.repo.commit(message)?;
        }
        Ok(test_bed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let test_bed = TestBed::empty().unwrap();
        assert!(test_bed.path().to_str().unwrap().contains("commitfmt-git-test-"));
        assert_eq!(test_bed.repo.get_branch_name(), Some("main".to_string()));
        assert_eq!(test_bed.repo.get_root(), test_bed.dir.path());
    }

    #[test]
    fn test_new_with_history() {
        let test_bed = TestBed::with_default_history().unwrap();
        assert_eq!(test_bed.repo.get_branch_name(), Some("main".to_string()));
        assert_eq!(test_bed.repo.get_root(), test_bed.dir.path());
        assert_eq!(
            test_bed.repo.run(&["rev-list", "--count", "--all"]).unwrap(),
            COMMIT_HISTORY.len().to_string()
        );
    }
}
