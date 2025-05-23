use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks that the commit scope is exists.
///
/// ## Why is this bad?
/// Insufficient Scope can make it difficult to understand the domain of change.
///
/// ## Example
/// ```git-commit
/// feat: my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat(ui): my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct ScopeRequired;

impl Violation for ScopeRequired {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    #[allow(clippy::useless_format)]
    fn message(&self) -> String {
        format!("Scope is required")
    }
}

/// Checks for scope case consistency
pub(crate) fn scope_required(report: &mut Report, message: &Message) {
    if message.header.scope.is_empty() {
        report.add_violation(Box::new(ScopeRequired));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_scope_required() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat(ui): my feature"),
            body: None,
            footers: footer_vec![],
        };

        scope_required(&mut report, &message);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![],
        };

        scope_required(&mut report, &message);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "ScopeRequired");
    }
}
