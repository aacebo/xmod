use proc_macro::TokenStream;
use quote::quote;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let fields: Vec<_> = data.fields.iter().filter_map(|f| f.ident.clone()).collect();
    let len = fields.len();
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

            fn type_id(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<Self>()
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
