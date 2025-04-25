use commitfmt_cc::{Footer, Message, SeparatorAlignment};
use commitfmt_config::{params::OnConflictAction, AdditionalFooter};
use colored::Colorize;
// use commitfmt_config::{params::AdditionalFooterKind, AdditionalFooter};
use commitfmt_git::Repository;
use regex::Regex;
use thiserror::Error;

use crate::print_debug;

#[derive(Debug, Error)]
pub enum FooterError {
    #[error("Footer template cannot be rendered: {0}")]
    TemplateError(#[from] commitfmt_tpl_exec::TplError),

    #[error("Footer '{0}' is missing a value")]
    MissingValue(String),

    #[error("Unable to parse regex: {0}")]
    BadPattern(#[from] regex::Error),

    #[error("Unable to get value from branch: {0}")]
    ValueNotFoundInBranch(String),

    #[error("Footer with key '{0} already exists")]
    AlreadyExists(String),
}

pub(crate) fn append_footers(
    message: &mut Message,
    footers: &Vec<AdditionalFooter>,
    repo: &Repository,
) -> Result<(), FooterError> {
    let mut branch: Option<String> = None;
    for footer in footers {
        if footer.branch_pattern.is_none() && footer.value_template.is_none() {
            return Err(FooterError::MissingValue(footer.key.clone()));
        }
        // TODO: add `skip-branches` rule
        if message.footers.contains_key(&footer.key) {
            match footer.on_conflict {
                OnConflictAction::Replace => {
                    // TODO: add handling for duplicated keys
                    message.footers.remove(&footer.key);
                }
                OnConflictAction::Append => {
                    // Do nothing, the footer will be added later
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
            match commitfmt_tpl_exec::render(template) {
                Ok(value) => value,
                Err(err) => {
                    return Err(FooterError::TemplateError(err));
                }
            }
        } else if let Some(pattern) = footer.branch_pattern.as_ref() {
            if branch.as_ref().is_none() {
                branch = repo.get_branch_name();
            }
            // TODO: avoid extra calls
            let Some(branch) = &branch else {
                print_debug!("Branch is missing. HEAD is detached");
                continue
            };

            let re = Regex::new(pattern).map_err(FooterError::BadPattern)?;

            match re.captures(&branch) {
                Some(captures) => {
                    if captures.len() != 1  {
                        return Err(FooterError::ValueNotFoundInBranch(branch.clone()));
                    }

                    captures[1].to_string()
                }
                None => {
                    print_debug!("Branch {} does not match pattern {}", branch, pattern);
                    continue
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

pub(crate) struct FooterAdder<'f>(pub &'f mut Vec<Footer>);

impl<'f> FooterAdder<'f> {}
