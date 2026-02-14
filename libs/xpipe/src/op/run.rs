use crate::{Operator, Pipe, Task};

pub struct Run<T>(Box<dyn FnOnce(&T) + Send>);

impl<T> Run<T> {
    pub fn new<F: FnOnce(&T) + Send + 'static>(handler: F) -> Self {
        Self(Box::new(handler))
    }
}

impl<T: Send + 'static> Operator<T> for Run<T> {
    type Output = T;

    fn apply(self, task: Task<T>) -> Task<Self::Output> {
        Task::from_lazy(move || {
            let value = task.eval();
            (self.0)(&value);
            value
        })
    }
}

pub trait RunPipe<T>: Pipe<T> + Sized {
    fn run<F: FnOnce(&T) + Send + 'static>(self, f: F) -> Task<T>
    where
        T: Send + 'static,
    {
        self.pipe(Run::new(f))
    }
}

impl<T, P: Pipe<T> + Sized> RunPipe<T> for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::op::map::MapPipe;
    use crate::task;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn passes_value_through() {
        let result = task!(42).pipe(Run::new(|_| {})).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn executes_side_effect() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let result = task!(42)
            .pipe(Run::new(move |_| {
                called_clone.store(true, Ordering::SeqCst);
            }))
            .eval();

        assert_eq!(result, 42);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn receives_correct_value() {
        let result = task!(42).run(|x| assert_eq!(*x, 42)).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn chained_with_map() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let result = task!(10)
            .map(|x| x * 2)
            .run(move |x| {
                assert_eq!(*x, 20);
                called_clone.store(true, Ordering::SeqCst);
            })
            .map(|x| x + 1)
            .eval();

        assert_eq!(result, 21);
        assert!(called.load(Ordering::SeqCst));
    }
}
