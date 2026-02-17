use crate::{Context, ValidError, Validate, rules::RuleRegistry};

#[derive(Debug, Default, Clone)]
pub struct Enumerate(Vec<xval::Value>);

impl Enumerate {
    pub fn new(items: &[xval::Value]) -> Self {
        Self(items.to_vec())
    }
}

impl Validate for Enumerate {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        for option in &self.0 {
            if ctx.value == *option {
                break;
            }
        }

        Ok(ctx.value.clone())
    }
}

impl RuleRegistry {
    pub fn enumerate(&mut self, items: &[xval::Value]) -> &mut Self {
        self.register("enum", Enumerate::new(items))
    }
}
