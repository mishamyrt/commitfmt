use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for long body.
///
/// ## Why is this bad?
/// If feature or fix needs huge description, maybe it indicates something wrong.
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// My super long body, which is longer than 72 characters and should be split into multiple lines
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// My body
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MaxLength {
    pub(crate) max_length: usize,
}

impl Violation for MaxLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Body
    }

    fn message(&self) -> String {
        format!("Body is longer than {} characters", self.max_length)
    }
}

/// Checks for long body
pub(crate) fn max_length(report: &Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }
    let Some(body) = message.body.as_ref() else {
        return;
    };

    if body.len() > length {
        let violation = Box::new(MaxLength {
            max_length: length,
        });
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::header::Header;

    use super::*;

    #[test]
    fn test_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("\nBody with some text".to_string()),
            footers: vec![],
        };

        max_length(&mut report, &message, 72);
        assert_eq!(report.len(), 0);

        max_length(&mut report, &message, 5);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "MaxLength");
    }
}
