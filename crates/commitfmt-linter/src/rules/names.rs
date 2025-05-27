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
        // Header description
        (Header, "description-case")          => header::DescriptionCase,
        (Header, "description-full-stop")     => header::DescriptionFullStop,
        (Header, "description-max-length")    => header::DescriptionMaxLength,
        (Header, "description-min-length")    => header::DescriptionMinLength,
        // Header type
        (Header, "type-case")                 => header::TypeCase,
        (Header, "type-enum")                 => header::TypeEnum,
        (Header, "type-max-length")           => header::TypeMaxLength,
        (Header, "type-min-length")           => header::TypeMinLength,
        (Header, "type-required")             => header::TypeRequired,
        // Header scope
        (Header, "scope-case")                => header::ScopeCase,
        (Header, "scope-enum")                => header::ScopeEnum,
        (Header, "scope-max-length")          => header::ScopeMaxLength,
        (Header, "scope-min-length")          => header::ScopeMinLength,
        (Header, "scope-required")            => header::ScopeRequired,
        // Header global
        (Header, "max-length")                => header::MaxLength,
        (Header, "min-length")                => header::MinLength,
        // Body
        (Body, "case")                        => body::Case,
        (Body, "full-stop")                   => body::FullStop,
        (Body, "leading-newline")             => body::LeadingNewLine,
        (Body, "max-line-length")             => body::MaxLineLength,
        (Body, "max-length")                  => body::MaxLength,
        (Body, "min-length")                  => body::MinLength,
        // Footer
        (Footer, "breaking-exclamation")      => footer::BreakingExclamation,
        (Footer, "exists")                    => footer::Exists,
        (Footer, "max-length")                => footer::MaxLength,
        (Footer, "max-line-length")           => footer::MaxLineLength,
        (Footer, "min-length")                => footer::MinLength,
        _ => return None
    })
}
