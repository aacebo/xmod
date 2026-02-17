use crate::{Validate, rules::RuleRegistry};

#[derive(Debug, Default, Clone)]
pub struct AnySchema(RuleRegistry);

impl AnySchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rule<Rule: Validate + 'static>(mut self, name: &str, rule: Rule) -> Self {
        self.0.register(name, rule);
        self
    }
}
