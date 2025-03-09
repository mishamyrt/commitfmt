use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

use crate::case::TextCase;
use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};

/// ## What it does
/// Checks that the character case of the commit body is consistent
///
/// ## Why is this bad?
/// A random case in a generated changelog may not look very pretty.
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// My Feature IS SO COOL
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// My feature is so cool
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct Case {
    case: TextCase,
}

impl Violation for Case {
    fn group(&self) -> LinterGroup {
        LinterGroup::Body
    }

    fn message(&self) -> String {
        format!("Body case is inconsistent. Expected: {}", self.case)
    }
}

/// Checks that the character case of the commit body is consistent
pub(crate) fn case(report: &Report, message: &Message, case: TextCase) {
    let Some(body) = message.body.as_ref() else {
        return;
    };

    if !case.is_match(body) {
        let violation = Box::new(Case{ case });
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::header::Header;

    use super::*;

    #[test]
    fn test_case() {
        let mut report = Report::default();

        let mut message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("feature description".to_string()),
            footers: vec![],
        };

        case(&mut report, &message, TextCase::Lower);
        assert_eq!(report.len(), 0);

        message.body = Some("FEATURE description".to_string());
        case(&mut report, &message, TextCase::Lower);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "Case");
    }
}
