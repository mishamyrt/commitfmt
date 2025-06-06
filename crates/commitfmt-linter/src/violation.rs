use commitfmt_cc::Message;
use thiserror::Error;

use crate::rules::LinterGroup;

/// A violation of a lint rule
#[derive(Error, Debug)]
pub enum ViolationError {
    #[error("Unfixable violation")]
    Unfixable(),

    #[error("Body is empty")]
    EmptyBody(),

    #[error("Data is empty: {0}")]
    Empty(String),
}

/// The fix mode of a violation
#[derive(Debug, PartialEq, Eq)]
pub enum FixMode {
    Safe,
    Unsafe,
    Unfixable,
}

pub trait ViolationMetadata {
    /// Returns the rule name of this violation
    fn rule_name(&self) -> &'static str;

    /// Returns an explanation of what this violation catches,
    /// why it's bad, and what users should do instead.
    fn explain(&self) -> Option<&'static str>;
}

pub trait Violation: ViolationMetadata {
    /// The message used to describe the violation.
    fn message(&self) -> String;

    /// Returns the linter group of this violation
    fn group(&self) -> LinterGroup;

    /// Whether the violation is fixable
    fn fix_mode(&self) -> FixMode {
        FixMode::Unfixable
    }

    /// Attempt to fix the violation
    fn fix(&self, _: &mut Message) -> Result<(), ViolationError> {
        Err(ViolationError::Unfixable())
    }
}

impl std::fmt::Display for Box<dyn Violation> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rule_name = self.rule_name();
        let group = self.group().as_display();
        let message = self.message();

        write!(f, "{group}::{rule_name}: {message}")
    }
}

/// A test violation for testing purposes
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TestViolation;

impl ViolationMetadata for TestViolation {
    fn rule_name(&self) -> &'static str {
        "test"
    }

    fn explain(&self) -> Option<&'static str> {
        unimplemented!()
    }
}

impl Violation for TestViolation {
    fn group(&self) -> LinterGroup {
        LinterGroup::Header
    }

    fn message(&self) -> String {
        "test".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fix_mode() {
        let violation = TestViolation;
        assert_eq!(violation.fix_mode(), FixMode::Unfixable);
        assert!(violation.fix(&mut Message::default()).is_err());
    }

    #[test]
    fn test_display() {
        let violation = TestViolation;
        let violation_box = Box::new(violation) as Box<dyn Violation>;
        assert_eq!(format!("{violation_box}"), "header::test: test");
    }
}
