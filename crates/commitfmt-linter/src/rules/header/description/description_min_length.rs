use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for short commit description.
///
/// ## Why is this bad?
/// A description that is too short can hide the nature of what is happening in it.
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
pub(crate) struct DescriptionMinLength {
    pub(crate) length: usize,
}

impl Violation for DescriptionMinLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        let length = self.length;
        format!("Description is shorter than {length} characters")
    }
}

/// Checks for scope maximum length
pub(crate) fn description_min_length(report: &mut Report, message: &Message, length: usize) {
    if message.header.description.len() < length {
        report.add_violation(Box::new(DescriptionMinLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_description_min_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("test: add more cases for parser"),
            body: None,
            footers: footer_vec![],
        };
        description_min_length(&mut report, &message, 5);
        assert_eq!(report.len(), 0);

        let message: Message =
            Message { header: Header::from("test: add"), body: None, footers: footer_vec![] };
        description_min_length(&mut report, &message, 5);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "DescriptionMinLength");
    }
}
