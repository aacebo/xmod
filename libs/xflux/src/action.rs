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
            .cloned()
            .unwrap_or_else(|| xsch::any().into());

        let mut value = match schema.validate(&xsch::Context::from(ctx.input.clone())) {
            Err(err) => return Err(err.boxed()),
            Ok(v) => v,
        };

        for invoke in &self.actions {
            let input = if invoke.input.is_null() {
                value.clone()
            } else {
                invoke.input.clone()
            };

            value = ctx.execute(&invoke.action, input).await?;

            if let Some(alias) = &invoke.alias {
                ctx.var(alias, value.clone());
            }
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

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use xval::ToValue;

    use super::*;
    use crate::{Context, Execute, Invoke};

    struct AddOne;

    #[async_trait]
    impl Execute for AddOne {
        async fn exec(&self, ctx: &mut Context) -> xok::Result<xval::Value> {
            Ok((ctx.input.to_i32() + 1).to_value())
        }
    }

    struct Echo;

    #[async_trait]
    impl Execute for Echo {
        async fn exec(&self, ctx: &mut Context) -> xok::Result<xval::Value> {
            Ok(ctx.input.clone())
        }
    }

    struct SetVar;

    #[async_trait]
    impl Execute for SetVar {
        async fn exec(&self, ctx: &mut Context) -> xok::Result<xval::Value> {
            ctx.var("was_here", true.to_value());
            Ok(ctx.input.clone())
        }
    }

    fn action_ref(name: &str) -> ActionRef {
        ActionRef {
            name: name.to_string(),
            version: None,
        }
    }

    fn invoke(name: &str) -> Invoke {
        Invoke {
            alias: None,
            action: action_ref(name),
            description: None,
            input: xval::Value::Null,
        }
    }

    fn test_action(actions: Vec<Invoke>) -> Action {
        Action {
            name: "test".to_string(),
            version: semver::Version::new(0, 1, 0),
            description: None,
            input: None,
            actions,
        }
    }

    #[tokio::test]
    async fn sequential_piping() {
        let action = test_action(vec![invoke("add_one"), invoke("add_one")]);
        let mut ctx = Context::new(10_i32);
        ctx.register(action_ref("add_one"), AddOne);
        let result = action.exec(&mut ctx).await.unwrap();
        assert_eq!(result.to_i32(), 12);
    }

    #[tokio::test]
    async fn alias_stores_output() {
        let mut inv = invoke("echo");
        inv.alias = Some("result".to_string());
        let action = test_action(vec![inv]);
        let mut ctx = Context::new(42_i32);
        ctx.register(action_ref("echo"), Echo);
        action.exec(&mut ctx).await.unwrap();
        assert_eq!(ctx.data().get("result").unwrap().to_i32(), 42);
    }

    #[tokio::test]
    async fn invoke_input_overrides_piped_value() {
        let mut inv = invoke("echo");
        inv.input = 99_i32.to_value();
        let action = test_action(vec![inv]);
        let mut ctx = Context::new(1_i32);
        ctx.register(action_ref("echo"), Echo);
        let result = action.exec(&mut ctx).await.unwrap();
        assert_eq!(result.to_i32(), 99);
    }

    #[tokio::test]
    async fn child_data_merges_to_parent() {
        let action = test_action(vec![invoke("set_var")]);
        let mut ctx = Context::new(xval::Value::Null);
        ctx.register(action_ref("set_var"), SetVar);
        action.exec(&mut ctx).await.unwrap();
        assert_eq!(ctx.data().get("was_here").unwrap().to_bool(), true);
    }

    #[tokio::test]
    async fn schema_validation_accepts_valid_input() {
        let mut a = test_action(vec![]);
        a.input = Some(xsch::int().required().into());
        let mut ctx = Context::new(42_i32);
        let result = a.exec(&mut ctx).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn schema_validation_rejects_invalid_input() {
        let mut a = test_action(vec![]);
        a.input = Some(xsch::int().required().into());
        let mut ctx = Context::new("not a number");
        let result = a.exec(&mut ctx).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn action_not_found_includes_ref() {
        let action = test_action(vec![invoke("nonexistent")]);
        let mut ctx = Context::new(xval::Value::Null);
        let err = action.exec(&mut ctx).await.unwrap_err();
        assert!(err.to_string().contains("nonexistent@latest"));
    }
}
