mod rules;
mod schema_type;
mod structs;

use proc_macro::TokenStream;

#[proc_macro_derive(Validate, attributes(schema))]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => structs::derive(&input, data),
        _ => syn::Error::new_spanned(&input, "Validate can only be derived for structs")
            .to_compile_error()
            .into(),
    }
}
