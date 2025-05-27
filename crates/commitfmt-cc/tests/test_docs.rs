#![cfg(test)]
extern crate test_generator;

use std::path::PathBuf;

use commitfmt_cc::{Footer, Message, Scope, SeparatorAlignment};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::char;
use nom::multi::many0;
use nom::{IResult, Parser};
use serde_derive::Deserialize;
use test_generator::test_resources;
use thiserror::Error;
use toml::map::Map;
use toml::{Table, Value};

const TAG_PREFIX: &str = "<!--<";
const TAG_INPUT: &str = "<!--<test-input>-->";
const TAG_RESULT: &str = "<!--<test-result>-->";
const TAG_OPTIONS: &str = "<!--<test-options>-->";
const TAG_START_PREFIX: &str = "<!--<test-case id=\"";
const TAG_START_SUFFIX: &str = "\">-->";
const TAG_END: &str = "<!--</test-case>-->";
const TAG_MD_CODE_BLOCK: &str = "```";

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unable to parse input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, PartialEq, Default, Deserialize)]
struct CaseOptions {
    separators: String,
}

#[derive(Debug, PartialEq, Default)]
struct Case {
    id: String,
    input: String,
    expected: Message,
    options: CaseOptions,
}

impl Case {
    fn take_case(input: &str) -> IResult<&str, Case> {
        // Find and skip start tag
        let (input, _) = take_until(TAG_START_PREFIX).parse(input)?;
        let (input, _) = tag(TAG_START_PREFIX).parse(input)?;
        let (input, id) = take_until(TAG_START_SUFFIX).parse(input)?;
        let (input, _) = (tag(TAG_START_SUFFIX), char('\n')).parse(input)?;

        let mut case = Case { id: id.to_string(), ..Default::default() };

        let mut rest: &str = input;
        loop {
            // Find next test tag and extract it
            (rest, _) = take_until(TAG_PREFIX).parse(rest)?;
            let (input, tag) =
                alt((tag(TAG_INPUT), tag(TAG_RESULT), tag(TAG_OPTIONS), tag(TAG_END)))
                    .parse(rest)?;
            (rest, _) = (char('\n')).parse(input)?;

            rest = match tag {
                TAG_INPUT => {
                    let (input, code_block) = Self::take_code_block(rest)?;
                    case.input = code_block.to_string();
                    input
                }
                TAG_RESULT => {
                    let (input, result_text) = Self::take_code_block(rest)?;
                    let result = toml::from_str::<Table>(result_text).unwrap();
                    case.expected = Self::parse_expected(&result);
                    input
                }
                TAG_OPTIONS => {
                    let (input, case_options_text) = Self::take_code_block(rest)?;
                    case.options = toml::from_str(case_options_text).unwrap();
                    input
                }
                TAG_END => {
                    assert_ne!(case.input, "", "Case input is empty");
                    assert_ne!(
                        case.expected,
                        Message::default(),
                        "Case expected result is empty"
                    );

                    if case.options.separators.is_empty() {
                        case.options.separators = Footer::DEFAULT_SEPARATOR.to_string();
                    }

                    return Ok((rest, case));
                }
                _ => unreachable!(),
            };
        }
    }

    fn parse_expected(input: &Map<String, Value>) -> Message {
        let mut result = Message::default();
        for key in input.keys() {
            match key.as_str() {
                "kind" => {
                    result.header.kind = Some(input[key].as_str().unwrap().to_string());
                }
                "scope" => {
                    let scope: Vec<String> = input["scope"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|s| s.as_str().unwrap().to_string())
                        .collect();
                    result.header.scope = Scope::from(scope);
                }
                "breaking" => {
                    result.header.breaking = input[key].as_bool().unwrap();
                }
                "description" => {
                    result.header.description = input[key].as_str().unwrap().to_string();
                }
                "body" => {
                    result.body = Some(input[key].as_str().unwrap().to_string());
                }
                "footers" => {
                    #[rustfmt::skip]
                    let footers: Vec<Table> = input[key].as_array().unwrap()
                        .iter()
                        .map(|f| f.as_table().unwrap().clone())
                        .collect();
                    result.footers =
                        footers.iter().map(|f| Self::parse_footer(f).unwrap()).collect();
                }
                _ => (),
            }
        }

        result
    }

    fn parse_footer(input: &Map<String, Value>) -> Result<Footer, ParseError> {
        let Some(key) = input["key"].as_str() else {
            return Err(ParseError::InvalidInput("footer key is not a string".to_string()));
        };
        let Some(value) = input["value"].as_str() else {
            return Err(ParseError::InvalidInput("footer value is not a string".to_string()));
        };
        let Some(alignment_str) = input["alignment"].as_str() else {
            return Err(ParseError::InvalidInput(
                "footer alignment is not a string".to_string(),
            ));
        };
        let Some(alignment) = SeparatorAlignment::from(alignment_str) else {
            return Err(ParseError::InvalidInput(
                "footer alignment is not valid. Must be 'left' or 'right'".to_string(),
            ));
        };

        let Some(separator_string) = input["separator"].as_str() else {
            return Err(ParseError::InvalidInput(
                "footer separator is not a string".to_string(),
            ));
        };
        if separator_string.len() != 1 {
            return Err(ParseError::InvalidInput(
                "footer separator must be a single character".to_string(),
            ));
        }
        let separator = separator_string.chars().next().unwrap();

        Ok(Footer { key: key.to_string(), value: value.to_string(), alignment, separator })
    }

    fn take_code_block(input: &str) -> IResult<&str, &str> {
        let (input, _) = tag(TAG_MD_CODE_BLOCK).parse(input)?;
        let (input, _) = take_until("\n").parse(input)?;
        let (input, _) = char('\n').parse(input)?;
        let (input, code_block) = take_until(TAG_MD_CODE_BLOCK).parse(input)?;
        let (input, _) = (tag(TAG_MD_CODE_BLOCK), char('\n')).parse(input)?;
        Ok((input, code_block))
    }

    fn parse_all(input: &str) -> IResult<&str, Vec<Case>> {
        many0(Self::take_case).parse(input)
    }
}

#[allow(clippy::print_stdout)]
#[test_resources("resources/test_docs/*.md")]
fn verify_resource(resource: &str) {
    let md_path = PathBuf::from(resource);
    let input = std::fs::read_to_string(&md_path).unwrap();
    let (_, cases) = Case::parse_all(&input).unwrap();

    let stem = md_path.file_stem().unwrap().to_str().unwrap();

    println!("testcases___: {}", cases.len());

    for case in cases {
        let case_path = format!("{}/{}", stem, case.id);

        println!("Case: {}", case.id);
        let expected = case.expected;
        // TODO: pass custom separators and comment symbol
        let actual = Message::parse(&case.input, None, None).unwrap();

        assert_eq!(actual.header.kind, expected.header.kind, "kind at {case_path}");
        assert_eq!(actual.header.scope, expected.header.scope, "scope at {case_path}");
        assert_eq!(
            actual.header.breaking, expected.header.breaking,
            "breaking at {case_path}"
        );
        assert_eq!(
            actual.header.description, expected.header.description,
            "description at {case_path}"
        );
        assert_eq!(actual.body, expected.body, "body at {case_path}");
        assert_eq!(actual.footers, expected.footers, "footers at {case_path}");
    }
}
