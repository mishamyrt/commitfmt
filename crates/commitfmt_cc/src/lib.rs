pub mod footer;
mod header;
mod message;

pub(crate) mod body;

pub use {
    footer::{Footer, SeparatorAlignment},
    header::{Header, Scope},
    message::{Message,ParseError}
};
