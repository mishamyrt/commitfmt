use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for short header.
///
/// ## Why is this bad?
/// A commit header that is too short can hide the nature of what is happening in it.
///
/// ## Example
/// ```git-commit
/// test: add
/// ```
///
/// Use instead:
/// ```git-commit
/// test: add more cases for parser
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MinLength {
    pub(crate) length: usize,
}

impl Violation for MinLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        format!("Header is shorter than {} characters", self.length)
    }
}

/// Checks for short body
pub(crate) fn min_length(report: &Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }

    if message.header.len() < length {
        report.add_violation(Box::new(MinLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::Header;

    use super::*;

    #[test]
    fn test_min_length() {
        let mut report = Report::default();

        let message: Message =
            Message { header: Header::from("test"), body: None, footers: vec![] };

        min_length(&mut report, &message, 4);
        assert_eq!(report.len(), 0);

        min_length(&mut report, &message, 8);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "MinLength");
    }
}
