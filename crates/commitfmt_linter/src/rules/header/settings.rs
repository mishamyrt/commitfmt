pub struct Settings {
    pub max_length: usize,
    pub min_length: usize,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            max_length: 0,
            min_length: 0,
        }
    }
}
