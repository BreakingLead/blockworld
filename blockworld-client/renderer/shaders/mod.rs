use std::borrow::Cow;

use anyhow::*;
use blockworld_utils::ResourceLocation;
use wgpu::*;

use super::bytes_provider::BytesProvider;

pub trait ToWgpuShader: Send + Sync {
    fn get_frag(&self) -> (&ShaderModule, &str);
    fn get_vert(&self) -> (&ShaderModule, &str);
}

#[derive(Debug)]
pub struct WgslShader {
    pub module: ShaderModule,
    pub frag_entry: String,
    pub vert_entry: String,
}

impl WgslShader {
    pub fn new(
        resource: &ResourceLocation,
        rp: &dyn BytesProvider,
        device: &wgpu::Device,
        frag_entry: &str,
        vert_entry: &str,
    ) -> Result<Self> {
        let shader_src = rp.get_bytes(resource)?;

        let shader_src = std::str::from_utf8(&shader_src)?;

        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::from(shader_src)),
        });

        Ok(Self {
            module,
            frag_entry: frag_entry.to_string(),
            vert_entry: vert_entry.to_string(),
        })
    }
}
impl ToWgpuShader for WgslShader {
    fn get_frag(&self) -> (&ShaderModule, &str) {
        (&self.module, &self.frag_entry)
    }

    fn get_vert(&self) -> (&ShaderModule, &str) {
        (&self.module, &self.vert_entry)
    }
}
