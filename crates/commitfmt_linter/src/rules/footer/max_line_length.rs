use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for too long lines in footers.
///
/// ## Why is this bad?
/// Lines that are too long may not look good in the limited space of the terminal.
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// BREAKING CHANGES: I had to heavily rework several modules. Compatibility of TreeView and Card components may be broken due to the library update.
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// BREAKING CHANGES: I had to heavily rework several modules.
///  Compatibility of TreeView and Card components may be broken
///  due to the library update.
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MaxLineLength {
    key: String,
    length: usize,
}

impl Violation for MaxLineLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Footer
    }

    fn message(&self) -> String {
        format!(
            "Footer {} contains a line that length is longer than {} characters",
            self.key, self.length
        )
    }
}

/// Checks for long footers
pub(crate) fn max_line_length(report: &Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }

    for footer in &message.footers {
        for line in footer.value.lines() {
            if line.len() > length {
                let violation = Box::new(MaxLineLength {
                    key: footer.key.clone(),
                    length
                });
                report.add_violation(violation);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{Footer, Header, SeparatorAlignment};

    use super::*;

    #[test]
    fn test_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: vec![Footer {
                key: "BREAKING CHANGES".to_string(),
                value: "some breaking changes".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        max_line_length(&mut report, &message, 72);
        assert_eq!(report.len(), 0);

        max_line_length(&mut report, &message, 5);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "MaxLineLength");
    }
}
