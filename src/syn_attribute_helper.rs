pub fn construct_attribute(name: &str, args: &[&str]) -> syn::Attribute {
    syn::Attribute {
        pound_token: Default::default(),
        bracket_token: Default::default(),
        path: syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter([syn::PathSegment {
                ident: syn::Ident::new(name, proc_macro2::Span::call_site()),
                arguments: construct_attribute_args(args),
            }]),
        },
        style: syn::AttrStyle::Outer,
        tokens: Default::default(),
    }
}

pub fn construct_doc_comment(comment: &str) -> syn::Attribute {
    syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        path: syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter([syn::PathSegment {
                ident: syn::Ident::new("doc", proc_macro2::Span::call_site()),
                arguments: Default::default(),
            }]),
        },
        tokens: quote::quote!(= #comment),
    }
}

fn construct_attribute_args(args: &[&str]) -> syn::PathArguments {
    if args.is_empty() {
        return syn::PathArguments::None;
    }
    syn::PathArguments::Parenthesized(syn::ParenthesizedGenericArguments {
        inputs: syn::punctuated::Punctuated::from_iter(args.into_iter().map(|arg| {
            syn::Type::Path(syn::TypePath {
                path: syn::Path {
                    leading_colon: None,
                    segments: syn::punctuated::Punctuated::from_iter([syn::PathSegment {
                        ident: syn::Ident::new(arg, proc_macro2::Span::call_site()),
                        arguments: Default::default(),
                    }]),
                },
                qself: None,
            })
        })),
        output: syn::ReturnType::Default,
        paren_token: Default::default(),
    })
}
