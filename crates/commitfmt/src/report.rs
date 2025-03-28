use log::info;
use colored::Colorize;
use commitfmt_linter::{rules::Rule, violation::Violation};

/// Reports all violations to the logger and returns the total count.
///
/// This function iterates through the provided violations, logs each one
/// with its message and rule identifier, and keeps track of the total count.
///
/// # Arguments
///
/// * `violations` - An iterator of references to `Box<dyn Violation>` objects
///
/// # Returns
///
/// The total number of violations reported
///
/// # Panics
///
/// Panics if a rule cannot be obtained from a violation
pub(crate) fn report_violations<'a, T: Iterator>(violations: T) -> usize
where
    T: Iterator<Item = &'a Box<dyn Violation>>,
{
    let mut count: usize = 0;
    for violation_box in violations {
        count += 1;
        let violation = violation_box.as_ref();
        let Some(rule) = Rule::from_violation(violation) else {
            panic!("Failed to get rule from violation");
        };

        let line = format!("- {} {}", violation.message(), rule.as_display().dimmed());
        info!("{}", line);
    }

    count
}
