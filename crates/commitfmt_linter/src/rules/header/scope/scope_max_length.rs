use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::message::Message;
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
        format!("Scope is longer than {} characters", self.length)
    }
}

/// Checks for scope maximum length
pub(crate) fn scope_max_length(report: &Report, message: &Message, length: usize) {
    if message.header.scope.str_len() > length {
        report.add_violation(Box::new(ScopeMaxLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_list::FooterList, header::Header};

    use super::*;

    #[test]
    fn test_scope_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat(db, ui): my feature"),
            body: None,
            footers: FooterList::default()
        };
        scope_max_length(&mut report, &message, 10);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat(db-core, ui-core, req-internal): my feature"),
            body: None,
            footers: FooterList::default()
        };
        scope_max_length(&mut report, &message, 10);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "ScopeMaxLength");
    }
}
