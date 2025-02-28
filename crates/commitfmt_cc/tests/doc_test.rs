#![cfg(test)]
extern crate test_generator;

use commitfmt_cc::message::Message;
use nom::multi::many0;
use nom::{
    bytes::complete::{tag, take_until},
    sequence::delimited,
};
use nom::{IResult, Parser};
use test_generator::test_resources;
use toml::Table;

#[derive(Debug)]
struct Case {
    git_commit: String,
    toml: String,
}

fn parse_case(input: &str) -> IResult<&str, Case> {
    let (input, _) = take_until("<!-- <DOC_TEST> -->").parse(input)?;
    let (input, _) = tag("<!-- <DOC_TEST> -->\n").parse(input)?;

    let (input, git_commit) = delimited(tag("```git-commit\n"), take_until("```"), tag("```\n")).parse(input)?;

    let (input, _) = many0(tag("\n")).parse(input)?;

    let (input, toml) = delimited(tag("```toml\n"), take_until("```"), tag("```\n")).parse(input)?;

    let (input, _) = tag("<!-- </DOC_TEST> -->\n").parse(input)?;

    Ok((
        input,
        Case {
            git_commit: git_commit.trim().to_string(),
            toml: toml.trim().to_string(),
        },
    ))
}

fn parse_cases(input: &str) -> IResult<&str, Vec<Case>> {
    many0(parse_case).parse(input)
}

#[test_resources("resources/doc_tests/*.md")]
fn verify_resource(resource: &str) {
    let input = std::fs::read_to_string(resource).unwrap();
    let (_, cases) = parse_cases(&input).unwrap();

    for case in cases {
        let expected = toml::from_str::<Table>(&case.toml).unwrap();

        let actual = Message::parse(&case.git_commit).unwrap();
        if expected.contains_key("kind") {
            assert_eq!(actual.header.kind, Some(expected["kind"].as_str().unwrap().to_string()));
        } else {
            assert_eq!(actual.header.kind, None);
        }

        if expected.contains_key("scope") {
            assert_eq!(actual.header.scope.len(), expected["scope"].as_array().unwrap().len());
            for i in 0..expected["scope"].as_array().unwrap().len() {
                assert_eq!(actual.header.scope[i], expected["scope"][i].as_str().unwrap().to_string());
            }
        }

        if expected.contains_key("description") {
            assert_eq!(actual.header.description, expected["description"].as_str().unwrap().to_string());
        }

        if expected.contains_key("body") {
            assert_eq!(actual.body, Some(expected["body"].as_str().unwrap().to_string()));
        } else {
            assert_eq!(actual.body, None);
        }

        if expected.contains_key("footers") {
            assert_eq!(actual.footers.len(), expected["footers"].as_array().unwrap().len());
            for i in 0..expected["footers"].as_array().unwrap().len() {
                let footer = expected["footers"][i].as_table().unwrap();
                assert_eq!(actual.footers[i].key, footer["key"].as_str().unwrap().to_string());
                assert_eq!(actual.footers[i].value, footer["value"].as_str().unwrap().to_string());
            }
        }
    }

    // assert_eq!(cases.len(), 1);
}
