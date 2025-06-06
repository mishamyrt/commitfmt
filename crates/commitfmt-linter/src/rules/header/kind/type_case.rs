use crate::case::IdentifierCase;
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
/// Feat: my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct TypeCase {
    pub(crate) case: IdentifierCase,
}

impl Violation for TypeCase {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        let case = self.case;
        format!("Type case is inconsistent. Expected: {case}")
    }
}

/// Checks for scope case consistency
pub(crate) fn type_case(report: &mut Report, message: &Message, case: IdentifierCase) {
    let Some(kind) = &message.header.kind else {
        return;
    };

    if !case.is_match(kind) {
        report.add_violation(Box::new(TypeCase { case }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_type_case() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![],
        };

        type_case(&mut report, &message, IdentifierCase::Lower);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("Feat: my feature"),
            body: None,
            footers: footer_vec![],
        };

        type_case(&mut report, &message, IdentifierCase::Lower);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "TypeCase");
    }
}
