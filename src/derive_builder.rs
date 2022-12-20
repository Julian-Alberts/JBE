use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemEnum;

use crate::data::{DeriveData, Field, Fields};

pub fn derive_builder(data: DeriveData) -> syn::Result<TokenStream> {
    let mut result = proc_macro2::TokenStream::new();
    build_struct(&data).to_tokens(&mut result);
    build_impl(&data).to_tokens(&mut result);
    build_error(&data).to_tokens(&mut result);
    build_error_impl(&data).to_tokens(&mut result);
    Ok(result)
}

fn build_struct(
    DeriveData {
        struct_ident: _,
        builder_ident,
        fields: Fields { fields },
        generics,
        error_ident: _,
    }: &DeriveData,
) -> syn::ItemStruct {
    let fields = fields
        .into_iter()
        .map(
            |Field {
                 ident,
                 default: _,
                 ty,
                 is_optional,
             }| {
                let ty = if let Some(ty) = is_optional {
                    ty.clone()
                } else {
                    ty.clone()
                };
                generate_new_builder_field(ident.clone(), ty)
            },
        )
        .collect();
    syn::ItemStruct {
        attrs: vec![
            construct_attribute("derive", &["Default"]),
            construct_doc_comment("Test comment"),
        ],
        fields: syn::Fields::Named(syn::FieldsNamed {
            named: fields,
            brace_token: Default::default(),
        }),
        generics: generics.clone(),
        ident: builder_ident.clone(),
        semi_token: None,
        struct_token: Default::default(),
        vis: syn::Visibility::Public(syn::VisPublic {
            pub_token: Default::default(),
        }),
    }
}

fn generate_new_builder_field(ident: syn::Ident, ty: syn::Type) -> syn::Field {
    syn::Field {
        ident: Some(ident),
        vis: syn::Visibility::Inherited,
        attrs: Vec::new(),
        ty: syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: syn::punctuated::Punctuated::from_iter([syn::PathSegment {
                    ident: syn::Ident::new("Option", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: syn::token::Lt::default(),
                            args: syn::punctuated::Punctuated::from_iter([
                                syn::GenericArgument::Type(ty),
                            ]),
                            gt_token: syn::token::Gt::default(),
                        },
                    ),
                }]),
            },
        }),
        colon_token: Default::default(),
    }
}

fn build_impl(
    DeriveData {
        struct_ident,
        builder_ident,
        fields: Fields { fields },
        error_ident,
        ..
    }: &DeriveData,
) -> TokenStream {
    let setter = fields.into_iter().fold(
        proc_macro2::TokenStream::new(),
        |prev,
         Field {
             ident,
             default,
             ty,
             is_optional,
         }| {
            let ty = if let Some(ty) = is_optional { ty } else { ty };

            let comment_is_optional = if is_optional.is_some() || default.is_some() {
                "This value is optional"
            } else {
                "This value is required"
            };
            let comments = [
                construct_doc_comment(format!("Set the {ident} to the given value.").as_str()),
                construct_doc_comment(comment_is_optional),
            ];

            quote::quote!(
                #prev
                #(#comments)*
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            )
        },
    );
    let build = {
        let build_body = fields.iter().fold(
            proc_macro2::TokenStream::new(),
            |prev,
             Field {
                 ident,
                 default,
                 ty: _,
                 is_optional,
             }| {
                if let Some(default) = default {
                    quote::quote!(
                        #prev
                        #ident: self.#ident.clone().unwrap_or_else(|| #default),
                    )
                } else {
                    if is_optional.is_some() {
                        quote::quote!(
                            #prev
                            #ident: self.#ident.clone(),
                        )
                    } else {
                        let error_variant_error = field_ident_to_error_variant_ident(ident);
                        quote::quote!(
                            #prev
                            #ident: match self.#ident.clone() {
                                Some(#ident) => #ident,
                                None => return Err(#error_ident::#error_variant_error)
                            },
                        )
                    }
                }
            },
        );
        let try_build_comments = [
            construct_doc_comment(format!("Construct a new {struct_ident} instance. This function returns an error if not all required values are set").as_str()),
            construct_doc_comment("# Required values"),
            construct_doc_comment(
                fields
                    .iter()
                    .filter(|f| f.is_optional.is_none() && f.default.is_none())
                    .map(|f| format!("* {}\n", f.ident.to_string()))
                    .collect::<String>()
                    .as_str(),
            ),
        ];
        let build_comments = [
            construct_doc_comment(format!("Construct a new {struct_ident} instance.").as_str()),
            construct_doc_comment("# Required values"),
            construct_doc_comment(
                fields
                    .iter()
                    .filter(|f| f.is_optional.is_none() && f.default.is_none())
                    .map(|f| format!("* {}\n", f.ident.to_string()))
                    .collect::<String>()
                    .as_str(),
            ),
            construct_doc_comment("# Panics"),
            construct_doc_comment("This function may panic if not all required values are set."),
        ];
        quote::quote!(
            #(#try_build_comments)*
            pub fn try_build(&self) -> Result<#struct_ident, #error_ident> {
                Ok(#struct_ident {
                    #build_body
                })
            }
            #(#build_comments)*
            pub fn build(&self) -> #struct_ident {
                self.try_build().unwrap()
            }
        )
    };
    quote::quote!(
        impl #builder_ident {
            #setter
            #build
        }
    )
}

fn build_error(
    DeriveData {
        struct_ident: _,
        builder_ident: _,
        fields: Fields { fields },
        error_ident,
        ..
    }: &DeriveData,
) -> ItemEnum {
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
    ItemEnum {
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

fn build_error_impl(
    DeriveData {
        fields: Fields { fields },
        error_ident,
        ..
    }: &DeriveData,
) -> proc_macro2::TokenStream {
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

fn field_ident_to_error_variant_ident(field: &syn::Ident) -> syn::Ident {
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

fn construct_attribute(name: &str, args: &[&str]) -> syn::Attribute {
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

fn construct_doc_comment(comment: &str) -> syn::Attribute {
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
