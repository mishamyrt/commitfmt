use colored::Colorize;
use commitfmt_linter::{rules::Rule, violation::Violation};
use log::info;

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
pub(crate) fn report_violations<'a>(
    violations: impl Iterator<Item = &'a Box<dyn Violation>>,
) -> usize {
    let mut count: usize = 0;
    for violation_box in violations {
        count += 1;
        print_violation(violation_box.as_ref());
    }

    count
}

/// Prints a single violation to the logger
pub(crate) fn print_violation(violation: &dyn Violation) {
    let Some(rule) = Rule::from_violation(violation) else {
        panic!("Failed to get rule from violation");
    };
    let line = format!("- {} {}", violation.message(), rule.as_display().dimmed());
    info!("{}", line);
}
