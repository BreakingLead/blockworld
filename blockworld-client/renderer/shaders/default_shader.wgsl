// Vertex shader
struct CameraUniform {
    matrix: mat4x4<f32>,
};
@group(1) @binding(30)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(10) position: vec3<f32>,
    @location(11) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(12) uv: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = model.uv;
    out.clip_position = camera.matrix * vec4<f32>(model.position, 1.0);
    return out;
}

// Group correspond to `render_pass.set_bind_group()`'s first argument.
@group(0) @binding(20)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(21)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let u = in.uv.x;
    let v = in.uv.y;
    return textureSample(t_diffuse, s_diffuse, vec2<f32>(u, v));
}

