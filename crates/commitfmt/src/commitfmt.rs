use std::path::Path;

use colored::Colorize;

use commitfmt_cc::Message;
use commitfmt_format::append_footers;
use commitfmt_git::Repository;
use commitfmt_linter::{Check, FixMode, Rule, Violation};
use commitfmt_workspace::{open_settings, CommitSettings};

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

        if let Some(branch) = self.repo.get_branch_name() {
            append_footers(&mut message, &self.settings.footers.borrow(), &branch)?;
        }

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
