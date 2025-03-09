use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationError, ViolationMetadata};
use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for missing newlines at the start of the body
///
/// ## Why is this bad?
/// Missing newlines at the start of the body can make it hard to read and parse.
///
/// ## Example
/// ```git-commit
/// feat: my feature
/// body
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// body
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct LeadingNewLine;

impl Violation for LeadingNewLine {
    fn group(&self) -> LinterGroup {
        LinterGroup::Body
    }

    fn message(&self) -> String {
        String::from("Body must start with a newline")
    }

    fn fix(&self, message: &mut Message) -> Result<(), crate::violation::ViolationError> {
        match message.body.as_mut() {
            Some(body) => {
                body.insert(0, '\n');
            }
            None => return Err(ViolationError::EmptyBody()),
        };

        Ok(())
    }
}

/// Checks for missing newlines at the start of the body
pub(crate) fn leading_nl(report: &Report, message: &Message) {
    let Some(body) = message.body.as_ref() else {
        return;
    };
    if !body.starts_with('\n') {
        let violation = Box::new(LeadingNewLine);
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{header::Header, message::Message};

    use super::*;

    #[test]
    fn test_leading_nl_correct() {
        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("\nbody".to_string()),
            footers: vec![],
        };
        let mut checker = Report::default();
        leading_nl(&mut checker, &message);
        assert_eq!(checker.violations.borrow().len(), 0);
    }

    #[test]
    fn test_leading_nl_correct_violation() {
        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("body".to_string()),
            footers: vec![],
        };
        let mut checker = Report::default();
        leading_nl(&mut checker, &message);
        assert_eq!(checker.violations.borrow().len(), 1);
    }

    #[test]
    fn test_leading_nl_empty_body() {
        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: vec![],
        };
        let mut checker = Report::default();
        leading_nl(&mut checker, &message);
        assert_eq!(checker.violations.borrow().len(), 0);
    }
}
