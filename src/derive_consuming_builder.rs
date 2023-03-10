use quote::ToTokens;

use crate::{
    builder_error_enum::{build_error, build_error_impl},
    builder_struct::{build_impl, build_struct},
    data::{DeriveData, Field},
};

pub fn derive_consuming_builder(data: DeriveData) -> syn::Result<proc_macro2::TokenStream> {
    let mut result = proc_macro2::TokenStream::new();
    let optional_fields = data
        .fields
        .as_ref()
        .into_iter()
        .filter(|f| f.is_optional.is_some() || f.default.is_some())
        .map(Field::clone)
        .collect::<Vec<_>>();
    let required_fields = data
        .fields
        .as_ref()
        .into_iter()
        .filter(|f| f.is_optional.is_none() && f.default.is_none())
        .map(Field::clone)
        .collect::<Vec<_>>();
    build_struct(
        &data.builder_ident,
        optional_fields.as_slice(),
        &data.generics,
    )
    .to_tokens(&mut result);
    build_impl(
        &data.struct_ident,
        &data.builder_ident,
        &optional_fields,
        &required_fields,
        &data.error_ident,
        true,
        false
    )
    .to_tokens(&mut result);
    build_error(&data.fields.as_ref(), &data.error_ident).to_tokens(&mut result);
    build_error_impl(&data.fields.as_ref(), &data.error_ident).to_tokens(&mut result);
    Ok(result)
}
