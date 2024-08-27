use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::resource::api::{ResourceIdentifier, ResourceKind, ResourcePath};

/*
 dMMMMMMP dMMMMb  .aMMMb  dMP dMMMMMMP .dMMMb
   dMP   dMP.dMP dMP"dMP amr    dMP   dMP" VP
  dMP   dMMMMK" dMMMMMP dMP    dMP    VMMMb
 dMP   dMP"AMF dMP dMP dMP    dMP   dP .dMP
dMP   dMP dMP dMP dMP dMP    dMP    VMMMP"
*/

/// Indicates that a type can enumerate available resources.
pub trait EnumerateResources {
    /// Enumerates the available resources of the given [`ResourceKind`] in the
    /// given namespace.
    fn enumerate_resources(
        &self,
        namespace: &str,
        kind: ResourceKind,
    ) -> Result<Vec<ResourceIdentifier<'static>>, io::Error>;
}

/// Indicates that a type can load provide the raw data of resources.
pub trait LoadResource {
    /// Returns the raw bytes of the resource referenced by the given
    /// [`ResourceIdentifier`].
    fn load_resource(&self, id: &ResourceIdentifier) -> Result<Vec<u8>, io::Error>;
}

/// Marker trait for types that are [`EnumerateResources`] and [`LoadResource`].
pub trait ResourceProvider: EnumerateResources + LoadResource {}

impl<T: EnumerateResources + LoadResource> ResourceProvider for T {}

/*
    dMMMMMP dMP dMP     dMMMMMP        .dMMMb  dMP dMP .dMMMb dMMMMMMP dMMMMMP dMMMMMMMMb
   dMP     amr dMP     dMP            dMP" VP dMP.dMP dMP" VP   dMP   dMP     dMP"dMP"dMP
  dMMMP   dMP dMP     dMMMP           VMMMb   VMMMMP  VMMMb    dMP   dMMMP   dMP dMP dMP
 dMP     dMP dMP     dMP            dP .dMP dA .dMP dP .dMP   dMP   dMP     dMP dMP dMP
dMP     dMP dMMMMMP dMMMMMP         VMMMP"  VMMMP"  VMMMP"   dMP   dMMMMMP dMP dMP dMP

    dMMMMb  dMMMMb  .aMMMb  dMP dMP dMP dMMMMb  dMMMMMP dMMMMb
   dMP.dMP dMP.dMP dMP"dMP dMP dMP amr dMP VMP dMP     dMP.dMP
  dMMMMP" dMMMMK" dMP dMP dMP dMP dMP dMP dMP dMMMP   dMMMMK"
 dMP     dMP"AMF dMP.aMP  YMvAP" dMP dMP.aMP dMP     dMP"AMF
dMP     dMP dMP  VMMMP"    VP"  dMP dMMMMP" dMMMMMP dMP dMP

*/

/// A [`ResourceProvider`] that provides resources from the local file system.
pub struct FileSystemResourceProvider {
    root: PathBuf,
}

impl FileSystemResourceProvider {
    /// Returns a new provider that provides resources from the given root directory.
    ///
    /// The root directory should be the directory that contains the `assets/`
    /// and (optionally) `data/` directory.
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: PathBuf::from(root.as_ref()),
        }
    }
}

impl EnumerateResources for FileSystemResourceProvider {
    fn enumerate_resources(
        &self,
        namespace: &str,
        kind: ResourceKind,
    ) -> Result<Vec<ResourceIdentifier<'static>>, io::Error> {
        let directory = ResourcePath::for_kind(&self.root, namespace, kind);
        Ok(ResourceIter::new(directory, kind)?.collect())
    }
}

impl LoadResource for FileSystemResourceProvider {
    fn load_resource(&self, id: &ResourceIdentifier) -> Result<Vec<u8>, io::Error> {
        let path = ResourcePath::for_resource(&self.root, id);
        fs::read(path)
    }
}

/*
    dMP dMMMMMMP dMMMMMP dMMMMb
   amr    dMP   dMP     dMP.dMP
  dMP    dMP   dMMMP   dMMMMK"
 dMP    dMP   dMP     dMP"AMF
dMP    dMP   dMMMMMP dMP dMP

*/

/// An iterator over a directory that yields [`ResourceIdentifier`]s for every
/// file of a certain [`ResourceKind`].
pub struct ResourceIter {
    // Stack of directory iterators.
    dir_iters: Vec<fs::ReadDir>,
    // Stack of directory names.
    dir_names: Vec<String>,
    kind: ResourceKind,
}

enum DirOrResource {
    Dir { name: String, iter: fs::ReadDir },
    Resource(ResourceIdentifier<'static>),
}

impl ResourceIter {
    pub fn new(directory: impl AsRef<Path>, kind: ResourceKind) -> Result<Self, io::Error> {
        let dir_iter = fs::read_dir(directory)?;

        Ok(Self {
            dir_iters: vec![dir_iter],
            dir_names: vec![],
            kind,
        })
    }

    #[inline]
    fn next_dir_or_resource(&mut self) -> Option<DirOrResource> {
        // Continue iteration in the childmost directory.
        let dir_iter = self.dir_iters.last_mut().unwrap();

        dir_iter
            .filter_map(|dir_entry| {
                dir_entry
                    // Skip over errorneous entries.
                    .ok()
                    // Get file type of entry and skip over fs errors.
                    .and_then(|dir_entry| {
                        dir_entry
                            .file_type()
                            .ok()
                            .map(|file_type| (dir_entry, file_type))
                    })
                    .and_then(|(dir_entry, file_type)| {
                        if file_type.is_dir() {
                            // Start new ReadDir in subdirectory.
                            fs::read_dir(dir_entry.path())
                                // Skip over fs errors.
                                .ok()
                                .map(|iter| DirOrResource::Dir {
                                    name: dir_entry.file_name().to_string_lossy().into_owned(),
                                    iter,
                                })
                        } else {
                            // Get file name and skip over UTF-8 errors.
                            dir_entry.file_name().to_str().and_then(|file_name| {
                                (
                                    // Skip over files starting with '_'.
                                    !file_name.starts_with('_') &&
                                    // Skip over resources of the wrong kind (check the extension).
                                    file_name.ends_with(self.kind.extension())
                                )
                                .then(|| {
                                    // Cut the extension off the file name to
                                    // get the resource name.
                                    let dot_index =
                                        file_name.len() - self.kind.extension().len() - 1;

                                    let file_name = &file_name[..dot_index];

                                    // Prepend any subdirectory paths
                                    let mut components = self.dir_names.clone();
                                    components.push(file_name.to_string());

                                    let resource_path = components.join("/");

                                    let id =
                                        ResourceIdentifier::new_owned(self.kind, resource_path);
                                    DirOrResource::Resource(id)
                                })
                            })
                        }
                    })
            })
            .next()
    }
}

impl Iterator for ResourceIter {
    type Item = ResourceIdentifier<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get the next directory or resource id.
            let next_dir_or_resource = self.next_dir_or_resource();

            // A value of `None` here indicates that the childmost directory has no
            // more entries, so pop the child or return the final `None`.
            if next_dir_or_resource.is_none() {
                if self.dir_iters.len() > 1 {
                    self.dir_iters.pop();
                    self.dir_names.pop();
                    continue;
                } else {
                    return None;
                }
            }

            match next_dir_or_resource.unwrap() {
                // If the next entry is a directory, push a new child and continue
                // iterating inside the subdirectory.
                DirOrResource::Dir { name, iter } => {
                    self.dir_iters.push(iter);
                    self.dir_names.push(name);
                    continue;
                }
                DirOrResource::Resource(id) => return Some(id),
            }
        }
    }
}
