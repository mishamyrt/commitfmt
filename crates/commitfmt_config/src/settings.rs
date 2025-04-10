use std::cell::RefCell;

use serde_derive::Deserialize;

use commitfmt_linter::{rule_set::RuleSet, rules};

/// Additional footer information
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct AdditionalFooter {
    source: String,
    key: String,
    value: String,
}

/// Parsed formatting settings
#[derive(Debug, PartialEq)]
pub struct FormattingSettings {
    pub unsafe_fixes: bool,

    pub footers: RefCell<Vec<AdditionalFooter>>,
}

/// Parsed commit settings
#[derive(Debug, PartialEq)]
pub struct CommitParams {
    pub rules: RuleSet,
    pub settings: rules::Settings,
    pub formatting: FormattingSettings,
}

impl Default for CommitParams {
    fn default() -> Self {
        Self {
            rules: RuleSet::default(),
            settings: rules::Settings::default(),
            formatting: FormattingSettings {
                unsafe_fixes: false,
                footers: RefCell::new(vec![]),
            },
        }
    }
}
