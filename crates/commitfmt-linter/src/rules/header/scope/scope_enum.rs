use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks that the character case of the commit scope is consistent
///
/// ## Why is this bad?
/// Scopes are used to categorize commits into groups based on the domain of the change.
/// If you write them differently, automatic tools will not be able to match commits
///
/// ## Example
/// ```git-commit
/// feat(DB-Core, ui_core, reqInternal): my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat(db-core, ui-core, req-internal): my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct ScopeEnum {
    miss: String,
}

impl Violation for ScopeEnum {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        let miss = &self.miss;
        format!("Scope is not allowed: {miss}")
    }
}

/// Checks for scope case consistency
pub(crate) fn scope_enum(report: &mut Report, message: &Message, allowed: &[Box<str>]) {
    for scope in message.header.scope.iter() {
        if !allowed.contains(scope) {
            report.add_violation(Box::new(ScopeEnum { miss: scope.to_string() }));
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_scope_enum() {
        let mut report = Report::default();
        let allowed_str = ["db", "ui"];
        let allowed: Vec<Box<str>> = allowed_str.iter().map(|s| Box::from(*s)).collect();

        let message: Message = Message {
            header: Header::from("feat(db, ui): my feature"),
            body: None,
            footers: footer_vec![],
        };

        scope_enum(&mut report, &message, &allowed);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat(DB_Core, UICore): my feature"),
            body: None,
            footers: footer_vec![],
        };

        scope_enum(&mut report, &message, &allowed);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "ScopeEnum");
    }
}
