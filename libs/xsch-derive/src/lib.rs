use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Validate)]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => derive_struct(&input, data),
        _ => quote!().into(),
    }
}

fn derive_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let fields: Vec<syn::Ident> = data.fields.iter().filter_map(|f| f.ident.clone()).collect();

    quote! {
        impl ::xsch::AsSchema for #ident {
            fn as_schema(&self) -> ::xsch::Schema {
                let mut schema = ::xsch::object();

                #(
                    schema = schema.field(stringify!(#fields), xsch::AsSchema::as_schema(&self.#fields));
                )*

                schema.into()
            }
        }
    }
    .into()
}
