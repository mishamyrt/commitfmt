use serde_derive::Deserialize;

use crate::{
    params::{AdditionalFooter, OnConflictAction},
    ConfigError,
};

#[derive(Debug, PartialEq, Deserialize, Default, Clone)]
pub(crate) struct AdditionalFooterConfig {
    pub key: String,

    #[serde(alias = "on-conflict")]
    pub on_conflict: Option<String>,
    #[serde(alias = "value-template")]
    pub template: Option<String>,
    #[serde(alias = "branch-value-pattern")]
    pub branch_pattern: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
pub(crate) struct FormattingSettingsConfig {
    #[serde(alias = "unsafe-fixes")]
    pub unsafe_fixes: Option<bool>,

    #[serde(alias = "additional-footers")]
    pub footers: Option<Vec<AdditionalFooterConfig>>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
pub(crate) struct LintConfig {
    #[serde(alias = "unsafe-fixes")]
    pub unsafe_fixes: Option<bool>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
pub(crate) struct CommitConfig {
    pub lint: Option<LintConfig>,

    #[serde(alias = "additional-footers")]
    pub footers: Option<Vec<AdditionalFooterConfig>>,
}

impl AdditionalFooterConfig {
    pub(crate) fn to_settings(&self) -> Result<AdditionalFooter, ConfigError> {
        let mut value_template: Option<&String> = None;
        if let Some(format) = &self.template {
            value_template = Some(format);
        }

        let mut branch_pattern: Option<&String> = None;
        if let Some(pattern) = &self.branch_pattern {
            branch_pattern = Some(pattern);
        }

        if branch_pattern.is_none() && value_template.is_none() {
            return Err(ConfigError::FooterValueNotFound(self.key.clone()));
        }

        let mut on_conflict = OnConflictAction::default();
        if let Some(on_conflict_str) = &self.on_conflict {
            match OnConflictAction::from_config(on_conflict_str) {
                Some(action) => on_conflict = action,
                None => {
                    return Err(ConfigError::InvalidOnConflictAction(
                        self.key.clone(),
                        on_conflict_str.clone(),
                    ))
                }
            }
        }

        if let Some(value_template) = value_template {
            return Ok(AdditionalFooter::with_template(
                self.key.clone(),
                value_template,
                on_conflict,
            ));
        } else if let Some(branch_pattern) = branch_pattern {
            return Ok(AdditionalFooter::with_branch_pattern(
                self.key.clone(),
                branch_pattern,
                on_conflict,
            ));
        }

        unreachable!();
    }
}
