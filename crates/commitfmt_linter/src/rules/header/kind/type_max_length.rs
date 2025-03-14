use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use commitfmt_cc::message::Message;
use commitfmt_macros::ViolationMetadata;

/// ## What it does
/// Checks for scope maximum length.
///
/// ## Why is this bad?
/// While Scopes are useful, they take up space in the header,
/// taking it away from the description.
///
/// ## Example
/// ```git-commit
/// feat(db-core, ui-core, ui-widgets, db-internal): my feature
/// ```
///
/// Use instead:
/// ```git-commit
/// feat(db, ui): my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct TypeMaxLength {
    pub(crate) length: usize,
}

impl Violation for TypeMaxLength {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        format!("Type is longer than {} characters", self.length)
    }
}

/// Checks for scope maximum length
pub(crate) fn type_max_length(report: &Report, message: &Message, length: usize) {
    let Some(kind) = &message.header.kind else {
        return;
    };

    if kind.len() > length {
        report.add_violation(Box::new(TypeMaxLength { length }));
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_list::FooterList, header::Header};

    use super::*;

    #[test]
    fn test_type_max_length() {
        let mut report = Report::default();

        let message: Message = Message {
            header: Header::from("i18n:  add greek support"),
            body: None,
            footers: FooterList::default()
        };
        type_max_length(&mut report, &message, 10);
        assert_eq!(report.len(), 0);

        let message: Message = Message {
            header: Header::from("internationalization: add greek support"),
            body: None,
            footers: FooterList::default()
        };
        type_max_length(&mut report, &message, 10);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations.borrow()[0].rule_name(), "TypeMaxLength");
    }
}
