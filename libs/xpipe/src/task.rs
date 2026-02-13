use crate::{Operator, Pipe};

pub enum Task<T> {
    Static(T),
    Lazy(Box<dyn Fn() -> T + Send>),
}

impl<T> Task<T> {
    pub fn from_static(value: T) -> Self {
        Self::Static(value)
    }

    pub fn from_lazy<H: Fn() -> T + Send + 'static>(handler: H) -> Self {
        Self::Lazy(Box::new(handler))
    }

    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    pub fn is_lazy(&self) -> bool {
        matches!(self, Self::Lazy(_))
    }

    pub fn eval(self) -> T {
        match self {
            Self::Static(v) => v,
            Self::Lazy(v) => v(),
        }
    }
}

impl<T> From<T> for Task<T> {
    fn from(value: T) -> Self {
        Self::Static(value)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(v) => write!(
                f,
                "Task<{}>::Static({:#?})",
                std::any::type_name_of_val(v),
                v
            ),
            Self::Lazy(_) => write!(f, "Task<{}>::Lazy", std::any::type_name::<T>()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(v) => write!(f, "Task<{}>::Static({})", std::any::type_name_of_val(v), v),
            Self::Lazy(_) => write!(f, "Task<{}>::Lazy", std::any::type_name::<T>()),
        }
    }
}

impl<T: 'static> Pipe<T> for Task<T> {
    fn pipe<Op: Operator<T>>(self, op: Op) -> Task<Op::Output> {
        op.apply(self)
    }
}
