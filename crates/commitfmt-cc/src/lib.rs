pub mod footer;
mod header;
mod message;

pub(crate) mod body;

pub use {
    footer::{Footer, SeparatorAlignment},
    header::{Header, Scope},
    message::{Message, ParseError},
};

#[inline]
pub(crate) fn char_count(value: &str) -> usize {
    value.chars().count()
}
