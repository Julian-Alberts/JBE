use proc_macro2::TokenStream;

use crate::{
    data::Field,
    syn_attribute_helper::{construct_attribute, construct_doc_comment}, builder_error_enum::field_ident_to_error_variant_ident,
};

pub fn build_struct(
    builder_ident: &syn::Ident,
    fields: &[Field],
    generics: &syn::Generics,
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

pub fn build_impl(
    struct_ident: &syn::Ident,
    builder_ident: &syn::Ident,
    fields: &[Field],
    error_ident: &syn::Ident,
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