use proc_macro::TokenStream;
use quote::quote;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let len = data.fields.len();
    let indices: Vec<syn::Index> = (0..len).map(syn::Index::from).collect();
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::xval::ToValue for #ident #type_generics #where_generics {
            fn to_value(&self) -> ::xval::Value {
                ::xval::Value::from_tuple(self.clone())
            }
        }

        impl #impl_generics ::xval::Tuple for #ident #type_generics #where_generics {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn type_id(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<Self>()
            }

            fn len(&self) -> usize {
                #len
            }

            fn items(&self) -> ::xval::TupleIter<'_> {
                ::xval::TupleIter::new(
                    [#( &self.#indices as &dyn ::xval::ToValue, )*].into_iter()
                )
            }

            fn index(&self, i: usize) -> Option<&dyn ::xval::ToValue> {
                match i {
                    #( #indices => Some(&self.#indices as &dyn ::xval::ToValue), )*
                    _ => None,
                }
            }
        }
    }
    .into()
}
