use anyhow::*;
use std::path::{Path, PathBuf};

use blockworld_utils::ResourceLocation;

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
    fn get_bytes(&self, id: &ResourceLocation) -> Result<Vec<u8>>;
}

/// A resource provider that provides resources from a static value (embedded in the binary).
pub struct StaticBytesProvider;

impl BytesProvider for StaticBytesProvider {
    fn get_bytes(&self, id: &ResourceLocation) -> Result<Vec<u8>> {
        if id == &"minecraft:assets/shaders/wireframe_shader.wgsl".into() {
            let r = include_bytes!("shaders/wireframe_shader.wgsl").to_vec();
            return Ok(r);
        }
        if id == &"minecraft:assets/shaders/default_shader.wgsl".into() {
            let r = include_bytes!("shaders/default_shader.wgsl").to_vec();
            return Ok(r);
        }
        bail!("Resource not found: {:?}", id);
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
    fn get_bytes(&self, identifier: &ResourceLocation) -> anyhow::Result<Vec<u8>> {
        let path = self
            .root_dir
            .join(Path::new("assets/").join(identifier.get_namespace()))
            .join(identifier.get_path());
        dbg!(&path);

        if !path.exists() {
            anyhow::bail!("File not found: {:?}", path);
        }

        let buf = std::fs::read(path)?;

        Ok(buf)
    }
}
