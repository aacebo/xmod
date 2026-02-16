#[derive(Debug, Clone)]
pub struct Context {
    pub path: String,
    pub value: xval::Value,
}
