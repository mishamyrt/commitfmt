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
pub enum KeyCase {
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

impl std::fmt::Display for KeyCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl KeyCase {
    pub fn is_match(&self, word: &str) -> bool {
        match self {
            KeyCase::Any => true,
            KeyCase::Camel => all_consuming(Self::camel_case).parse(word).is_ok(),
            KeyCase::Kebab => all_consuming(Self::kebab_case).parse(word).is_ok(),
            KeyCase::Pascal => all_consuming(Self::pascal_case).parse(word).is_ok(),
            KeyCase::CapitalizedKebab => {
                all_consuming(Self::capitalized_kebab_case).parse(word).is_ok()
            }
            KeyCase::Lower => word.chars().all(char::is_lowercase),
            KeyCase::Upper => word.chars().all(char::is_uppercase),
        }
    }

    pub fn from_name(name: &str) -> Option<KeyCase> {
        match name {
            NAME_ANY => Some(KeyCase::Any),
            NAME_CAMEL => Some(KeyCase::Camel),
            NAME_KEBAB => Some(KeyCase::Kebab),
            NAME_PASCAL => Some(KeyCase::Pascal),
            NAME_CAPITALIZED_KEBAB => Some(KeyCase::CapitalizedKebab),
            NAME_LOWER => Some(KeyCase::Lower),
            NAME_UPPER => Some(KeyCase::Upper),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            KeyCase::Any => NAME_ANY,
            KeyCase::Camel => NAME_CAMEL,
            KeyCase::Kebab => NAME_KEBAB,
            KeyCase::CapitalizedKebab => NAME_CAPITALIZED_KEBAB,
            KeyCase::Pascal => NAME_PASCAL,
            KeyCase::Lower => NAME_LOWER,
            KeyCase::Upper => NAME_UPPER,
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

    use super::KeyCase;

    #[test]
    fn test_key_match_any() {
        assert!(KeyCase::Any.is_match("f_o-bAr"));
    }

    #[test]
    fn test_key_match_camel() {
        assert!(KeyCase::Camel.is_match("fooBar"));
        assert!(!KeyCase::Camel.is_match("FooBar"));
        assert!(!KeyCase::Camel.is_match("foo-Bar"));
    }

    #[test]
    fn test_key_match_capitalized_kebab() {
        assert!(KeyCase::CapitalizedKebab.is_match("Foo-Bar"));
        assert!(!KeyCase::CapitalizedKebab.is_match("foo-bar"));
        assert!(!KeyCase::CapitalizedKebab.is_match("fooBar"));
    }

    #[test]
    fn test_key_match_kebab() {
        assert!(KeyCase::Kebab.is_match("foo-bar"));
        assert!(KeyCase::Kebab.is_match("foobar"));
        assert!(!KeyCase::Kebab.is_match("FooBar"));
        assert!(!KeyCase::Kebab.is_match("foo_bar"));
    }

    #[test]
    fn test_key_match_pascal() {
        assert!(KeyCase::Pascal.is_match("FooBar"));
        assert!(KeyCase::Pascal.is_match("Foobar"));
        assert!(!KeyCase::Pascal.is_match("foo-bar"));
        assert!(!KeyCase::Pascal.is_match("fooBar"));
    }

    #[test]
    fn test_key_match_lower() {
        assert!(KeyCase::Lower.is_match("foobar"));
        assert!(!KeyCase::Lower.is_match("FOOBAR"));
        assert!(!KeyCase::Lower.is_match("FooBar"));
    }

    #[test]
    fn test_key_match_upper() {
        assert!(KeyCase::Upper.is_match("FOOBAR"));
        assert!(!KeyCase::Upper.is_match("foobar"));
        assert!(!KeyCase::Upper.is_match("FooBar"));
    }

    #[test]
    fn test_key_from_name() {
        assert_eq!(KeyCase::from_name("any"), Some(KeyCase::Any));
        assert_eq!(KeyCase::from_name("camel"), Some(KeyCase::Camel));
        assert_eq!(KeyCase::from_name("kebab"), Some(KeyCase::Kebab));
        assert_eq!(KeyCase::from_name("pascal"), Some(KeyCase::Pascal));
        assert_eq!(KeyCase::from_name("capitalized-kebab"), Some(KeyCase::CapitalizedKebab));
        assert_eq!(KeyCase::from_name("lower"), Some(KeyCase::Lower));
        assert_eq!(KeyCase::from_name("upper"), Some(KeyCase::Upper));
        assert_eq!(KeyCase::from_name("foo"), None);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(KeyCase::Any.name(), "any");
        assert_eq!(KeyCase::Camel.name(), "camel");
        assert_eq!(KeyCase::Kebab.name(), "kebab");
        assert_eq!(KeyCase::Pascal.name(), "pascal");
        assert_eq!(KeyCase::CapitalizedKebab.name(), "capitalized-kebab");
        assert_eq!(KeyCase::Lower.name(), "lower");
        assert_eq!(KeyCase::Upper.name(), "upper");
    }

    #[test]
    fn test_key_case_display() {
        assert_eq!(KeyCase::Any.to_string(), "any");
        assert_eq!(KeyCase::Kebab.to_string(), "kebab");
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
