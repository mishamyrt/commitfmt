use crate::case::TextCase;
use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks that the character case of the commit description is consistent
///
/// ## Why is this bad?
/// The commit description is primarily used by automated tools to generate
/// the changelog so it is important that the descriptions are consistent
///
/// ## Example
/// ```git-commit
/// feat: My feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct DescriptionCase {
    pub(crate) case: TextCase,
}

impl Violation for DescriptionCase {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        format!("Description case is inconsistent. Expected: {}", self.case)
    }
}

/// Checks for scope case consistency
pub(crate) fn description_case(report: &Report, message: &Message, case: TextCase) {
    if !case.is_match(&message.header.description) {
        report.add_violation(Box::new(DescriptionCase {
            case,
        }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::Header;

    use super::*;

    #[test]
    fn test_description_case() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat(db-core, ui-core, req-internal): my feature"),
            body: None,
            footers: vec![],
        };

        description_case(&mut report, &message, TextCase::Lower);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("feat: My feature"),
            body: None,
            footers: vec![],
        };

        description_case(&report, &message, TextCase::Lower);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "DescriptionCase");
    }
}
