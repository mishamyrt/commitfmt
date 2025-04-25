use std::cell::RefCell;

use commitfmt_linter::{rule_set::RuleSet, rules};

#[derive(Debug, Copy, Clone, Default)]
pub enum OnConflictAction {
    #[default]
    Skip,
    Replace,
    Append,
    Error,
}

impl OnConflictAction {
    pub fn from_str(s: &str) -> Option<OnConflictAction> {
        match s {
            "skip" => Some(OnConflictAction::Skip),
            "replace" => Some(OnConflictAction::Replace),
            "append" => Some(OnConflictAction::Append),
            "error" => Some(OnConflictAction::Error),
            _ => None,
        }
    }
}

/// Additional footer information
#[derive(Debug, Clone)]
pub struct AdditionalFooter {
    pub key: String,
    pub value_template: Option<String>,
    pub value_pattern: Option<String>,
    pub branch_pattern: Option<String>,
    pub on_conflict: OnConflictAction,
    // TODO: add custom separator
}

impl AdditionalFooter {
    pub fn with_template(
        key: String,
        value_template: &String,
        on_conflict: OnConflictAction,
    ) -> Self {
        Self {
            key,
            value_template: Some(value_template.to_string()),
            value_pattern: None,
            branch_pattern: None,
            on_conflict,
        }
    }

    pub fn with_branch_pattern(
        key: String,
        branch_pattern: &str,
        on_conflict: OnConflictAction,
    ) -> Self {
        Self {
            key,
            value_template: None,
            value_pattern: None,
            branch_pattern: Some(branch_pattern.to_string()),
            on_conflict,
        }
    }
}

impl PartialEq for AdditionalFooter {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value_template == other.value_template
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct RulesParams {
    pub set: RuleSet,
    pub settings: rules::Settings,
}

/// Parsed formatting settings
#[derive(Debug, PartialEq, Default)]
pub struct LintParams {
    pub unsafe_fixes: bool,
}

/// Parsed commit settings
#[derive(Debug, PartialEq, Default)]
pub struct CommitParams {
    pub rules: RulesParams,
    pub lint: LintParams,
    pub footers: RefCell<Vec<AdditionalFooter>>,
}
