#![allow(unused)]

use proc_macro::TokenStream;

#[proc_macro_derive(ToValue)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    quote::quote!().into()
}
