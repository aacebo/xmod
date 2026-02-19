mod schema_type;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use schema_type::*;

#[proc_macro_derive(Validate, attributes(field))]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => derive_struct(&input, data),
        _ => quote!().into(),
    }
}

fn derive_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;

    let field_stmts: Vec<proc_macro2::TokenStream> = data
        .fields
        .iter()
        .filter_map(|field| {
            let field_ident = field.ident.as_ref()?;
            let field_name = field_ident.to_string();
            let kind = SchemaType::from_type(&field.ty);
            let base = kind.to_token_stream();
            let field_attr = field.attrs.iter().find(|a| a.meta.path().is_ident("field"));
            let schema_expr = match field_attr {
                None => quote!(#base.into()),
                Some(attr) => {
                    let punct: syn::punctuated::Punctuated<syn::Meta, syn::Token![,]> =
                        match attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated) {
                            Ok(v) => v,
                            Err(e) => return Some(e.to_compile_error()),
                        };

                    let rule_calls: Vec<proc_macro2::TokenStream> = punct
                        .iter()
                        .map(|meta| {
                            parse_rule(meta, &kind).unwrap_or_else(|e| e.to_compile_error())
                        })
                        .collect();

                    quote! {{
                        let schema = #base;
                        #(let schema = schema.#rule_calls;)*
                        schema.into()
                    }}
                }
            };

            Some(quote! {
                schema = schema.field(#field_name, #schema_expr);
            })
        })
        .collect();

    quote! {
        impl ::xsch::AsSchema for #ident {
            fn as_schema(&self) -> ::xsch::Schema {
                let mut schema = ::xsch::object();
                #(#field_stmts)*
                schema.into()
            }
        }

        impl ::xsch::Validate for #ident {
            fn validate(&self) -> Result<::xval::Value, ::xsch::ValidError> {
                self.as_schema().validate(&::xval::AsValue::as_value(self).into())
            }
        }
    }
    .into()
}

fn parse_rule(meta: &syn::Meta, kind: &SchemaType) -> Result<proc_macro2::TokenStream, syn::Error> {
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
        Ok(quote!(required()))
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
            SchemaType::Int { ctor } => Ok(quote!(equals(#ctor(#val)))),
            SchemaType::Float { ctor } => Ok(quote!(equals(#ctor(#val)))),
            SchemaType::Bool => Ok(quote!(equals(#val))),
            SchemaType::Any => Ok(quote!(equals(::xval::AsValue::as_value(&#val)))),
        }
    } else if rule_path.is_ident("options") {
        let val = rule_value.ok_or_else(|| {
            syn::Error::new_spanned(meta, "options requires a value: `options = [A, B, C]`")
        })?;
        if let syn::Expr::Array(expr_array) = val {
            let elems: Vec<&syn::Expr> = expr_array.elems.iter().collect();
            match kind {
                SchemaType::String => Ok(quote!(options(&[#(#elems),*]))),
                SchemaType::Int { ctor } => Ok(quote!(options(&[#(#ctor(#elems)),*]))),
                SchemaType::Float { ctor } => Ok(quote!(options(&[#(#ctor(#elems)),*]))),
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
        let val = rule_value.ok_or_else(|| {
            syn::Error::new_spanned(meta, "pattern requires a value: `pattern = \"...\"`")
        })?;
        Ok(quote!(pattern(#val)))
    } else {
        Err(syn::Error::new_spanned(
            meta,
            format!("unknown field rule `{}`", rule_path.to_token_stream()),
        ))
    }
}
