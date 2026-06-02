use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use wgpu::*;
use wgpu::util::DeviceExt;

use blockworld_utils::Identifier;
use crate::game::client::BlockworldClient;

use crate::resource::RESOURCE_MANAGER;

use super::{
    camera::Camera,
    input_manager::InputManager,
    meshing::meshing_manager::MeshingManager,
    resource_manager::BLOCK_ATLAS,
    shaders::WgslShader,
    vertex::TexturedVertex,
};

// -- RawMat4: GPU-compatible 4x4 matrix -----------------------------------

/// Column-major 4x4 matrix matching WGSL's `mat4x4<f32>` layout.
/// Converted from `glam::Mat4` via `to_cols_array_2d()`.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct RawMat4([[f32; 4]; 4]);

impl From<Mat4> for RawMat4 {
    fn from(mat: Mat4) -> Self {
        Self(mat.to_cols_array_2d())
    }
}

// -- WorldRenderer ---------------------------------------------------------

/// Holds all GPU resources and orchestrates rendering.
///
/// ## Bind group layout
///
/// ```ignore
/// @group(0)  binding 0  — texture_2d    (block atlas)
///            binding 1  — sampler       (nearest-neighbor)
/// @group(1)  binding 30 — uniform mat4  (MVP matrix)
/// ```
///
/// ## Per-frame flow
///
/// `update()` → camera movement → upload MVP to GPU → chunk mesh regeneration
/// `render()` → bind pipeline → bind groups → draw all chunk meshes
pub struct WorldRenderer {
    pub camera: Camera,
    /// Toggle wireframe mode (F1 key).
    pub debug_mode: bool,

    // --- depth buffer ---
    /// The depth texture (must be kept alive so the view remains valid).
    depth_texture: Texture,
    /// Exposed publicly so `RenderState` can attach it to the render pass.
    pub depth_view: TextureView,

    // --- block atlas texture ---
    /// `@group(0)`: texture view + sampler, pre-baked into one bind group.
    diffuse_bind_group: BindGroup,
    diffuse_bind_group_layout: BindGroupLayout,

    // --- camera matrix uniform ---
    /// `@group(1) @binding(30)`: model-view-projection matrix buffer.
    matrix_buffer: Buffer,
    matrix_bind_group: BindGroup,
    matrix_bind_group_layout: BindGroupLayout,

    // --- pipelines ---
    main_pipeline: RenderPipeline,
    wireframe_pipeline: RenderPipeline,

    // --- world ---
    pub game: BlockworldClient,
    pub meshing_manager: MeshingManager,
}

impl WorldRenderer {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        queue: &Queue,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let camera = Camera::new(size.width as f32 / size.height as f32);

        // --- diffuse texture from atlas ---
        let atlas_image = BLOCK_ATLAS.get_image();
        let rgba = atlas_image.to_owned();
        let dims = atlas_image.dimensions();
        let tex_size = Extent3d {
            width: dims.0,
            height: dims.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&TextureDescriptor {
            label: Some("Diffuse Texture"),
            size: tex_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                aspect: TextureAspect::All,
                texture: &diffuse_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            &rgba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dims.0),
                rows_per_image: Some(dims.1),
            },
            tex_size,
        );

        let diffuse_view = diffuse_texture.create_view(&TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let diffuse_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Diffuse Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let diffuse_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Diffuse Bind Group"),
            layout: &diffuse_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&diffuse_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&diffuse_sampler),
                },
            ],
        });

        // --- matrix uniform ---
        let mvp: RawMat4 = camera.build_mvp().into();
        let matrix_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Matrix Uniform"),
            contents: bytemuck::cast_slice(&[mvp]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let matrix_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Matrix Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 30,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let matrix_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Matrix Bind Group"),
            layout: &matrix_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 30,
                resource: matrix_buffer.as_entire_binding(),
            }],
        });

        // --- depth texture ---
        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        // --- shaders ---
        let rm = RESOURCE_MANAGER.lock().unwrap();
        let shader = WgslShader::new(
            &Identifier::new(&format!(
                "{}:assets/shaders/default_shader.wgsl",
                blockworld_utils::GAME_NAME
            )),
            &rm,
            device,
            "fs",
            "vs",
        )
        .expect("Failed to load default shader");

        let wireframe_shader = WgslShader::new(
            &Identifier::new(&format!(
                "{}:assets/shaders/wireframe_shader.wgsl",
                blockworld_utils::GAME_NAME
            )),
            &rm,
            device,
            "fs",
            "vs",
        )
        .expect("Failed to load wireframe shader");
        drop(rm);

        // --- pipeline layout (shared) ---
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Main Pipeline Layout"),
            bind_group_layouts: &[
                Some(&diffuse_bind_group_layout),
                Some(&matrix_bind_group_layout),
            ],
            immediate_size: 0,
        });

        let main_pipeline = create_render_pipeline(
            device,
            &pipeline_layout,
            config,
            &shader,
            PolygonMode::Fill,
            Some(Face::Back),
        );

        let wireframe_pipeline = create_render_pipeline(
            device,
            &pipeline_layout,
            config,
            &wireframe_shader,
            PolygonMode::Line,
            None,
        );

        // --- game ---
        let mut game = BlockworldClient::new(4);

        let meshing_manager = MeshingManager::new();

        Self {
            camera,
            debug_mode: false,
            depth_texture,
            depth_view,
            diffuse_bind_group,
            diffuse_bind_group_layout,
            matrix_buffer,
            matrix_bind_group,
            matrix_bind_group_layout,
            main_pipeline,
            wireframe_pipeline,
            game,
            meshing_manager,
        }
    }

    pub fn update(&mut self, queue: &Queue, device: &Device, input: &InputManager) {
        // Move camera based on keyboard input
        self.camera.update(input);

        // Upload new MVP matrix to GPU uniform buffer
        let mvp: RawMat4 = self.camera.build_mvp().into();
        queue.write_buffer(&self.matrix_buffer, 0, bytemuck::cast_slice(&[mvp]));

        // Load/unload chunks around the player, generate at most 2 new chunks
        self.game.update_view(self.camera.position);
        self.game.process_queue(1);

        // Rebuild at most 2 stale chunk meshes per frame
        self.meshing_manager.update(device, &mut self.game.chunks);
    }

    pub fn resize(&mut self, device: &Device, config: &SurfaceConfiguration, aspect_ratio: f32) {
        self.depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        self.depth_view = self.depth_texture.create_view(&TextureViewDescriptor::default());
        self.camera.update_aspect_ratio(aspect_ratio);
    }

    /// Bind pipeline and draw all chunk meshes.
    /// `rpass` is set up by `RenderState` with the correct color/depth attachments.
    pub fn render<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {
        if self.debug_mode {
            rpass.set_pipeline(&self.wireframe_pipeline);
        } else {
            rpass.set_pipeline(&self.main_pipeline);
        }
        rpass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        rpass.set_bind_group(1, &self.matrix_bind_group, &[]);
        self.meshing_manager.render(rpass);
    }
}

/// Build a `RenderPipeline` from a shader.
///
/// Shared by both the regular (filled) and wireframe (line) pipelines.
/// The only differences are `polygon_mode` and `cull_mode`.
fn create_render_pipeline(
    device: &Device,
    layout: &PipelineLayout,
    config: &SurfaceConfiguration,
    shader: &WgslShader,
    polygon_mode: PolygonMode,
    cull_mode: Option<Face>,
) -> RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(layout),
        vertex: VertexState {
            module: &shader.module,
            entry_point: Some(&shader.vert_entry),
            buffers: &[TexturedVertex::get_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(FragmentState {
            module: &shader.module,
            entry_point: Some(&shader.frag_entry),
            targets: &[Some(ColorTargetState {
                format: config.format,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode,
            unclipped_depth: false,
            polygon_mode,
            conservative: false,
        },
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(CompareFunction::Less),
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
