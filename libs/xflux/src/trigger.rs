use crate::Action;

pub trait Trigger: Send + Sync {
    fn subscribe(&mut self, action: Box<dyn Action>);
}
