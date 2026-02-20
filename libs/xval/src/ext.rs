use crate::{Ident, ToValue, object::Struct};

pub trait StructExt: Struct {
    fn get(&self, key: &str) -> Option<&dyn ToValue> {
        self.field(Ident::from(key))
    }
}

impl<T: Struct + ?Sized> StructExt for T {}
