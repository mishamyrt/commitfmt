use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for too long footers.
///
/// ## Why is this bad?
/// If the footer contains a lot of information, something probably
/// didn't go according to plan. Maybe it should be in the body?
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// BREAKING CHANGES: I had to heavily rework several modules.
///  Compatibility of TreeView and Card components may be broken
///  due to the library update.
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// I had to heavily rework several modules. Compatibility
/// of TreeView and Card components may be broken due
/// to the library update.
///
/// BREAKING CHANGES: TreeView and Card APIs
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MaxLength {
    key: String,
    length: usize,
}

impl Violation for MaxLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Footer
    }

    fn message(&self) -> String {
        let key = &self.key;
        let length = self.length;
        format!("Footer '{key}' length is longer than {length} characters")
    }
}

/// Checks for long footers
pub(crate) fn max_length(report: &mut Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }

    for footer in message.footers.iter() {
        if footer.len() > length {
            let violation = Box::new(MaxLength { key: footer.key.clone(), length });
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

        max_length(&mut report, &message, 72);
        assert_eq!(report.len(), 0);

        max_length(&mut report, &message, 5);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "MaxLength");
    }
}
