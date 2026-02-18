mod method;

pub use method::*;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct HttpActionSpec {
    pub name: HttpMethod,
    pub version: semver::Version,
    pub url: xtera::Template,
    pub description: Option<String>,
    pub input: Option<xsch::Schema>,
    pub output: Option<xsch::Schema>,
}
