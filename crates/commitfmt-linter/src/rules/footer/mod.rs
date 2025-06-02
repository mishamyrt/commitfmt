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
    breaking_exclamation::{breaking_exclamation, BreakingExclamation},
    exists::{exists, Exists},
    key_case::{key_case, KeyCase},
    max_length::{max_length, MaxLength},
    max_line_length::{max_line_length, MaxLineLength},
    min_length::{min_length, MinLength},
    settings::Settings,
};
