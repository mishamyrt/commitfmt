use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for scope maximum length.
///
/// ## Why is this bad?
/// While Scopes are useful, they take up space in the header,
/// taking it away from the description.
///
/// ## Example
/// ```git-commit
/// feat(db-core, ui-core, ui-widgets, db-internal): my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat(db, ui): my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct ScopeMaxLength {
    pub(crate) length: usize,
}

impl Violation for ScopeMaxLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        let length = self.length;
        format!("Scope is longer than {length} characters")
    }
}

/// Checks for scope maximum length
pub(crate) fn scope_max_length(report: &mut Report, message: &Message, length: usize) {
    if length == 0 || message.header.scope.is_empty() {
        return;
    }

    let scope_length =
        message.header.scope.iter().map(|scope| scope.chars().count()).sum::<usize>();
    if scope_length > length {
        report.add_violation(Box::new(ScopeMaxLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_scope_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat(db, ui): my feature"),
            body: None,
            footers: footer_vec![],
        };
        scope_max_length(&mut report, &message, 4);
        assert_eq!(report.len(), 0);

        scope_max_length(&mut report, &message, 0);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![],
        };
        scope_max_length(&mut report, &message, 10);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat(db-core, ui-core, req-internal): my feature"),
            body: None,
            footers: footer_vec![],
        };
        scope_max_length(&mut report, &message, 10);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "ScopeMaxLength");
    }

    #[test]
    fn test_scope_max_length_counts_unicode_characters() {
        let message = Message {
            header: Header::from("feat(é, 界): my feature"),
            body: None,
            footers: footer_vec![],
        };
        let mut report = Report::default();

        scope_max_length(&mut report, &message, 2);
        assert_eq!(report.len(), 0);

        scope_max_length(&mut report, &message, 1);
        assert_eq!(report.len(), 1);
    }
}
