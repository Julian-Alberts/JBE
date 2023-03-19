use proc_macro2::TokenStream;

use crate::{
    builder_error_enum::field_ident_to_error_variant_ident,
    data::Field,
    syn_attribute_helper::{construct_attribute, construct_doc_comment},
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
    setter_attributes: &[Field],
    required_build_fields: &[Field],
    error_ident: &syn::Ident,
    copy_on_build: bool,
    generics: syn::Generics
) -> TokenStream {
    let generics_without_bounds = generics.params.iter().map(|p| {
        if let syn::GenericParam::Type(ty) = p {
            syn::GenericParam::Type(
                syn::TypeParam {
                    attrs: ty.attrs.clone(),
                    ident: ty.ident.clone(),
                    colon_token: None,
                    bounds: Default::default(),
                    eq_token: Default::default(),
                    default: Default::default(),
                }
            )
        } else {
            p.clone()
        }
    }).collect::<syn::punctuated::Punctuated<_,_>>();
    let generics_without_bounds = syn::Generics {
        lt_token: Default::default(),
        params: generics_without_bounds,
        gt_token: Default::default(),
        where_clause: Default::default(),
    };

    let setter = build_setter_functions(setter_attributes);
    let build = build_builder_functions(
        struct_ident,
        setter_attributes,
        required_build_fields,
        error_ident,
        copy_on_build,
        &generics_without_bounds,
    );
    let where_clause = if let Some(where_clause) = &generics.where_clause {
        let mut ts = quote::quote!(where);
        where_clause.predicates.iter().for_each(|p| {
            ts.extend(quote::quote!(#p,))
        });
        Some(ts)
    } else {
        None
    };
    quote::quote!(
        impl #generics #builder_ident #generics_without_bounds #where_clause {
            #setter
            #build
        }
    )
}

fn build_setter_functions(fields: &[Field]) -> proc_macro2::TokenStream {
    fields.into_iter().fold(
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

            let fn_ident_with = syn::Ident::new(format!("with_{}", ident.to_string()).as_str(), ident.span());
            let fn_ident_set = syn::Ident::new(format!("set_{}", ident.to_string()).as_str(), ident.span());
            quote::quote!(
                #prev
                #(#comments)*
                #[must_use]
                pub fn #fn_ident_with(mut self, #ident: #ty) -> Self {
                    self.#ident = Some(#ident);
                    self
                }

                #(#comments)*
                pub fn #fn_ident_set(&mut self, #ident: #ty) {
                    self.#ident = Some(#ident)
                }
            )
        },
    )
}

fn build_builder_functions(
    struct_ident: &syn::Ident,
    setter_attributes: &[Field],
    required_build_fields: &[Field],
    error_ident: &syn::Ident,
    copy_on_build: bool,
    generics: &syn::Generics
) -> proc_macro2::TokenStream {
    let clone_fn = if !copy_on_build {
        proc_macro2::TokenStream::new()
    } else {
        quote::quote!(.clone())
    };
    let build_body = setter_attributes.iter().fold(
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
                    #ident: self.#ident #clone_fn.unwrap_or_else(|| #default),
                )
            } else {
                if is_optional.is_some() && copy_on_build {
                    quote::quote!(
                        #prev
                        #ident: self.#ident.clone(),
                    )
                } else if is_optional.is_some() && !copy_on_build {
                    quote::quote!(
                        #prev
                        #ident: self.#ident,
                    )
                } else {
                    let error_variant_error = field_ident_to_error_variant_ident(ident);
                    quote::quote!(
                        #prev
                        #ident: match self.#ident #clone_fn {
                            Some(#ident) => #ident,
                            None => return Err(#error_ident::#error_variant_error)
                        },
                    )
                }
            }
        },
    );
    let build_body = required_build_fields.iter().fold(build_body, |prev, Field { ident, default: _, ty: _, is_optional: _}| {
        quote::quote!(
            #prev
            #ident: #ident,
        )
    });
    
    let try_build_comments = [
        construct_doc_comment(format!("Construct a new {struct_ident} instance. This function returns an error if not all required values are set").as_str()),
        construct_doc_comment("# Required values"),
        construct_doc_comment(
            setter_attributes
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
            setter_attributes
                .iter()
                .filter(|f| f.is_optional.is_none() && f.default.is_none())
                .map(|f| format!("* {}\n", f.ident.to_string()))
                .collect::<String>()
                .as_str(),
        ),
        construct_doc_comment("# Panics"),
        construct_doc_comment("This function may panic if not all required values are set."),
    ];
    let self_token = if copy_on_build {
        quote::quote!(&self)
    } else {
        quote::quote!(self)
    };
    quote::quote!(
            #(#try_build_comments)*
            pub fn try_build(#self_token) -> Result<#struct_ident #generics, #error_ident> {
                Ok(#struct_ident {
                    #build_body
                })
            }
            #(#build_comments)*
            pub fn build(#self_token) -> #struct_ident #generics {
                self.try_build().unwrap()
            }
    )
    
}
