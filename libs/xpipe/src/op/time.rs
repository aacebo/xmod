use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::{Operator, Pipe, Task};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeoutError {
    pub duration: Duration,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "operation timed out after {:?}", self.duration)
    }
}

impl std::error::Error for TimeoutError {}

pub struct Timeout {
    duration: Duration,
}

impl Timeout {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<T> Operator<T> for Timeout
where
    T: Send + 'static,
{
    type Output = Result<T, TimeoutError>;

    fn apply(self, task: Task<T>) -> Task<Self::Output> {
        let duration = self.duration;
        Task::from_lazy(move || {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let result = task.eval();
                let _ = tx.send(result);
            });

            match rx.recv_timeout(duration) {
                Ok(result) => Ok(result),
                Err(_) => Err(TimeoutError { duration }),
            }
        })
    }
}

pub struct Delay {
    duration: Duration,
}

impl Delay {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<T> Operator<T> for Delay
where
    T: Send + 'static,
{
    type Output = T;

    fn apply(self, task: Task<T>) -> Task<Self::Output> {
        let duration = self.duration;
        Task::from_lazy(move || {
            thread::sleep(duration);
            task.eval()
        })
    }
}

pub trait TimePipe<T>: Pipe<T> + Sized
where
    T: Send + 'static,
{
    fn timeout(self, duration: Duration) -> Task<Result<T, TimeoutError>> {
        self.pipe(Timeout::new(duration))
    }

    fn delay(self, duration: Duration) -> Task<T> {
        self.pipe(Delay::new(duration))
    }
}

impl<T: Send + 'static, P: Pipe<T> + Sized> TimePipe<T> for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task;
    use std::time::Instant;

    #[test]
    fn timeout_succeeds_for_fast_operation() {
        let result = task!(42).pipe(Timeout::new(Duration::from_secs(1))).eval();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn timeout_fails_for_slow_operation() {
        let result = task!(|| {
            thread::sleep(Duration::from_millis(200));
            42
        })
        .pipe(Timeout::new(Duration::from_millis(50)))
        .eval();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.duration, Duration::from_millis(50));
    }

    #[test]
    fn timeout_error_display() {
        let err = TimeoutError {
            duration: Duration::from_secs(5),
        };
        assert_eq!(format!("{}", err), "operation timed out after 5s");
    }

    #[test]
    fn timeout_pipe_trait() {
        let result = task!(42).timeout(Duration::from_secs(1)).eval();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn delay_waits_before_execution() {
        let start = Instant::now();
        let delay_duration = Duration::from_millis(100);
        let result = task!(42).pipe(Delay::new(delay_duration)).eval();
        let elapsed = start.elapsed();
        
        assert!(elapsed >= delay_duration);
        assert_eq!(result, 42);
    }

    #[test]
    fn delay_preserves_value() {
        let result = task!("hello".to_string())
            .pipe(Delay::new(Duration::from_millis(10)))
            .eval();

        assert_eq!(result, "hello");
    }

    #[test]
    fn delay_pipe_trait() {
        let start = Instant::now();
        let delay_duration = Duration::from_millis(50);
        let result = task!(42).delay(delay_duration).eval();

        assert!(start.elapsed() >= delay_duration);
        assert_eq!(result, 42);
    }

    #[test]
    fn delay_then_timeout_succeeds() {
        let result = task!(42)
            .delay(Duration::from_millis(10))
            .timeout(Duration::from_secs(1))
            .eval();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn timeout_includes_delay_time() {
        let result = task!(42)
            .delay(Duration::from_millis(200))
            .timeout(Duration::from_millis(50))
            .eval();

        assert!(result.is_err());
    }
}
