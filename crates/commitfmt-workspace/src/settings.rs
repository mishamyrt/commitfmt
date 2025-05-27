use std::{cell::RefCell, path::Path};

use commitfmt_linter::{
    rule_set::RuleSet,
    rules::{self, Rule},
};
use toml::Table;

use crate::{
    config::{AdditionalFooterConfig, CommitParams},
    rules::parse_rule_setting,
    WorkspaceError, WorkspaceResult,
};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum OnConflictAction {
    #[default]
    Skip,
    Append,
    Error,
}

impl OnConflictAction {
    pub fn from_config(s: &str) -> WorkspaceResult<Self> {
        match s {
            "skip" => Ok(OnConflictAction::Skip),
            "append" => Ok(OnConflictAction::Append),
            "error" => Ok(OnConflictAction::Error),
            _ => Err(WorkspaceError::UnknownOnConflictAction(s.to_string())),
        }
    }
}

/// Additional footer information
#[derive(Debug, Clone)]
pub struct AdditionalFooter {
    pub key: String,
    pub value_template: Option<String>,
    pub branch_value_pattern: Option<String>,
    pub on_conflict: OnConflictAction,
    // TODO: add custom separator
}

impl AdditionalFooter {
    pub(crate) fn from_config(config: AdditionalFooterConfig) -> Self {
        let on_conflict = {
            match config.on_conflict {
                Some(on_conflict) => OnConflictAction::from_config(on_conflict.as_str())
                    .unwrap_or(OnConflictAction::Skip),
                None => OnConflictAction::Skip,
            }
        };

        Self {
            key: config.key,
            value_template: config.template,
            branch_value_pattern: config.branch_pattern,
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
pub struct RulesSettings {
    pub set: RuleSet,
    pub settings: rules::Settings,
}

impl RulesSettings {
    fn parse_group(
        &mut self,
        linter: rules::LinterGroup,
        linter_table: &Table,
    ) -> WorkspaceResult<(RuleSet, RuleSet)> {
        let mut enabled_rules = RuleSet::empty();
        let mut disabled_rules = RuleSet::empty();

        for key in linter_table.keys() {
            let Some(rule) = Rule::from_name(linter, key) else {
                return Err(WorkspaceError::UnknownRule(linter, key.to_owned()));
            };

            if parse_rule_setting(rule, &mut self.settings, &linter_table[key])? {
                enabled_rules.insert(rule);
            } else {
                disabled_rules.insert(rule);
            }
        }

        Ok((enabled_rules, disabled_rules))
    }

    pub(crate) fn from_params(params: &CommitParams) -> WorkspaceResult<Self> {
        let mut settings = Self::default();

        if params.lint_values.is_empty() {
            return Ok(settings);
        }

        for linter in rules::LinterGroup::iter() {
            let Some(table_value) = params.lint_values.get(linter.as_display()) else {
                continue;
            };
            let Some(linter_table) = table_value.as_table() else {
                return Err(WorkspaceError::UnexpectedFieldType(
                    linter.as_display().to_string(),
                    "table".to_string(),
                ));
            };

            let (enabled_rules, disabled_rules) =
                settings.parse_group(linter, linter_table)?;

            settings.set = settings.set.subtract(disabled_rules);
            settings.set = settings.set.union(enabled_rules);
        }

        Ok(settings)
    }
}

/// Parsed formatting settings
#[derive(Debug, PartialEq, Default)]
pub struct LintSettings {
    pub unsafe_fixes: bool,
}

impl LintSettings {
    pub(crate) fn from_params(params: &CommitParams) -> Self {
        let lint_ref = params.config.lint.clone();
        let lint_config = lint_ref.unwrap_or_default();

        Self { unsafe_fixes: lint_config.unsafe_fixes.unwrap_or(false) }
    }
}

/// Parsed commit settings
#[derive(Debug, PartialEq, Default)]
pub struct CommitSettings {
    pub rules: RulesSettings,
    pub lint: LintSettings,
    pub footers: RefCell<Vec<AdditionalFooter>>,
}

impl CommitSettings {
    pub(crate) fn from_params(params: CommitParams) -> WorkspaceResult<Self> {
        let lint = LintSettings::from_params(&params);
        let rules = RulesSettings::from_params(&params)?;

        let footers: Vec<AdditionalFooter> = params
            .config
            .footers
            .unwrap_or_default()
            .into_iter()
            .map(AdditionalFooter::from_config)
            .collect();

        Ok(Self { rules, lint, footers: RefCell::new(footers) })
    }
}

pub fn open_settings(dir_path: &Path) -> WorkspaceResult<CommitSettings> {
    let Ok(config_path) = CommitParams::find_config_path(dir_path) else {
        return Ok(CommitSettings::default());
    };

    let params = CommitParams::open(&config_path)?;
    CommitSettings::from_params(params)
}

#[cfg(test)]
mod tests {
    use toml::map::Map;

    use crate::config::{CommitConfig, LintConfig};

    use super::*;

    #[test]
    fn test_on_conflict_from_config() {
        assert_eq!(OnConflictAction::from_config("skip").unwrap(), OnConflictAction::Skip);
        assert_eq!(OnConflictAction::from_config("append").unwrap(), OnConflictAction::Append);
        assert_eq!(OnConflictAction::from_config("error").unwrap(), OnConflictAction::Error);
        assert!(OnConflictAction::from_config("unknown").is_err());
    }

    #[test]
    fn test_additional_footer_from_config() {
        let config = AdditionalFooterConfig {
            key: "Footer".to_string(),
            on_conflict: Some("error".to_string()),
            template: Some("{{ echo $USER }}".to_string()),
            branch_pattern: None,
        };

        let footer = AdditionalFooter::from_config(config);
        assert_eq!(footer.on_conflict, OnConflictAction::Error);
        assert_eq!(footer.key, "Footer");
        assert_eq!(footer.value_template, Some("{{ echo $USER }}".to_string()));
        assert_eq!(footer.branch_value_pattern, None);
    }

    #[test]
    fn test_rules_settings_from_params() {
        let params = CommitParams {
            config: CommitConfig {
                extends: None,
                lint: Some(LintConfig { unsafe_fixes: Some(true) }),
                footers: Some(vec![AdditionalFooterConfig {
                    key: "Footer".to_string(),
                    on_conflict: Some("error".to_string()),
                    template: Some("{{ echo $USER }}".to_string()),
                    branch_pattern: None,
                }]),
            },
            lint_values: Map::new(),
        };

        let settings = RulesSettings::from_params(&params).unwrap();
        assert_eq!(settings.set, RuleSet::default());
        assert_eq!(settings.settings, rules::Settings::default());
    }
}
