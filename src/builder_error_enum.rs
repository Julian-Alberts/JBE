use crate::{data::Field, syn_attribute_helper::construct_attribute};

pub fn build_error(fields: &[Field], error_ident: &syn::Ident) -> syn::ItemEnum {
    let variants = fields
        .into_iter()
        .filter(|f| f.default.is_none() && f.is_optional.is_none())
        .map(|f| {
            let ident = field_ident_to_error_variant_ident(&f.ident);
            syn::Variant {
                attrs: Default::default(),
                discriminant: Default::default(),
                fields: syn::Fields::Unit,
                ident,
            }
        })
        .collect();
    syn::ItemEnum {
        attrs: vec![construct_attribute("derive", &["Debug", "PartialEq", "Eq"])],
        vis: syn::Visibility::Public(syn::VisPublic {
            pub_token: Default::default(),
        }),
        enum_token: Default::default(),
        ident: error_ident.clone(),
        generics: Default::default(),
        brace_token: Default::default(),
        variants,
    }
}

pub fn build_error_impl(fields: &[Field], error_ident: &syn::Ident) -> proc_macro2::TokenStream {
    let arms = fields
        .into_iter()
        .filter(|f| f.default.is_none() && f.is_optional.is_none())
        .map(|f| {
            let variant = field_ident_to_error_variant_ident(&f.ident);
            let field_ident = &f.ident;
            quote::quote!(
                Self::#variant => write!(f, stringify!(Error #field_ident not set)),
            )
        });

    quote::quote!(
        impl std::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#arms)*
                }
            }
        }
        impl std::error::Error for #error_ident {}
    )
}

pub fn field_ident_to_error_variant_ident(field: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        format!("Unset{}", snake_to_pascal(field.to_string().as_str())).as_str(),
        proc_macro2::Span::call_site(),
    )
}

fn snake_to_pascal(s: &str) -> String {
    let mut pascal_case_string = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                pascal_case_string.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                pascal_case_string.push(c);
            }
        }
    }

    pascal_case_string
}
