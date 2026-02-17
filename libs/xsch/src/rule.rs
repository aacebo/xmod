use crate::{Context, Equals, Options, Required, ValidError, ValidErrorBuilder, Validate};

#[derive(Debug, Clone)]
pub enum Rule {
    Equals(Equals),
    Required(Required),
    Options(Options),
}

impl Rule {
    pub fn name(&self) -> &str {
        match self {
            Self::Equals(_) => "equals",
            Self::Options(_) => "options",
            Self::Required(_) => "required",
        }
    }
}

impl Validate for Rule {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::Equals(v) => v.validate(ctx),
            Self::Options(v) => v.validate(ctx),
            Self::Required(v) => v.validate(ctx),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RuleSet(Vec<Rule>);

impl RuleSet {
    pub fn add(mut self, rule: Rule) -> Self {
        self.0.push(rule);
        self
    }
}

impl std::ops::Deref for RuleSet {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self.0)
    }
}

impl Validate for RuleSet {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let mut next = ctx.clone();
        let mut error = ValidErrorBuilder::new(ctx.path.clone()).build();

        for rule in &self.0 {
            next.rule = Some(rule.name().to_string());
            next.value = match rule.validate(&next) {
                Ok(v) => v,
                Err(err) => {
                    error.errors.push(err);
                    continue;
                }
            };
        }

        if !error.errors.is_empty() {
            return Err(error);
        }

        Ok(next.value)
    }
}
