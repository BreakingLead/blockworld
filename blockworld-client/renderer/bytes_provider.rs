use std::path::{Path, PathBuf};

use blockworld_utils::ResourceLocation;
use thiserror::Error;

/// A abstraction over the way resources are loaded.
/// This trait is implemented by different resource providers,
/// such as a filesystem provider,
/// a web request provider or a resource pack provider.
pub trait BytesProvider: Send + Sync {
    /// id format:
    ///
    /// `assets/<id.namespace>/<id.path>`
    ///
    /// `minecraft:textures/block/stone.png`
    /// `assets/minecraft/textures/block/stone.png`
    fn get_bytes(&self, id: &ResourceLocation) -> Result<Vec<u8>, ResourceError>;
}

/// A resource provider that provides resources from a static value (embedded in the binary).
pub struct StaticBytesProvider;

impl BytesProvider for StaticBytesProvider {
    fn get_bytes(&self, id: &ResourceLocation) -> Result<Vec<u8>, ResourceError> {
        if id == &"blockworld:assets/shaders/wireframe_shader.wgsl".into() {
            let r = include_bytes!("shaders/wireframe_shader.wgsl").to_vec();
            return Ok(r);
        }
        if id == &"blockworld:assets/shaders/default_shader.wgsl".into() {
            let r = include_bytes!("shaders/default_shader.wgsl").to_vec();
            return Ok(r);
        }
        Err(ResourceError::NotFound(id.clone()))
    }
}

pub struct FilesystemBytesProvider {
    root_dir: PathBuf,
}

impl FilesystemBytesProvider {
    fn new<Q: AsRef<Path>>(root_dir: Q) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }
}

impl BytesProvider for FilesystemBytesProvider {
    fn get_bytes(&self, identifier: &ResourceLocation) -> Result<Vec<u8>, ResourceError> {
        let path = self
            .root_dir
            .join(Path::new("assets/").join(identifier.get_namespace()))
            .join(identifier.get_path());
        dbg!(&path);

        if !path.exists() {
            return Err(ResourceError::NotFound(identifier.clone()));
        }

        let buf = std::fs::read(path).map_err(|e| ResourceError::Io(e))?;

        Ok(buf)
    }
}

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("Resource not found: {0}")]
    NotFound(ResourceLocation),
    #[error("IO error: {0}")]
    Io(std::io::Error),
}

#[test]
fn filesystem_resource_provider_test() {
    let p = FilesystemBytesProvider::new(".");
    let bytes = p
        .get_bytes(&ResourceLocation::new("minecraft:texts/splashes.txt"))
        .unwrap();
    let s = String::from_utf8(bytes).unwrap();
}
