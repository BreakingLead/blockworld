//! An API for programmatically accessing Minecraft resources and associated
//! metadata.
//!
//! ## Resource Identifiers
//!
//! Every resource is associated with a unique [`ResourceIdentifier`], which is a
//! combination of a [`ResourceKind`] and a *namespaced identifier*.
//!
//! ## Providers
//!
//! Resources can be enumerated and loaded using the [`ResourceProvider`] trait.
//! This crate provides the [`FileSystemResourceProvider`] as a convenient
//! implementation of this trait.
//!
//! ## Asset Pack
//!
//! Resources can be ergonomically loaded through the [`AssetPack`] API.

use std::io;

mod asset_pack;
mod provider;
mod resolve;
mod resource;

pub use asset_pack::AssetPack;
pub use provider::{
    EnumerateResources, FileSystemResourceProvider, LoadResource, ResourceProvider,
};
pub use resolve::ModelResolver;
pub use resource::{
    ModelIdentifier, ResourceCategory, ResourceIdentifier, ResourceKind, ResourcePath,
    MINECRAFT_NAMESPACE,
};

/// Error types that can be returned from API methods.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
}

/// Result alias for convenience.
pub type Result<T, E = Error> = std::result::Result<T, E>;
