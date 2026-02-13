pub trait Pipe<Input> {
    type Output;

    fn pipe(self, input: Input) -> Self::Output;
}
