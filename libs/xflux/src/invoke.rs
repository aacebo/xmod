#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Invoke {
    pub alias: Option<String>,
    pub name: String,
    pub version: Option<semver::Version>,
    pub description: Option<String>,
    pub input: Option<xval::Value>,
}
