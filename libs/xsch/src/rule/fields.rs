use std::collections::BTreeMap;

use crate::{Context, Rule, Schema, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Fields(BTreeMap<String, Schema>);

impl Fields {
    pub const KEY: &str = "fields";

    pub fn new(fields: BTreeMap<String, Schema>) -> Self {
        Self(fields)
    }

    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.0.get(name)
    }

    pub fn set(&mut self, name: &str, schema: Schema) -> &mut Self {
        self.0.insert(name.to_string(), schema);
        self
    }

    pub fn field(mut self, name: &str, schema: Schema) -> Self {
        self.0.insert(name.to_string(), schema);
        self
    }
}

impl From<BTreeMap<String, Schema>> for Fields {
    fn from(value: BTreeMap<String, Schema>) -> Self {
        Self(value)
    }
}

impl From<Fields> for Rule {
    fn from(value: Fields) -> Self {
        Self::Fields(value)
    }
}

impl Validator for Fields {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if !ctx.value.is_null() && ctx.value.is_struct() {
            let input = ctx.value.as_struct();
            let mut error = ValidError::new(ctx.path.clone()).build();

            // Check for unexpected fields in the input
            for (ident, _) in input.items() {
                if !self.0.contains_key(&ident.to_string()) {
                    let path = ctx.path.child(xpath::Ident::parse(&ident.to_string()));
                    error.errors.push(
                        ValidError::new(path)
                            .message(&format!("unexpected field '{}'", &ident))
                            .build(),
                    );
                }
            }

            // Validator all schema-defined fields
            for (name, schema) in &self.0 {
                let mut next = ctx.clone();
                next.path = ctx.path.child(xpath::Ident::parse(name));
                next.value = input
                    .field(xpath::Ident::key(name))
                    .map(|v| v.as_value())
                    .unwrap_or(xval::valueof!(null));

                if let Err(err) = schema.validate(&next) {
                    error.errors.push(err);
                }
            }

            if !error.is_empty() {
                return Err(error);
            }
        }

        Ok(ctx.value.clone())
    }
}
