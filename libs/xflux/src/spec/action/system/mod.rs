pub mod http;

pub(crate) use http::*;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum SystemActionSpec {
    Http(HttpActionSpec),
}
