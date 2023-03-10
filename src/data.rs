use syn::DeriveInput;

pub struct DeriveData {
    pub struct_ident: syn::Ident,
    pub builder_ident: syn::Ident,
    pub error_ident: syn::Ident,
    pub copy_on_build: bool,
    pub generics: syn::Generics,
    pub fields: Fields,
}

pub struct Fields {
    pub fields: Vec<Field>,
}

impl AsRef<[Field]> for Fields {
    fn as_ref(&self) -> &[Field] {
        &self.fields
    }
}

#[derive(Clone)]
pub struct Field {
    pub ident: syn::Ident,
    pub default: Option<syn::Expr>,
    pub ty: syn::Type,
    pub is_optional: Option<syn::Type>,
}

pub struct StructAttrs {
    builder_ident: Option<syn::Ident>,
    error_ident: Option<proc_macro2::Ident>,
    copy: bool
}

pub struct FieldAttrs {
    default: Option<syn::Expr>,
}

impl DeriveData {
    pub fn new(di: DeriveInput, data_attr: &str) -> Result<Self, syn::Error> {
        let attrs = StructAttrs::new(di.attrs.as_slice(), data_attr)?;
        let struct_ident = di.ident.clone();
        let builder_ident = if let Some(bi) = attrs.builder_ident {
            bi
        } else {
            let bi = format!("{}Builder", di.ident.to_string());
            syn::Ident::new(bi.as_str(), proc_macro2::Span::call_site())
        };
        let error_ident = if let Some(ei) = attrs.error_ident {
            ei
        } else {
            let ei = format!("{}Error", builder_ident.to_string());
            syn::Ident::new(ei.as_str(), proc_macro2::Span::call_site())
        };
        let generics = di.generics;
        let fields = match &di.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(fields),
                ..
            }) => Fields::new(fields, data_attr)?,
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Unnamed(fields),
                ..
            }) => Fields::new_unnamed(fields, data_attr)?,
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Unit,
                ..
            }) => Fields {
                fields: Vec::default(),
            },
            syn::Data::Enum(_) => {
                return Err(syn::Error::new(
                    di.ident.span(),
                    "Can not derive Builder for enum",
                ))
            }
            syn::Data::Union(_) => {
                return Err(syn::Error::new(
                    di.ident.span(),
                    "Can not derive Builder for union",
                ))
            }
        };

        Ok(Self {
            builder_ident,
            error_ident,
            struct_ident,
            fields,
            generics,
            copy_on_build: attrs.copy
        })
    }
}

impl StructAttrs {
    fn new(attrs: &[syn::Attribute], data_attr: &str) -> Result<Self, syn::Error> {
        let builder_data: crate::attr::Attr = match find_attr(attrs, data_attr) {
            Some(Ok(attr)) => attr,
            Some(Err(e)) => {
                return Err(e);
            }
            None => {
                return Ok(Self {
                    builder_ident: None,
                    error_ident: None,
                    copy: false
                })
            }
        };

        let builder_ident = builder_data.find_field::<syn::Ident>("builder_ident");
        let builder_ident = match builder_ident {
            Some(Ok(builder_ident)) => Some(builder_ident),
            Some(Err(e)) => return Err(e),
            None => None,
        };

        let error_ident = builder_data.find_field::<syn::Ident>("error_ident");
        let error_ident = match error_ident {
            Some(Ok(error_ident)) => Some(error_ident),
            Some(Err(e)) => return Err(e),
            None => None,
        };

        let copy = builder_data.find_field::<syn::LitBool>("copy");
        let copy = match copy {
            Some(Ok(syn::LitBool { value, span: _ })) => value,
            Some(Err(e)) => return Err(e),
            None => false,
        };

        Ok(Self {
            builder_ident,
            error_ident,
            copy
        })
    }
}

impl FieldAttrs {
    fn new(attrs: &[syn::Attribute], data_attr: &str) -> Result<Self, syn::Error> {
        let attr: crate::attr::Attr = match find_attr(attrs, data_attr) {
            None => return Ok(Self { default: None }),
            Some(Ok(attr)) => attr,
            Some(Err(e)) => return Err(e),
        };

        let default = match attr.find_field("default") {
            Some(Ok(d)) => Some(d),
            Some(Err(e)) => return Err(e),
            None => None,
        };

        Ok(Self { default })
    }
}

impl Fields {
    fn new(fields: &syn::FieldsNamed, data_attr: &str) -> Result<Self, syn::Error> {
        let fields = fields
            .named
            .iter()
            .map(|field| {
                let attrs = FieldAttrs::new(field.attrs.as_slice(), data_attr)?;
                let ident = field.ident.clone().unwrap();
                let ty = field.ty.clone();
                let is_optional = is_optional(&ty);
                Ok(Field {
                    ident,
                    default: attrs.default,
                    ty,
                    is_optional,
                })
            })
            .collect::<Result<Vec<_>, syn::Error>>()?;
        Ok(Fields { fields })
    }
}

impl Fields {
    fn new_unnamed(fields: &syn::FieldsUnnamed, data_attr: &str) -> Result<Self, syn::Error> {
        let fields = fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let attrs = FieldAttrs::new(field.attrs.as_slice(), data_attr)?;
                let ident =
                    syn::Ident::new(index.to_string().as_str(), proc_macro2::Span::call_site());
                let ty = field.ty.clone();
                let is_optional = is_optional(&ty);
                Ok(Field {
                    ident,
                    default: attrs.default,
                    ty,
                    is_optional,
                })
            })
            .collect::<Result<Vec<Field>, syn::Error>>()?;
        Ok(Fields { fields })
    }
}

fn is_optional(ty: &syn::Type) -> Option<syn::Type> {
    match ty {
        syn::Type::Path(syn::TypePath {
            qself: None,
            path:
                syn::Path {
                    leading_colon: None,
                    segments,
                },
        }) if segments.len() == 1 => match segments.first().unwrap() {
            syn::PathSegment {
                ident: option_ident,
                arguments:
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: _,
                        args,
                        gt_token: _,
                    }),
            } if option_ident == "Option" && args.len() == 1 => {
                let syn::GenericArgument::Type(ty) = args.first().unwrap() else {
                        return None
                    };
                Some(ty.clone())
            }
            _ => None,
        },
        _ => None,
    }
}

fn find_attr<'a, T: syn::parse::Parse>(
    attrs: &'a [syn::Attribute],
    name: &str,
) -> Option<Result<T, syn::Error>> {
    struct DefaultValue<T> {
        pub _paren_token: syn::token::Paren,
        pub data: T,
    }
    impl<T> syn::parse::Parse for DefaultValue<T>
    where
        T: syn::parse::Parse,
    {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                _paren_token: syn::parenthesized!(content in input),
                data: content.parse()?,
            })
        }
    }
    attrs
        .iter()
        .find(|attr| attr.path.is_ident(name))
        .map(|attr| syn::parse2::<DefaultValue<T>>(attr.tokens.clone()))
        .map(|data| data.map(|data| data.data))
}
