use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use crate::resource::api::{ResourceIdentifier, ResourceKind};

/// Represents the full path to a resource, e.g., on the local file system.
pub struct ResourcePath(PathBuf);

impl ResourcePath {
    /// Returns the full path to the given resource.
    ///
    /// The `root` argument should be the path to the directory containing the
    /// `assets/` and (optionally) `data/` directories.
    ///
    /// **NOTE:** no validation of the path is performed. The returned path may
    /// not point to an existing file. This method simply computes what the path
    /// should be for a given resource.
    pub fn for_resource(root: impl AsRef<Path>, resource: &ResourceIdentifier) -> Self {
        let mut path = Self::for_kind(root, resource.namespace(), resource.kind());

        path.push(resource.path());

        Self(path.with_extension(resource.kind().extension()))
    }

    /// Returns the full path to the directory that contains resources of the
    /// given type for the given namespace.
    ///
    /// The `root` argument should be the path to the directory containing the
    /// `assets/` and (optionally) `data/` directories.
    ///
    /// **NOTE:** no validation of the path is performed. The returned path may
    /// not point to an existing directory. This method simply computes what the
    /// path should be for a given resource type.
    pub fn for_kind(root: impl AsRef<Path>, namespace: &str, kind: ResourceKind) -> Self {
        let mut path = PathBuf::from(root.as_ref());

        // `assets/` or `data/`.
        path.push(kind.category().directory());
        path.push(namespace);
        path.push(kind.directory());

        Self(path)
    }

    /// Consumes `self` and returns the inner [`PathBuf`].
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl AsRef<Path> for ResourcePath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Deref for ResourcePath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ResourcePath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
