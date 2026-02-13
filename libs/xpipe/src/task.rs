use std::cell::LazyCell;

use crate::{Operator, Pipe};

pub struct Task<T>(LazyCell<T, Box<dyn FnOnce() -> T + Send>>);

impl<T: Send + 'static> Task<T> {
    pub fn from_static(value: T) -> Self {
        Self(LazyCell::new(Box::new(move || value)))
    }
}

impl<T> Task<T> {
    pub fn from_lazy(factory: impl FnOnce() -> T + Send + 'static) -> Self {
        Self(LazyCell::new(Box::new(factory)))
    }

    pub fn eval(self) -> T {
        let ptr = LazyCell::force(&self.0) as *const T;
        let value = unsafe { std::ptr::read(ptr) };
        std::mem::forget(self);
        value
    }
}

impl<T> std::ops::Deref for Task<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Send + 'static> From<T> for Task<T> {
    fn from(value: T) -> Self {
        Self::from_static(value)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task<{}>({:#?})", std::any::type_name::<T>(), &**self)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task<{}>({})", std::any::type_name::<T>(), &**self)
    }
}

impl<T: 'static> Pipe<T> for Task<T> {
    fn pipe<Op: Operator<T>>(self, op: Op) -> Task<Op::Output> {
        op.apply(self)
    }
}
