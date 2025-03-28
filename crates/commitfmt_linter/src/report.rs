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
