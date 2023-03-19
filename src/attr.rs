#[derive(Debug)]
pub struct Attr {
    _brace_token: syn::token::Brace,
    fields: syn::punctuated::Punctuated<AttrField, syn::Token![,]>,
}

impl Attr {
    pub fn find_field<T: syn::parse::Parse>(&self, ident: &str) -> Option<syn::Result<T>> {
        self.fields
            .iter()
            .find(|field| field.ident.to_string().as_str() == ident)
            .map(|field| syn::parse2(field.expr.clone()))
    }
}

impl syn::parse::Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let brace_token = syn::braced!(content in input);

        let fields =
            syn::punctuated::Punctuated::parse_terminated_with(&content, AttrField::parse)?;

        Ok(Attr {
            _brace_token: brace_token,
            fields,
        })
    }
}

#[derive(Debug)]
pub struct AttrField {
    ident: syn::Ident,
    _colon_token: syn::Token![:],
    expr: proc_macro2::TokenStream,
}

impl syn::parse::Parse for AttrField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _colon_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}
