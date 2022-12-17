use data::DeriveData;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod attr;
mod data;
mod derive_builder;
mod derive_consuming_builder;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let input = match DeriveData::new(input, "builder") {
        Ok(i) => i,
        Err(e) => return e.to_compile_error().into(),
    };
    derive_builder::derive_builder(input)
        .unwrap_or_else(|e| syn::Error::to_compile_error(&e))
        .into()
}

#[proc_macro_derive(ConsumingBuilder)]
pub fn derive_consuming_builder(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let _input = match DeriveData::new(input, "consuming_builder") {
        Ok(i) => i,
        Err(e) => return e.to_compile_error().into(),
    };
    todo!()
}
