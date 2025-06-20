use std::{cell::RefCell, path::Path};

use regex::Regex;
use toml::Table;

use commitfmt_cc::footer::SeparatorAlignment;
use commitfmt_linter::{rules, Rule, RuleSet};
use commitfmt_tpl::Template;

use crate::configuration::{AdditionalFooterConfig, CommitParams};
use crate::rules::parse_rule_setting;
use crate::{Error, Result};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum OnConflictAction {
    #[default]
    Skip,
    Append,
    Error,
}

impl OnConflictAction {
    pub fn from_config(s: &str) -> Result<Self> {
        match s {
            "skip" => Ok(OnConflictAction::Skip),
            "append" => Ok(OnConflictAction::Append),
            "error" => Ok(OnConflictAction::Error),
            _ => Err(Error::UnknownOnConflictAction(s.to_string())),
        }
    }
}

/// Additional footer information
#[derive(Debug, Clone)]
pub struct AdditionalFooter {
    pub key: String,
    pub value: Template,
    pub branch_pattern: Option<Regex>,
    pub on_conflict: OnConflictAction,
    pub separator: Option<char>,
    pub alignment: Option<SeparatorAlignment>,
}

impl AdditionalFooter {
    pub(crate) fn from_config(config: AdditionalFooterConfig) -> Result<Self> {
        let value = Template::parse(&config.value)?;
        let branch_pattern: Option<Regex> = match config.branch_pattern {
            Some(pattern) => Some(Regex::new(&pattern).map_err(Error::InvalidPattern)?),
            None => None,
        };

        let on_conflict = match config.on_conflict {
            Some(on_conflict) => OnConflictAction::from_config(on_conflict.as_str())?,
            None => OnConflictAction::Skip,
        };

        Ok(Self {
            key: config.key,
            value,
            branch_pattern,
            on_conflict,
            separator: config.separator,
            alignment: config.alignment,
        })
    }
}

impl PartialEq for AdditionalFooter {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.value == other.value
            && self.on_conflict == other.on_conflict
            && self.separator == other.separator
            && self.alignment == other.alignment
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
    ) -> Result<(RuleSet, RuleSet)> {
        let mut enabled_rules = RuleSet::empty();
        let mut disabled_rules = RuleSet::empty();

        for key in linter_table.keys() {
            let Some(rule) = Rule::from_name(linter, key) else {
                return Err(Error::UnknownRule(linter, key.to_owned()));
            };

            if parse_rule_setting(rule, &mut self.settings, &linter_table[key])? {
                enabled_rules.insert(rule);
            } else {
                disabled_rules.insert(rule);
            }
        }

        Ok((enabled_rules, disabled_rules))
    }

    pub(crate) fn from_params(params: &CommitParams) -> Result<Self> {
        let mut settings = Self::default();

        if params.lint_values.is_empty() {
            return Ok(settings);
        }

        for linter in rules::LinterGroup::iter() {
            let Some(table_value) = params.lint_values.get(linter.as_display()) else {
                continue;
            };
            let Some(linter_table) = table_value.as_table() else {
                return Err(Error::UnexpectedFieldType(
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
    pub footer_separators: Option<String>,
    pub comment_symbol: Option<String>,
}

impl CommitSettings {
    pub fn from_toml(data: &str) -> Result<Self> {
        let params = CommitParams::parse_toml(data)?;
        Self::from_params(params)
    }

    pub(crate) fn from_params(params: CommitParams) -> Result<Self> {
        let lint = LintSettings::from_params(&params);
        let rules = RulesSettings::from_params(&params)?;

        let footer_configs = params.config.additional_footers.unwrap_or_default();

        let mut footers: Vec<AdditionalFooter> = Vec::with_capacity(footer_configs.len());

        for config in footer_configs {
            let footer = AdditionalFooter::from_config(config)?;
            footers.push(footer);
        }

        Ok(Self {
            rules,
            lint,
            footers: RefCell::new(footers),
            footer_separators: params.config.footer_separators,
            comment_symbol: params.config.comment_symbol,
        })
    }
}

pub fn open_settings(dir_path: &Path) -> Result<CommitSettings> {
    let Ok(config_path) = CommitParams::find_config_path(dir_path) else {
        return Ok(CommitSettings::default());
    };

    let params = CommitParams::open(&config_path)?;
    CommitSettings::from_params(params)
}

#[cfg(test)]
mod tests {
    use toml::map::Map;

    use crate::configuration::{CommitConfiguration, LintConfiguration};
    use commitfmt_tpl::Segment;

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
            value: "{{ echo $USER }}".to_string(),
            branch_pattern: None,
            separator: None,
            alignment: None,
        };

        let footer = AdditionalFooter::from_config(config).unwrap();
        assert_eq!(footer.on_conflict, OnConflictAction::Error);
        assert_eq!(footer.key, "Footer");
        assert_eq!(footer.value.segments_len(), 1);
        assert_eq!(
            footer.value.segments_iter().next().unwrap(),
            &Segment::Command("echo $USER".to_string())
        );
    }

    #[test]
    fn test_rules_settings_from_params() {
        let params = CommitParams {
            config: CommitConfiguration {
                extends: None,
                lint: Some(LintConfiguration { unsafe_fixes: Some(true) }),
                footer_separators: None,
                comment_symbol: None,
                additional_footers: Some(vec![AdditionalFooterConfig {
                    key: "Footer".to_string(),
                    on_conflict: Some("error".to_string()),
                    value: "{{ echo $USER }}".to_string(),
                    branch_pattern: None,
                    separator: None,
                    alignment: None,
                }]),
            },
            lint_values: Map::new(),
        };

        let settings = RulesSettings::from_params(&params).unwrap();
        assert_eq!(settings.set, RuleSet::default());
        assert_eq!(settings.settings, rules::Settings::default());
    }
}
