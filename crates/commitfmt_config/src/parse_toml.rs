use toml::{map::Map, Table, Value};

use commitfmt_linter::rule_set::RuleSet;
use commitfmt_linter::rules::{LinterGroup, Rule, Settings};

use crate::config::FormattingConfig;
use crate::settings::CommitSettings;
use crate::ConfigError;

/// Parse the rule configuration for the given linter
/// and return the enabled and disabled rules
trait TomlParser {
    fn parse_rule(&mut self, rule: Rule, value: &Value) -> Result<bool, ConfigError>;

    fn parse(&mut self, linter: LinterGroup, config: &Map<String, Value>) -> Result<(RuleSet, RuleSet), ConfigError>;
}

impl TomlParser for Settings {
    /// Parse the rule configuration for the given rule.
    /// Returns `true` if the rule should be enabled,
    /// `false` if the rule should be disabled.
    ///
    /// There is the place where we should handle settings for rules.
    fn parse_rule(&mut self, rule: Rule, value: &Value) -> Result<bool, ConfigError> {
        match rule {
            Rule::BodyMaxLineLength => {
                let parsed_value = require_usize(value)?;
                if parsed_value == 0 {
                    return Ok(false);
                }

                self.body.max_line_length = parsed_value;
                Ok(true)
            }
            _ => match value.as_bool() {
                Some(is_enabled) => Ok(is_enabled),
                None => Err(ConfigError::UnexpectedFieldType(rule.as_display().to_owned(), "bool".to_owned())),
            },
        }
    }

    /// Parse the rule configuration for the given linter
    /// and return the enabled and disabled rules
    fn parse(&mut self, linter: LinterGroup, config: &Map<String, Value>) -> Result<(RuleSet, RuleSet), ConfigError> {
        let Some(linter_config) = config.get(linter.as_display()) else {
            return Ok((RuleSet::empty(), RuleSet::empty()));
        };
        let Some(linter_table) = linter_config.as_table() else {
            return Err(ConfigError::UnexpectedFieldType(linter.as_display().to_owned(), "table".to_owned()));
        };

        let mut enabled_rules = RuleSet::empty();
        let mut disabled_rules = RuleSet::empty();

        for key in linter_table.keys() {
            let Some(rule) = Rule::from_name(linter, key) else {
                return Err(ConfigError::UnknownRule(linter, key.to_owned()));
            };

            if self.parse_rule(rule, &linter_table[key])? {
                enabled_rules.insert(rule);
            } else {
                disabled_rules.insert(rule);
            }
        }

        Ok((enabled_rules, disabled_rules))
    }
}

fn require_usize(value: &Value) -> Result<usize, ConfigError> {
    let Some(parsed) = value.as_integer() else {
        return Err(ConfigError::UnexpectedFieldType("max-line-length".to_string(), "integer".to_string()));
    };
    if parsed < 0 {
        return Err(ConfigError::ParseError("Max line length must be greater or equal to 0".to_string()));
    }

    match usize::try_from(parsed) {
        Ok(parsed) => Ok(parsed),
        Err(err) => Err(ConfigError::ParseError(err.to_string())),
    }
}

pub(crate) fn parse_toml(data: &str) -> Result<CommitSettings, ConfigError> {
    let Ok(config_map) = data.parse::<Table>() else {
        return Err(ConfigError::TomlError("Unable to parse TOML".to_string()));
    };

    let mut settings = Settings::default();
    let mut rules = RuleSet::default();

    for linter in LinterGroup::iter() {
        let (enabled_rules, disabled_rules) = settings.parse(linter, &config_map)?;

        rules = rules.subtract(disabled_rules);
        rules = rules.union(enabled_rules);
    }

    let config: FormattingConfig = toml::from_str(data).map_err(|err| ConfigError::TomlError(err.to_string()))?;

    Ok(CommitSettings {
        formatting: config.to_settings(),
        rules,
        settings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toml() {
        let config = "
[body]
max-line-length = 80

[header]
description-leading-space = true";
        let config = parse_toml(config).unwrap();
        assert!(config.settings.body.max_line_length == 80);
        assert!(config.rules.contains(Rule::BodyMaxLineLength));
        assert!(config.rules.contains(Rule::HeaderLeadingSpace));
        assert!(!config.formatting.unsafe_fixes);
    }

    #[test]
    fn test_parse_toml_with_disabled() {
        let config = "
[body]
max-line-length = 80

[header]
description-leading-space = false";
        let config = parse_toml(config).unwrap();
        assert!(config.rules.contains(Rule::BodyMaxLineLength));
        assert!(!config.rules.contains(Rule::HeaderLeadingSpace));
    }

    #[test]
    fn test_parse_toml_with_format() {
        let config = "
[body]
max-line-length = 80

[formatting]
unsafe-fixes = true";
        let config = parse_toml(config).unwrap();
        assert!(config.rules.contains(Rule::BodyMaxLineLength));
        assert!(config.formatting.unsafe_fixes);
    }
}
