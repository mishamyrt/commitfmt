use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{FixMode, Violation, ViolationMetadata};
use commitfmt_cc::{Footer, Message};
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for the presence of a flag (exclamation mark)
/// in a message containing `BREAKING CHANGES`.
///
/// ## Why is this bad?
/// Some utilities may not check commit footers and count on the presence of an exclamation mark.
/// And they would be right
///
/// ## Example
/// ```git-commit
/// feat: my super feature
///
/// BREAKING CHANGES: some breaking changes
/// ```
///
/// Use instead:
/// ```git-commit
/// feat!: my super feature
///
/// BREAKING CHANGES: some breaking changes
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct BreakingExclamation;

impl Violation for BreakingExclamation {
    fn group(&self) -> LinterGroup {
        LinterGroup::Footer
    }

    fn fix_mode(&self) -> FixMode {
        FixMode::Safe
    }

    fn fix(&self, message: &mut Message) -> Result<(), crate::violation::ViolationError> {
        message.header.breaking = true;
        Ok(())
    }

    fn message(&self) -> String {
        "Message contains breaking changes footer but no exclamation mark".to_string()
    }
}

/// Checks for exclamation mark in a message containing `BREAKING CHANGES`.
pub(crate) fn breaking_exclamation(report: &mut Report, message: &Message) {
    if message.footers.is_empty() || message.header.breaking {
        return;
    }

    if message.footers.iter().any(Footer::is_breaking_change) {
        report.add_violation(Box::new(BreakingExclamation));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header, SeparatorAlignment};

    use super::*;

    #[test]
    fn test_max_length() {
        let mut report = Report::default();

        let footers = footer_vec![{
            key: "BREAKING CHANGES".to_string(),
            value: "some breaking changes".to_string(),
            separator: ':',
            alignment: SeparatorAlignment::Left,
        }];

        let message: Message = Message {
            header: Header::from("feat!: my feature"),
            body: None,
            footers: footers.clone(),
        };

        breaking_exclamation(&mut report, &message);
        assert_eq!(report.len(), 0);

        let message: Message =
            Message { header: Header::from("feat: my feature"), body: None, footers };

        breaking_exclamation(&mut report, &message);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "BreakingExclamation");
    }

    #[test]
    fn test_fix() {
        let mut message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![
                {
                    key: "BREAKING CHANGES".to_string(),
                    value: "some breaking changes".to_string(),
                    separator: ':',
                    alignment: SeparatorAlignment::Left,
                }
            ],
        };

        let mut report = Report::default();
        breaking_exclamation(&mut report, &message);
        assert_eq!(report.len(), 1);

        let violation = report.violations[0].as_ref();
        assert_eq!(violation.fix_mode(), FixMode::Safe);
        violation.fix(&mut message).unwrap();
        assert!(message.header.breaking);
    }
}
