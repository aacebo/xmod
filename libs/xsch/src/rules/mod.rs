mod one_of;
mod required;

pub use one_of::*;
pub use required::*;

use std::{collections::BTreeMap, sync::Arc};

use crate::{Context, ValidError, Validate};

#[derive(Default, Clone)]
pub struct RuleRegistry(BTreeMap<String, Arc<dyn Validate>>);

impl RuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&dyn Validate> {
        self.0.get(name).map(|v| &**v)
    }

    pub fn register<Rule: Validate + 'static>(&mut self, name: &str, rule: Rule) -> &mut Self {
        self.0.insert(name.to_string(), Arc::new(rule));
        self
    }
}

impl std::fmt::Debug for RuleRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();

        for (key, _) in &self.0 {
            list.entry(key);
        }

        list.finish()
    }
}

impl std::fmt::Display for RuleRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Validate for RuleRegistry {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let mut next = ctx.clone();
        let mut error = ValidError::new(&ctx.rule, ctx.path.clone()).build();

        for (rule, value) in &self.0 {
            next.rule = rule.to_string();
            next.value = match value.validate(&next) {
                Ok(v) => v,
                Err(err) => {
                    error.errors.push(err);
                    continue;
                }
            };
        }

        if !error.errors.is_empty() {
            return Err(error);
        }

        Ok(next.value)
    }
}
