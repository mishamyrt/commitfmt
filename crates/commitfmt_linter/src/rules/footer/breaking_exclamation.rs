use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
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

    fn message(&self) -> String {
        "Message contains breaking changes footer but no exclamation mark".to_string()
    }
}

/// Checks for exclamation mark in a message containing `BREAKING CHANGES`.
pub(crate) fn breaking_exclamation(report: &Report, message: &Message) {
    if message.footers.is_empty() || message.header.breaking {
        return;
    }

    if message.footers.iter().any(Footer::is_breaking_change) {
        report.add_violation(Box::new(BreakingExclamation));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{Footer, Header, SeparatorAlignment};

    use super::*;

    #[test]
    fn test_max_length() {
        let mut report = Report::default();

        let footers = vec![Footer {
            key: "BREAKING CHANGES".to_string(),
            value: "some breaking changes".to_string(),
            separator: ':',
            alignment: SeparatorAlignment::Left,
        }];

        let message: Message = Message {
            header: Header::from("feat!: my feature"),
            body: None,
            footers: footers.to_owned(),
        };

        breaking_exclamation(&mut report, &message);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footers.to_owned(),
        };

        breaking_exclamation(&mut report, &message);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "BreakingExclamation");
    }
}
