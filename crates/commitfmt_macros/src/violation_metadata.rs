use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Lit, LitStr, Meta};

pub(crate) fn violation_metadata(input: DeriveInput) -> Result<TokenStream, Error> {
    let docs = get_docs(&input.attrs)?;

    let name = input.ident;

    Ok(quote! {
        #[automatically_derived]
        #[allow(deprecated)]
        impl ViolationMetadata for #name {
            fn rule_name(&self) -> &'static str {
                stringify!(#name)
            }

            fn explain(&self) -> Option<&'static str> {
                Some(#docs)
            }
        }
    })
}

/// Collect all doc comment attributes into a string
fn get_docs(attrs: &[Attribute]) -> syn::Result<String> {
    let mut explanation = String::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Some(lit) = parse_attr(["doc"], attr) {
                let value = lit.value();
                // `/// ` adds
                let line = value.strip_prefix(' ').unwrap_or(&value);
                explanation.push_str(line);
                explanation.push('\n');
            } else {
                return Err(Error::new_spanned(attr, "unimplemented doc comment style"));
            }
        }
    }
    Ok(explanation)
}

fn parse_attr<'a, const LEN: usize>(
    path: [&'static str; LEN],
    attr: &'a Attribute,
) -> Option<&'a LitStr> {
    if let Meta::NameValue(name_value) = &attr.meta {
        let path_idents = name_value.path.segments.iter().map(|segment| &segment.ident);

        if itertools::equal(path_idents, path) {
            if let syn::Expr::Lit(syn::ExprLit { lit: Lit::Str(lit), .. }) = &name_value.value
            {
                return Some(lit);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_docs() {
        let input = quote! {
            #[doc = "foo"]
            #[doc = "bar"]
            #[doc = "baz"]
            struct Foo;
        };
        let input: DeriveInput = syn::parse2(input).unwrap();
        let docs = get_docs(&input.attrs).unwrap();
        assert_eq!(docs, "foo\nbar\nbaz\n");
    }

    #[test]
    fn test_parse_attr() {
        let input = quote! {
            #[doc = "foo"]
            #[doc = "bar"]
            #[doc = "baz"]
            struct Foo;
        };
        let input: DeriveInput = syn::parse2(input).unwrap();
        let doc = parse_attr(["doc"], &input.attrs[0]).unwrap();
        assert_eq!(doc.value(), "foo");

        let doc = parse_attr(["doc"], &input.attrs[1]).unwrap();
        assert_eq!(doc.value(), "bar");

        let doc = parse_attr(["doc"], &input.attrs[2]).unwrap();
        assert_eq!(doc.value(), "baz");
    }

    #[test]
    fn test_violation_metadata() {
        let input = quote! {
            #[doc = "foo"]
            #[doc = "bar"]
            #[doc = "baz"]
            struct Foo;
        };
        let input: DeriveInput = syn::parse2(input).unwrap();
        let docs = violation_metadata(input).unwrap();
        assert!(docs.to_string().contains("fn rule_name"));
    }
}
