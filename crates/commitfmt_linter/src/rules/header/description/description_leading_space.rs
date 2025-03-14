use crate::rules::LinterGroup;
use crate::violation::{Violation, ViolationMetadata};
use crate::report::Report;
use commitfmt_cc::Message;
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
pub(crate) struct DescriptionLeadingSpace;

impl Violation for DescriptionLeadingSpace {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        String::from("Body must start with a newline")
    }

    fn fix(&self, message: &mut Message) -> Result<(), crate::violation::ViolationError> {
        let description = &mut message.header.description;
        description.insert(0, ' ');

        Ok(())
    }
}

pub(crate) fn description_leading_space(report: &Report, message: &Message) {
    let description = &message.header.description;
    if !description.starts_with(' ') {
        report.add_violation(Box::new(DescriptionLeadingSpace));
    }
}
