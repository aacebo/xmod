use async_trait::async_trait;
use xok::XError;
use xsch::Validator;

use crate::{Context, Execute, Invoke};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Action {
    pub name: String,
    pub version: semver::Version,
    pub description: Option<String>,
    pub input: Option<xsch::Schema>,
    pub actions: Vec<Invoke>,
}

#[async_trait]
impl Execute for Action {
    async fn exec(&self, ctx: &mut Context) -> xok::Result<xval::Value> {
        let schema = self
            .input
            .as_ref()
            .map(|v| v.clone())
            .unwrap_or_else(|| xsch::any().into());

        let mut value = match schema.validate(&ctx.input.clone().into()) {
            Err(err) => return Err(err.boxed()),
            Ok(v) => v,
        };

        for invoke in &self.actions {
            value = ctx.execute(&invoke.action, value).await?;
        }

        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct ActionRef {
    pub name: String,
    pub version: Option<semver::Version>,
}

impl std::fmt::Display for ActionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.version {
            None => write!(f, "{}@latest", &self.name),
            Some(ver) => write!(f, "{}@{}", &self.name, &ver),
        }
    }
}
