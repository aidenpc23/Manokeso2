// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct InstanceInput {
    @location(2) position: vec2<u32>,
};

struct CameraUniform {
    pos: vec2<f32>,
    proj: vec2<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) my_index: u32,
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    var pos = model.position + vec2<f32>(instance.position);
    pos -= camera.pos;
    pos *= camera.proj;
    out.clip_position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
