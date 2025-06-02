use crate::case::IdentifierCase;
use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks that the character case of the footer keys is consistent
///
/// ## Why is this bad?
/// Footer keys are used to provide additional metadata about a commit.
/// If you write them differently, automatic tools will not be able to match footers
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// Fixes: #123
/// BreakingChange: removed API
/// Signed_off_by: John Doe
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// fixes: #123
/// breaking-change: removed API
/// signed-off-by: John Doe
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct KeyCase {
    pub(crate) case: IdentifierCase,
}

impl Violation for KeyCase {
    fn group(&self) -> LinterGroup {
        LinterGroup::Footer
    }

    fn message(&self) -> String {
        let case = self.case;
        format!("Footer key case is inconsistent. Expected: {case}")
    }
}

/// Checks for footer key case consistency
pub(crate) fn key_case(report: &mut Report, message: &Message, case: IdentifierCase) {
    for footer in message.footers.iter() {
        if !case.is_match(&footer.key) {
            report.add_violation(Box::new(KeyCase { case }));
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header, SeparatorAlignment};

    use super::*;

    #[test]
    fn test_key_case() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![
                {
                    key: "fixes".to_string(),
                    value: "#123".to_string(),
                    separator: ':',
                    alignment: SeparatorAlignment::Left,
                },
                {
                    key: "breaking-change".to_string(),
                    value: "removed API".to_string(),
                    separator: ':',
                    alignment: SeparatorAlignment::Left,
                },
                {
                    key: "signed-off-by".to_string(),
                    value: "John Doe".to_string(),
                    separator: ':',
                    alignment: SeparatorAlignment::Left,
                }
            ],
        };

        key_case(&mut report, &message, IdentifierCase::Kebab);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![
                {
                    key: "Fixes".to_string(),
                    value: "#123".to_string(),
                    separator: ':',
                    alignment: SeparatorAlignment::Left,
                },
                {
                    key: "BreakingChange".to_string(),
                    value: "removed API".to_string(),
                    separator: ':',
                    alignment: SeparatorAlignment::Left,
                },
                {
                    key: "signed_off_by".to_string(),
                    value: "John Doe".to_string(),
                    separator: ':',
                    alignment: SeparatorAlignment::Left,
                }
            ],
        };

        key_case(&mut report, &message, IdentifierCase::Kebab);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "KeyCase");
    }
}
