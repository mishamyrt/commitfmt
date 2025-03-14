use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for long body lines.
///
/// ## Why is this bad?
/// Long body lines can make it hard to read and parse.
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
/// My super long body, which is longer than 72 characters
/// and should be split into multiple lines
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MaxLineLength {
    pub(crate) max_length: usize,
}

impl Violation for MaxLineLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Body
    }

    fn message(&self) -> String {
        format!("Body line is longer than {} characters", self.max_length)
    }
}

/// Checks for long body lines
pub(crate) fn max_line_length(report: &Report, message: &Message, max_length: usize) {
    if max_length == 0 {
        return;
    }
    let Some(body) = message.body.as_ref() else {
        return;
    };
    for line in body.lines() {
        if line.len() > max_length {
            let violation = Box::new(MaxLineLength {
                max_length,
            });
            report.add_violation(violation);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_list::FooterList, header::Header};

    use super::*;

    #[test]
    fn test_max_line_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: Some("\nBody\nWith some text".to_string()),
            footers: FooterList::default()
        };

        max_line_length(&mut report, &message, 72);
        assert_eq!(report.len(), 0);

        max_line_length(&mut report, &message, 5);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "MaxLineLength");
    }
}
