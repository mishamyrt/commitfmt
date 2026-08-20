use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, ChildStdout, Command, Stdio};

use crate::{GitError, GitResult};

fn string_from_git(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(value) => value,
        Err(err) => String::from_utf8_lossy(err.as_bytes()).into_owned(),
    }
}

/// Represents a Git commit parsed from log output.
#[derive(Debug, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub message: String,
}

/// Streaming iterator over commits produced by `git log`.
pub struct CommitLog {
    child: Child,
    stdout: BufReader<ChildStdout>,
    finished: bool,
}

impl CommitLog {
    pub(crate) fn spawn(dir: &Path, from: &str, to: &str) -> GitResult<Self> {
        let range = format!("{from}..{to}");
        let mut child = Command::new("git")
            .args(["log", "-z", "--format=%h%x00%B", &range])
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().expect("stdout is configured as piped");

        Ok(Self { child, stdout: BufReader::new(stdout), finished: false })
    }

    fn read_field(&mut self) -> GitResult<Option<Vec<u8>>> {
        let mut field = Vec::new();
        if self.stdout.read_until(0, &mut field)? == 0 {
            return Ok(None);
        }
        if field.pop() != Some(0) {
            return Err(GitError::InvalidOutput("unterminated git log field".to_string()));
        }
        Ok(Some(field))
    }

    fn finish(&mut self) -> GitResult<()> {
        let mut stderr = Vec::new();
        if let Some(mut stream) = self.child.stderr.take() {
            stream.read_to_end(&mut stderr)?;
        }
        let status = self.child.wait()?;
        self.finished = true;

        if status.success() {
            return Ok(());
        }
        Err(GitError::CommandFailed(
            status.code().unwrap_or(-1),
            String::from_utf8_lossy(&stderr).into_owned(),
        ))
    }

    fn fail(&mut self, err: GitError) -> GitResult<Commit> {
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.finished = true;
        Err(err)
    }
}

impl Iterator for CommitLog {
    type Item = GitResult<Commit>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let sha = match self.read_field() {
            Ok(Some(sha)) => sha,
            Ok(None) => return self.finish().err().map(Err),
            Err(err) => return Some(self.fail(err)),
        };
        let message = match self.read_field() {
            Ok(Some(message)) => message,
            Ok(None) => {
                return Some(self.fail(GitError::InvalidOutput(
                    "git log ended before the commit message".to_string(),
                )))
            }
            Err(err) => return Some(self.fail(err)),
        };

        Some(Ok(Commit { sha: string_from_git(sha), message: string_from_git(message) }))
    }
}

impl Drop for CommitLog {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::string_from_git;

    #[test]
    fn test_string_from_git_replaces_invalid_utf8() {
        assert_eq!(string_from_git(vec![b'f', 0x80, b'o']), "f\u{fffd}o");
    }
}
