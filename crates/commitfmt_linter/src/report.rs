use std::cell::RefCell;

use crate::violation::Violation;

pub struct Report {
    pub violations: RefCell<Vec<Box<dyn Violation>>>,
}

impl Default for Report {
    fn default() -> Self {
        Self {
            violations: RefCell::new(Vec::new()),
        }
    }
}

impl Report {
    /// Adds a violation
    pub fn add_violation(&self, violation: Box<dyn Violation>) {
        self.violations.borrow_mut().push(violation);
    }

    /// Returns the number of violations
    pub fn len(&self) -> usize {
        self.violations.borrow().len()
    }

    /// Returns true if there are no violations
    pub fn is_empty(&self) -> bool {
        self.violations.borrow().is_empty()
    }
}
