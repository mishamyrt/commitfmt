use commitfmt_cc::Message;
use commitfmt_macros::ViolationMetadata;

use crate::report::Report;
use crate::rules::LinterGroup;
use crate::violation::{FixMode, Violation, ViolationMetadata};

/// ## What it does
/// Checks for header not ending with full stop
///
/// ## Why is this bad?
/// Automatically generated changelogs can be hard to read
/// if the header ends with a full stop.
///
/// ## Example
/// ```git-commit
/// feat: my feature.
/// ```
///
/// Use instead:
/// ```git-commit
/// feat: my feature
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct DescriptionFullStop;

impl Violation for DescriptionFullStop {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn fix_mode(&self) -> FixMode {
        FixMode::Safe
    }

    fn fix(&self, message: &mut Message) -> Result<(), crate::violation::ViolationError> {
        message.header.description =
            message.header.description.trim_end_matches('.').to_string();
        Ok(())
    }

    #[allow(clippy::useless_format)]
    fn message(&self) -> String {
        format!("Header description is ended with a full stop")
    }
}

/// Checks for body ending with full stop
pub(crate) fn description_full_stop(report: &mut Report, message: &Message) {
    if message.header.description.ends_with('.') {
        let violation = Box::new(DescriptionFullStop);
        report.add_violation(violation);
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::{footer_vec, Header};

    use super::*;

    #[test]
    fn test_full_stop() {
        let mut report = Report::default();

        let mut message: Message = Message {
            header: Header::from("feat: my feature"),
            body: None,
            footers: footer_vec![],
        };

        description_full_stop(&mut report, &message);
        assert_eq!(report.len(), 0);

        message.header.description = " my feature.".to_string();
        description_full_stop(&mut report, &message);
        assert_eq!(report.len(), 1);
        assert_eq!(report.violations[0].rule_name(), "DescriptionFullStop");
    }
}
