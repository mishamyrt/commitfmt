use std::cell::RefCell;

use crate::violation::Violation;

pub struct Report {
    pub violations: RefCell<Vec<Box<dyn Violation>>>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            violations: RefCell::new(Vec::new()),
        }
    }

    pub fn add_violation(&self, rule: Box<dyn Violation>) {
        self.violations.borrow_mut().push(rule);
    }
}
