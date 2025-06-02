use commitfmt_cc::Message;

use crate::report::Report;
use crate::rule_set::RuleSet;
use crate::rules::{body, footer, header, Rule, Settings};

pub struct Check<'a> {
    pub report: Report,

    settings: &'a Settings,
    rules: RuleSet,
}

impl<'a> Check<'a> {
    pub fn new(settings: &'a Settings, rules: RuleSet) -> Self {
        Self { report: Report::default(), settings, rules }
    }

    fn lint_header_type(&mut self, message: &Message) {
        if self.rules.contains(Rule::HeaderTypeCase) {
            header::type_case(&mut self.report, message, self.settings.header.type_case);
        }
        if self.rules.contains(Rule::HeaderTypeMaxLength) {
            header::type_max_length(
                &mut self.report,
                message,
                self.settings.header.type_max_length,
            );
        }
        if self.rules.contains(Rule::HeaderTypeMinLength) {
            header::type_min_length(
                &mut self.report,
                message,
                self.settings.header.type_min_length,
            );
        }
        if self.rules.contains(Rule::HeaderTypeEnum) {
            header::type_enum(&mut self.report, message, &self.settings.header.type_enum);
        }
        if self.rules.contains(Rule::HeaderTypeRequired) {
            header::type_required(&mut self.report, message);
        }
    }

    fn lint_header_scope(&mut self, message: &Message) {
        if self.rules.contains(Rule::HeaderScopeCase) {
            header::scope_case(&mut self.report, message, self.settings.header.scope_case);
        }
        if self.rules.contains(Rule::HeaderScopeMaxLength) {
            header::scope_max_length(
                &mut self.report,
                message,
                self.settings.header.scope_max_length,
            );
        }
        if self.rules.contains(Rule::HeaderScopeMinLength) {
            header::scope_min_length(
                &mut self.report,
                message,
                self.settings.header.scope_min_length,
            );
        }
        if self.rules.contains(Rule::HeaderScopeEnum) {
            header::scope_enum(&mut self.report, message, &self.settings.header.scope_enum);
        }
        if self.rules.contains(Rule::HeaderScopeRequired) {
            header::scope_required(&mut self.report, message);
        }
    }

    fn lint_header_description(&mut self, message: &Message) {
        if self.rules.contains(Rule::HeaderDescriptionFullStop) {
            header::description_full_stop(&mut self.report, message);
        }
        if self.rules.contains(Rule::HeaderDescriptionCase) {
            header::description_case(
                &mut self.report,
                message,
                self.settings.header.description_case,
            );
        }
        if self.rules.contains(Rule::HeaderDescriptionMaxLength) {
            header::description_max_length(
                &mut self.report,
                message,
                self.settings.header.description_max_length,
            );
        }
        if self.rules.contains(Rule::HeaderDescriptionMinLength) {
            header::description_min_length(
                &mut self.report,
                message,
                self.settings.header.description_min_length,
            );
        }
    }

    fn lint_header(&mut self, message: &Message) {
        self.lint_header_type(message);
        self.lint_header_scope(message);
        self.lint_header_description(message);

        // Common rules
        if self.rules.contains(Rule::HeaderMaxLength) {
            header::max_length(&mut self.report, message, self.settings.header.max_length);
        }
        if self.rules.contains(Rule::HeaderMinLength) {
            header::min_length(&mut self.report, message, self.settings.header.min_length);
        }
    }

    fn lint_body(&mut self, message: &Message) {
        if self.rules.contains(Rule::BodyMaxLineLength) {
            body::max_line_length(
                &mut self.report,
                message,
                self.settings.body.max_line_length,
            );
        }
        if self.rules.contains(Rule::BodyMaxLength) {
            body::max_length(&mut self.report, message, self.settings.body.max_length);
        }
        if self.rules.contains(Rule::BodyMinLength) {
            body::min_length(&mut self.report, message, self.settings.body.min_length);
        }
        if self.rules.contains(Rule::BodyFullStop) {
            body::full_stop(&mut self.report, message);
        }
        if self.rules.contains(Rule::BodyCase) {
            body::case(&mut self.report, message, self.settings.body.case);
        }
    }

    fn lint_footers(&mut self, message: &Message) {
        if self.rules.contains(Rule::FooterMaxLength) {
            footer::max_length(&mut self.report, message, self.settings.footer.max_length);
        }
        if self.rules.contains(Rule::FooterBreakingExclamation) {
            footer::breaking_exclamation(&mut self.report, message);
        }
        if self.rules.contains(Rule::FooterMaxLineLength) {
            footer::max_line_length(
                &mut self.report,
                message,
                self.settings.footer.max_line_length,
            );
        }
        if self.rules.contains(Rule::FooterMinLength) {
            footer::min_length(&mut self.report, message, self.settings.footer.min_length);
        }
    }

    pub fn lint(&mut self, message: &Message) {
        self.lint_header(message);

        if message.body.is_some() {
            self.lint_body(message);
        }

        // First check required.
        // It will fail on empty footers.
        if self.rules.contains(Rule::FooterExists) {
            footer::exists(&mut self.report, message, &self.settings.footer.required);
        }
        if !message.footers.is_empty() {
            self.lint_footers(message);
        }
    }
}

impl std::fmt::Display for Check<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for violation in &self.report.violations {
            writeln!(f, "- {violation}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use commitfmt_cc::Message;

    use crate::{check::Check, rule_set::RuleSet, rules};

    #[test]
    fn test_check() {
        let mut settings = rules::Settings::default();
        settings.header.scope_min_length = 1;
        let rules = RuleSet::from_rules(&[rules::Rule::HeaderScopeMinLength]);

        let mut check = Check::new(&settings, rules);
        let message = Message::parse("feat: my feature\nbody", None, None)
            .expect("Unable to parse commit message");
        check.lint(&message);
        assert_eq!(check.report.violations.len(), 1);
    }

    // #[test]
    // fn test_empty_check() {
    //     let settings = rules::Settings::default();
    //     let rules = RuleSet::from_rules(&[rules::Rule::BodyLeadingNewLine]);

    //     let mut check = Check::new(&settings, rules);
    //     let message = Message::parse("feat: my feature\n\nbody", None, None)
    //         .expect("Unable to parse commit message");
    //     check.lint(&message);
    //     assert_eq!(check.report.violations.len(), 0);
    // }
}
