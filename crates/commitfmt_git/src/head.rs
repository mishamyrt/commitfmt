const HEAD_PREFIX: &str = "ref: refs/heads/";

/// Extracts the name of the current branch from the HEAD file.
/// Returns None if the HEAD file does not contain a branch name (e.g. detached HEAD)
pub fn branch_name_from_head(head: &str) -> Option<&str> {
    if !head.starts_with(HEAD_PREFIX) {
        return None;
    }

    Some(head.trim_start_matches(HEAD_PREFIX))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_name_from_head() {
        let head = "ref: refs/heads/main";
        let result = branch_name_from_head(head);
        assert_eq!(result, Some("main"));
    }

    #[test]
    fn test_branch_name_from_commit() {
        let head = "f1c61c8f8120c1c45031f3eb675af9dfd91dd830";
        let result = branch_name_from_head(head);
        assert_eq!(result, None);
    }

    #[test]
    fn test_branch_name_from_empty() {
        let head = "";
        let result = branch_name_from_head(head);
        assert_eq!(result, None);
    }
}
