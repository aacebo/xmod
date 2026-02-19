use xval::{ToValue, Value};

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub payload: Value,
}

impl Event {
    pub fn new(name: &str, payload: impl ToValue) -> Self {
        Self {
            name: name.to_string(),
            payload: payload.to_value(),
        }
    }
}
