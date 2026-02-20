use quote::{ToTokens, quote};

use crate::schema_type::SchemaType;

pub fn parse_rule(
    meta: &syn::Meta,
    kind: &SchemaType,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let kind = kind.inner();

    let rule_path = match meta {
        syn::Meta::Path(v) => v,
        syn::Meta::NameValue(v) => &v.path,
        syn::Meta::List(v) => {
            return Err(syn::Error::new_spanned(
                v,
                "use `rule = value` syntax, not `rule(value)`",
            ));
        }
    };

    let rule_value: Option<&syn::Expr> = match meta {
        syn::Meta::NameValue(v) => Some(&v.value),
        _ => None,
    };

    if rule_path.is_ident("required") {
        match rule_value {
            None => Ok(quote!(required())),
            Some(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(b),
                ..
            })) => {
                if b.value {
                    Ok(quote!(required()))
                } else {
                    Ok(quote!())
                }
            }
            Some(v) => Err(syn::Error::new_spanned(
                v,
                "required expects a boolean value or no value",
            )),
        }
    } else if rule_path.is_ident("min") {
        let val = rule_value
            .ok_or_else(|| syn::Error::new_spanned(meta, "min requires a value: `min = N`"))?;
        Ok(quote!(min(#val)))
    } else if rule_path.is_ident("max") {
        let val = rule_value
            .ok_or_else(|| syn::Error::new_spanned(meta, "max requires a value: `max = N`"))?;
        Ok(quote!(max(#val)))
    } else if rule_path.is_ident("equals") || rule_path.is_ident("eq") {
        let val = rule_value.ok_or_else(|| {
            syn::Error::new_spanned(meta, "equals requires a value: `equals = V`")
        })?;
        match kind {
            SchemaType::String => Ok(quote!(equals(#val))),
            SchemaType::Int { ctor } | SchemaType::UInt { ctor } => Ok(quote!(equals(#ctor(#val)))),
            SchemaType::Float { ctor } => Ok(quote!(equals(#ctor(#val)))),
            SchemaType::Bool => Ok(quote!(equals(#val))),
            SchemaType::Any => Ok(quote!(equals(::xval::ToValue::to_value(&#val)))),
            _ => Err(syn::Error::new_spanned(
                meta,
                "equals is not supported for this field type",
            )),
        }
    } else if rule_path.is_ident("options") {
        let val = rule_value.ok_or_else(|| {
            syn::Error::new_spanned(meta, "options requires a value: `options = [A, B, C]`")
        })?;
        if let syn::Expr::Array(expr_array) = val {
            let elems: Vec<&syn::Expr> = expr_array.elems.iter().collect();
            match kind {
                SchemaType::String => Ok(quote!(options(&[#(#elems),*]))),
                SchemaType::Int { ctor } | SchemaType::UInt { ctor } => {
                    Ok(quote!(options(&[#(#ctor(#elems)),*])))
                }
                SchemaType::Float { ctor } => Ok(quote!(options(&[#(#ctor(#elems)),*]))),
                SchemaType::Bool => Ok(quote!(options(&[#(#elems),*]))),
                SchemaType::Any => Ok(quote!(options(&[#(::xval::ToValue::to_value(&#elems)),*]))),
                _ => Err(syn::Error::new_spanned(
                    meta,
                    "options is not supported for this field type",
                )),
            }
        } else {
            Err(syn::Error::new_spanned(
                val,
                "options requires an array literal: `options = [A, B, C]`",
            ))
        }
    } else if rule_path.is_ident("pattern") {
        if !matches!(kind, SchemaType::String) {
            return Err(syn::Error::new_spanned(
                meta,
                "pattern is only supported for String fields",
            ));
        }
        let val = rule_value.ok_or_else(|| {
            syn::Error::new_spanned(meta, "pattern requires a value: `pattern = \"...\"`")
        })?;
        Ok(quote!(pattern(#val)))
    } else {
        Err(syn::Error::new_spanned(
            meta,
            format!("unknown schema rule `{}`", rule_path.to_token_stream()),
        ))
    }
}
