use crate::violation::Violation;

#[derive(Default)]
pub struct Report {
    pub violations: Vec<Box<dyn Violation>>,
}

impl Report {
    /// Adds a violation
    pub fn add_violation(&mut self, violation: Box<dyn Violation>) {
        self.violations.push(violation);
    }

    /// Returns the number of violations
    pub fn len(&self) -> usize {
        self.violations.len()
    }

    /// Returns true if there are no violations
    pub fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }

    /// Clears the violations
    pub fn clear(&mut self) {
        self.violations.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::violation::TestViolation;

    use super::*;

    #[test]
    fn test_report_add_violation() {
        let mut report = Report::default();
        assert_eq!(report.len(), 0);
        report.add_violation(Box::new(TestViolation));
        assert_eq!(report.len(), 1);
    }

    #[test]
    fn test_report_is_empty() {
        let mut report = Report::default();
        assert!(report.is_empty());
        report.add_violation(Box::new(TestViolation));
        assert!(!report.is_empty());
    }

    #[test]
    fn test_report_clear() {
        let mut report = Report::default();
        report.add_violation(Box::new(TestViolation));
        assert!(!report.is_empty());
        report.clear();
        assert!(report.is_empty());
    }
}
