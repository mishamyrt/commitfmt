use crate::case::WordCase;
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
pub(crate) struct ScopeCase {
    pub(crate) case: WordCase,
}

impl Violation for ScopeCase {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        let case = self.case;
        format!("Scope case is inconsistent. Expected: {case}")
    }
}

/// Checks for scope case consistency
pub(crate) fn scope_case(report: &mut Report, message: &Message, case: WordCase) {
    for scope in message.header.scope.iter() {
        if !case.is_match(scope) {
            report.add_violation(Box::new(ScopeCase { case }));
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_scope_case() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat(db-core, ui-core, req-internal): my feature"),
            body: None,
            footers: footer_vec![],
        };

        scope_case(&mut report, &message, WordCase::Kebab);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat(DB_Core, UICore, req-internal): my feature"),
            body: None,
            footers: footer_vec![],
        };

        scope_case(&mut report, &message, WordCase::Kebab);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "ScopeCase");
    }
}
