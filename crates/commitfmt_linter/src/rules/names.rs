use crate::rules::LinterGroup;

#[allow(unused_imports)]
use crate::rules::body;
use crate::violation::Violation;

#[commitfmt_macros::map_names]
pub fn name_to_rule(linter: Linter, code: &str) -> Option<Rule> {
    #[allow(clippy::enum_glob_use)]
    use LinterGroup::*;

    #[rustfmt::skip]
    Some(match (linter, code) {
        (Header, "description-leading-space") => header::LeadingSpace,
        (Body, "leading-newline")             => body::LeadingNewLine,
        (Body, "max-line-length")             => body::MaxLineLength,
        (Body, "max-length")                  => body::MaxLength,
        (Body, "min-length")                  => body::MinLength,
        (Body, "full-stop")                   => body::FullStop,
        (Body, "case")                        => body::Case,
        _ => return None
    })
}
