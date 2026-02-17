use crate::rules::RuleRegistry;

#[derive(Default, Clone)]
pub struct AnySchema(RuleRegistry);

impl AnySchema {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct AnySchemaBuilder {}
