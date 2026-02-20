use proc_macro::TokenStream;
use quote::quote;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();
    let match_arms = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_idents: Vec<_> = fields
                    .named
                    .iter()
                    .filter_map(|f| f.ident.as_ref())
                    .collect();

                quote! {
                    Self::#variant_ident { #( #field_idents ),* } => {
                        let mut map = ::std::collections::BTreeMap::new();
                        #(
                            map.insert(
                                ::xval::Ident::from(stringify!(#field_idents)),
                                #field_idents.to_value(),
                            );
                        )*
                        ::xval::Value::from_struct(map)
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let bindings: Vec<syn::Ident> = (0..fields.unnamed.len())
                    .map(|i| quote::format_ident!("_{}", i))
                    .collect();

                quote! {
                    Self::#variant_ident( #( #bindings ),* ) => {
                        ::xval::Value::from_tuple(( #( #bindings.to_value(), )* ))
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    Self::#variant_ident => ::xval::Value::Null
                }
            }
        }
    });

    quote! {
        impl #impl_generics ::xval::ToValue for #ident #type_generics #where_generics {
            fn to_value(&self) -> ::xval::Value {
                match self {
                    #( #match_arms ),*
                }
            }
        }
    }
    .into()
}
