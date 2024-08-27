use std::path::Path;

use blockworld_utils::ResourceLocation;
use thiserror::Error;

/// A abstraction over the way resources are loaded.
/// This trait is implemented by different resource providers,
/// such as a filesystem provider,
/// a web request provider or a resource pack provider.
pub trait ResourceProvider: Send + Sync {
    fn get_bytes(&self, id: &ResourceLocation) -> Result<Vec<u8>, ResourceError>;
}

/// A resource provider that provides resources from a static value (embedded in the binary).
pub struct StaticResourceProvider;

impl ResourceProvider for StaticResourceProvider {
    fn get_bytes(&self, id: &ResourceLocation) -> Result<Vec<u8>, ResourceError> {
        if id == &"blockworld:assets/shaders/wireframe_shader.wgsl".into() {
            let r = include_bytes!("../assets/shaders/wireframe_shader.wgsl").to_vec();
            return Ok(r);
        }
        if id == &"blockworld:assets/shaders/default_shader.wgsl".into() {
            let r = include_bytes!("../assets/shaders/default_shader.wgsl").to_vec();
            return Ok(r);
        }
        Err(ResourceError::NotFound(id.clone()))
    }
}

/// Only used for debugging purposes
/// When 1.0 is released, this should be replaced with a more robust solution
pub const TEMP_ASSETS_ROOT: &str = "assets/";
pub struct FilesystemResourceProvider;

impl ResourceProvider for FilesystemResourceProvider {
    fn get_bytes(&self, identifier: &ResourceLocation) -> Result<Vec<u8>, ResourceError> {
        let path = TEMP_ASSETS_ROOT.to_string() + identifier.get_path().to_str().unwrap();
        let path = Path::new(&path);

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
    std::fs::write("assets/test.txt", "hello").unwrap();

    let p = FilesystemResourceProvider;
    let bytes = p.get_bytes(&ResourceLocation::new("test.txt")).unwrap();
    let s = String::from_utf8(bytes).unwrap();
    assert_eq!(s, "hello");
}
