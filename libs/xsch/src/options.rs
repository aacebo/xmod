use crate::{Context, Rule, ValidError, Validate};

#[derive(Debug, Clone)]
pub struct Options(Vec<xval::Value>);

impl From<Vec<xval::Value>> for Options {
    fn from(value: Vec<xval::Value>) -> Self {
        Self(value)
    }
}

impl From<&[xval::Value]> for Options {
    fn from(value: &[xval::Value]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Options> for Rule {
    fn from(value: Options) -> Self {
        Self::Options(value)
    }
}

impl Validate for Options {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        for option in &self.0 {
            if ctx.value == *option {
                return Ok(ctx.value.clone());
            }
        }

        let options = self
            .0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(ctx.error(&format!("must be one of [{}]", options)))
    }
}
