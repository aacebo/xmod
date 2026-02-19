use quote::{TokenStreamExt, quote};

/// Describes how to generate schema code for a given Rust field type.
///
/// `Int` and `Float` carry the exact `xval::Int::from_*` / `xval::Float::from_*`
/// constructor so that options/equals comparisons use the same variant as the
/// actual runtime value produced by `ToValue`.
pub enum SchemaType {
    String,
    Int { ctor: proc_macro2::TokenStream },
    Float { ctor: proc_macro2::TokenStream },
    Bool,
    Any,
}

impl SchemaType {
    pub fn from_type(ty: &syn::Type) -> Self {
        if let syn::Type::Path(tp) = ty {
            if let Some(ident) = tp.path.get_ident() {
                return match ident.to_string().as_str() {
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
                    // Unsigned ints: xsch has no UIntSchema, fall back to any()
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => Self::Any,
                    "f32" => Self::Float {
                        ctor: quote!(::xval::Float::from_f32),
                    },
                    "f64" => Self::Float {
                        ctor: quote!(::xval::Float::from_f64),
                    },
                    _ => Self::Any,
                };
            }
        }

        Self::Any
    }
}

impl quote::ToTokens for SchemaType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Self::String => quote!(::xsch::string()),
            Self::Int { .. } => quote!(::xsch::int()),
            Self::Float { .. } => quote!(::xsch::float()),
            Self::Bool => quote!(::xsch::bool()),
            Self::Any => quote!(::xsch::any()),
        });
    }
}
