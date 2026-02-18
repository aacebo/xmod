mod custom;
pub mod system;

pub use custom::*;
pub use system::*;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged, rename_all = "snake_case")
)]
pub enum ActionSpec {
    System(SystemActionSpec),
    Custom(CustomActionSpec),
}
