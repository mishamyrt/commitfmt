use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{
    parenthesized,
    spanned::Spanned,
    Error, Expr, ExprCall, ExprMatch, Ident, ItemFn, LitStr, Pat, Path, Stmt, Token,
};

struct Rule {
    variant_name: Ident,
    linter: Ident,
    name: LitStr,
    struct_name: Ident,
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        let linter: Ident = content.parse()?;
        content.parse::<Token![,]>()?;
        let name: LitStr = content.parse()?;
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }

        input.parse::<Token![=>]>()?;

        let path: Path = input.parse()?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let last_seg = &path.segments.last().unwrap().ident;
        let variant_name = format_ident!("{}{}", linter, last_seg);

        Ok(Self {
            variant_name,
            linter,
            name,
            struct_name: last_seg.clone(),
        })
    }
}

pub(crate) fn map_names(func: &ItemFn) -> syn::Result<TokenStream> {
    let Some(last_stmt) = func.block.stmts.last() else {
        return Err(Error::new(func.block.span(), "expected body to end in an expression"));
    };
    let Stmt::Expr(
        Expr::Call(ExprCall {
            args: some_args,
            ..
        }),
        _,
    ) = last_stmt
    else {
        return Err(Error::new(last_stmt.span(), "expected last expression to be `Some(match (..) { .. })`"));
    };
    let mut some_args = some_args.into_iter();
    let (
        Some(Expr::Match(ExprMatch {
            arms,
            ..
        })),
        None,
    ) = (some_args.next(), some_args.next())
    else {
        return Err(Error::new(last_stmt.span(), "expected last expression to be `Some(match (..) { .. })`"));
    };

    let mut rules: Vec<Rule> = Vec::with_capacity(arms.len());
    for arm in arms {
        if matches!(arm.pat, Pat::Wild(..)) {
            break;
        }

        let rule = syn::parse::<Rule>(arm.into_token_stream().into())?;
        rules.push(rule);
    }

    // Build each enum variants, BodyLeadingNewLine
    let variants = rules.iter().map(|rule| {
        let variant_ident = &rule.variant_name;
        quote! { #variant_ident }
    });

    // Build each match arm, Rule::BodyLeadingNewLine => "leading-newline"
    let rule_to_lit_matches = rules.iter().map(|rule| {
        let variant_ident = &rule.variant_name;
        let rule_lit = &rule.name;

        quote! {
            Rule::#variant_ident => #rule_lit
        }
    });

    // Build each match arm, (Body, "leading-newline") => Rule::BodyLeadingNewLine
    let lit_to_rule_matches = rules.iter().map(|rule| {
        let linter_ident = &rule.linter;
        let variant_ident = &rule.variant_name;
        let rule_lit = &rule.name;

        quote! {
            (#linter_ident, #rule_lit) => Rule::#variant_ident
        }
    });

    // Build each match arm, (Header, "LeadingSpace") => Rule::HeaderLeadingSpace,
    let violation_to_rule_matches = rules.iter().map(|rule| {
        let linter_ident = &rule.linter;
        let variant_ident = &rule.variant_name;
        let struct_name = &rule.struct_name;
        let name = LitStr::new(&struct_name.to_string(), struct_name.span());

        quote! {
            (#linter_ident, #name) => Rule::#variant_ident
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

            pub fn from_name(linter: LinterGroup, name: &str) -> Option<Self> {
                #![allow(clippy::enum_glob_use)]
                use LinterGroup::*;

                Some(match (linter, name) {
                    #(#lit_to_rule_matches),*
                    ,_ => return None,
                })
            }

            pub fn from_violation(violation: &dyn Violation) -> Option<Self> {
                use LinterGroup::*;

                Some(match (violation.group(), violation.rule_name()) {
                    #(#violation_to_rule_matches),*
                    ,_ => return None,
                })
            }
        }
    };

    Ok(expanded)
}
