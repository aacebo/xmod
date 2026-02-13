mod error;
pub mod op;
mod task;

pub use error::*;

pub use task::*;

pub trait Pipe<Input> {
    fn pipe<Op: Operator<Input>>(self, op: Op) -> Task<Op::Output>;
}

pub trait Operator<Input> {
    type Output;

    fn apply(self, task: Task<Input>) -> Task<Self::Output>;
}
