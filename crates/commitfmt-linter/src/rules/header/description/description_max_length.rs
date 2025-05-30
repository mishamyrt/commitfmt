use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for description maximum length.
///
/// ## Why is this bad?
/// Long description will be truncated when displayed in the logs.
///
/// ## Example
/// ```git-commit
/// feat: my feature description where i added some bugs and fixed some others which are longer than 72 characters
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature description
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct DescriptionMaxLength {
    pub(crate) length: usize,
}

impl Violation for DescriptionMaxLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        let length = self.length;
        format!("Description is longer than {length} characters")
    }
}

/// Checks for scope maximum length
pub(crate) fn description_max_length(report: &mut Report, message: &Message, length: usize) {
    if message.header.description.len() > length {
        report.add_violation(Box::new(DescriptionMaxLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_description_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("feat(db, ui): my feature"),
            body: None,
            footers: footer_vec![],
        };
        description_max_length(&mut report, &message, 72);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from(
                "feat: my feature description where i added some bugs and fixed some others which are longer than 72 characters",
            ),
            body: None,
            footers: footer_vec![],
        };
        description_max_length(&mut report, &message, 72);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "DescriptionMaxLength");
    }
}
