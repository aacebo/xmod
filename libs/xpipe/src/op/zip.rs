use crate::{Operator, Pipe, Task};

pub struct Zip<U>(Task<U>);

impl<U> Zip<U> {
    pub fn new(task: Task<U>) -> Self {
        Self(task)
    }
}

impl<T, U> Operator<T> for Zip<U>
where
    T: Send + 'static,
    U: Send + 'static,
{
    type Output = (T, U);

    fn apply(self, task: Task<T>) -> Task<Self::Output> {
        Task::from_lazy(move || {
            let a = task.eval();
            let b = self.0.eval();
            (a, b)
        })
    }
}

pub trait ZipPipe<T>: Pipe<T> + Sized
where
    T: Send + 'static,
{
    fn zip<U: Send + 'static>(self, other: Task<U>) -> Task<(T, U)> {
        self.pipe(Zip::new(other))
    }
}

impl<T: Send + 'static, P: Pipe<T> + Sized> ZipPipe<T> for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task;

    #[test]
    fn zip_two_tasks() {
        let result = task!(1).zip(task!(2)).eval();
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn zip_different_types() {
        let result = task!(42).zip(task!("hello")).eval();
        assert_eq!(result, (42, "hello"));
    }

    #[test]
    fn zip_chained() {
        let result = task!(1).zip(task!(2)).zip(task!(3)).eval();
        assert_eq!(result, ((1, 2), 3));
    }

    #[test]
    fn zip_with_lazy() {
        let result = task!(() => 10).zip(task!(() => 20)).eval();
        assert_eq!(result, (10, 20));
    }
}
