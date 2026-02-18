#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct CustomActionSpec {
    pub name: String,
    pub version: semver::Version,
    pub description: Option<String>,
    pub input: xsch::Schema,
    pub output: xsch::Schema,
}
