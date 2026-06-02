use std::borrow::Cow;

use anyhow::*;
use blockworld_utils::Identifier;
use wgpu::*;

use super::resource::ResourceManager;

#[derive(Debug)]
pub struct WgslShader {
    pub module: ShaderModule,
    pub frag_entry: String,
    pub vert_entry: String,
}

impl WgslShader {
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
