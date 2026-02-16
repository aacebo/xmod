use crate::{Validate, rules::RuleRegistry};

#[derive(Debug, Default, Clone)]
pub struct Required;

impl Required {
    pub fn new() -> Self {
        Self
    }
}

impl Validate for Required {
    fn validate(&self, input: &xval::Value) -> Result<xval::Value, String> {
        if input.is_null() {
            return Err("required".to_string());
        }

        Ok(input.clone())
    }
}

impl RuleRegistry {
    pub fn required(&mut self) -> &mut Self {
        self.register("required", Required::new())
    }
}
