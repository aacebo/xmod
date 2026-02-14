use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};

use crate::{Operator, Pipe, Task};

struct State<T> {
    result: Option<T>,
    waker: Option<Waker>,
}

pub struct AsyncTask<T>(Arc<(Mutex<State<T>>, Condvar)>);

impl<T: Send + 'static> AsyncTask<T> {
    pub(crate) fn spawn(task: Task<T>) -> Self {
        let shared = Arc::new((
            Mutex::new(State {
                result: None,
                waker: None,
            }),
            Condvar::new(),
        ));

        let shared_clone = shared.clone();

        std::thread::spawn(move || {
            let value = task.eval();
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
                return value;
            }

            state = cvar.wait(state).unwrap();
        }
    }

    pub fn join(self) -> Task<T> {
        Task::from_lazy(move || self.eval())
    }
}

impl<T: Send + 'static> Future for AsyncTask<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let (lock, _) = &*self.0;
        let mut state = lock.lock().unwrap();

        if let Some(value) = state.result.take() {
            Poll::Ready(value)
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl<T: Send + 'static> Pipe<T> for AsyncTask<T> {
    fn pipe<Op: Operator<T>>(self, op: Op) -> Task<Op::Output> {
        op.apply(self.join())
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
        let spawned_id = task!(move || std::thread::current().id()).fork().eval();
        assert_ne!(main_id, spawned_id);
    }

    #[test]
    fn fork_enables_parallelism() {
        let start = std::time::Instant::now();
        let a = task!(|| {
            std::thread::sleep(std::time::Duration::from_millis(50));
            1
        })
        .fork();
        let b = task!(|| {
            std::thread::sleep(std::time::Duration::from_millis(50));
            2
        })
        .fork();

        let result = a.eval() + b.eval();
        assert_eq!(result, 3);
        assert!(start.elapsed() < std::time::Duration::from_millis(150));
    }

    #[test]
    fn pipe_operators_on_async_task() {
        let result = task!(10).fork().map(|x| x * 2).eval();
        assert_eq!(result, 20);
    }

    #[test]
    fn future_impl() {
        use std::task::{RawWaker, RawWakerVTable};

        fn noop_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> RawWaker {
                RawWaker::new(p, &VTABLE)
            }

            const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        let mut task = task!(42).fork();
        std::thread::sleep(std::time::Duration::from_millis(50));

        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut cx = Context::from_waker(&waker);
        let result = Pin::new(&mut task).poll(&mut cx);
        assert!(matches!(result, Poll::Ready(42)));
    }
}
