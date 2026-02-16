use crate::{Validate, rules::RuleRegistry};

#[derive(Debug, Default, Clone)]
pub struct Enumerate(Vec<xval::Value>);

impl Enumerate {
    pub fn new(items: &[xval::Value]) -> Self {
        Self(items.to_vec())
    }
}

impl Validate for Enumerate {
    fn validate(&self, input: &xval::Value) -> Result<xval::Value, String> {
        for option in &self.0 {
            if input == option {
                break;
            }
        }

        Ok(input.clone())
    }
}

impl RuleRegistry {
    pub fn enumerate(&mut self, items: &[xval::Value]) -> &mut Self {
        self.register("enum", Enumerate::new(items))
    }
}
