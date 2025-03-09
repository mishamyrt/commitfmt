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
    pub fn add_violation(&self, rule: Box<dyn Violation>) {
        self.violations.borrow_mut().push(rule);
    }
}
