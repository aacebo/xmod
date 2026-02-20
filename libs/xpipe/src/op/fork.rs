use std::any::Any;
use std::future::Future;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};

use crate::{Operator, Pipe, Task};

struct State<T> {
    result: Option<Result<T, Box<dyn Any + Send>>>,
    waker: Option<Waker>,
}

#[must_use]
pub struct ForkHandle<T>(Arc<(Mutex<State<T>>, Condvar)>);

impl<T: Send + 'static> ForkHandle<T> {
    fn spawn(task: Task<T>) -> Self {
        let shared = Arc::new((
            Mutex::new(State {
                result: None,
                waker: None,
            }),
            Condvar::new(),
        ));

        let shared_clone = shared.clone();

        std::thread::spawn(move || {
            let value = catch_unwind(AssertUnwindSafe(|| task.eval()));
            let (lock, cvar) = &*shared_clone;
            let mut state = lock.lock().unwrap();

            state.result = Some(value);

            if let Some(waker) = state.waker.take() {
                waker.wake();
            }

            cvar.notify_all();
        });

        Self(shared)
    }

    pub fn eval(self) -> T {
        let (lock, cvar) = &*self.0;
        let mut state = lock.lock().unwrap();

        loop {
            if let Some(value) = state.result.take() {
                match value {
                    Ok(v) => return v,
                    Err(payload) => resume_unwind(payload),
                }
            }

            state = cvar.wait(state).unwrap();
        }
    }

    pub fn join(self) -> Task<T> {
        Task::from_lazy(move || self.eval())
    }
}

impl<T: Send + 'static> Future for ForkHandle<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let (lock, _) = &*self.0;
        let mut state = lock.lock().unwrap();

        if let Some(value) = state.result.take() {
            match value {
                Ok(v) => Poll::Ready(v),
                Err(payload) => resume_unwind(payload),
            }
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl<T: Send + 'static> Pipe<T> for ForkHandle<T> {
    fn pipe<Op: Operator<T>>(self, op: Op) -> Task<Op::Output> {
        op.apply(self.join())
    }
}

pub trait ForkPipe<T: Send + 'static> {
    fn fork(self) -> ForkHandle<T>;
}

impl<T: Send + 'static> ForkPipe<T> for Task<T> {
    fn fork(self) -> ForkHandle<T> {
        ForkHandle::spawn(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::op::MapPipe;
    use crate::task;

    #[test]
    fn fork_and_eval() {
        let result = task!(42).fork().eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn fork_and_join() {
        let result = task!(42).fork().join().eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn fork_runs_on_different_thread() {
        let main_id = std::thread::current().id();
        let spawned_id = task!(move () => std::thread::current().id()).fork().eval();
        assert_ne!(main_id, spawned_id);
    }

    #[test]
    fn fork_enables_parallelism() {
        let start = std::time::Instant::now();
        let a = task!(() => {
            std::thread::sleep(std::time::Duration::from_millis(50));
            1
        })
        .fork();
        let b = task!(() => {
            std::thread::sleep(std::time::Duration::from_millis(50));
            2
        })
        .fork();

        let result = a.eval() + b.eval();
        assert_eq!(result, 3);
        assert!(start.elapsed() < std::time::Duration::from_millis(150));
    }

    #[test]
    fn pipe_operators_on_fork_handle() {
        let result = task!(10).fork().map(|x| x * 2).eval();
        assert_eq!(result, 20);
    }

    #[tokio::test]
    async fn future_impl() {
        let result = task!(42).fork().await;
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic(expected = "task panicked")]
    fn fork_propagates_panic() {
        Task::from_lazy(|| -> i32 { panic!("task panicked") })
            .fork()
            .eval();
    }
}
