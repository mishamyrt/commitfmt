use crate::case::{IdentifierCase, TextCase};

#[derive(Debug, PartialEq, Default)]
pub struct Settings {
    pub max_length: usize,
    pub min_length: usize,
    pub scope_max_length: usize,
    pub scope_min_length: usize,
    pub scope_case: IdentifierCase,
    pub scope_enum: Vec<Box<str>>,
    pub description_case: TextCase,
    pub description_max_length: usize,
    pub description_min_length: usize,
    pub type_case: IdentifierCase,
    pub type_max_length: usize,
    pub type_min_length: usize,
    pub type_enum: Vec<Box<str>>,
}
