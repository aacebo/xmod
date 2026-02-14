use std::collections::BTreeMap;

use xval::Value;

#[derive(Debug, Clone)]
pub struct Context {
    pub started_at: std::time::Instant,
    pub input: Value,

    data: BTreeMap<String, Value>,
}

impl Context {
    pub fn new(input: Value) -> Self {
        Self {
            started_at: std::time::Instant::now(),
            input,
            data: BTreeMap::new(),
        }
    }

    pub fn elapse(&self) -> std::time::Duration {
        std::time::Instant::now() - self.started_at
    }

    pub fn merge(mut self, other: Context) -> Self {
        for (key, value) in other.data {
            self.set(key, value);
        }

        self
    }
}

impl Context {
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) -> &mut Self {
        self.data.insert(key.into(), value);
        self
    }
}
