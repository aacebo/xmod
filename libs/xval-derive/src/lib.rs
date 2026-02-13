#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Value)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => derive_struct_value(&input, data),
        // syn::Data::Enum(data) => derive_enum_value(&input, data),
        _ => quote!().into(),
    }
}

fn derive_struct_value(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let len = data.fields.len();
    let fields: Vec<syn::Ident> = data.fields.iter().filter_map(|f| f.ident.clone()).collect();

    quote! {
        impl ::xval::AsValue for #ident {
            fn as_value(&self) -> ::xval::Value {
                ::xval::Value::from_struct(self.clone())
            }
        }

        impl ::xval::Struct for #ident {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }

            fn len(&self) -> usize {
                #len
            }

            fn items(&self) -> ::xval::StructIter<'_> {
                ::xval::StructIter::new(
                    [#((
                        ::xval::Ident::from(stringify!(#fields)),
                        &self.#fields as &dyn ::xval::AsValue,
                    ),)*].into_iter()
                )
            }

            fn field(&self, ident: ::xval::Ident) -> Option<&dyn ::xval::AsValue> {
                #(
                    if ident == stringify!(#fields) {
                        return Some(&self.#fields as &dyn ::xval::AsValue);
                    }
                )*

                None
            }
        }
    }
    .into()
}

// fn derive_enum_value(input: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
//     let ident = &input.ident;
//     let len = data.variants.len();
//     let variants = data.variants
//         .iter()
//         .map(|variant| {
//             let variant_ident = variant.ident.clone();

//             match variant.fields {
//                 syn::Fields::Named(named) => quote!(),
//                 _ => todo!(),
//             }
//         })
//         .collect::<Vec<_>>();

//     quote! {
//         impl ::xval::AsValue for #ident {
//             fn as_value(&self) -> ::xval::Value {
//                 match self {
//                     #(
//                         Self::#variants =>
//                     )*
//                 }
//             }
//         }
//     }
//     .into()
// }
