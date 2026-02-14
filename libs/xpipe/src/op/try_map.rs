use crate::{Operator, Pipe, Task};

pub struct TryMap<Input, Output>(Box<dyn FnOnce(Input) -> crate::Result<Output> + Send>);

impl<Input, Output> TryMap<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Input) -> crate::Result<Output> + Send + 'static,
    {
        Self(Box::new(f))
    }
}

impl<Input, Output> Operator<Input> for TryMap<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    type Output = crate::Result<Output>;

    fn apply(self, task: Task<Input>) -> Task<Self::Output> {
        Task::from_lazy(move || (self.0)(task.eval()))
    }
}

pub trait TryMapPipe<T>: Pipe<T> + Sized
where
    T: Send + 'static,
{
    fn try_map<O, F>(self, f: F) -> Task<crate::Result<O>>
    where
        O: Send + 'static,
        F: FnOnce(T) -> crate::Result<O> + Send + 'static,
    {
        self.pipe(TryMap::new(f))
    }
}

impl<T: Send + 'static, P: Pipe<T> + Sized> TryMapPipe<T> for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TaskError;

    #[test]
    fn success() {
        let result = Task::from("42".to_string())
            .pipe(TryMap::new(|s: String| {
                s.parse::<i32>().map_err(|e| TaskError::new(e.to_string()))
            }))
            .eval();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn failure() {
        let result = Task::from("not_a_number".to_string())
            .pipe(TryMap::new(|s: String| {
                s.parse::<i32>().map_err(|e| TaskError::new(e.to_string()))
            }))
            .eval();
        assert!(result.is_err());
    }

    #[test]
    fn with_custom_error() {
        let result = Task::from(10)
            .pipe(TryMap::new(|x: i32| {
                if x > 5 {
                    Ok(x * 2)
                } else {
                    Err(TaskError::new("value too small"))
                }
            }))
            .eval();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 20);
    }

    #[test]
    fn with_custom_error_failure() {
        let result = Task::from(3)
            .pipe(TryMap::new(|x: i32| {
                if x > 5 {
                    Ok(x * 2)
                } else {
                    Err(TaskError::new("value too small"))
                }
            }))
            .eval();
        assert!(result.is_err());
    }

    #[test]
    fn try_map_pipe_trait() {
        let result = Task::from("42".to_string())
            .try_map(|s| s.parse::<i32>().map_err(|e| TaskError::new(e.to_string())))
            .eval();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }
}
