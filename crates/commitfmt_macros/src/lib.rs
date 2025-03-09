mod violation_metadata;
mod map_names;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error, ItemFn};

use violation_metadata::violation_metadata;

#[proc_macro_derive(ViolationMetadata)]
pub fn derive_violation_metadata(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);

    violation_metadata(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn map_names(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    map_names::map_names(&func)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
