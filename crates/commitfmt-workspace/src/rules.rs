use commitfmt_linter::{
    case::{IdentifierCase, TextCase},
    rules::{Rule, Settings},
};
use toml::Value;

use crate::{Error, Result};

/// Parse a rule setting from a TOML value
///
/// Returns `true` if the setting is enabled, `false` if the setting is disabled.
pub(crate) fn parse_rule_setting(
    rule: Rule,
    settings: &mut Settings,
    value: &Value,
) -> Result<bool> {
    let reader = RuleSettingsReader::new(rule, value);

    match rule {
        Rule::HeaderDescriptionMaxLength => {
            reader.usize(&mut settings.header.description_max_length)
        }
        Rule::HeaderDescriptionMinLength => {
            reader.usize(&mut settings.header.description_min_length)
        }
        Rule::HeaderDescriptionCase => reader.text_case(&mut settings.header.description_case),
        Rule::HeaderScopeEnum => reader.str_vec(&mut settings.header.scope_enum),
        Rule::HeaderScopeCase => reader.id_case(&mut settings.header.scope_case),
        Rule::HeaderMaxLength => reader.usize(&mut settings.header.max_length),
        Rule::HeaderMinLength => reader.usize(&mut settings.header.min_length),
        Rule::HeaderScopeMaxLength => reader.usize(&mut settings.header.scope_max_length),
        Rule::HeaderScopeMinLength => reader.usize(&mut settings.header.scope_min_length),
        Rule::HeaderTypeCase => reader.id_case(&mut settings.header.type_case),

        Rule::HeaderTypeMaxLength => reader.usize(&mut settings.header.type_max_length),
        Rule::HeaderTypeMinLength => reader.usize(&mut settings.header.type_min_length),
        Rule::HeaderTypeEnum => reader.str_vec(&mut settings.header.type_enum),

        Rule::BodyMaxLineLength => reader.usize(&mut settings.body.max_line_length),
        Rule::BodyMaxLength => reader.usize(&mut settings.body.max_length),
        Rule::BodyMinLength => reader.usize(&mut settings.body.min_length),
        Rule::BodyCase => reader.text_case(&mut settings.body.case),

        Rule::FooterMaxLength => reader.usize(&mut settings.footer.max_length),
        Rule::FooterMinLength => reader.usize(&mut settings.footer.min_length),
        Rule::FooterMaxLineLength => reader.usize(&mut settings.footer.max_line_length),
        Rule::FooterKeyCase => reader.id_case(&mut settings.footer.key_case),
        Rule::FooterExists => reader.str_vec(&mut settings.footer.required),

        _ => match value.as_bool() {
            Some(is_enabled) => Ok(is_enabled),
            None => Err(Error::UnexpectedFieldType(
                rule.as_display().to_owned(),
                "bool".to_owned(),
            )),
        },
    }
}

struct RuleSettingsReader<'a> {
    rule: Rule,
    value: &'a Value,
}

impl<'a> RuleSettingsReader<'a> {
    fn new(rule: Rule, value: &'a Value) -> Self {
        Self { rule, value }
    }

    fn id_case(&self, target: &mut IdentifierCase) -> Result<bool> {
        let Some(case_str) = self.value.as_str() else {
            return Err(Error::UnexpectedFieldType(
                self.rule.as_display().to_string(),
                "string".to_string(),
            ));
        };

        let Some(case) = IdentifierCase::from_name(case_str) else {
            return Err(Error::InvalidWordCase(case_str.to_string()));
        };

        *target = case;
        Ok(true)
    }

    fn text_case(&self, target: &mut TextCase) -> Result<bool> {
        let Some(case_str) = self.value.as_str() else {
            return Err(Error::UnexpectedFieldType(
                self.rule.as_display().to_string(),
                "string".to_string(),
            ));
        };

        let Some(case) = TextCase::from_name(case_str) else {
            return Err(Error::InvalidTextCase(case_str.to_string()));
        };

        *target = case;
        Ok(true)
    }

    fn usize(&self, target: &mut usize) -> Result<bool> {
        let Some(parsed) = self.value.as_integer() else {
            return Err(Error::UnexpectedFieldType(
                self.rule.as_display().to_string(),
                "integer".to_string(),
            ));
        };

        let parsed = match usize::try_from(parsed) {
            Ok(parsed) => parsed,
            Err(err) => return Err(Error::ParseError(err.to_string())),
        };

        if parsed == 0 {
            return Ok(false);
        }

        *target = parsed;
        Ok(true)
    }

    fn str_vec(&self, target: &mut Vec<Box<str>>) -> Result<bool> {
        let Some(parsed) = self.value.as_array() else {
            return Err(Error::UnexpectedFieldType(
                self.rule.as_display().to_string(),
                "array".to_string(),
            ));
        };

        let mut result: Vec<Box<str>> = Vec::new();

        for item in parsed {
            let Some(value) = item.as_str() else {
                return Err(Error::UnexpectedValueType("string".to_string()));
            };
            result.push(Box::from(value));
        }

        *target = result;
        Ok(true)
    }
}
