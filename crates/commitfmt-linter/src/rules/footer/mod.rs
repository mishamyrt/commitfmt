mod breaking_exclamation;
mod exists;
mod key_case;
mod max_length;
mod max_line_length;
mod min_length;

mod settings;

// TODO: add case check
#[allow(unused)]
pub(crate) use {
    breaking_exclamation::{BreakingExclamation, breaking_exclamation},
    exists::{Exists, exists},
    key_case::{KeyCase, key_case},
    max_length::{MaxLength, max_length},
    max_line_length::{MaxLineLength, max_line_length},
    min_length::{MinLength, min_length},
    settings::Settings,
};
