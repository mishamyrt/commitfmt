use commitfmt_cc::message::Message;

use crate::report::Report;
use crate::rule_set::RuleSet;
use crate::rules::{body, header, Rule, Settings};

pub struct Check {
    pub(crate) report: Report,
    pub(crate) settings: Settings,
    pub(crate) rules: RuleSet,
}

impl Check {
    pub fn new(settings: Settings, rules: RuleSet) -> Self {
        Self {
            report: Report::new(),
            settings,
            rules,
        }
    }

    fn lint_header(&self, message: &Message) {
        if self.rules.enabled(Rule::HeaderDescriptionLeadingSpace) {
            header::description_leading_space(&self.report, message);
        }
    }

    fn lint_body(&self, message: &Message) {
        if self.rules.enabled(Rule::BodyLeadingNewLine) {
            body::leading_nl(&self.report, message);
        }
        if self.rules.enabled(Rule::BodyMaxLineLength) {
            body::max_line_length(&self.report, message, self.settings.body.max_line_length);
        }
    }

    pub fn run(&self, message: &Message) {
        self.lint_header(message);

        if message.body.is_none() {
            return;
        }

        self.lint_body(message);
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
