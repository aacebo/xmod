mod enums;
mod structs;
mod tuples;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Value)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(_) => structs::derive(&input, data),
            syn::Fields::Unnamed(_) => tuples::derive(&input, data),
            syn::Fields::Unit => {
                let ident = &input.ident;
                let (ig, tg, wg) = input.generics.split_for_impl();
                quote! {
                    impl #ig ::xval::ToValue for #ident #tg #wg {
                        fn to_value(&self) -> ::xval::Value {
                            ::xval::Value::Null
                        }
                    }
                }
                .into()
            }
        },
        syn::Data::Enum(data) => enums::derive(&input, data),
        _ => syn::Error::new_spanned(&input, "Value cannot be derived for unions")
            .to_compile_error()
            .into(),
    }
}
