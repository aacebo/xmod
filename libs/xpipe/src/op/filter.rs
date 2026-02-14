use crate::{Operator, Pipe, Task};

pub struct Filter<T> {
    predicate: Box<dyn Fn(&T) -> bool + Send>,
}

impl<T> Filter<T>
where
    T: Send + 'static,
{
    pub fn new<P>(predicate: P) -> Self
    where
        P: Fn(&T) -> bool + Send + 'static,
    {
        Self {
            predicate: Box::new(predicate),
        }
    }

    pub fn allow<P>(predicate: P) -> Self
    where
        P: Fn(&T) -> bool + Send + 'static,
    {
        Self::new(predicate)
    }

    pub fn block<P>(predicate: P) -> Self
    where
        P: Fn(&T) -> bool + Send + 'static,
    {
        Self::new(move |x| !predicate(x))
    }
}

impl<T> Operator<T> for Filter<T>
where
    T: Send + 'static,
{
    type Output = Option<T>;

    fn apply(self, task: Task<T>) -> Task<Self::Output> {
        Task::from_lazy(move || {
            let input = task.eval();
            if (self.predicate)(&input) {
                Some(input)
            } else {
                None
            }
        })
    }
}

pub trait FilterPipe<T>: Pipe<T> + Sized
where
    T: Send + 'static,
{
    fn filter<P>(self, predicate: P) -> Task<Option<T>>
    where
        P: Fn(&T) -> bool + Send + 'static,
    {
        self.pipe(Filter::new(predicate))
    }

    fn filter_allow<P>(self, predicate: P) -> Task<Option<T>>
    where
        P: Fn(&T) -> bool + Send + 'static,
    {
        self.pipe(Filter::allow(predicate))
    }

    fn filter_block<P>(self, predicate: P) -> Task<Option<T>>
    where
        P: Fn(&T) -> bool + Send + 'static,
    {
        self.pipe(Filter::block(predicate))
    }
}

impl<T: Send + 'static, P: Pipe<T> + Sized> FilterPipe<T> for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task;

    #[test]
    fn allows_matching_single_value() {
        let result = task!(42).pipe(Filter::allow(|x| *x > 0)).eval();
        assert_eq!(result, Some(42));
    }

    #[test]
    fn blocks_non_matching_single_value() {
        let result = task!(-5).pipe(Filter::allow(|x| *x > 0)).eval();
        assert_eq!(result, None);
    }

    #[test]
    fn block_blocks_matching_value() {
        let result = task!(42).pipe(Filter::block(|x| *x > 0)).eval();
        assert_eq!(result, None);
    }

    #[test]
    fn block_allows_non_matching_value() {
        let result = task!(-5).pipe(Filter::block(|x| *x > 0)).eval();
        assert_eq!(result, Some(-5));
    }

    #[test]
    fn filter_pipe_trait_single() {
        let result = task!(42).filter(|x| *x > 0).eval();
        assert_eq!(result, Some(42));
    }

    #[test]
    fn filter_allow_pipe_trait() {
        let result = task!(42).filter_allow(|x| *x > 0).eval();
        assert_eq!(result, Some(42));
    }

    #[test]
    fn filter_block_pipe_trait() {
        let result = task!(42).filter_block(|x| *x > 0).eval();
        assert_eq!(result, None);
    }
}
