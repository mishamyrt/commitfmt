use crate::rules::Rule;

/// Default set of rules
const DEFAULT_RULES: RuleSet = RuleSet::from_rules(&[
    Rule::BodyMaxLineLength,
    Rule::BodyLeadingNewLine,
    Rule::HeaderLeadingSpace,
]);

/// Rule Set implements a set of rules using bit sets in a u64.
/// Each bit corresponds to a rule.
/// For now it has a maximum of 64 rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleSet(pub u64);

// TODO: Implement buckets, when we have more than 64 rules

impl RuleSet {
    const EMPTY: u64 = 0;

    /// Returns an empty rule set.
    #[inline]
    pub const fn empty() -> Self {
        Self(Self::EMPTY)
    }

    /// Returns default set of rules
    #[inline]
    pub const fn default() -> Self {
        DEFAULT_RULES
    }

    /// Returns a rule set containing the provided rules
    #[inline]
    pub const fn from_rules(rules: &[Rule]) -> Self {
        let mut set = RuleSet::empty();

        // Uses a while because for loops are not allowed in const functions.
        let mut i = 0;
        while i < rules.len() {
            set = set.add(rules[i]);
            i += 1;
        }

        set
    }

    /// Returns the union of the two rule sets `self` and `other`
    #[must_use]
    pub const fn union(&mut self, other: Self) -> Self {
        self.0 |= other.0;

        *self
    }

    /// Returns `self` without any of the rules contained in `other`.
    #[must_use]
    pub const fn subtract(&mut self, other: Self) -> Self {
        self.0 &= !other.0;

        *self
    }

    /// Inserts `rule` into the set.
    #[inline]
    pub fn insert(&mut self, rule: Rule) {
        *self = self.add(rule);
    }

    /// Returns the new set with `rule` added
    #[must_use]
    pub const fn add(&self, rule: Rule) -> Self {
        Self(self.0 | (1 << (rule as u64)))
    }

    /// Removes `rule` from the set.
    #[must_use]
    pub const fn remove(&self, rule: Rule) -> Self {
        Self(self.0 & !(1 << (rule as u64)))
    }

    /// Returns the number of rules in this set.
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns `true` if this set is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if `rule` is in this set.
    #[inline]
    pub const fn contains(&self, rule: Rule) -> bool {
        self.0 & (1 << (rule as u64)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty_set = RuleSet::empty();
        assert_eq!(empty_set.0, 0);
        assert_eq!(empty_set.len(), 0);
        assert!(empty_set.is_empty());
    }

    #[test]
    fn test_add_and_contains() {
        let mut set = RuleSet::empty();
        set.insert(Rule::BodyLeadingNewLine);
        assert!(set.contains(Rule::BodyLeadingNewLine));
        assert!(!set.contains(Rule::BodyMaxLineLength));
        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());

        set.insert(Rule::BodyMaxLineLength);
        assert!(set.contains(Rule::BodyMaxLineLength));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut set = RuleSet::empty();
        set.insert(Rule::BodyLeadingNewLine);
        set.insert(Rule::BodyMaxLineLength);
        assert_eq!(set.len(), 2);

        set = set.remove(Rule::BodyLeadingNewLine);
        assert!(!set.contains(Rule::BodyLeadingNewLine));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_union() {
        let mut set1 = RuleSet::empty();
        set1.insert(Rule::BodyLeadingNewLine);

        let mut set2 = RuleSet::empty();
        set2.insert(Rule::BodyMaxLineLength);

        let union_set = set1.union(set2);
        assert!(union_set.contains(Rule::BodyLeadingNewLine));
        assert!(union_set.contains(Rule::BodyMaxLineLength));
        assert_eq!(union_set.len(), 2);
    }

    #[test]
    fn test_subtract() {
        let mut set1 = RuleSet::empty();
        set1.insert(Rule::BodyLeadingNewLine);
        set1.insert(Rule::BodyMaxLineLength);

        let mut set2 = RuleSet::empty();
        set2.insert(Rule::BodyMaxLineLength);

        let subtract_set = set1.subtract(set2);
        assert!(subtract_set.contains(Rule::BodyLeadingNewLine));
        assert!(!subtract_set.contains(Rule::BodyMaxLineLength));
        assert_eq!(subtract_set.len(), 1);
    }

    #[test]
    fn test_from_rules() {
        let rules = [Rule::BodyLeadingNewLine, Rule::HeaderLeadingSpace];
        let set = RuleSet::from_rules(&rules);
        assert!(set.contains(Rule::BodyLeadingNewLine));
        assert!(set.contains(Rule::HeaderLeadingSpace));
        assert_eq!(set.len(), 2);
    }
}
