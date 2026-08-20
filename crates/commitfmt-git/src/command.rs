use std::{path::Path, process::Command};

use crate::{GitError, GitResult};

/// Runs a git command and returns the output
pub(crate) fn run_git(args: &[&str], dir: &Path) -> GitResult<String> {
    let output = Command::new("git").args(args).current_dir(dir).output()?;
    if !output.status.success() {
        return Err(GitError::CommandFailed(
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    match String::from_utf8(output.stdout) {
        Ok(mut stdout) => {
            stdout.truncate(stdout.trim_end().len());
            Ok(stdout)
        }
        Err(err) => Ok(String::from_utf8_lossy(err.as_bytes()).trim_end().to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dir() -> &'static Path {
        #[cfg(unix)]
        {
            Path::new("/tmp")
        }
        #[cfg(windows)]
        {
            Path::new("C:\\Windows")
        }
    }

    #[test]
    fn test_run_git() {
        let output = run_git(&["version"], get_dir()).unwrap();
        assert!(output.contains("git version"));
    }

    #[test]
    fn test_run_git_error() {
        let result = run_git(&["invalid-command"], get_dir());
        assert!(result.is_err());
    }
}
