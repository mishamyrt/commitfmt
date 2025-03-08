pub struct Settings {
    pub(crate) max_line_length: usize,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            max_line_length: 0,
        }
    }
}
