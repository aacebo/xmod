use std::sync::Arc;

use crate::{AsValue, Object, Value};

pub trait Tuple: Send + Sync {
    fn name(&self) -> &str;
    fn type_id(&self) -> std::any::TypeId;
    fn len(&self) -> usize;
    fn items(&self) -> TupleIter<'_>;
    fn index(&self, i: usize) -> Option<&dyn AsValue>;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::fmt::Debug for dyn Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_tuple(self.name());

        for v in self.items() {
            dbg.field(&v.as_value());
        }

        dbg.finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Tuple {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;

        let len = self.len();
        let mut tup = serializer.serialize_tuple(len)?;

        for item in self.items() {
            tup.serialize_element(&item.as_value())?;
        }

        tup.end()
    }
}

pub struct TupleIter<'a>(Box<dyn Iterator<Item = &'a dyn AsValue> + 'a>);

impl<'a> TupleIter<'a> {
    pub fn new(iter: impl Iterator<Item = &'a dyn AsValue> + 'a) -> Self {
        Self(Box::new(iter))
    }
}

impl<'a> Iterator for TupleIter<'a> {
    type Item = &'a dyn AsValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

macro_rules! impl_tuple {
    ($name:expr, $($idx:tt),+) => {
        impl Tuple for ( $(impl_tuple!(@replace $idx Value),)+ ) {
            fn name(&self) -> &str {
                $name
            }

            fn type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }

            fn len(&self) -> usize {
                impl_tuple!(@count $($idx),+)
            }

            fn items(&self) -> TupleIter<'_> {
                TupleIter::new(vec![
                    $( &self.$idx as &dyn AsValue, )+
                ].into_iter())
            }

            fn index(&self, i: usize) -> Option<&dyn AsValue> {
                match i {
                    $( $idx => Some(&self.$idx as &dyn AsValue), )+
                    _ => None,
                }
            }
        }

        impl From<( $(impl_tuple!(@replace $idx Value),)+ )> for Object {
            fn from(value: ( $(impl_tuple!(@replace $idx Value),)+ )) -> Self {
                Self::Tuple(Arc::new(value))
            }
        }

        impl From<( $(impl_tuple!(@replace $idx Value),)+ )> for Value {
            fn from(value: ( $(impl_tuple!(@replace $idx Value),)+ )) -> Self {
                Self::Object(Object::from(value))
            }
        }
    };
    (@replace $_t:tt $sub:tt) => { $sub };
    (@count $($t:tt),+) => { 0usize $(+ impl_tuple!(@replace $t 1usize))+ };
}

impl_tuple!("Tuple1", 0);
impl_tuple!("Tuple2", 0, 1);
impl_tuple!("Tuple3", 0, 1, 2);
impl_tuple!("Tuple4", 0, 1, 2, 3);
impl_tuple!("Tuple5", 0, 1, 2, 3, 4);
impl_tuple!("Tuple6", 0, 1, 2, 3, 4, 5);
impl_tuple!("Tuple7", 0, 1, 2, 3, 4, 5, 6);
impl_tuple!("Tuple8", 0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!("Tuple9", 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!("Tuple10", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_tuple!("Tuple11", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_tuple!("Tuple12", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

macro_rules! impl_tuple_as_value {
    ($($idx:tt $T:ident),+) => {
        impl<$($T: Clone + AsValue + 'static),+> AsValue for ($($T,)+) {
            fn as_value(&self) -> Value {
                Value::from_tuple(($(self.$idx.as_value(),)+))
            }
        }
    };
}

impl_tuple_as_value!(0 A);
impl_tuple_as_value!(0 A, 1 B);
impl_tuple_as_value!(0 A, 1 B, 2 C);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
impl_tuple_as_value!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

#[cfg(test)]
mod tests {
    use crate::*;

    fn sample_tuple() -> (Value, Value, Value) {
        (valueof!(1_i32), valueof!(true), valueof!("hello"))
    }

    #[test]
    fn name() {
        let t = sample_tuple();
        assert_eq!(t.name(), "Tuple3");
    }

    #[test]
    fn type_id() {
        let t = sample_tuple();
        assert_eq!(t.type_id(), std::any::TypeId::of::<(Value, Value, Value)>());
    }

    #[test]
    fn len() {
        let t = sample_tuple();
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn is_empty() {
        let single = (valueof!(1_i32),);
        assert!(!single.is_empty());
        assert!(!sample_tuple().is_empty());
    }

    #[test]
    fn items() {
        let t = sample_tuple();
        let items: Vec<_> = t.items().collect();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].as_value().to_i32(), 1);
        assert_eq!(items[1].as_value().to_bool(), true);
        assert_eq!(items[2].as_value().as_str(), "hello");
    }

    #[test]
    fn index() {
        let t = sample_tuple();
        let v = t.index(0).unwrap();
        assert_eq!(v.as_value().to_i32(), 1);

        let v = t.index(2).unwrap();
        assert_eq!(v.as_value().as_str(), "hello");

        assert!(t.index(99).is_none());
    }

    #[test]
    fn as_value() {
        let t = sample_tuple();
        let v = t.as_value();
        assert!(v.is_object());
        assert!(v.is_tuple());
    }

    #[test]
    fn tuple_i32_as_value() {
        let v = (1i32,).as_value();
        assert!(v.is_tuple());
        let t = v.as_tuple();
        assert_eq!(t.len(), 1);
        assert_eq!(t.index(0).unwrap().as_value().to_i32(), 1);
    }

    #[test]
    fn tuple_mixed_as_value() {
        let v = (1i32, true, String::from("hi")).as_value();
        assert!(v.is_tuple());
        let t = v.as_tuple();
        assert_eq!(t.len(), 3);
        assert_eq!(t.index(0).unwrap().as_value().to_i32(), 1);
        assert_eq!(t.index(1).unwrap().as_value().to_bool(), true);
        assert_eq!(t.index(2).unwrap().as_value().as_str(), "hi");
    }

    #[test]
    fn tuple_nested_as_value() {
        let v = (vec![1i32, 2], true).as_value();
        assert!(v.is_tuple());
        let t = v.as_tuple();
        assert_eq!(t.len(), 2);
        let inner = t.index(0).unwrap().as_value();
        assert!(inner.is_array());
        assert_eq!(inner.as_array().len(), 2);
        assert_eq!(t.index(1).unwrap().as_value().to_bool(), true);
    }
}
