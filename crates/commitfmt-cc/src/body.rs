use crate::footer::Footers;

pub(crate) const DEFAULT_COMMENT_SYMBOL: &str = "#";
const OLD_CONFLICTS_TITLE: &str = "Conflicts:";

/// Parse body and footer
pub(crate) fn parse_body(
    input: &str,
    footer_separators: &str,
    comment_symbol: &str,
) -> (Option<String>, Option<Footers>) {
    if input.is_empty() {
        return (None, None);
    }

    let meaningful_input = trim_meaningless_end(input, comment_symbol);

    // Try to find last block of text.
    // If no block is found, than input is single block.
    let last_block_index: usize = meaningful_input.rfind("\n\n").unwrap_or(0);
    if last_block_index == 0 {
        let meaningful_input = trim_meaningless_start(meaningful_input, comment_symbol);
        if meaningful_input.is_empty() {
            return (None, None);
        }

        match Footers::parse(meaningful_input, footer_separators) {
            Ok((_rest, footers)) => return (None, Some(footers)),
            Err(_) => return (Some(meaningful_input.to_string()), None),
        }
    }

    let last_block = &meaningful_input[last_block_index + 2..];

    match Footers::parse(last_block, footer_separators) {
        Ok((_rest, footers)) => {
            let body =
                trim_meaningless_start(&meaningful_input[..last_block_index], comment_symbol)
                    .to_string();
            (Some(body), Some(footers))
        }
        Err(_) => {
            let body = trim_meaningless_start(input, comment_symbol).to_string();
            (Some(body), None)
        }
    }
}

/// Return the offset of `line` in `input`
#[allow(clippy::cast_sign_loss)]
unsafe fn line_offset(input: &str, line: &str) -> usize {
    let ptr = line.as_ptr();
    let offset = ptr.offset_from(input.as_ptr());

    offset as usize
}

/// Remove meaningless lines from the start of the string.
///
/// Meaningless lines are:
/// - Empty lines
/// - Comments (lines starting with comment symbol)
fn trim_meaningless_start<'input>(input: &'input str, comment_symbol: &str) -> &'input str {
    for line in input.lines() {
        let offset = unsafe { line_offset(input, line) };
        if !(line.is_empty() || line.starts_with(comment_symbol)) {
            return input[offset..].trim_start();
        }
    }

    ""
}

/// Remove meaningless lines from the end of the string.
///
/// Meaningless lines are:
/// - Empty lines
/// - Comments (lines starting with comment symbol)
/// - Old conflicts block (lines starting with `Conflicts:`)
fn trim_meaningless_end<'input>(input: &'input str, comment_symbol: &str) -> &'input str {
    let mut meaningful_end = input.len();
    let mut in_old_conflicts_block = false;

    for line in input.lines() {
        let offset = unsafe { line_offset(input, line) };

        if line.starts_with(comment_symbol) {
            if meaningful_end == input.len() {
                meaningful_end = offset;
            }
        } else if line == OLD_CONFLICTS_TITLE {
            in_old_conflicts_block = true;
            if meaningful_end == input.len() {
                meaningful_end = offset;
            }
        } else if in_old_conflicts_block && line.starts_with('\t') {
            // Part of the conflict block, so we ignore it
        } else if !line.trim().is_empty() {
            // Reset the meaningful_end if a new meaningful line is encountered
            meaningful_end = input.len();
        }
    }

    input[..meaningful_end].trim_end()
}

#[cfg(test)]
mod tests {
    use crate::{footer::SeparatorAlignment, footer_vec};

    use super::*;

    #[test]
    fn test_trim_meaningless_end() {
        let input = "my body\n\nmyfooter: my value";
        assert_eq!(trim_meaningless_end(input, "#"), "my body\n\nmyfooter: my value");

        let input = "my body\nConflicts:\n\tfile1\n\tfile2";
        assert_eq!(trim_meaningless_end(input, "#"), "my body");

        let input = "my body\n# some comment\n# another comment";
        assert_eq!(trim_meaningless_end(input, "#"), "my body");

        let input = "my body\n# some comment\n# another comment\nAnd body again\n";
        assert_eq!(
            trim_meaningless_end(input, "#"),
            "my body\n# some comment\n# another comment\nAnd body again"
        );

        let input = "my body\nConflicts:\n\tfile1\n\tfile2\n# some comment";
        assert_eq!(trim_meaningless_end(input, "#"), "my body");
    }

    #[test]
    fn test_trim_meaningless_start() {
        let input = "my body";
        assert_eq!(trim_meaningless_start(input, "#"), "my body");

        let input = "\nmy body";
        assert_eq!(trim_meaningless_start(input, "#"), "my body");

        let input = "\n\nmy body";
        assert_eq!(trim_meaningless_start(input, "#"), "my body");

        let input = "# some comment\nmy body";
        assert_eq!(trim_meaningless_start(input, "#"), "my body");

        let input = "# some comment\n# some comment\n\nmy body";
        assert_eq!(trim_meaningless_start(input, "#"), "my body");

        let input = "// some comment\n// some comment\n\nmy body";
        assert_eq!(trim_meaningless_start(input, "//"), "my body");
    }

    #[test]
    fn test_parse_body() {
        let input = "my body";
        let expected = (Some("my body".to_string()), None);
        assert_eq!(parse_body(input, ":", "#"), expected);

        let input = "\nmy body";
        let expected = (Some("my body".to_string()), None);
        assert_eq!(parse_body(input, ":", "#"), expected);

        let input = "\n\nmy body";
        let expected = (Some("my body".to_string()), None);
        assert_eq!(parse_body(input, ":", "#"), expected);

        let input = "\n\n\nmy body";
        let expected = (Some("my body".to_string()), None);
        assert_eq!(parse_body(input, ":", "#"), expected);

        let input = "my body\n\nmyfooter: my value";
        let expected = (
            Some("my body".to_string()),
            Some(footer_vec![{
                key: "myfooter".to_string(),
                value: "my value".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }]),
        );
        assert_eq!(parse_body(input, ":", "#"), expected);
    }

    #[test]
    fn test_parse_body_with_comments() {
        let input = "my cool feature

Authored-By: Co Mitter <comitter@example.com>

# This is a comment
# This is another comment";
        let expected = (
            Some("my cool feature".to_string()),
            Some(footer_vec![ {
                key: "Authored-By".to_string(),
                value: "Co Mitter <comitter@example.com>".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }]),
        );
        assert_eq!(parse_body(input, ":", "#"), expected);
    }

    #[test]
    fn test_parse_footers() {
        let input = "\nmyfooter: my value\n# some comment\n# another comment\n";
        let expected = (
            None,
            Some(footer_vec![{
                key: "myfooter".to_string(),
                value: "my value".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }]),
        );
        assert_eq!(parse_body(input, ":", "#"), expected);
    }
}
