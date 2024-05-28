//! Serde-(de)serializable data types for files in the Minecraft `assets/`
//! directory.

pub mod blockstates;
pub mod models;

pub use blockstates::BlockStates;
pub use models::Model;
