use nom::{
    bytes::complete::take_while1,
    character::complete::char,
    combinator::{all_consuming, recognize},
    multi::separated_list1,
    sequence::pair,
    IResult, Parser,
};

const NAME_ANY: &str = "any";
const NAME_LOWER: &str = "lower";
const NAME_UPPER: &str = "upper";
const NAME_CAMEL: &str = "camel";
const NAME_KEBAB: &str = "kebab";
const NAME_PASCAL: &str = "pascal";
const NAME_SNAKE: &str = "snake";
const NAME_START: &str = "start";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum WordCase {
    #[default]
    Any,
    Lower,
    Upper,
    Camel,
    Kebab,
    Pascal,
    Snake,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum TextCase {
    #[default]
    Any,
    Lower,
    Upper,
    Start,
}

impl std::fmt::Display for WordCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl WordCase {
    pub fn is_match(&self, word: &str) -> bool {
        match self {
            WordCase::Any => true,
            WordCase::Lower => word.chars().all(char::is_lowercase),
            WordCase::Upper => word.chars().all(char::is_uppercase),
            WordCase::Camel => {
                let mut chars = word.chars();
                chars.next().is_some_and(char::is_lowercase) && chars.any(char::is_uppercase)
            }
            WordCase::Kebab => all_consuming(Self::kebab_case).parse(word).is_ok(),
            WordCase::Pascal => all_consuming(Self::pascal_case).parse(word).is_ok(),
            WordCase::Snake => all_consuming(Self::snake_case).parse(word).is_ok(),
        }
    }

    pub fn from_name(name: &str) -> Option<WordCase> {
        match name {
            NAME_ANY => Some(WordCase::Any),
            NAME_LOWER => Some(WordCase::Lower),
            NAME_UPPER => Some(WordCase::Upper),
            NAME_CAMEL => Some(WordCase::Camel),
            NAME_KEBAB => Some(WordCase::Kebab),
            NAME_PASCAL => Some(WordCase::Pascal),
            NAME_SNAKE => Some(WordCase::Snake),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            WordCase::Any => NAME_ANY,
            WordCase::Lower => NAME_LOWER,
            WordCase::Upper => NAME_UPPER,
            WordCase::Camel => NAME_CAMEL,
            WordCase::Kebab => NAME_KEBAB,
            WordCase::Pascal => NAME_PASCAL,
            WordCase::Snake => NAME_SNAKE,
        }
    }

    fn kebab_case(input: &str) -> IResult<&str, &str> {
        recognize(separated_list1(char('-'), take_while1(|c: char| c.is_lowercase())))
            .parse(input)
    }

    fn pascal_case(input: &str) -> IResult<&str, &str> {
        recognize(pair(
            take_while1(|c: char| c.is_uppercase()),
            take_while1(|c: char| c.is_alphanumeric()),
        ))
        .parse(input)
    }

    fn snake_case(input: &str) -> IResult<&str, &str> {
        recognize(separated_list1(char('_'), take_while1(|c: char| c.is_lowercase())))
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
            TextCase::Lower => text.chars().all(|c| {
                if c.is_alphabetic() {
                    return c.is_lowercase();
                }
                true
            }),
            TextCase::Upper => text.chars().all(|c| {
                if c.is_alphabetic() {
                    return c.is_uppercase();
                }
                true
            }),
            TextCase::Start => text.starts_with(|c: char| c.is_uppercase()),
        }
    }

    pub fn from_name(name: &str) -> Option<TextCase> {
        match name {
            NAME_ANY => Some(TextCase::Any),
            NAME_LOWER => Some(TextCase::Lower),
            NAME_UPPER => Some(TextCase::Upper),
            NAME_START => Some(TextCase::Start),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TextCase::Any => NAME_ANY,
            TextCase::Lower => NAME_LOWER,
            TextCase::Upper => NAME_UPPER,
            TextCase::Start => NAME_START,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::case::TextCase;

    use super::WordCase;

    #[test]
    fn test_match_any() {
        assert!(WordCase::Any.is_match("f_o-bAr"));
    }

    #[test]
    fn test_match_lower() {
        assert!(WordCase::Lower.is_match("foobar"));
        assert!(!WordCase::Lower.is_match("Foobar"));
        assert!(!WordCase::Lower.is_match("foo bar"));
    }

    #[test]
    fn test_match_upper() {
        assert!(WordCase::Upper.is_match("FOOBAR"));
        assert!(!WordCase::Upper.is_match("foobar"));
        assert!(!WordCase::Upper.is_match("FOO BAR"));
    }

    #[test]
    fn test_match_camel() {
        assert!(WordCase::Camel.is_match("fooBar"));
        assert!(!WordCase::Camel.is_match("FooBar"));
        assert!(!WordCase::Camel.is_match("foo-bar"));
    }

    #[test]
    fn test_match_kebab() {
        assert!(WordCase::Kebab.is_match("foo-bar"));
        assert!(WordCase::Kebab.is_match("foobar"));
        assert!(!WordCase::Kebab.is_match("FooBar"));
        assert!(!WordCase::Kebab.is_match("foo_bar"));
    }

    #[test]
    fn test_match_pascal() {
        assert!(WordCase::Pascal.is_match("FooBar"));
        assert!(WordCase::Pascal.is_match("Foobar"));
        assert!(!WordCase::Pascal.is_match("foo-bar"));
        assert!(!WordCase::Pascal.is_match("fooBar"));
    }

    #[test]
    fn test_match_snake() {
        assert!(WordCase::Snake.is_match("foo_bar"));
        assert!(!WordCase::Snake.is_match("FooBar"));
    }

    #[test]
    fn test_text_lower() {
        assert!(TextCase::Lower.is_match("foo bar"));
        assert!(!TextCase::Lower.is_match("FOO BAR"));
        assert!(!TextCase::Lower.is_match("Foo bar"));
    }

    #[test]
    fn test_text_upper() {
        assert!(TextCase::Upper.is_match("FOO BAR"));
        assert!(!TextCase::Upper.is_match("foo bar"));
        assert!(!TextCase::Upper.is_match("Foo bar"));
    }

    #[test]
    fn test_text_start() {
        assert!(TextCase::Start.is_match("Foo bar"));
        assert!(TextCase::Start.is_match("FOO BAR"));
        assert!(!TextCase::Start.is_match("foo bar"));
    }

    #[test]
    fn test_text_any() {
        assert!(TextCase::Any.is_match("foo bar"));
        assert!(TextCase::Any.is_match("FOO BAR"));
        assert!(TextCase::Any.is_match("Foo bar"));
    }
}
