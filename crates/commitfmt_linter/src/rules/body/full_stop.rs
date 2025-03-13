use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{FixMode, Violation, ViolationError, ViolationMetadata};

/// ## What it does
/// Checks for body ending with full stop
///
/// ## Why is this bad?
/// Automatically generated changelogs can be hard to read
/// if the body not ends with a full stop.
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// My feature is so cool. I can't even describe it
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// My feature is so cool. I can't even describe it.
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct FullStop;

impl Violation for FullStop {
    fn group(&self) -> LinterGroup {
        LinterGroup::Body
    }

    fn fix_mode(&self) -> FixMode {
        FixMode::Unsafe
    }

    fn fix(&self, message: &mut Message) -> Result<(), ViolationError> {
        let Some(body) = message.body.as_mut() else {
            return Err(ViolationError::Empty("body".to_string()));
        };

        body.push('.');
        Ok(())
    }

    fn message(&self) -> String {
        "Body is not ended with a full stop".to_string()
    }
}

/// Checks for body ending with full stop
pub(crate) fn full_stop(report: &Report, message: &Message) {
    let Some(body) = message.body.as_ref() else {
        return;
    };

    if !body.ends_with('.') {
        let violation = Box::new(FullStop);
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::header::Header;

    use super::*;

    #[test]
    fn test_full_stop() {
        let mut report = Report::default();

        let mut message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("\nDescription.".to_string()),
            footers: vec![],
        };

        full_stop(&mut report, &message);
        assert_eq!(report.len(), 0);

        message.body = Some("\nDescription".to_string());
        full_stop(&mut report, &message);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "FullStop");
    }

    #[test]
    fn test_full_stop_fix() {
        let mut report = Report::default();
        let mut message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("\nDescription".to_string()),
            footers: vec![],
        };

        full_stop(&mut report, &message);
        report.violations.borrow()[0].fix(&mut message).unwrap();

        assert_eq!(message.body, Some("\nDescription.".to_string()));
    }
}
