mod description;
mod kind;
mod scope;

mod breaking_exclamation;
mod max_length;
mod min_length;
mod settings;

#[allow(unused)]
pub(crate) use {
    settings::Settings,
    description::{
        description_case, DescriptionCase,
        description_full_stop, DescriptionFullStop,
        description_leading_space, DescriptionLeadingSpace,
        description_max_length, DescriptionMaxLength,
        description_min_length, DescriptionMinLength,
    },
    kind::{
        type_case, TypeCase,
        type_enum, TypeEnum,
        type_max_length, TypeMaxLength,
        type_min_length, TypeMinLength,
        type_required, TypeRequired
    },
    scope::{
        scope_case, ScopeCase,
        scope_enum, ScopeEnum,
        scope_max_length, ScopeMaxLength,
        scope_min_length, ScopeMinLength,
        scope_required, ScopeRequired
    },
    breaking_exclamation::{breaking_exclamation, BreakingExclamation},
    max_length::{max_length, MaxLength},
    min_length::{min_length, MinLength},
};

