use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    builder_error_enum::{build_error, build_error_impl},
    builder_struct::{build_struct, build_impl},
    data::DeriveData,
};

pub fn derive_builder(data: DeriveData) -> syn::Result<TokenStream> {
    let mut result = proc_macro2::TokenStream::new();
    build_struct(&data.builder_ident, data.fields.as_ref(), &data.generics).to_tokens(&mut result);
    build_impl(&data.struct_ident, &data.builder_ident, data.fields.as_ref(), &data.error_ident).to_tokens(&mut result);
    build_error(&data.fields.as_ref(), &data.error_ident).to_tokens(&mut result);
    build_error_impl(&data.fields.as_ref(), &data.error_ident).to_tokens(&mut result);
    Ok(result)
}

