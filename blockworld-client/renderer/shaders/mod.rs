//! WGSL shader loading and compilation.
//!
//! Shaders are loaded from the `ResourceManager` (supporting both
//! embedded bytes and filesystem overrides).

use std::borrow::Cow;

use anyhow::*;
use blockworld_utils::Identifier;
use wgpu::*;

use crate::resource::ResourceManager;

/// A compiled WGSL shader module with named entry points.
///
/// Both the vertex (`vs`) and fragment (`fs`) stages share the same
/// module; entry points are specified by name.
#[derive(Debug)]
pub struct WgslShader {
    pub module: ShaderModule,
    pub frag_entry: String,
    pub vert_entry: String,
}

impl WgslShader {
    /// Load WGSL source from the `ResourceManager`, compile it,
    /// and record the fragment/vertex entry point names.
    pub fn new(
        resource: &Identifier,
        rm: &ResourceManager,
        device: &Device,
        frag_entry: &str,
        vert_entry: &str,
    ) -> Result<Self> {
        let bytes = rm
            .get(resource)
            .ok_or_else(|| anyhow!("Shader not found: {:?}", resource))?;
        let src = std::str::from_utf8(&bytes)?;
        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::from(src)),
        });

        Ok(Self {
            module,
            frag_entry: frag_entry.to_string(),
            vert_entry: vert_entry.to_string(),
        })
    }
}
