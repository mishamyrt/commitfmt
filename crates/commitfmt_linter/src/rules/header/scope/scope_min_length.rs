use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for scope minimum length.
///
/// ## Why is this bad?
/// Insufficient Scope can make it difficult to understand the domain of change
///
/// ## Example
/// ```git-commit
/// feat(db, core): my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat(db-core, ui-core): my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct ScopeMinLength {
    pub(crate) length: usize,
}

impl Violation for ScopeMinLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        format!("Scope is shorter than {} characters", self.length)
    }
}

/// Checks for scope minimum length
pub(crate) fn scope_min_length(report: &mut Report, message: &Message, length: usize) {
    if message.header.scope.str_len() < length {
        report.add_violation(Box::new(ScopeMinLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::Header;

    use super::*;

    #[test]
    fn test_scope_min_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat(db-core, ui-core): my feature"),
            body: None,
            footers: vec![],
        };
        scope_min_length(&mut report, &message, 5);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat(db, ui): my feature"),
            body: None,
            footers: vec![],
        };
        scope_min_length(&mut report, &message, 10);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "ScopeMinLength");
    }
}
