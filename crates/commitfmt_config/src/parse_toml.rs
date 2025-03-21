use commitfmt_linter::case::{TextCase, WordCase};
use toml::{map::Map, Table, Value};

use commitfmt_linter::rule_set::RuleSet;
use commitfmt_linter::rules::{LinterGroup, Rule, Settings};

use crate::config::FormattingConfig;
use crate::settings::CommitParams;
use crate::ConfigError;

/// Parse the rule configuration for the given linter
/// and return the enabled and disabled rules
trait TomlParser {
    fn parse_rule(&mut self, rule: Rule, value: &Value) -> Result<bool, ConfigError>;

    fn parse(
        &mut self,
        linter: LinterGroup,
        config: &Map<String, Value>,
    ) -> Result<(RuleSet, RuleSet), ConfigError>;
}

impl TomlParser for Settings {
    /// Parse the rule configuration for the given rule.
    /// Returns `true` if the rule should be enabled,
    /// `false` if the rule should be disabled.
    ///
    /// There is the place where we should handle custom settings for rules.
    fn parse_rule(&mut self, rule: Rule, value: &Value) -> Result<bool, ConfigError> {
        match rule {
            Rule::HeaderDescriptionMaxLength => {
                require_usize(value, &mut self.header.description_max_length)
            }
            Rule::HeaderDescriptionMinLength => {
                require_usize(value, &mut self.header.description_min_length)
            }
            Rule::HeaderDescriptionCase => {
                require_text_case(value, &mut self.header.description_case)
            }
            Rule::HeaderScopeEnum => require_str_vec(value, &mut self.header.scope_enum),
            Rule::HeaderScopeCase => require_word_case(value, &mut self.header.scope_case),
            Rule::HeaderMaxLength => require_usize(value, &mut self.header.max_length),
            Rule::HeaderMinLength => require_usize(value, &mut self.header.min_length),
            Rule::HeaderScopeMaxLength => {
                require_usize(value, &mut self.header.scope_max_length)
            }
            Rule::HeaderScopeMinLength => {
                require_usize(value, &mut self.header.scope_min_length)
            }
            Rule::HeaderTypeCase => require_word_case(value, &mut self.header.type_case),

            Rule::HeaderTypeMaxLength => {
                require_usize(value, &mut self.header.type_max_length)
            }
            Rule::HeaderTypeMinLength => {
                require_usize(value, &mut self.header.type_min_length)
            }
            Rule::HeaderTypeEnum => require_str_vec(value, &mut self.header.type_enum),

            Rule::BodyMaxLineLength => require_usize(value, &mut self.body.max_line_length),
            Rule::BodyMaxLength => require_usize(value, &mut self.body.max_length),
            Rule::BodyMinLength => require_usize(value, &mut self.body.min_length),
            Rule::BodyCase => require_text_case(value, &mut self.body.case),

            Rule::FooterMaxLength => require_usize(value, &mut self.footer.max_length),
            Rule::FooterMinLength => require_usize(value, &mut self.footer.min_length),
            Rule::FooterMaxLineLength => {
                require_usize(value, &mut self.footer.max_line_length)
            }
            Rule::FooterExists => require_str_vec(value, &mut self.footer.required),

            _ => match value.as_bool() {
                Some(is_enabled) => Ok(is_enabled),
                None => Err(ConfigError::UnexpectedFieldType(
                    rule.as_display().to_owned(),
                    "bool".to_owned(),
                )),
            },
        }
    }

    /// Parse the rule configuration for the given linter
    /// and return the enabled and disabled rules
    fn parse(
        &mut self,
        linter: LinterGroup,
        config: &Map<String, Value>,
    ) -> Result<(RuleSet, RuleSet), ConfigError> {
        let Some(linter_config) = config.get(linter.as_display()) else {
            return Ok((RuleSet::empty(), RuleSet::empty()));
        };
        let Some(linter_table) = linter_config.as_table() else {
            return Err(ConfigError::UnexpectedFieldType(
                linter.as_display().to_owned(),
                "table".to_owned(),
            ));
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

fn require_text_case(value: &Value, target: &mut TextCase) -> Result<bool, ConfigError> {
    let Some(parsed) = value.as_str() else {
        return Err(ConfigError::UnexpectedFieldType(
            "case".to_string(),
            "string".to_string(),
        ));
    };

    let Some(parsed) = TextCase::from_name(parsed) else {
        return Err(ConfigError::ParseError("Invalid text case".to_string()));
    };

    *target = parsed;
    Ok(true)
}

fn require_str_vec(value: &Value, target: &mut Vec<Box<str>>) -> Result<bool, ConfigError> {
    let Some(parsed) = value.as_array() else {
        return Err(ConfigError::UnexpectedFieldType(
            "case".to_string(),
            "string".to_string(),
        ));
    };

    let mut result: Vec<Box<str>> = Vec::new();

    for item in parsed {
        let Some(value) = item.as_str() else {
            return Err(ConfigError::UnexpectedValueType("string".to_string()));
        };
        result.push(Box::from(value));
    }

    *target = result;
    Ok(true)
}

fn require_word_case(value: &Value, target: &mut WordCase) -> Result<bool, ConfigError> {
    let Some(parsed) = value.as_str() else {
        return Err(ConfigError::UnexpectedFieldType(
            "case".to_string(),
            "string".to_string(),
        ));
    };

    let Some(parsed) = WordCase::from_name(parsed) else {
        return Err(ConfigError::ParseError("Invalid word case".to_string()));
    };

    *target = parsed;
    Ok(true)
}

fn require_usize(value: &Value, target: &mut usize) -> Result<bool, ConfigError> {
    let Some(parsed) = value.as_integer() else {
        return Err(ConfigError::UnexpectedFieldType(
            "max-line-length".to_string(),
            "integer".to_string(),
        ));
    };
    if parsed < 0 {
        return Err(ConfigError::ParseError(
            "Max line length must be greater or equal to 0".to_string(),
        ));
    }

    let parsed = match usize::try_from(parsed) {
        Ok(parsed) => parsed,
        Err(err) => return Err(ConfigError::ParseError(err.to_string())),
    };

    if parsed == 0 {
        return Ok(false);
    }

    *target = parsed;
    Ok(true)
}

pub(crate) fn parse_toml(data: &str) -> Result<CommitParams, ConfigError> {
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

    let config: FormattingConfig =
        toml::from_str(data).map_err(|err| ConfigError::TomlError(err.to_string()))?;

    Ok(CommitParams { formatting: config.to_settings(), rules, settings })
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
description-full-stop = true";
        let config = parse_toml(config).unwrap();
        assert!(config.settings.body.max_line_length == 80);
        assert!(config.rules.contains(Rule::BodyMaxLineLength));
        assert!(config.rules.contains(Rule::HeaderDescriptionFullStop));
        assert!(!config.formatting.unsafe_fixes);
    }

    #[test]
    fn test_parse_toml_with_disabled() {
        let config = "
[body]
max-line-length = 80

[header]
description-full-stop = false";
        let config = parse_toml(config).unwrap();
        assert!(config.rules.contains(Rule::BodyMaxLineLength));
        assert!(!config.rules.contains(Rule::HeaderDescriptionFullStop));
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

    #[test]
    fn test_parse_toml_with_text_case() {
        let config = "
[body]
case = \"upper\"

[formatting]
unsafe-fixes = true";
        let config = parse_toml(config).unwrap();
        assert!(config.rules.contains(Rule::BodyCase));
        assert!(config.settings.body.case == TextCase::Upper);
    }
}
