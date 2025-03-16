mod description_case;
mod description_full_stop;
mod description_leading_space;
mod description_max_length;
mod description_min_length;

#[allow(unused)]
pub(crate) use {
    description_case::{description_case, DescriptionCase},
    description_full_stop::{description_full_stop, DescriptionFullStop},
    description_leading_space::{description_leading_space, DescriptionLeadingSpace},
    description_max_length::{description_max_length, DescriptionMaxLength},
    description_min_length::{description_min_length, DescriptionMinLength},
};
