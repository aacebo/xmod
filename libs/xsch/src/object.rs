use crate::{Context, Fields, Required, RuleSet, Schema, ToSchema, ValidError, Validator};

pub fn object() -> ObjectSchema {
    ObjectSchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct ObjectSchema(pub(crate) RuleSet);

impl ObjectSchema {
    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }

    pub fn fields(mut self, fields: Fields) -> Self {
        self.0 = self.0.add(fields.into());
        self
    }

    pub fn field(mut self, name: &str, schema: Schema) -> Self {
        if let Some(fields) = self.0.get_mut(Fields::KEY).and_then(|v| v.as_fields_mut()) {
            fields.set(name, schema);
            return self;
        }

        self.0 = self.0.add(Fields::default().field(name, schema).into());
        self
    }

    pub fn get_field(&self, name: &str) -> Option<&Schema> {
        self.0
            .get(Fields::KEY)
            .and_then(|r| r.as_fields())
            .and_then(|f| f.get(name))
    }
}

impl ToSchema for ObjectSchema {
    fn to_schema(&self) -> Schema {
        Schema::Object(self.clone())
    }
}

impl From<ObjectSchema> for Schema {
    fn from(value: ObjectSchema) -> Self {
        Self::Object(value)
    }
}

impl Validator for ObjectSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_null() && !value.is_struct() {
            return Err(ctx.error("expected object"));
        }

        Ok(value)
    }
}
