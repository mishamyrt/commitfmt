// use crate::case::{TextCase, WordCase};
use crate::case::IdentifierCase;

#[derive(Debug, PartialEq, Default)]
pub struct Settings {
    pub max_line_length: usize,
    pub max_length: usize,
    pub min_length: usize,
    pub key_case: IdentifierCase,
    // pub title_case: WordCase,
    // pub value_case: TextCase,
    pub required: Vec<Box<str>>,
}
