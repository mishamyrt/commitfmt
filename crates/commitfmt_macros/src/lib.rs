mod violation_metadata;
mod rules_enum;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

use violation_metadata::violation_metadata;

#[proc_macro_derive(ViolationMetadata)]
pub fn derive_violation_metadata(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);

    violation_metadata(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn rules_enum(input: TokenStream) -> TokenStream {
    rules_enum::generate_rule_enum(input)
}
