use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for short body.
///
/// ## Why is this bad?
/// Short body can make it hard to understand.
///
/// ## Example
/// ```git-commit
/// feat: my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// My feature description
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MinLength {
    pub(crate) length: usize,
}

impl Violation for MinLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Body
    }

    fn message(&self) -> String {
        format!("Body is shorter than {} characters", self.length)
    }
}

/// Checks for short body
pub(crate) fn min_length(report: &Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }
    let violation = Box::new(MinLength {
        length,
    });

    let Some(body) = message.body.as_ref() else {
        report.add_violation(violation);
        return;
    };

    if body.len() < length {
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::header::Header;

    use super::*;

    #[test]
    fn test_min_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("\nBody with some text".to_string()),
            footers: vec![],
        };

        min_length(&mut report, &message, 5);
        assert_eq!(report.len(), 0);

        min_length(&mut report, &message, 72);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "MinLength");
    }
}
