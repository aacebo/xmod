#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged, rename_all = "snake_case")
)]
pub enum Phase {
    Presence,   // required/optional/nullable
    Type,       // string/number/int/bool checks
    Coerce,     // if you support coercion/casting
    Constraint, // min/max/len/regex/range
    Refine,     // custom predicates, cross-field rules, etc.
}

impl Phase {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Presence => "presence",
            Self::Type => "type",
            Self::Coerce => "coerce",
            Self::Constraint => "constraint",
            Self::Refine => "refine",
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
