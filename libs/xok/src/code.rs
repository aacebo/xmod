#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Code {
    Internal,
    NotFound,
    BadArgument,
    UnAuthorized,
    Timeout,
    Conflict,
    Duplicate,
}
