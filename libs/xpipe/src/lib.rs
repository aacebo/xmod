mod error;
pub mod op;
mod routine;
mod task;

pub use error::*;
pub use routine::*;
pub use task::*;

pub trait Pipe<Input> {
    fn pipe<Op: Operator<Input>>(self, op: Op) -> Task<Op::Output>;
}

pub trait Operator<Input> {
    type Output;

    fn apply(self, task: Task<Input>) -> Task<Self::Output>;
}

#[macro_export]
macro_rules! task {
    (async move () => $($body:tt)+) => {{
        use $crate::op::ForkPipe;
        $crate::Task::from_lazy(move || $($body)+).fork()
    }};
    (async () => $($body:tt)+) => {{
        use $crate::op::ForkPipe;
        $crate::Task::from_lazy(|| $($body)+).fork()
    }};
    (move () => $($body:tt)+) => {
        $crate::Task::from_lazy(move || $($body)+)
    };
    (() => $($body:tt)+) => {
        $crate::Task::from_lazy(|| $($body)+)
    };
    (($($args:tt)*) => $($body:tt)+) => {
        $crate::Routine::new(|$($args)*| $($body)+)
    };
    ($input:literal) => {
        $crate::Task::from_static($input)
    };
    ($input:expr) => {
        $crate::Task::from_static($input)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn task_literal() {
        let result = task!(42).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn task_string_literal() {
        let result = task!("hello").eval();
        assert_eq!(result, "hello");
    }

    #[test]
    fn task_lazy() {
        let result = task!(() => 1 + 2).eval();
        assert_eq!(result, 3);
    }

    #[test]
    fn task_move_lazy() {
        let x = 10;
        let result = task!(move () => x * 2).eval();
        assert_eq!(result, 20);
    }

    #[test]
    fn task_expr() {
        let val = 99;
        let result = task!(val).eval();
        assert_eq!(result, 99);
    }

    #[test]
    fn task_field_access() {
        struct Obj {
            val: i32,
        }
        let obj = Obj { val: 7 };
        let result = task!(obj.val).eval();
        assert_eq!(result, 7);
    }

    #[test]
    fn task_async() {
        let result = task!(async () => 42).eval();
        assert_eq!(result, 42);
    }

    #[test]
    fn task_async_move() {
        let x = 5;
        let result = task!(async move () => x * 3).eval();
        assert_eq!(result, 15);
    }

    #[test]
    fn task_async_runs_on_different_thread() {
        let main_id = std::thread::current().id();
        let spawned_id = task!(async () => std::thread::current().id()).eval();
        assert_ne!(main_id, spawned_id);
    }

    #[test]
    fn task_routine() {
        let result = task!((x) => x + 1).eval(1);
        assert_eq!(result, 2);
    }
}
