use quote::{TokenStreamExt, quote};

pub enum SchemaType {
    String,
    Int { ctor: proc_macro2::TokenStream },
    UInt { ctor: proc_macro2::TokenStream },
    Float { ctor: proc_macro2::TokenStream },
    Bool,
    Array(Box<SchemaType>),
    Optional(Box<SchemaType>),
    Any,
}

impl SchemaType {
    pub fn from_type(ty: &syn::Type) -> Self {
        if let syn::Type::Path(tp) = ty {
            if let Some(ident) = tp.path.get_ident() {
                return Self::from_ident_str(&ident.to_string());
            }

            if let Some(inner) = extract_inner_type(&tp.path, "Option") {
                return Self::Optional(Box::new(Self::from_type(inner)));
            }

            if let Some(inner) = extract_inner_type(&tp.path, "Vec") {
                return Self::Array(Box::new(Self::from_type(inner)));
            }

            if let Some(last) = tp.path.segments.last() {
                return Self::from_ident_str(&last.ident.to_string());
            }
        }

        Self::Any
    }

    fn from_ident_str(s: &str) -> Self {
        match s {
            "String" | "str" => Self::String,
            "bool" => Self::Bool,
            "i8" => Self::Int {
                ctor: quote!(::xval::Int::from_i8),
            },
            "i16" => Self::Int {
                ctor: quote!(::xval::Int::from_i16),
            },
            "i32" => Self::Int {
                ctor: quote!(::xval::Int::from_i32),
            },
            "i64" => Self::Int {
                ctor: quote!(::xval::Int::from_i64),
            },
            "i128" => Self::Int {
                ctor: quote!(::xval::Int::from_i128),
            },
            "isize" => Self::Int {
                ctor: quote!(::xval::Int::from_isize),
            },
            "u8" => Self::UInt {
                ctor: quote!(::xval::UInt::from_u8),
            },
            "u16" => Self::UInt {
                ctor: quote!(::xval::UInt::from_u16),
            },
            "u32" => Self::UInt {
                ctor: quote!(::xval::UInt::from_u32),
            },
            "u64" => Self::UInt {
                ctor: quote!(::xval::UInt::from_u64),
            },
            "u128" => Self::UInt {
                ctor: quote!(::xval::UInt::from_u128),
            },
            "usize" => Self::UInt {
                ctor: quote!(::xval::UInt::from_usize),
            },
            "f32" => Self::Float {
                ctor: quote!(::xval::Float::from_f32),
            },
            "f64" => Self::Float {
                ctor: quote!(::xval::Float::from_f64),
            },
            _ => Self::Any,
        }
    }

    pub fn inner(&self) -> &Self {
        match self {
            Self::Optional(inner) => inner,
            other => other,
        }
    }
}

fn extract_inner_type<'a>(path: &'a syn::Path, wrapper: &str) -> Option<&'a syn::Type> {
    let segment = path.segments.last()?;
    if segment.ident != wrapper {
        return None;
    }

    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
            return Some(ty);
        }
    }

    None
}

impl quote::ToTokens for SchemaType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Self::String => quote!(::xsch::string()),
            Self::Int { .. } => quote!(::xsch::int()),
            Self::UInt { .. } => quote!(::xsch::number()),
            Self::Float { .. } => quote!(::xsch::float()),
            Self::Bool => quote!(::xsch::bool()),
            Self::Array(inner) => {
                let inner_tokens = inner.to_token_stream();
                quote!(::xsch::array().items(#inner_tokens.into()))
            }
            Self::Optional(inner) => inner.to_token_stream(),
            Self::Any => quote!(::xsch::any()),
        });
    }
}
