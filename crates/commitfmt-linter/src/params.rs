use crate::{rules, RuleSet};

#[derive(Debug, Default, PartialEq)]
pub struct Params {
    pub settings: rules::Settings,
    pub rules: RuleSet,
}
