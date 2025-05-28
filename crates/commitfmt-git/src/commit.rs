use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{hex_digit1, line_ending},
    combinator::{map, opt},
    multi::many0,
    sequence::terminated,
    IResult, Parser,
};

/// Represents a Git commit parsed from log output
#[derive(Debug, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub message: String,
}

const COMMIT_SEPARATOR: &str = "#-eoc-#";

/// Parses a single commit from the git log output
fn parse_commit(input: &str) -> IResult<&str, Commit> {
    let (input, hash) = terminated(hex_digit1, line_ending).parse(input)?;
    let (input, message) = take_until(COMMIT_SEPARATOR)(input)?;

    Ok((input, Commit { sha: hash.to_string(), message: message.to_string() }))
}

/// Parses the entire git log output containing multiple commits
pub(crate) fn parse_git_log(input: &str) -> IResult<&str, Vec<Commit>> {
    let commit_parser =
        map((parse_commit, tag(COMMIT_SEPARATOR), opt(line_ending)), |(commit, _, _)| commit);

    many0(commit_parser).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_log() {
        let input = "ee0b330\nfeat: test commit\nbody\nFooter: value\n#-eoc-#\n36c13b5\nfeat(linter): remove description leading space rule\n#-eoc-#\n56b2cbb\nfeat(cc): trim description on parse\n#-eoc-#\n";

        let expected = vec![
            Commit {
                sha: "ee0b330".to_string(),
                message: "feat: test commit\nbody\nFooter: value\n".to_string(),
            },
            Commit {
                sha: "36c13b5".to_string(),
                message: "feat(linter): remove description leading space rule\n".to_string(),
            },
            Commit {
                sha: "56b2cbb".to_string(),
                message: "feat(cc): trim description on parse\n".to_string(),
            },
        ];

        let (rest, result) = parse_git_log(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_git_log_with_issue_number() {
        let input = "ee0b330\nfeat: test commit\nbody\nIssue-ID #12\n#-eoc-#\n36c13b5\nfeat(linter): remove description leading space rule\n#-eoc-#\n56b2cbb\nfeat(cc): trim description on parse\n#-eoc-#\n";

        let expected = vec![
            Commit {
                sha: "ee0b330".to_string(),
                message: "feat: test commit\nbody\nIssue-ID #12\n".to_string(),
            },
            Commit {
                sha: "36c13b5".to_string(),
                message: "feat(linter): remove description leading space rule\n".to_string(),
            },
            Commit {
                sha: "56b2cbb".to_string(),
                message: "feat(cc): trim description on parse\n".to_string(),
            },
        ];

        let (rest, result) = parse_git_log(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }
}
