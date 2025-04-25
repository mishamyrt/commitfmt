use commitfmt_cc::{Footer, Message, SeparatorAlignment};
use commitfmt_config::{params::OnConflictAction, AdditionalFooter};
use regex::Regex;
use thiserror::Error;

use crate::template::{render_template, TemplateError};

#[derive(Debug, Error)]
pub enum FooterError {
    #[error("Footer template cannot be rendered: {0}")]
    TemplateError(#[from] TemplateError),

    #[error("Footer '{0}' is missing a value")]
    MissingValue(String),

    #[error("Unable to parse regular expression: {0}")]
    BadPattern(#[from] regex::Error),

    #[error("Unable to get value from branch: {0}")]
    ValueNotFoundInBranch(String),

    #[error("Footer with key '{0} already exists")]
    AlreadyExists(String),
}

pub fn append_footers(
    message: &mut Message,
    footers: &Vec<AdditionalFooter>,
    branch: &str,
) -> Result<(), FooterError> {
    for footer in footers {
        if footer.branch_value_pattern.is_none() && footer.value_template.is_none() {
            return Err(FooterError::MissingValue(footer.key.clone()));
        }

        if message.footers.contains_key(&footer.key) {
            match footer.on_conflict {
                OnConflictAction::Append => {
                    // Do nothing there, the footer will be added later
                }
                OnConflictAction::Skip => {
                    continue;
                }
                OnConflictAction::Error => {
                    return Err(FooterError::AlreadyExists(footer.key.clone()));
                }
            }
        }

        let value = if let Some(template) = footer.value_template.as_ref() {
            match render_template(template) {
                Ok(value) => value,
                Err(err) => {
                    return Err(FooterError::TemplateError(err));
                }
            }
        } else if let Some(pattern) = footer.branch_value_pattern.as_ref() {
            let re = Regex::new(pattern).map_err(FooterError::BadPattern)?;

            match re.captures(branch) {
                Some(captures) => {
                    if captures.len() < 2 {
                        // Branch is not matched by the pattern, skip the footer
                        continue;
                    }

                    captures[1].to_string()
                }
                None => {
                    continue;
                }
            }
        } else {
            return Err(FooterError::MissingValue(footer.key.clone()));
        };

        message.footers.push(Footer {
            key: footer.key.clone(),
            // TODO: add support for separator and alignment from config
            separator: Footer::DEFAULT_SEPARATOR.chars().next().unwrap(),
            alignment: SeparatorAlignment::default(),
            value,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use commitfmt_cc::Header;

    fn create_test_message() -> Message {
        Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Default::default(),
                description: "test feature".to_string(),
                breaking: false,
            },
            body: None,
            footers: Default::default(),
        }
    }

    #[test]
    fn test_append_footers_with_value_template() {
        let mut message = create_test_message();
        let footers = vec![AdditionalFooter {
            key: "Signed-off-by".to_string(),
            value_template: Some("Test User <test@example.com>".to_string()),
            branch_value_pattern: None,
            on_conflict: OnConflictAction::Append,
        }];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 1);
        let footer = message.footers.get(0).unwrap();
        assert_eq!(footer.key, "Signed-off-by");
        assert_eq!(footer.value, "Test User <test@example.com>");
    }

    #[test]
    fn test_append_footers_with_value_pattern() {
        let mut message = create_test_message();
        let footers = vec![AdditionalFooter {
            key: "Task-ID".to_string(),
            value_template: None,
            branch_value_pattern: Some("feature/([A-Z]+-[0-9]+)".to_string()),
            on_conflict: OnConflictAction::Append,
        }];

        let result = append_footers(&mut message, &footers, "feature/ABC-123");
        if let Err(err) = &result {
            println!("Error: {:?}", err);
        }
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 1);
        let footer = message.footers.get(0).unwrap();
        assert_eq!(footer.key, "Task-ID");
        assert_eq!(footer.value, "ABC-123");
    }

    #[test]
    fn test_append_footers_on_conflict_skip() {
        let mut message = create_test_message();

        let footer = Footer {
            key: "Signed-off-by".to_string(),
            separator: ':',
            alignment: SeparatorAlignment::Left,
            value: "Existing User <existing@example.com>".to_string(),
        };
        message.footers.push(footer);

        let footers = vec![AdditionalFooter {
            key: "Signed-off-by".to_string(),
            value_template: Some("New User <new@example.com>".to_string()),
            branch_value_pattern: None,
            on_conflict: OnConflictAction::Skip,
        }];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 1);
        let footer = message.footers.get(0).unwrap();
        assert_eq!(footer.value, "Existing User <existing@example.com>");
    }

    #[test]
    fn test_append_footers_on_conflict_append() {
        let mut message = create_test_message();

        let footer = Footer {
            key: "Signed-off-by".to_string(),
            separator: ':',
            alignment: SeparatorAlignment::Left,
            value: "Existing User <existing@example.com>".to_string(),
        };
        message.footers.push(footer);

        let footers = vec![AdditionalFooter {
            key: "Signed-off-by".to_string(),
            value_template: Some("New User <new@example.com>".to_string()),
            branch_value_pattern: None,
            on_conflict: OnConflictAction::Append,
        }];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 2);
        let footer = message.footers.get(1).unwrap();
        assert_eq!(footer.value, "New User <new@example.com>");
    }

    #[test]
    fn test_append_footers_on_conflict_error() {
        let mut message = create_test_message();

        let footer = Footer {
            key: "Signed-off-by".to_string(),
            separator: ':',
            alignment: SeparatorAlignment::Left,
            value: "Existing User <existing@example.com>".to_string(),
        };
        message.footers.push(footer);

        let footers = vec![AdditionalFooter {
            key: "Signed-off-by".to_string(),
            value_template: Some("New User <new@example.com>".to_string()),
            branch_value_pattern: None,
            on_conflict: OnConflictAction::Error,
        }];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_err());
        match result {
            Err(FooterError::AlreadyExists(key)) => {
                assert_eq!(key, "Signed-off-by");
            }
            _ => panic!("Expected AlreadyExists error"),
        }
        assert_eq!(message.footers.len(), 1);
    }

    #[test]
    fn test_append_footers_missing_value() {
        let mut message = create_test_message();
        let footers = vec![AdditionalFooter {
            key: "Missing-Value".to_string(),
            value_template: None,
            branch_value_pattern: None,
            on_conflict: OnConflictAction::Append,
        }];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_err());
        match result {
            Err(FooterError::MissingValue(key)) => {
                assert_eq!(key, "Missing-Value");
            }
            _ => panic!("Expected MissingValue error"),
        }
    }

    #[test]
    fn test_append_footers_bad_pattern() {
        let mut message = create_test_message();
        let footers = vec![AdditionalFooter {
            key: "Bad-Pattern".to_string(),
            value_template: None,
            branch_value_pattern: Some("(unclosed pattern".to_string()),
            on_conflict: OnConflictAction::Append,
        }];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_err());
        match result {
            Err(FooterError::BadPattern(_)) => {}
            _ => panic!("Expected BadPattern error"),
        }
    }

    #[test]
    fn test_append_footers_value_not_found_in_branch() {
        let mut message = create_test_message();
        let footers = vec![AdditionalFooter {
            key: "Task-ID".to_string(),
            value_template: None,
            branch_value_pattern: Some("feature/([A-Z]+-[0-9]+)".to_string()),
            on_conflict: OnConflictAction::Append,
        }];

        let result = append_footers(&mut message, &footers, "hotfix/ABC-123");
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 0);
    }

    #[test]
    fn test_append_multiple_footers() {
        let mut message = create_test_message();
        let footers = vec![
            AdditionalFooter {
                key: "Signed-off-by".to_string(),
                value_template: Some("Test User <test@example.com>".to_string()),
                branch_value_pattern: None,
                on_conflict: OnConflictAction::Append,
            },
            AdditionalFooter {
                key: "Reviewed-by".to_string(),
                value_template: Some("Reviewer <reviewer@example.com>".to_string()),
                branch_value_pattern: None,
                on_conflict: OnConflictAction::Append,
            },
        ];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 2);

        let footer1 = message.footers.get(0).unwrap();
        assert_eq!(footer1.key, "Signed-off-by");
        assert_eq!(footer1.value, "Test User <test@example.com>");

        let footer2 = message.footers.get(1).unwrap();
        assert_eq!(footer2.key, "Reviewed-by");
        assert_eq!(footer2.value, "Reviewer <reviewer@example.com>");
    }

    #[test]
    fn test_branch_exclusion_pattern() {
        let mut message = create_test_message();

        let mut footers = vec![AdditionalFooter {
            key: "Test-Branch".to_string(),
            value_template: None,
            branch_value_pattern: Some("^[a-z]+/([A-Z]+-[0-9]+)/?(?:.*)?".to_string()),
            on_conflict: OnConflictAction::Append,
        }];

        let result = append_footers(&mut message, &footers, "main");
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 0, "Footer should not be added for branch main");

        message = create_test_message();
        let result = append_footers(&mut message, &footers, "dev");
        assert!(result.is_ok());
        assert_eq!(message.footers.len(), 0, "Footer should not be added for branch dev");

        message = create_test_message();
        let result = append_footers(&mut message, &footers, "release/1.0.0");
        assert!(result.is_ok());
        assert_eq!(
            message.footers.len(),
            0,
            "Footer should not be added for branches release/*"
        );

        message = create_test_message();

        let result = append_footers(&mut message, &footers, "feature/ABC-123");
        assert!(result.is_ok());
        assert_eq!(
            message.footers.len(),
            1,
            "Footer should be added for branch feature/ABC-123"
        );

        let footer = message.footers.get(0).unwrap();
        assert_eq!(footer.key, "Test-Branch");
        assert_eq!(footer.value, "ABC-123");

        footers.clear();

        let result = append_footers(&mut message, &footers, "feature/ABC-123/test");
        assert!(result.is_ok());
        assert_eq!(
            message.footers.len(),
            1,
            "Footer should be added for branch feature/ABC-123/test"
        );

        let footer = message.footers.get(0).unwrap();
        assert_eq!(footer.key, "Test-Branch");
        assert_eq!(footer.value, "ABC-123");
    }
}
