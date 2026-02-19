use xval::AsValue;

use crate::{Context, Rule, Schema, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Items(Schema);

impl Items {
    pub const KEY: &str = "items";

    pub fn new(items: Schema) -> Self {
        Self(items)
    }
}

impl From<Schema> for Items {
    fn from(value: Schema) -> Self {
        Self(value)
    }
}

impl From<Items> for Rule {
    fn from(value: Items) -> Self {
        Self::Items(value)
    }
}

impl Validator for Items {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if !ctx.value.is_null() && ctx.value.is_array() {
            let mut items = vec![];

            for (i, item) in ctx.value.as_array().items().enumerate() {
                let mut next = ctx.clone();
                next.path = ctx.path.child(i.into());
                next.value = item.as_value();
                items.push(self.0.validate(&next)?);
            }

            return Ok(items.as_value());
        }

        Ok(ctx.value.clone())
    }
}
