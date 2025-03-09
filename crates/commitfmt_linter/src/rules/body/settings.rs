use crate::case::TextCase;

#[derive(Debug, PartialEq, Default)]
pub struct Settings {
    pub max_line_length: usize,
    pub max_length: usize,
    pub min_length: usize,
    pub case: TextCase,
}
