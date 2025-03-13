
use commitfmt_cc::message::Message;
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
#[derive(Debug)]
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

