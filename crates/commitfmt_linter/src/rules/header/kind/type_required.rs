use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks that the commit type is exists
///
/// ## Why is this bad?
/// The commit type is necessary for utilities analyzing git logs.
/// Its absence will prevent them from assigning the commit to a certain group.
///
/// ## Example
/// ```git-commit
/// my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct TypeRequired;

impl Violation for TypeRequired {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        "Commit type is required".to_string()
    }
}

/// Checks for scope case consistency
pub(crate) fn type_required(report: &Report, message: &Message) {
    if message.header.kind.is_none() {
        report.add_violation(Box::new(TypeRequired));
    };
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{FooterList, Header};

    use super::*;

    #[test]
    fn test_type_required() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: FooterList::default()
        };

        type_required(&mut report, &message);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("my feature"),
            body: None,
            footers: FooterList::default()
        };

        type_required(&mut report, &message);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "TypeRequired");
    }
}
