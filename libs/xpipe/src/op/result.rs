use std::marker::PhantomData;
use std::time::Duration;

use crate::{Operator, Pipe, Task};

pub struct Retry<Input, Output, E, F>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
    E: Send + 'static,
{
    operation: F,
    max_attempts: usize,
    initial_delay: Duration,
    backoff_multiplier: f64,
    _marker: PhantomData<fn(Input) -> Result<Output, E>>,
}

impl<Input, Output, E, F> Retry<Input, Output, E, F>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
    E: Send + 'static,
    F: Fn(Input) -> Result<Output, E> + Send + 'static,
{
    pub fn new(
        operation: F,
        max_attempts: usize,
        initial_delay: Duration,
        backoff_multiplier: f64,
    ) -> Self {
        Self {
            operation,
            max_attempts,
            initial_delay,
            backoff_multiplier,
            _marker: PhantomData,
        }
    }
}

impl<Input, Output, E, F> Operator<Input> for Retry<Input, Output, E, F>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
    E: Send + 'static,
    F: Fn(Input) -> Result<Output, E> + Send + 'static,
{
    type Output = Result<Output, E>;

    fn apply(self, task: Task<Input>) -> Task<Self::Output> {
        Task::from_lazy(move || {
            let input = task.eval();
            let mut attempts = 0;
            let mut delay = self.initial_delay;

            loop {
                match (self.operation)(input.clone()) {
                    Ok(v) => return Ok(v),
                    Err(_) if attempts < self.max_attempts => {
                        attempts += 1;
                        std::thread::sleep(delay);
                        delay =
                            Duration::from_secs_f64(delay.as_secs_f64() * self.backoff_multiplier);
                    }
                    Err(e) => return Err(e),
                }
            }
        })
    }
}

pub trait RetryPipe<T>: Pipe<T> + Sized
where
    T: Clone + Send + 'static,
{
    fn retry<O, E>(self) -> RetryBuilder<T, O, E, Self>
    where
        O: Send + 'static,
        E: Send + 'static,
    {
        RetryBuilder::new(self)
    }
}

impl<T: Clone + Send + 'static, P: Pipe<T> + Sized> RetryPipe<T> for P {}

pub struct RetryBuilder<Input, Output, E, P> {
    source: P,
    max_attempts: usize,
    initial_delay: Duration,
    backoff_multiplier: f64,
    _marker: PhantomData<(Input, Output, E)>,
}

impl<Input, Output, E, P> RetryBuilder<Input, Output, E, P>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
    E: Send + 'static,
    P: Pipe<Input>,
{
    fn new(source: P) -> Self {
        Self {
            source,
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            _marker: PhantomData,
        }
    }

    pub fn attempts(mut self, n: usize) -> Self {
        self.max_attempts = n;
        self
    }

    pub fn delay(mut self, d: Duration) -> Self {
        self.initial_delay = d;
        self
    }

    pub fn backoff(mut self, m: f64) -> Self {
        self.backoff_multiplier = m;
        self
    }

    pub fn run<F>(self, operation: F) -> Task<Result<Output, E>>
    where
        F: Fn(Input) -> Result<Output, E> + Send + 'static,
    {
        self.source.pipe(Retry::new(
            operation,
            self.max_attempts,
            self.initial_delay,
            self.backoff_multiplier,
        ))
    }
}

pub struct Unwrap;

impl<T, E> Operator<Result<T, E>> for Unwrap
where
    T: Send + 'static,
    E: std::fmt::Debug + Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Result<T, E>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().unwrap())
    }
}

pub struct Expect {
    message: &'static str,
}

impl Expect {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl<T, E> Operator<Result<T, E>> for Expect
where
    T: Send + 'static,
    E: std::fmt::Debug + Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Result<T, E>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().expect(self.message))
    }
}

pub struct UnwrapOr<T> {
    default: T,
}

impl<T> UnwrapOr<T> {
    pub fn new(default: T) -> Self {
        Self { default }
    }
}

impl<T, E> Operator<Result<T, E>> for UnwrapOr<T>
where
    T: Send + 'static,
    E: Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Result<T, E>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().unwrap_or(self.default))
    }
}

pub struct UnwrapOrElse<F> {
    default_fn: F,
}

impl<F> UnwrapOrElse<F> {
    pub fn new(default_fn: F) -> Self {
        Self { default_fn }
    }
}

impl<T, E, F> Operator<Result<T, E>> for UnwrapOrElse<F>
where
    T: Send + 'static,
    E: Send + 'static,
    F: FnOnce(E) -> T + Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Result<T, E>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().unwrap_or_else(self.default_fn))
    }
}

pub struct ResultOk;

impl<T, E> Operator<Result<T, E>> for ResultOk
where
    T: Send + 'static,
    E: Send + 'static,
{
    type Output = Option<T>;

    fn apply(self, task: Task<Result<T, E>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().ok())
    }
}

pub trait ResultPipe<T, E>: Pipe<Result<T, E>> + Sized
where
    T: Send + 'static,
    E: Send + 'static,
{
    fn unwrap(self) -> Task<T>
    where
        E: std::fmt::Debug,
    {
        self.pipe(Unwrap)
    }

    fn expect(self, message: &'static str) -> Task<T>
    where
        E: std::fmt::Debug,
    {
        self.pipe(Expect::new(message))
    }

    fn unwrap_or(self, default: T) -> Task<T> {
        self.pipe(UnwrapOr::new(default))
    }

    fn unwrap_or_else<F>(self, f: F) -> Task<T>
    where
        F: FnOnce(E) -> T + Send + 'static,
    {
        self.pipe(UnwrapOrElse::new(f))
    }

    fn ok(self) -> Task<Option<T>> {
        self.pipe(ResultOk)
    }
}

impl<T, E, P> ResultPipe<T, E> for P
where
    T: Send + 'static,
    E: Send + 'static,
    P: Pipe<Result<T, E>> + Sized,
{
}

pub struct OptionUnwrap;

impl<T> Operator<Option<T>> for OptionUnwrap
where
    T: Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Option<T>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().unwrap())
    }
}

pub struct OptionExpect {
    message: &'static str,
}

impl OptionExpect {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl<T> Operator<Option<T>> for OptionExpect
where
    T: Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Option<T>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().expect(self.message))
    }
}

pub struct OptionUnwrapOr<T> {
    default: T,
}

impl<T> OptionUnwrapOr<T> {
    pub fn new(default: T) -> Self {
        Self { default }
    }
}

impl<T> Operator<Option<T>> for OptionUnwrapOr<T>
where
    T: Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Option<T>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().unwrap_or(self.default))
    }
}

pub struct OptionUnwrapOrElse<F> {
    default_fn: F,
}

impl<F> OptionUnwrapOrElse<F> {
    pub fn new(default_fn: F) -> Self {
        Self { default_fn }
    }
}

impl<T, F> Operator<Option<T>> for OptionUnwrapOrElse<F>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<Option<T>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().unwrap_or_else(self.default_fn))
    }
}

pub struct OptionOkOr<E> {
    error: E,
}

impl<E> OptionOkOr<E> {
    pub fn new(error: E) -> Self {
        Self { error }
    }
}

impl<T, E> Operator<Option<T>> for OptionOkOr<E>
where
    T: Send + 'static,
    E: Send + 'static,
{
    type Output = Result<T, E>;

    fn apply(self, task: Task<Option<T>>) -> Task<Self::Output> {
        Task::from_lazy(move || task.eval().ok_or(self.error))
    }
}

pub trait OptionPipe<T>: Pipe<Option<T>> + Sized
where
    T: Send + 'static,
{
    fn unwrap(self) -> Task<T> {
        self.pipe(OptionUnwrap)
    }

    fn expect(self, message: &'static str) -> Task<T> {
        self.pipe(OptionExpect::new(message))
    }

    fn unwrap_or(self, default: T) -> Task<T> {
        self.pipe(OptionUnwrapOr::new(default))
    }

    fn unwrap_or_else<F>(self, f: F) -> Task<T>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        self.pipe(OptionUnwrapOrElse::new(f))
    }

    fn ok_or<E>(self, error: E) -> Task<Result<T, E>>
    where
        E: Send + 'static,
    {
        self.pipe(OptionOkOr::new(error))
    }
}

impl<T, P> OptionPipe<T> for P
where
    T: Send + 'static,
    P: Pipe<Option<T>> + Sized,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn retry_succeeds_first_try() {
        let result: Result<i32, &str> = task!(10).retry().attempts(3).run(|x| Ok(x * 2)).eval();

        assert_eq!(result, Ok(20));
    }

    #[test]
    fn retry_succeeds_after_failures() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        let result: Result<i32, &str> = task!(10)
            .retry()
            .attempts(3)
            .delay(Duration::from_millis(1))
            .run(move |x| {
                let count = counter_clone.fetch_add(1, Ordering::SeqCst);
                if count < 2 { Err("not yet") } else { Ok(x * 2) }
            })
            .eval();

        assert_eq!(result, Ok(20));
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn retry_exhausts_attempts() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        let result: Result<i32, &str> = task!(10)
            .retry()
            .attempts(2)
            .delay(Duration::from_millis(1))
            .run(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Err("always fails")
            })
            .eval();

        assert_eq!(result, Err("always fails"));
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn result_unwrap_ok() {
        let result = task!(Ok::<i32, &str>(42)).unwrap().eval();
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic]
    fn result_unwrap_err_panics() {
        let _ = task!(Err::<i32, &str>("error")).unwrap().eval();
    }

    #[test]
    fn result_expect_ok() {
        let result = task!(Ok::<i32, &str>(42)).expect("should not fail").eval();
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic(expected = "custom message")]
    fn result_expect_err_panics_with_message() {
        let _ = task!(Err::<i32, &str>("error"))
            .expect("custom message")
            .eval();
    }

    #[test]
    fn result_unwrap_or_ok() {
        let result = task!(Ok::<i32, &str>(42)).unwrap_or(0).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn result_unwrap_or_err() {
        let result = task!(Err::<i32, &str>("error")).unwrap_or(0).eval();
        assert_eq!(result, 0);
    }

    #[test]
    fn result_unwrap_or_else_ok() {
        let result = task!(Ok::<i32, &str>(42)).unwrap_or_else(|_| 0).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn result_unwrap_or_else_err() {
        let result = task!(Err::<i32, &str>("error"))
            .unwrap_or_else(|e| e.len() as i32)
            .eval();
        assert_eq!(result, 5);
    }

    #[test]
    fn result_ok_some() {
        let result = task!(Ok::<i32, &str>(42)).ok().eval();
        assert_eq!(result, Some(42));
    }

    #[test]
    fn result_ok_none() {
        let result = task!(Err::<i32, &str>("error")).ok().eval();
        assert_eq!(result, None);
    }

    #[test]
    fn option_unwrap_some() {
        let result = task!(Some(42)).unwrap().eval();
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic]
    fn option_unwrap_none_panics() {
        let _ = task!(None::<i32>).unwrap().eval();
    }

    #[test]
    fn option_expect_some() {
        let result = task!(Some(42)).expect("should not fail").eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn option_unwrap_or_some() {
        let result = task!(Some(42)).unwrap_or(0).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn option_unwrap_or_none() {
        let result = task!(None::<i32>).unwrap_or(0).eval();
        assert_eq!(result, 0);
    }

    #[test]
    fn option_unwrap_or_else_some() {
        let result = task!(Some(42)).unwrap_or_else(|| 0).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn option_unwrap_or_else_none() {
        let result = task!(None::<i32>).unwrap_or_else(|| 100).eval();
        assert_eq!(result, 100);
    }

    #[test]
    fn option_ok_or_some() {
        let result: Result<i32, &str> = task!(Some(42)).ok_or("missing").eval();
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn option_ok_or_none() {
        let result: Result<i32, &str> = task!(None::<i32>).ok_or("missing").eval();
        assert_eq!(result, Err("missing"));
    }
}
