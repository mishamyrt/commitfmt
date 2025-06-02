use nom::{
    bytes::complete::take_while1,
    character::complete::char,
    combinator::{all_consuming, recognize},
    multi::separated_list1,
    sequence::pair,
    IResult, Parser,
};

const NAME_ANY: &str = "any";
const NAME_LOWER_FIRST: &str = "lower-first";
const NAME_UPPER_FIRST: &str = "upper-first";
const NAME_LOWER: &str = "lower";
const NAME_UPPER: &str = "upper";
const NAME_CAMEL: &str = "camel";
const NAME_KEBAB: &str = "kebab";
const NAME_PASCAL: &str = "pascal";
const NAME_CAPITALIZED_KEBAB: &str = "capitalized-kebab";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum IdentifierCase {
    #[default]
    Any,
    Camel,
    Kebab,
    Pascal,
    CapitalizedKebab,
    Lower,
    Upper,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum TextCase {
    #[default]
    Any,
    LowerFirst,
    UpperFirst,
}

impl std::fmt::Display for IdentifierCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl IdentifierCase {
    pub fn is_match(&self, word: &str) -> bool {
        match self {
            IdentifierCase::Any => true,
            IdentifierCase::Camel => all_consuming(Self::camel_case).parse(word).is_ok(),
            IdentifierCase::Kebab => all_consuming(Self::kebab_case).parse(word).is_ok(),
            IdentifierCase::Pascal => all_consuming(Self::pascal_case).parse(word).is_ok(),
            IdentifierCase::CapitalizedKebab => {
                all_consuming(Self::capitalized_kebab_case).parse(word).is_ok()
            }
            IdentifierCase::Lower => word.chars().all(char::is_lowercase),
            IdentifierCase::Upper => word.chars().all(char::is_uppercase),
        }
    }

    pub fn from_name(name: &str) -> Option<IdentifierCase> {
        match name {
            NAME_ANY => Some(IdentifierCase::Any),
            NAME_CAMEL => Some(IdentifierCase::Camel),
            NAME_KEBAB => Some(IdentifierCase::Kebab),
            NAME_PASCAL => Some(IdentifierCase::Pascal),
            NAME_CAPITALIZED_KEBAB => Some(IdentifierCase::CapitalizedKebab),
            NAME_LOWER => Some(IdentifierCase::Lower),
            NAME_UPPER => Some(IdentifierCase::Upper),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            IdentifierCase::Any => NAME_ANY,
            IdentifierCase::Camel => NAME_CAMEL,
            IdentifierCase::Kebab => NAME_KEBAB,
            IdentifierCase::CapitalizedKebab => NAME_CAPITALIZED_KEBAB,
            IdentifierCase::Pascal => NAME_PASCAL,
            IdentifierCase::Lower => NAME_LOWER,
            IdentifierCase::Upper => NAME_UPPER,
        }
    }

    /// Matches lower-kebab-case
    /// e.g. `foo-bar`
    fn kebab_case(input: &str) -> IResult<&str, &str> {
        recognize(separated_list1(char('-'), take_while1(|c: char| c.is_lowercase())))
            .parse(input)
    }

    /// Matches capitalized-kebab-case
    /// e.g. `Foo-Bar`
    fn capitalized_kebab_case(input: &str) -> IResult<&str, &str> {
        recognize(separated_list1(
            char('-'),
            pair(
                take_while1(|c: char| c.is_uppercase()),
                take_while1(|c: char| c.is_lowercase()),
            ),
        ))
        .parse(input)
    }

    /// Matches pascal-case
    /// e.g. `FooBar`
    fn pascal_case(input: &str) -> IResult<&str, &str> {
        recognize(pair(
            take_while1(|c: char| c.is_uppercase()),
            take_while1(|c: char| c.is_alphanumeric()),
        ))
        .parse(input)
    }

    /// Matches camel-case
    /// e.g. `fooBar`
    fn camel_case(input: &str) -> IResult<&str, &str> {
        recognize(pair(
            take_while1(|c: char| c.is_lowercase()),
            take_while1(|c: char| c.is_alphanumeric()),
        ))
        .parse(input)
    }
}

impl std::fmt::Display for TextCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl TextCase {
    pub fn is_match(&self, text: &str) -> bool {
        match self {
            TextCase::Any => true,
            TextCase::LowerFirst => text.chars().next().is_some_and(char::is_lowercase),
            TextCase::UpperFirst => text.chars().next().is_some_and(char::is_uppercase),
        }
    }

    pub fn from_name(name: &str) -> Option<TextCase> {
        match name {
            NAME_ANY => Some(TextCase::Any),
            NAME_LOWER_FIRST => Some(TextCase::LowerFirst),
            NAME_UPPER_FIRST => Some(TextCase::UpperFirst),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TextCase::Any => NAME_ANY,
            TextCase::LowerFirst => NAME_LOWER_FIRST,
            TextCase::UpperFirst => NAME_UPPER_FIRST,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::case::TextCase;

    use super::IdentifierCase;

    #[test]
    fn test_id_match_any() {
        assert!(IdentifierCase::Any.is_match("f_o-bAr"));
    }

    #[test]
    fn test_id_match_camel() {
        assert!(IdentifierCase::Camel.is_match("fooBar"));
        assert!(!IdentifierCase::Camel.is_match("FooBar"));
        assert!(!IdentifierCase::Camel.is_match("foo-Bar"));
    }

    #[test]
    fn test_id_match_capitalized_kebab() {
        assert!(IdentifierCase::CapitalizedKebab.is_match("Foo-Bar"));
        assert!(!IdentifierCase::CapitalizedKebab.is_match("foo-bar"));
        assert!(!IdentifierCase::CapitalizedKebab.is_match("fooBar"));
    }

    #[test]
    fn test_id_match_kebab() {
        assert!(IdentifierCase::Kebab.is_match("foo-bar"));
        assert!(IdentifierCase::Kebab.is_match("foobar"));
        assert!(!IdentifierCase::Kebab.is_match("FooBar"));
        assert!(!IdentifierCase::Kebab.is_match("foo_bar"));
    }

    #[test]
    fn test_id_match_pascal() {
        assert!(IdentifierCase::Pascal.is_match("FooBar"));
        assert!(IdentifierCase::Pascal.is_match("Foobar"));
        assert!(!IdentifierCase::Pascal.is_match("foo-bar"));
        assert!(!IdentifierCase::Pascal.is_match("fooBar"));
    }

    #[test]
    fn test_id_match_lower() {
        assert!(IdentifierCase::Lower.is_match("foobar"));
        assert!(!IdentifierCase::Lower.is_match("FOOBAR"));
        assert!(!IdentifierCase::Lower.is_match("FooBar"));
    }

    #[test]
    fn test_id_match_upper() {
        assert!(IdentifierCase::Upper.is_match("FOOBAR"));
        assert!(!IdentifierCase::Upper.is_match("foobar"));
        assert!(!IdentifierCase::Upper.is_match("FooBar"));
    }

    #[test]
    fn test_id_from_name() {
        assert_eq!(IdentifierCase::from_name("any"), Some(IdentifierCase::Any));
        assert_eq!(IdentifierCase::from_name("camel"), Some(IdentifierCase::Camel));
        assert_eq!(IdentifierCase::from_name("kebab"), Some(IdentifierCase::Kebab));
        assert_eq!(IdentifierCase::from_name("pascal"), Some(IdentifierCase::Pascal));
        assert_eq!(
            IdentifierCase::from_name("capitalized-kebab"),
            Some(IdentifierCase::CapitalizedKebab)
        );
        assert_eq!(IdentifierCase::from_name("lower"), Some(IdentifierCase::Lower));
        assert_eq!(IdentifierCase::from_name("upper"), Some(IdentifierCase::Upper));
        assert_eq!(IdentifierCase::from_name("foo"), None);
    }

    #[test]
    fn test_id_name() {
        assert_eq!(IdentifierCase::Any.name(), "any");
        assert_eq!(IdentifierCase::Camel.name(), "camel");
        assert_eq!(IdentifierCase::Kebab.name(), "kebab");
        assert_eq!(IdentifierCase::Pascal.name(), "pascal");
        assert_eq!(IdentifierCase::CapitalizedKebab.name(), "capitalized-kebab");
        assert_eq!(IdentifierCase::Lower.name(), "lower");
        assert_eq!(IdentifierCase::Upper.name(), "upper");
    }

    #[test]
    fn test_id_case_display() {
        assert_eq!(IdentifierCase::Any.to_string(), "any");
        assert_eq!(IdentifierCase::Kebab.to_string(), "kebab");
    }

    #[test]
    fn test_text_match_lower_first() {
        assert!(TextCase::LowerFirst.is_match("foo bar"));
        assert!(!TextCase::LowerFirst.is_match("FOO BAR"));
        assert!(!TextCase::LowerFirst.is_match("Foo bar"));
    }

    #[test]
    fn test_text_match_upper_first() {
        assert!(TextCase::UpperFirst.is_match("Foo bar"));
        assert!(!TextCase::UpperFirst.is_match("foo bar"));
        assert!(!TextCase::UpperFirst.is_match("fOOBAR"));
    }

    #[test]
    fn test_text_match_any() {
        assert!(TextCase::Any.is_match("foo bar"));
        assert!(TextCase::Any.is_match("FOO BAR"));
        assert!(TextCase::Any.is_match("Foo bar"));
    }

    #[test]
    fn test_text_from_name() {
        assert_eq!(TextCase::from_name("any"), Some(TextCase::Any));
        assert_eq!(TextCase::from_name("lower-first"), Some(TextCase::LowerFirst));
        assert_eq!(TextCase::from_name("upper-first"), Some(TextCase::UpperFirst));
        assert_eq!(TextCase::from_name("foo"), None);
    }

    #[test]
    fn test_text_name() {
        assert_eq!(TextCase::Any.name(), "any");
        assert_eq!(TextCase::LowerFirst.name(), "lower-first");
        assert_eq!(TextCase::UpperFirst.name(), "upper-first");
    }

    #[test]
    fn test_text_case_display() {
        assert_eq!(TextCase::Any.to_string(), "any");
        assert_eq!(TextCase::LowerFirst.to_string(), "lower-first");
    }
}
