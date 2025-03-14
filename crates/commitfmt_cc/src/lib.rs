mod footer;
mod header;
mod message;

pub(crate) mod body;

pub use {
    footer::{Footer, FooterList},
    header::{Header, Scope},
    message::{Message,ParseError}
};
