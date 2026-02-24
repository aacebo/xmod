use crate::Invoke;

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
