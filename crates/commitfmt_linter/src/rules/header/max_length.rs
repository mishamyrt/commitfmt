use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for long header.
///
/// ## Why is this bad?
/// Long commit messages will be truncated when displayed in the logs.
///
/// ## Example
/// ```git-commit
/// feat: my super feature with description which is longer than 72 characters and should be split into multiple lines
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my super feature
///
/// Description which is longer than 72 characters
/// and should be split into multiple lines.
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct MaxLength {
    pub(crate) max_length: usize,
}

impl Violation for MaxLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        format!("Header is longer than {} characters", self.max_length)
    }
}

/// Checks for long body
pub(crate) fn max_length(report: &mut Report, message: &Message, length: usize) {
    if length == 0 {
        return;
    }

    if message.header.len() > length {
        let violation = Box::new(MaxLength { max_length: length });
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![],
        };

        max_length(&mut report, &message, 72);
        assert_eq!(report.len(), 0);

        max_length(&mut report, &message, 5);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "MaxLength");
    }
}
