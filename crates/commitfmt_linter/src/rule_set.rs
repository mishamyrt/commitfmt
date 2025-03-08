use crate::rules::Rule;

pub struct RuleSet (pub u64);

/// Rule Set implements a set of rules using bit sets in a u64.
impl RuleSet {
    const EMPTY: u64 = 0;

    /// Returns an empty rule set.
    pub const fn empty() -> Self {
        Self(Self::EMPTY)
    }

    /// Returns the union of the two rule sets `self` and `other`
    #[inline]
    pub const fn from_rules(rules: &[Rule]) -> Self {
        let mut set = RuleSet::empty();

        let mut i = 0;

        while i < rules.len() {
            set = set.add(rules[i]);
            i += 1;
        }

        set
    }

    pub fn insert(&mut self, rule: Rule) {
        *self = self.add(rule);
    }

    pub const fn add(&self, rule: Rule) -> Self {
        Self(self.0 | (1 << (rule as u64)))
    }

    pub(crate) fn enabled(&self, rule: Rule) -> bool {
        self.0 & (1 << (rule as u64)) != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::rule_set::RuleSet;
    use crate::rules::Rule;

    #[test]
    fn test_rule_set() {
        let rule_set = RuleSet::from_rules(&[Rule::BodyMaxLineLength]);
        assert!(!rule_set.enabled(Rule::BodyLeadingNewLine));
        assert!(rule_set.enabled(Rule::BodyMaxLineLength));
    }
}
