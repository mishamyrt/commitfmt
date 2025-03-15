use crate::case::{TextCase, WordCase};

#[derive(Debug, PartialEq, Default)]
pub struct Settings {
    pub max_line_length: usize,
    pub max_length: usize,
    pub min_length: usize,
    pub title_case: WordCase,
    pub value_case: TextCase,
    pub required: Vec<Box<str>>,
}
