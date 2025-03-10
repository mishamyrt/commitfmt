use std::cell::RefCell;

use commitfmt_cc::message::Message;

use crate::report::Report;
use crate::rule_set::RuleSet;
use crate::rules::{body, header, Rule, Settings};
use crate::violation::Violation;

pub struct Check {
    pub(crate) report: Report,
    pub(crate) settings: Settings,
    pub(crate) rules: RuleSet,
}

impl Check {
    pub fn new(settings: Settings, rules: RuleSet) -> Self {
        Self {
            report: Report::default(),
            settings,
            rules,
        }
    }

    fn lint_header_type(&self, message: &Message) {
        if self.rules.contains(Rule::HeaderTypeCase) {
            header::type_case(&self.report, message, self.settings.header.type_case);
        }
        if self.rules.contains(Rule::HeaderTypeMaxLength) {
            header::type_max_length(&self.report, message, self.settings.header.type_max_length);
        }
        if self.rules.contains(Rule::HeaderTypeMinLength) {
            header::type_min_length(&self.report, message, self.settings.header.type_min_length);
        }
        if self.rules.contains(Rule::HeaderTypeEnum) {
            header::type_enum(&self.report, message, &self.settings.header.type_enum);
        }
        if self.rules.contains(Rule::HeaderTypeRequired) {
            header::type_required(&self.report, message);
        }
    }

    fn lint_header_scope(&self, message: &Message) {
        if self.rules.contains(Rule::HeaderScopeCase) {
            header::scope_case(&self.report, message, self.settings.header.scope_case);
        }
        if self.rules.contains(Rule::HeaderScopeMaxLength) {
            header::scope_max_length(&self.report, message, self.settings.header.scope_max_length);
        }
        if self.rules.contains(Rule::HeaderScopeMinLength) {
            header::scope_min_length(&self.report, message, self.settings.header.scope_min_length);
        }
        if self.rules.contains(Rule::HeaderScopeEnum) {
            header::scope_enum(&self.report, message, &self.settings.header.scope_enum);
        }
        if self.rules.contains(Rule::HeaderScopeRequired) {
            header::scope_required(&self.report, message);
        }
    }

    fn lint_header_description(&self, message: &Message) {
        if self.rules.contains(Rule::HeaderDescriptionLeadingSpace) {
            header::description_leading_space(&self.report, message);
        }
        if self.rules.contains(Rule::HeaderDescriptionFullStop) {
            header::description_full_stop(&self.report, message);
        }
        if self.rules.contains(Rule::HeaderDescriptionCase) {
            header::description_case(&self.report, message, self.settings.header.description_case);
        }
        if self.rules.contains(Rule::HeaderDescriptionMaxLength) {
            header::description_max_length(&self.report, message, self.settings.header.description_max_length);
        }
        if self.rules.contains(Rule::HeaderDescriptionMinLength) {
            header::description_min_length(&self.report, message, self.settings.header.description_min_length);
        }
    }

    fn lint_header(&self, message: &Message) {
        self.lint_header_type(message);
        self.lint_header_scope(message);
        self.lint_header_description(message);

        // Common rules
        if self.rules.contains(Rule::HeaderMaxLength) {
            header::max_length(&self.report, message, self.settings.header.max_length);
        }
        if self.rules.contains(Rule::HeaderMinLength) {
            header::min_length(&self.report, message, self.settings.header.min_length);
        }
        if self.rules.contains(Rule::HeaderBreakingExclamation) {
            header::breaking_exclamation(&self.report, message);
        }
    }

    fn lint_body(&self, message: &Message) {
        if self.rules.contains(Rule::BodyLeadingNewLine) {
            body::leading_nl(&self.report, message);
        }
        if self.rules.contains(Rule::BodyMaxLineLength) {
            body::max_line_length(&self.report, message, self.settings.body.max_line_length);
        }
        if self.rules.contains(Rule::BodyMaxLength) {
            body::max_length(&self.report, message, self.settings.body.max_length);
        }
        if self.rules.contains(Rule::BodyMinLength) {
            body::min_length(&self.report, message, self.settings.body.min_length);
        }
        if self.rules.contains(Rule::BodyFullStop) {
            body::full_stop(&self.report, message);
        }
        if self.rules.contains(Rule::BodyCase) {
            body::case(&self.report, message, self.settings.body.case);
        }
    }

    pub fn run(&self, message: &Message) {
        self.lint_header(message);

        if message.body.is_none() {
            return;
        }

        self.lint_body(message);
    }

    pub fn violations_ref(&self) -> &RefCell<Vec<Box<dyn Violation>>> {
        &self.report.violations
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::message::Message;

    use crate::{check::Check, rule_set::RuleSet, rules};

    #[test]
    fn test_check() {
        let settings = rules::Settings::default();
        let rules = RuleSet::from_rules(&[
            rules::Rule::BodyLeadingNewLine,
        ]);

        let check = Check::new(settings, rules);
        let message = Message::parse("feat: my feature\nbody").expect("Unable to parse commit message");
        check.run(&message);
        assert_eq!(check.report.violations.borrow().len(), 1);
    }

    #[test]
    fn test_empty_check() {
        let settings = rules::Settings::default();
        let rules = RuleSet::from_rules(&[
            rules::Rule::BodyLeadingNewLine,
        ]);

        let check = Check::new(settings, rules);
        let message = Message::parse("feat: my feature\n\nbody").expect("Unable to parse commit message");
        check.run(&message);
        assert_eq!(check.report.violations.borrow().len(), 0);
    }
}
