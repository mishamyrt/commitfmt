use crate::footer::FooterList;

const COMMENT_CHAR: &str = "#";
const OLD_CONFLICTS_TITLE: &str = "Conflicts:";

/// Return the offset of `line` in `input`
#[allow(clippy::cast_sign_loss)]
unsafe fn line_offset(input: &str, line: &str) -> usize {
    let ptr = line.as_ptr();
    let offset = ptr.offset_from(input.as_ptr());

    offset as usize
}

/// Extract the meaningful part of the body.
/// The meaningful part is everything before:
/// - finalizing comments block
/// - old conflicts block
/// - trailing new lines
fn extract_meaningful_body(input: &str) -> &str {
    let mut meaningful_end = input.len();
    let mut in_old_conflicts_block = false;

    for line in input.lines() {
        let offset = unsafe { line_offset(input, line) };

        if line.starts_with(COMMENT_CHAR) {
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

/// Parse body and footer
pub(crate) fn parse_body(input: &str, footer_separators: &str) -> (Option<String>, Option<FooterList>) {
    if input.is_empty() {
        return (None, None);
    }

    let meaningful_input = extract_meaningful_body(input);

    // Try to find last block of text.
    // If no block is found, than input is single block.
    let last_block_index: usize = meaningful_input.rfind("\n\n").unwrap_or(0);
    if last_block_index == 0 {
        match FooterList::parse(meaningful_input.trim_start(), footer_separators) {
            Ok((_rest, footers)) => return (None, Some(footers)),
            Err(_) => return (Some(input.to_string()), None),
        }
    }

    let last_block = &meaningful_input[last_block_index + 2..];

    match FooterList::parse(last_block, footer_separators) {
        Ok((_rest, footers)) => {
            let body = Some(meaningful_input[..last_block_index].to_string());
            (body, Some(footers))
        }
        Err(_) => (Some(meaningful_input.to_string()), None),
    }
}

#[cfg(test)]
mod tests {
    use crate::{footer::{Footer, SeparatorAlignment}, footer_list};

    use super::*;

    #[test]
    fn test_extract_meaningful_body() {
        let input = "my body\n\nmyfooter: my value";
        assert_eq!(extract_meaningful_body(input), "my body\n\nmyfooter: my value");
    }

    #[test]
    fn test_extract_meaningful_body_with_conflicts() {
        let input = "my body\nConflicts:\n\tfile1\n\tfile2";
        assert_eq!(extract_meaningful_body(input), "my body");
    }

    #[test]
    fn test_extract_meaningful_body_with_comments() {
        let input = "my body\n# some comment\n# another comment\n\n";
        assert_eq!(extract_meaningful_body(input), "my body");
    }

    #[test]
    fn test_extract_meaningful_body_with_continuation() {
        let input = "my body\n# some comment\n# another comment\nAnd body again\n";
        assert_eq!(extract_meaningful_body(input), "my body\n# some comment\n# another comment\nAnd body again");
    }

    #[test]
    fn test_parse_body() {
        let input = "my body\n\nmyfooter: my value";
        let expected = (
            Some("my body".to_string()),
            Some(footer_list![Footer {
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
            Some(footer_list![Footer {
                key: "Authored-By".to_string(),
                value: "Co Mitter <comitter@example.com>".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }]),
        );
        assert_eq!(parse_body(input, ":"), expected);
    }
}
