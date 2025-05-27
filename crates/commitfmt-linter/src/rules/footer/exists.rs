use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks that footer exists.
///
/// ## Why is this bad?
/// Automated tools may require certain footers and their absence can break processes.
///
/// ## Example
/// ```git-commit
/// feat: my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// Issue: PRJ-123
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct Exists {
    key: String,
}

impl Violation for Exists {
    fn group(&self) -> LinterGroup {
        LinterGroup::Footer
    }

    fn message(&self) -> String {
        let key = &self.key;
        format!("Footer {key} is required but not found")
    }
}

/// Checks that footer exists
pub(crate) fn exists(report: &mut Report, message: &Message, required: &Vec<Box<str>>) {
    for key in required {
        if !message.footers.iter().any(|f| f.key == **key) {
            report.add_violation(Box::new(Exists { key: key.to_string() }));
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header, SeparatorAlignment};

    use super::*;

    #[test]
    fn test_exists() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![{
                key: "Authored-by".to_string(),
                value: "John Doe".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        exists(&mut report, &message, &vec!["Authored-by".into()]);
        assert_eq!(report.len(), 0);

        exists(&mut report, &message, &vec!["Commited-by".into()]);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "Exists");
    }
}
