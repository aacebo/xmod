use crate::{Operator, Pipe, Task};

pub struct ResultFlatMap<Input, Output, E>(
    Box<dyn FnOnce(Input) -> Task<Result<Output, E>> + Send>,
);

impl<Input, Output, E> ResultFlatMap<Input, Output, E> {
    pub fn new<F>(handler: F) -> Self
    where
        F: FnOnce(Input) -> Task<Result<Output, E>> + Send + 'static,
    {
        Self(Box::new(handler))
    }
}

impl<Input, Output, E> Operator<Result<Input, E>> for ResultFlatMap<Input, Output, E>
where
    Input: Send + 'static,
    Output: Send + 'static,
    E: Send + 'static,
{
    type Output = Result<Output, E>;

    fn apply(self, task: Task<Result<Input, E>>) -> Task<Self::Output> {
        Task::from_lazy(move || match task.eval() {
            Ok(value) => (self.0)(value).eval(),
            Err(e) => Err(e),
        })
    }
}

pub struct OptionFlatMap<Input, Output>(Box<dyn FnOnce(Input) -> Task<Option<Output>> + Send>);

impl<Input, Output> OptionFlatMap<Input, Output> {
    pub fn new<F>(handler: F) -> Self
    where
        F: FnOnce(Input) -> Task<Option<Output>> + Send + 'static,
    {
        Self(Box::new(handler))
    }
}

impl<Input, Output> Operator<Option<Input>> for OptionFlatMap<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    type Output = Option<Output>;

    fn apply(self, task: Task<Option<Input>>) -> Task<Self::Output> {
        Task::from_lazy(move || match task.eval() {
            Some(value) => (self.0)(value).eval(),
            None => None,
        })
    }
}

pub trait ResultFlatMapPipe<T, E>: Pipe<Result<T, E>> + Sized
where
    T: Send + 'static,
    E: Send + 'static,
{
    fn and_then<O, F>(self, handler: F) -> Task<Result<O, E>>
    where
        O: Send + 'static,
        F: FnOnce(T) -> Task<Result<O, E>> + Send + 'static,
    {
        self.pipe(ResultFlatMap::new(handler))
    }
}

impl<T, E, P> ResultFlatMapPipe<T, E> for P
where
    T: Send + 'static,
    E: Send + 'static,
    P: Pipe<Result<T, E>> + Sized,
{
}

pub trait OptionFlatMapPipe<T>: Pipe<Option<T>> + Sized
where
    T: Send + 'static,
{
    fn and_then<O, F>(self, handler: F) -> Task<Option<O>>
    where
        O: Send + 'static,
        F: FnOnce(T) -> Task<Option<O>> + Send + 'static,
    {
        self.pipe(OptionFlatMap::new(handler))
    }
}

impl<T, P> OptionFlatMapPipe<T> for P
where
    T: Send + 'static,
    P: Pipe<Option<T>> + Sized,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task;

    #[test]
    fn result_and_then_ok_to_ok() {
        let result: Result<String, &str> = task!(Ok::<i32, &str>(42))
            .and_then(|x| task!(Ok(x.to_string())))
            .eval();
        assert_eq!(result, Ok("42".to_string()));
    }

    #[test]
    fn result_and_then_ok_to_err() {
        let result: Result<String, &str> = task!(Ok::<i32, &str>(42))
            .and_then(|_| task!(Err("failed")))
            .eval();
        assert_eq!(result, Err("failed"));
    }

    #[test]
    fn result_and_then_err_short_circuits() {
        let result: Result<String, &str> = task!(Err::<i32, &str>("early"))
            .and_then(|x| task!(Ok(x.to_string())))
            .eval();
        assert_eq!(result, Err("early"));
    }

    #[test]
    fn option_and_then_some_to_some() {
        let result = task!(Some(42))
            .and_then(|x| task!(Some(x.to_string())))
            .eval();
        assert_eq!(result, Some("42".to_string()));
    }

    #[test]
    fn option_and_then_some_to_none() {
        let result = task!(Some(42)).and_then(|_| task!(None::<String>)).eval();
        assert_eq!(result, None);
    }

    #[test]
    fn option_and_then_none_short_circuits() {
        let result = task!(None::<i32>)
            .and_then(|x| task!(Some(x.to_string())))
            .eval();
        assert_eq!(result, None);
    }
}
