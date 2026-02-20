use crate::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Pattern(String);

impl Pattern {
    pub const KEY: &str = "pattern";
    pub const PHASE: crate::Phase = crate::Phase::Constraint;

    pub fn new(pattern: String) -> Self {
        Self(pattern)
    }
}

impl From<String> for Pattern {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Pattern> for Rule {
    fn from(value: Pattern) -> Self {
        Self::Pattern(value)
    }
}

impl Validator for Pattern {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let reg = match regex::Regex::new(&self.0) {
            Ok(v) => v,
            Err(err) => return Err(ctx.error(&err.to_string())),
        };

        if !ctx.value.is_null() && !reg.is_match(ctx.value.as_str()) {
            return Err(ctx.error(&format!(
                "'{}' does not match pattern '{}'",
                &ctx.value, &self.0
            )));
        }

        Ok(ctx.value.clone())
    }
}
