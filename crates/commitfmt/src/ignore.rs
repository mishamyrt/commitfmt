/// Check if the commit message is ignored.
///
/// The following commit messages are ignored:
/// - Merge commit messages
/// - Revert commit messages
pub fn is_ignored_message(commit_message: &str) -> bool {
    commit_message.starts_with("Merge") || commit_message.starts_with("Revert")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_commit_ignored() {
        let merge_messages = vec![
            "Merge branch 'main' into test",
            "Merge branches 'feature/auth' and 'feature/payments' into main",
            "Merge tag 'v1.0.0' into main",
            "Merge pull request #123 from feature/auth",
            "Merge commit 'a1b2c3d' into main",
            "Merge remote-tracking branch 'origin/main'",
            "Merge",
        ];

        for message in merge_messages {
            assert!(
                is_ignored_message(message),
                "Expected merge message to be ignored: '{message}'"
            );
        }
    }

    #[test]
    fn test_revert_commit_ignored() {
        let revert_messages = vec![
            "Revert \"feat: add new feature\"",
            "Revert commit a1b2c3d",
            "Revert changes from PR #123",
            "Revert",
        ];

        for message in revert_messages {
            assert!(
                is_ignored_message(message),
                "Expected revert message to be ignored: '{message}'"
            );
        }
    }

    #[test]
    fn test_regular_commit_not_ignored() {
        let regular_messages = vec![
            "feat: add new feature",
            "fix: resolve bug in authentication",
            "docs: update README",
            "refactor: improve code structure",
            "test: add unit tests",
            "chore: update dependencies",
            "Initial commit",
            "WIP: work in progress",
            "Some random commit message",
            // Edge cases - messages that contain merge/revert but don't start with them
            "feat: merge two functions into one",
            "fix: revert unwanted changes manually",
            "Some commit that mentions Merge in the middle",
            "Another commit about Revert functionality",
        ];

        for message in regular_messages {
            assert!(
                !is_ignored_message(message),
                "Expected regular message to not be ignored: '{message}'"
            );
        }
    }

    #[test]
    fn test_empty_and_whitespace_messages() {
        let edge_cases = vec!["", " ", "\n", "\t", "   \n\t   "];

        for message in edge_cases {
            assert!(
                !is_ignored_message(message),
                "Expected empty/whitespace message to not be ignored: '{message:?}'"
            );
        }
    }

    #[test]
    fn test_case_sensitivity() {
        assert!(is_ignored_message("Merge branch 'main'"));
        assert!(!is_ignored_message("merge branch 'main'"));
        assert!(!is_ignored_message("MERGE branch 'main'"));

        assert!(is_ignored_message("Revert commit"));
        assert!(!is_ignored_message("revert commit"));
        assert!(!is_ignored_message("REVERT commit"));
    }

    #[test]
    fn test_multiline_messages() {
        let multiline_merge =
            "Merge branch 'feature/auth'\n\nThis merge includes authentication improvements";
        let multiline_revert =
            "Revert \"feat: add feature\"\n\nThis reverts commit a1b2c3d due to bugs";
        let multiline_regular =
            "feat: add authentication\n\nImplement OAuth2 flow\nAdd user management";

        assert!(is_ignored_message(multiline_merge));
        assert!(is_ignored_message(multiline_revert));
        assert!(!is_ignored_message(multiline_regular));
    }
}
