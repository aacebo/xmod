use proc_macro::TokenStream;
use quote::{ToTokens, quote};

use crate::rules;
use crate::schema_type::SchemaType;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();

    let field_stmts: Vec<proc_macro2::TokenStream> = data
        .fields
        .iter()
        .filter_map(|field| {
            let field_ident = field.ident.as_ref()?;
            let field_name = field_ident.to_string();
            let kind = SchemaType::from_type(&field.ty);
            let base = kind.to_token_stream();

            let attrs: Vec<_> = field
                .attrs
                .iter()
                .filter(|a| a.meta.path().is_ident("schema"))
                .collect();

            let mut rule_calls: Vec<proc_macro2::TokenStream> = Vec::new();

            for attr in &attrs {
                let punct: syn::punctuated::Punctuated<syn::Meta, syn::Token![,]> =
                    match attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated) {
                        Ok(v) => v,
                        Err(e) => return Some(e.to_compile_error()),
                    };

                for meta in &punct {
                    rule_calls.push(
                        rules::parse_rule(meta, &kind).unwrap_or_else(|e| e.to_compile_error()),
                    );
                }
            }

            let schema_expr = if rule_calls.is_empty() {
                quote!(#base.into())
            } else {
                quote! {{
                    let schema = #base;
                    #(let schema = schema.#rule_calls;)*
                    schema.into()
                }}
            };

            Some(quote! {
                schema = schema.field(#field_name, #schema_expr);
            })
        })
        .collect();

    quote! {
        impl #impl_generics ::xsch::ToSchema for #ident #type_generics #where_generics {
            fn to_schema(&self) -> ::xsch::Schema {
                let mut schema = ::xsch::object();
                #(#field_stmts)*
                schema.into()
            }
        }

        impl #impl_generics ::xsch::Validate for #ident #type_generics #where_generics {
            fn validate(&self) -> Result<::xval::Value, ::xsch::ValidError> {
                self.to_schema().validate(&::xval::ToValue::to_value(self).into())
            }
        }
    }
    .into()
}
