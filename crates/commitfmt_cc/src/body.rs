use crate::footer::Footer;

const COMMENT_CHAR: &str = "#";
const OLD_CONFLICTS_TITLE: &str = "Conflicts:";

/// Return the offset of `line` in `input`
#[allow(clippy::cast_sign_loss)]
unsafe fn line_offset(input: &str, line: &str) -> usize {
    let ptr = line.as_ptr();
    let offset = ptr.offset_from(input.as_ptr());

    offset as usize
}

trait MeaninglessTrimmer {
    fn trim_meaningless_start(&self) -> &str;
    fn trim_meaningless_end(&self) -> &str;
}

/// Remove meaningless lines from the start and end of the string
impl MeaninglessTrimmer for str {
    /// Remove meaningless lines from the start of the string.
    ///
    /// Meaningless lines are:
    /// - Empty lines
    /// - Comments (lines starting with `#`)
    fn trim_meaningless_start(&self) -> &str {
        for line in self.lines() {
            let offset = unsafe { line_offset(self, line) };
            if !(line.is_empty() || line.starts_with(COMMENT_CHAR)) {
                return &self[offset..].trim_start();
            }
        }

        ""
    }

    fn trim_meaningless_end(&self) -> &str {
        let mut meaningful_end = self.len();
        let mut in_old_conflicts_block = false;

        for line in self.lines() {
            let offset = unsafe { line_offset(self, line) };

            if line.starts_with(COMMENT_CHAR) {
                if meaningful_end == self.len() {
                    meaningful_end = offset;
                }
            } else if line == OLD_CONFLICTS_TITLE {
                in_old_conflicts_block = true;
                if meaningful_end == self.len() {
                    meaningful_end = offset;
                }
            } else if in_old_conflicts_block && line.starts_with('\t') {
                // Part of the conflict block, so we ignore it
            } else if !line.trim().is_empty() {
                // Reset the meaningful_end if a new meaningful line is encountered
                meaningful_end = self.len();
            }
        }

        self[..meaningful_end].trim_end()
    }
}

/// Parse body and footer
pub(crate) fn parse_body(input: &str, footer_separators: &str) -> (Option<String>, Option<Vec<Footer>>) {
    if input.is_empty() {
        return (None, None);
    }

    let meaningful_input = input.trim_meaningless_end();

    // Try to find last block of text.
    // If no block is found, than input is single block.
    let last_block_index: usize = meaningful_input.rfind("\n\n").unwrap_or(0);
    if last_block_index == 0 {
        match Footer::parse(meaningful_input.trim_meaningless_start(), footer_separators) {
            Ok((_rest, footers)) => return (None, Some(footers)),
            Err(_) => return (Some(input.to_string()), None),
        }
    }

    let last_block = &meaningful_input[last_block_index + 2..];

    match Footer::parse(last_block.trim_meaningless_start(), footer_separators) {
        Ok((_rest, footers)) => {
            let body = Some(meaningful_input[..last_block_index].to_string());
            (body, Some(footers))
        }
        Err(_) => (Some(meaningful_input.to_string()), None),
    }
}

#[cfg(test)]
mod tests {
    use crate::footer::{Footer, SeparatorAlignment};

    use super::*;

    #[test]
    fn test_trim_meaningless_end() {
        let input = "my body\n\nmyfooter: my value";
        assert_eq!(input.trim_meaningless_end(), "my body\n\nmyfooter: my value");

        let input = "my body\nConflicts:\n\tfile1\n\tfile2";
        assert_eq!(input.trim_meaningless_end(), "my body");

        let input = "my body\n# some comment\n# another comment";
        assert_eq!(input.trim_meaningless_end(), "my body");

        let input = "my body\n# some comment\n# another comment\nAnd body again\n";
        assert_eq!(input.trim_meaningless_end(), "my body\n# some comment\n# another comment\nAnd body again");

        let input = "my body\nConflicts:\n\tfile1\n\tfile2\n# some comment";
        assert_eq!(input.trim_meaningless_end(), "my body");
    }

    #[test]
    fn test_trim_meaningless_start() {
        let input = "my body";
        assert_eq!(input.trim_meaningless_start(), "my body");

        let input = "\nmy body";
        assert_eq!(input.trim_meaningless_start(), "my body");

        let input = "\n\nmy body";
        assert_eq!(input.trim_meaningless_start(), "my body");

        let input = "# some comment\nmy body";
        assert_eq!(input.trim_meaningless_start(), "my body");

        let input = "# some comment\n# some comment\n\nmy body";
        assert_eq!(input.trim_meaningless_start(), "my body");
    }

    #[test]
    fn test_parse_body() {
        let input = "my body\n\nmyfooter: my value";
        let expected = (
            Some("my body".to_string()),
            Some(vec![Footer {
                key: "myfooter".to_string(),
                value: "my value".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }]),
        );
        assert_eq!(parse_body(input, ":"), expected);
    }

    #[test]
    fn test_parse_body_with_comments() {
        let input = "my cool feature

Authored-By: Co Mitter <comitter@example.com>

# This is a comment
# This is another comment";
        let expected = (
            Some("my cool feature".to_string()),
            Some(vec![Footer {
                key: "Authored-By".to_string(),
                value: "Co Mitter <comitter@example.com>".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }]),
        );
        assert_eq!(parse_body(input, ":"), expected);
    }

    #[test]
    fn test_parse_footers() {
        let input = "\nmyfooter: my value\n# some comment\n# another comment\n";
        let expected = (
            None,
            Some(vec![Footer {
                key: "myfooter".to_string(),
                value: "my value".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }]),
        );
        assert_eq!(parse_body(input, ":"), expected);
    }
}
