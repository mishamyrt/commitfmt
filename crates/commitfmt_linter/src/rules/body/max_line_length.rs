use crate::report::Report;
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
    fn message(&self) -> String {
        format!("Body line is longer than {} characters", self.max_length)
    }
}

pub(crate) fn max_line_length(report: &Report, message: &Message, max_length: usize) {
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
