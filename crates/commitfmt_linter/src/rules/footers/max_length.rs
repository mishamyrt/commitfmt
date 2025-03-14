use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for too long footers.
///
/// ## Why is this bad?
/// If the footer contains a lot of information, something probably
/// didn't go according to plan. Maybe it should be in the body?
///
/// ## Example
/// ```git-commit
/// feat: my feature
///
/// BREAKING CHANGES: My very long footer where I talk about all the changes in this feature
/// Time: 2 hours
/// Complexity: 10
/// Mood: happy
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
///
/// My long body footer I talk about all the changes in this feature
///
/// BREAKING CHANGES: feature api breakage
/// Time: 2 hours
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MaxLength {
    pub(crate) length: usize,
}

impl Violation for MaxLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Footer
    }

    fn message(&self) -> String {
        format!("Total footers length is longer than {} characters", self.length)
    }
}

/// Checks for long footers
pub(crate) fn max_length(report: &Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }
    let Some(body) = message.body.as_ref() else {
        return;
    };

    if body.len() > length {
        let violation = Box::new(MaxLength {
            length: length,
        });
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_list::FooterList, header::Header};

    use super::*;

    #[test]
    fn test_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("\nBody with some text".to_string()),
            footers: FooterList::default(),
        };

        max_length(&mut report, &message, 72);
        assert_eq!(report.len(), 0);

        max_length(&mut report, &message, 5);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "MaxLength");
    }
}
