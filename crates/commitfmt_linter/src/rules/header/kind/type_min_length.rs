use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for scope minimum length.
///
/// ## Why is this bad?
/// Insufficient Scope can make it difficult to understand the domain of change.
///
/// ## Example
/// ```git-commit
/// tests
/// ```
///
/// Use instead:
/// ```git-commit
/// test: add cases
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct TypeMinLength {
    pub(crate) length: usize,
}

impl Violation for TypeMinLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        format!("Type is shorter than {} characters", self.length)
    }
}

/// Checks for scope maximum length
pub(crate) fn type_min_length(report: &mut Report, message: &Message, length: usize) {
    let Some(kind) = &message.header.kind else {
        report.add_violation(Box::new(TypeMinLength { length }));
        return;
    };

    if kind.len() < length {
        report.add_violation(Box::new(TypeMinLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_type_min_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("test: add more cases for parser"),
            body: None,
            footers: footer_vec![],
        };
        type_min_length(&mut report, &message, 1);
        assert_eq!(report.len(), 0);

        let message: Message =
            Message { header: Header::from("tests"), body: None, footers: footer_vec![] };
        type_min_length(&mut report, &message, 1);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "TypeMinLength");
    }
}
