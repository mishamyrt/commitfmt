use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks that the character case of the commit type is consistent
///
/// ## Why is this bad?
/// Type is a completely technical field. Different spellings of the same type
/// can confuse automatic documentation generation utilities.
///
/// ## Example
/// ```git-commit
/// feature: my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct TypeEnum;

impl Violation for TypeEnum {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        "Type value is not allowed by enum".to_string()
    }
}

/// Checks for scope case consistency
pub(crate) fn type_enum(report: &Report, message: &Message, allowed: &Vec<Box<str>>) {
    let Some(kind) = &message.header.kind else {
        return;
    };

    for item in allowed {
        if item.as_ref() == kind {
            return;
        }
    }

    report.add_violation(Box::new(TypeEnum));
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::Header;

    use super::*;

    #[test]
    fn test_type_enum() {
        let mut report = Report::default();
        let allowed_str = ["fix", "feat"];
        let allowed: Vec<Box<str>> = allowed_str.iter().map(|s| Box::from(*s)).collect();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: vec![],
        };

        type_enum(&mut report, &message, &allowed);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feature: my feature"),
            body: None,
            footers: vec![],
        };

        type_enum(&mut report, &message, &allowed);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "TypeEnum");
    }
}
