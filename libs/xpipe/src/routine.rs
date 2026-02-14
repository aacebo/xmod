pub struct Routine<In, Out>(Box<dyn Fn(In) -> Out>);

impl<In, Out> Routine<In, Out> {
    pub fn new(routine: impl Fn(In) -> Out + 'static) -> Self {
        Self(Box::new(routine))
    }

    pub fn eval(self, input: In) -> Out {
        self.0(input)
    }
}

impl<In: 'static, Out: 'static> std::ops::Deref for Routine<In, Out> {
    type Target = dyn Fn(In) -> Out;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
