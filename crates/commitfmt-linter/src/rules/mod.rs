pub mod linter_group;
mod names;

pub mod body;
pub mod footer;
pub mod header;

pub use linter_group::LinterGroup;
pub use names::Rule;

#[inline]
pub(crate) fn longer_than_chars(value: &str, length: usize) -> bool {
    value.len() > length && value.chars().nth(length).is_some()
}

#[inline]
pub(crate) fn shorter_than_chars(value: &str, length: usize) -> bool {
    length > 0 && (value.len() < length || value.chars().nth(length - 1).is_none())
}

#[derive(Default, Debug, PartialEq)]
pub struct Settings {
    pub body: body::Settings,
    pub header: header::Settings,
    pub footer: footer::Settings,
}

#[cfg(test)]
mod tests {
    use super::{longer_than_chars, shorter_than_chars};

    #[test]
    fn test_unicode_length_comparison() {
        assert!(!longer_than_chars("café", 4));
        assert!(longer_than_chars("café", 3));
        assert!(!shorter_than_chars("café", 4));
        assert!(shorter_than_chars("café", 5));

        assert!(!longer_than_chars("café\n界", 6));
        assert!(longer_than_chars("café\n界", 5));
        assert!(!shorter_than_chars("café\n界", 6));
        assert!(shorter_than_chars("café\n界", 7));
    }
}
