use std::fmt;

use crate::rules::Rule;

/// Default set of rules
const DEFAULT_RULES: RuleSet =
    RuleSet::from_rules(&[Rule::HeaderDescriptionFullStop, Rule::FooterBreakingExclamation]);

/// Rule Set implements a set of rules using bit sets in a u64.
/// Each bit corresponds to a rule.
/// For now it has a maximum of 64 rules.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RuleSet(pub u64);

/// Iterator over the rules in a `RuleSet`
pub struct RuleSetIter {
    bits: u64,
    current_bit: u8,
}

impl Iterator for RuleSetIter {
    type Item = Rule;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_bit < 64 {
            let bit_mask = 1u64 << self.current_bit;
            if self.bits & bit_mask != 0 {
                let rule = unsafe { std::mem::transmute::<u8, Rule>(self.current_bit) };
                self.current_bit += 1;
                return Some(rule);
            }
            self.current_bit += 1;
        }
        None
    }
}

impl IntoIterator for RuleSet {
    type Item = Rule;
    type IntoIter = RuleSetIter;

    fn into_iter(self) -> Self::IntoIter {
        RuleSetIter { bits: self.0, current_bit: 0 }
    }
}

impl IntoIterator for &RuleSet {
    type Item = Rule;
    type IntoIter = RuleSetIter;

    fn into_iter(self) -> Self::IntoIter {
        RuleSetIter { bits: self.0, current_bit: 0 }
    }
}

impl fmt::Display for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }

        let rules: Vec<String> =
            self.iter().map(|rule| rule.as_display().to_string()).collect();

        writeln!(f, "[")?;
        for (i, rule) in rules.iter().enumerate() {
            write!(f, "\t{rule}")?;
            if i < rules.len() - 1 {
                writeln!(f, ",")?;
            }
        }
        write!(f, "\n]")
    }
}

impl fmt::Debug for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

// TODO: Implement buckets, when we have more than 64 rules

impl Default for RuleSet {
    fn default() -> Self {
        DEFAULT_RULES
    }
}

impl RuleSet {
    const EMPTY: u64 = 0;

    /// Returns an empty rule set.
    #[inline]
    pub const fn empty() -> Self {
        Self(Self::EMPTY)
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

    /// Returns an iterator over the rules in this set.
    #[inline]
    pub fn iter(&self) -> RuleSetIter {
        self.into_iter()
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
        set.insert(Rule::HeaderScopeMinLength);
        assert!(set.contains(Rule::HeaderScopeMinLength));
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
        set.insert(Rule::HeaderScopeMinLength);
        set.insert(Rule::BodyMaxLineLength);
        assert_eq!(set.len(), 2);

        set = set.remove(Rule::HeaderScopeMinLength);
        assert!(!set.contains(Rule::HeaderScopeMinLength));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_union() {
        let mut set1 = RuleSet::empty();
        set1.insert(Rule::HeaderScopeMinLength);

        let mut set2 = RuleSet::empty();
        set2.insert(Rule::BodyMaxLineLength);

        let union_set = set1.union(set2);
        assert!(union_set.contains(Rule::HeaderScopeMinLength));
        assert!(union_set.contains(Rule::BodyMaxLineLength));
        assert_eq!(union_set.len(), 2);
    }

    #[test]
    fn test_subtract() {
        let mut set1 = RuleSet::empty();
        set1.insert(Rule::HeaderScopeMinLength);
        set1.insert(Rule::BodyMaxLineLength);

        let mut set2 = RuleSet::empty();
        set2.insert(Rule::BodyMaxLineLength);

        let subtract_set = set1.subtract(set2);
        assert!(subtract_set.contains(Rule::HeaderScopeMinLength));
        assert!(!subtract_set.contains(Rule::BodyMaxLineLength));
        assert_eq!(subtract_set.len(), 1);
    }

    #[test]
    fn test_from_rules() {
        let rules = [Rule::HeaderScopeMinLength, Rule::HeaderDescriptionFullStop];
        let set = RuleSet::from_rules(&rules);
        assert!(set.contains(Rule::HeaderScopeMinLength));
        assert!(set.contains(Rule::HeaderDescriptionFullStop));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_iterator() {
        let mut set = RuleSet::empty();
        set.insert(Rule::HeaderScopeMinLength);
        set.insert(Rule::HeaderDescriptionFullStop);

        let mut rules: Vec<Rule> = set.into_iter().collect();
        rules.sort();

        let mut expected = vec![Rule::HeaderScopeMinLength, Rule::HeaderDescriptionFullStop];
        expected.sort();

        assert_eq!(rules, expected);
    }

    #[test]
    fn test_iterator_empty() {
        let set = RuleSet::empty();
        let rules: Vec<Rule> = set.into_iter().collect();
        assert!(rules.is_empty());
    }

    #[test]
    fn test_iterator_reference() {
        let mut set = RuleSet::empty();
        set.insert(Rule::HeaderScopeMinLength);

        let rules: Vec<Rule> = (&set).into_iter().collect();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0], Rule::HeaderScopeMinLength);

        // Check that set can still be used
        assert!(set.contains(Rule::HeaderScopeMinLength));
    }

    #[test]
    fn test_iter_method() {
        let mut set = RuleSet::empty();
        set.insert(Rule::HeaderScopeMinLength);
        set.insert(Rule::HeaderDescriptionFullStop);

        let rules: Vec<Rule> = set.iter().collect();
        assert_eq!(rules.len(), 2);
        assert!(rules.contains(&Rule::HeaderScopeMinLength));
        assert!(rules.contains(&Rule::HeaderDescriptionFullStop));

        // Check that set can still be used after iter()
        assert!(set.contains(Rule::HeaderScopeMinLength));
    }

    #[test]
    fn test_display_empty() {
        let set = RuleSet::empty();
        assert_eq!(format!("{set}"), "[]");
    }

    #[test]
    fn test_display_single_rule() {
        let mut set = RuleSet::empty();
        set.insert(Rule::HeaderScopeMinLength);
        let expected = "[\n\tscope-min-length\n]";
        assert_eq!(format!("{set}"), expected);
    }

    #[test]
    fn test_display_multiple_rules() {
        let mut set = RuleSet::empty();
        set.insert(Rule::HeaderScopeMinLength);
        set.insert(Rule::HeaderDescriptionFullStop);

        let display = format!("{set}");
        assert!(display.starts_with("[\n"));
        assert!(display.ends_with("\n]"));
        assert!(display.contains("\tscope-min-length"));
        assert!(display.contains("\tdescription-full-stop"));
        assert!(display.contains(",\n"));
    }

    #[test]
    fn test_debug_format() {
        let mut set = RuleSet::empty();
        set.insert(Rule::HeaderScopeMinLength);

        let debug = format!("{set:?}");
        let display = format!("{set}");
        assert_eq!(debug, display);
    }
}
