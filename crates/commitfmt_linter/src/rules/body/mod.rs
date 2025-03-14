mod case;
mod full_stop;
mod leading_newline;
mod max_length;
mod max_line_length;
mod min_length;

mod settings;

#[allow(unused)]
pub(crate) use {
    settings::Settings,
    case::{Case,case},
    full_stop::{FullStop,full_stop},
    leading_newline::{LeadingNewLine,leading_nl},
    max_length::{MaxLength,max_length},
    max_line_length::{MaxLineLength,max_line_length},
    min_length::{MinLength,min_length},
};

