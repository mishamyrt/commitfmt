use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parenthesized, Ident, LitStr, Path, Token};
use syn::parse::{Parse, ParseStream};

/// One mapping from `(LinterVariant, "rule-string") => rules::some_module::SomeRule`
struct RuleEntry {
    linter: Ident,
    rule_name: LitStr,
    path: Path,
}

impl Parse for RuleEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        let linter: Ident = content.parse()?;
        content.parse::<Token![,]>()?;
        let rule_name: LitStr = content.parse()?;
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }

        input.parse::<Token![=>]>()?;

        let path: Path = input.parse()?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(Self {
            linter,
            rule_name,
            path,
        })
    }
}

/// A list of all `RuleEntry` items, parsed until input is exhausted.
struct RuleList {
    entries: Vec<RuleEntry>,
}

impl Parse for RuleList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();
        while !input.is_empty() {
            entries.push(input.parse()?);
        }
        Ok(Self {
            entries,
        })
    }
}

pub(crate) fn generate_rule_enum(input: TokenStream) -> TokenStream {
    // Parse the input into our structured RuleList
    let RuleList {
        entries,
    } = syn::parse_macro_input!(input as RuleList);

    // For each entry, build an enum variant name: e.g. Body + LeadingNewLine => BodyLeadingNewLine
    let variants = entries.iter().map(|entry| {
        let linter_ident = &entry.linter;
        let last_seg = &entry.path.segments.last().unwrap().ident;
        let variant_ident = format_ident!("{}{}", linter_ident, last_seg);
        quote! { #variant_ident }
    });

    // Build each match arm (Body, "leading-newline") => Some(Rule::BodyLeadingNewLine)
    let rule_to_lit_matches = entries.iter().map(|entry| {
        let linter_ident = &entry.linter;
        let last_seg = &entry.path.segments.last().unwrap().ident;
        let variant_ident = format_ident!("{}{}", linter_ident, last_seg);
        let rule_lit = &entry.rule_name;

        quote! {
            Rule::#variant_ident => #rule_lit
        }
    });

    let lit_to_rule_matches = entries.iter().map(|entry| {
        let linter_ident = &entry.linter;
        let last_seg = &entry.path.segments.last().unwrap().ident;
        let variant_ident = format_ident!("{}{}", linter_ident, last_seg);
        let rule_lit = &entry.rule_name;

        quote! {
            (#linter_ident, #rule_lit) => Rule::#variant_ident
        }
    });

    let body_rules = entries.iter()
        .filter(|entry| {
            entry.linter == "Body"
        }).map(|entry| {
            let linter_ident = &entry.linter;
            let last_seg = &entry.path.segments.last().unwrap().ident;
            let variant_ident = format_ident!("{}{}", linter_ident, last_seg);
            quote! {
                Rule::#variant_ident
            }
        });

    let expanded = quote! {
        /// An enum containing all named rules, with each variant prefixed
        /// by the Linter variant name and suffixed by the last path segment.
        #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Rule {
            #(#variants),*
        }


        impl Rule {
            pub fn as_display(&self) -> &'static str {
                match self {
                    #(#rule_to_lit_matches),*
                }
            }

            pub fn from_name(linter: Linter, name: &str) -> Option<Self> {
                #![allow(clippy::enum_glob_use)]
                use Linter::*;

                Some(match (linter, name) {
                    #(#lit_to_rule_matches),*
                    ,_ => return None,
                })
            }
        }

        pub const BODY_RULES: &[Rule] = &[#(#body_rules),*];

    };

    expanded.into()
}
