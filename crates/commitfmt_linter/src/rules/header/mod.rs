mod description;
mod scope;
mod kind;

mod max_length;
mod min_length;
mod settings;
mod breaking_exclamation;

#[allow(unused)]
pub(crate) use {
    description::{
        description_leading_space, DescriptionLeadingSpace,
        description_full_stop, DescriptionFullStop,
        description_max_length, DescriptionMaxLength,
        description_min_length, DescriptionMinLength,
        description_case, DescriptionCase
    },
    scope::{
        scope_case, ScopeCase,
        scope_enum, ScopeEnum,
        scope_max_length, ScopeMaxLength,
        scope_min_length, ScopeMinLength,
        scope_required, ScopeRequired
    },
    kind::{
        type_case, TypeCase,
        type_enum, TypeEnum,
        type_max_length, TypeMaxLength,
        type_min_length, TypeMinLength,
        type_required, TypeRequired
    },
    max_length::{max_length, MaxLength},
    min_length::{min_length, MinLength},
    breaking_exclamation::{breaking_exclamation, BreakingExclamation},
};

pub use settings::Settings;
