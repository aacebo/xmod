mod enumerate;
mod required;

pub use enumerate::*;
pub use required::*;

use std::{collections::BTreeMap, sync::Arc};

use crate::Validate;

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

    pub fn register<Rule: Validate + 'static>(&mut self, name: &str, rule: Rule) -> &mut Self {
        self.0.insert(name.to_string(), Arc::new(rule));
        self
    }

    pub fn get(&self, name: &str) -> Option<&dyn Validate> {
        self.0.get(name).map(|v| &**v)
    }
}

// impl Validate for RuleRegistry {
//     fn validate(&self, input: &xval::Value) -> Result<xval::Value, String> {
//         let mut output = input.clone();
//         let mut errors = vec![];

//         for (name, value) in &self.0 {
//             match value.validate(&output) {
//                 Err(err) => ,
//             };
//         }

//         Ok(output)
//     }
// }
