use std::collections::HashMap;
use std::path::Path;

use colored::Colorize;

use commitfmt_cc::{Footer, Message};
use commitfmt_git::Repository;
use commitfmt_linter::{Check, FixMode, Rule, Violation};
use commitfmt_workspace::{open_settings, AdditionalFooter, CommitSettings, OnConflictAction};

use crate::ignore::is_ignored_message;
use crate::logging::pluralize;
use crate::{print_error, print_info};
use crate::{CommitRange, Error, Result};

/// Commitfmt application.
pub struct Commitfmt {
    pub repo: Repository,
    pub settings: CommitSettings,
}

impl Commitfmt {
    /// Creates a new Commitfmt application with workspace from the given path.
    pub fn from_path(path: &Path) -> Result<Self> {
        let repo = Repository::open(path)?;
        let mut settings = open_settings(path)?;
        if settings.comment_symbol.is_none() {
            settings.comment_symbol = repo.comment_symbol();
        }
        if settings.footer_separators.is_none() {
            settings.footer_separators = repo.trailer_separators();
        }

        Ok(Self { repo, settings })
    }

    /// Lint a commit range (from..to).
    pub fn lint_commit_range(&self, range: CommitRange) -> Result<()> {
        let (from, to) = range;
        let commits = self.repo.get_log(from, to)?;

        let mut problems_count: usize = 0;
        let mut check = Check::new(&self.settings.rules.settings, self.settings.rules.set);

        for commit in &commits {
            if is_ignored_message(&commit.message) {
                // Skip ignored commits.
                continue;
            }
            let message = Message::parse(
                &commit.message,
                self.settings.footer_separators.as_deref(),
                self.settings.comment_symbol.as_deref(),
            )?;

            check.lint(&message);
            if !check.report.violations.is_empty() {
                let count = check.report.violations.len();
                let sha = &commit.sha;
                if count == 1 {
                    print_error!("Commit {sha} has violation");
                } else {
                    print_error!("Commit {sha} has {count} violations");
                }

                let _ = report_violations(check.report.violations.iter());
                problems_count += count;
                check.report.violations.clear();
            }
        }

        if problems_count > 0 {
            return Err(Error::Lint(problems_count));
        }

        let commit_pluralized = pluralize(commits.len(), "commit", "commits");
        print_info!("No problems found in {} {}", commits.len(), commit_pluralized);
        Ok(())
    }

    /// Formats a commit message.
    pub fn format_commit_message(&self, input: &str, lint_only: bool) -> Result<String> {
        let mut message = Message::parse(
            input,
            self.settings.footer_separators.as_deref(),
            self.settings.comment_symbol.as_deref(),
        )?;

        let mut check = Check::new(&self.settings.rules.settings, self.settings.rules.set);
        check.lint(&message);

        if lint_only {
            if check.report.violations.is_empty() {
                return Ok(message.to_string());
            }
            let count = report_violations(check.report.violations.iter());
            return Err(Error::Lint(count));
        }

        let mut unfixable_count: usize = 0;
        let message_ptr = &mut message;
        for violation_box in &check.report.violations {
            let violation = violation_box.as_ref();
            match violation.fix_mode() {
                FixMode::Unsafe => {
                    if self.settings.lint.unsafe_fixes {
                        violation.fix(message_ptr).expect("Failed to fix violation");
                    } else {
                        print_violation(violation, true);
                        unfixable_count += 1;
                    }
                }
                FixMode::Safe => {
                    violation.fix(message_ptr).expect("Failed to fix violation");
                }
                FixMode::Unfixable => {
                    print_violation(violation, false);
                    unfixable_count += 1;
                }
            }
        }

        if unfixable_count > 0 {
            return Err(Error::Unfixable(unfixable_count));
        }

        let Some(branch) = self.repo.get_branch_name() else {
            return Ok(message.to_string());
        };

        append_footers(&self.settings.footers.borrow(), &mut message, &branch)?;

        Ok(message.to_string())
    }
}

/// Reports all violations to the logger and returns the total count.
///
/// This function iterates through the provided violations, logs each one
/// with its message and rule identifier, and keeps track of the total count.
fn report_violations<'a>(violations: impl Iterator<Item = &'a Box<dyn Violation>>) -> usize {
    let mut count: usize = 0;
    for violation_box in violations {
        count += 1;
        print_violation(violation_box.as_ref(), false);
    }

    count
}

/// Prints a single violation to the logger
fn print_violation(violation: &dyn Violation, fix_available: bool) {
    let Some(rule) = Rule::from_violation(violation) else {
        panic!("Failed to get rule from violation");
    };
    let rule_name = format!("[{}]", rule.as_display());
    let line = if fix_available {
        format!(
            "- {} {} {}",
            violation.message(),
            rule_name.dimmed(),
            "(unsafe fix available)".bright_yellow()
        )
    } else {
        format!("- {} {}", violation.message(), rule_name.dimmed())
    };
    print_info!("{line}");
}

/// Appends footers to the message
///
/// This function iterates through the provided footers, renders their values with the given branch,
/// and appends them to the message.
fn append_footers(
    footers: &[AdditionalFooter],
    message: &mut Message,
    branch: &str,
) -> Result<()> {
    for footer in footers {
        if message.footers.contains_key(&footer.key) {
            match footer.on_conflict {
                OnConflictAction::Append => {
                    // Do nothing there, the footer will be added later
                }
                OnConflictAction::Skip => {
                    continue;
                }
                OnConflictAction::Error => {
                    return Err(Error::AlreadyExists(footer.key.clone()));
                }
            }
        }

        let mut variables = HashMap::new();
        if let Some(branch_pattern) = &footer.branch_pattern {
            let Some(caps) = branch_pattern.captures(branch) else {
                continue;
            };
            for (i, name) in branch_pattern.capture_names().enumerate() {
                if let Some(name) = name {
                    variables.insert(name.to_string(), caps[i].to_string());
                }
            }
        }

        let separator = footer.separator.unwrap_or(Footer::DEFAULT_SEPARATOR_CHAR);
        let alignment = footer.alignment.unwrap_or_default();

        let value = footer.value.render(&variables)?;
        message.footers.push(Footer { key: footer.key.clone(), value, separator, alignment });
    }
    Ok(())
}
