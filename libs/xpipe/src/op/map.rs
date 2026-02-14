use crate::{Operator, Pipe, Task};

pub struct Map<Input, Output>(Box<dyn FnOnce(Input) -> Output + Send>);

impl<Input, Output> Map<Input, Output> {
    pub fn new<H: FnOnce(Input) -> Output + Send + 'static>(handler: H) -> Self {
        Self(Box::new(handler))
    }
}

impl<Input: Send + 'static, Output: Send + 'static> Operator<Input> for Map<Input, Output> {
    type Output = Output;

    fn apply(self, task: Task<Input>) -> Task<Self::Output> {
        Task::from_lazy(|| (self.0)(task.eval()))
    }
}

pub trait MapPipe<T>: Pipe<T> + Sized {
    fn map<O: Send + 'static, F: FnOnce(T) -> O + Send + 'static>(self, f: F) -> Task<O>
    where
        T: Send + 'static,
    {
        self.pipe(Map(Box::new(f)))
    }
}

impl<T, P: Pipe<T> + Sized> MapPipe<T> for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pipe;
    use crate::task;

    #[test]
    fn transforms_value() {
        let result = task!(5).pipe(Map::new(|x: i32| x * 2)).eval();
        assert_eq!(result, 10);
    }

    #[test]
    fn changes_type() {
        let result = task!(42).pipe(Map::new(|x: i32| x.to_string())).eval();
        assert_eq!(result, "42");
    }

    #[test]
    fn with_closure() {
        let multiplier = 3;
        let result = task!(7).pipe(Map::new(move |x: i32| x * multiplier)).eval();
        assert_eq!(result, 21);
    }

    #[test]
    fn chained() {
        let result = task!(2)
            .map(|x| x + 1)
            .map(|x| x * 2)
            .map(|x| x.to_string())
            .eval();
        assert_eq!(result, "6");
    }

    #[test]
    fn map_pipe_trait() {
        let result = task!(10).map(|x| x * 3).eval();
        assert_eq!(result, 30);
    }
}
