use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Value)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => derive_struct(&input, data),
        syn::Data::Enum(data) => derive_enum(&input, data),
        _ => quote!().into(),
    }
}

fn derive_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let len = data.fields.len();
    let fields: Vec<_> = data.fields.iter().filter_map(|f| f.ident.clone()).collect();
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::xval::ToValue for #ident #type_generics #where_generics {
            fn to_value(&self) -> ::xval::Value {
                ::xval::Value::from_struct(self.clone())
            }
        }

        impl #impl_generics ::xval::Struct for #ident #type_generics #where_generics {
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
                        &self.#fields as &dyn ::xval::ToValue,
                    ),)*].into_iter()
                )
            }

            fn field(&self, ident: ::xval::Ident) -> Option<&dyn ::xval::ToValue> {
                #(
                    if ident == stringify!(#fields) {
                        return Some(&self.#fields as &dyn ::xval::ToValue);
                    }
                )*

                None
            }
        }
    }
    .into()
}

fn derive_enum(input: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
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
                        let mut map = ::std::collections::HashMap::new();
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
