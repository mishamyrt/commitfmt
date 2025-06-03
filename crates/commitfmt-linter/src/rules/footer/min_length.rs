use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for too short footers.
///
/// ## Why is this bad?
/// A footer that is too short can hide the nature of what is happening in it.
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// BREAKING CHANGES: api
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// BREAKING CHANGES: DB API interfaces are changed
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MinLength {
    key: String,
    length: usize,
}

impl Violation for MinLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Footer
    }

    fn message(&self) -> String {
        let key = &self.key;
        let length = self.length;
        format!("Footer '{key}' length is less than {length} characters")
    }
}

/// Checks for short footers
pub(crate) fn min_length(report: &mut Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }

    for footer in message.footers.iter() {
        if footer.len() < length {
            let violation = Box::new(MinLength { key: footer.key.clone(), length });
            report.add_violation(violation);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header, SeparatorAlignment};

    use super::*;

    #[test]
    fn test_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![{
                key: "BREAKING CHANGES".to_string(),
                value: "some breaking changes".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        min_length(&mut report, &message, 5);
        assert_eq!(report.len(), 0);

        min_length(&mut report, &message, 72);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "MinLength");
    }
}
