
// vertex shader

struct VertexOutput {
    @location(0) color: vec4<f32>,
    @location(1) center: vec2<f32>,
    @location(2) corner: vec2<f32>,
    @location(3) radius: f32,
    @location(4) inner_radius: f32,
    @location(5) thickness: f32,
    @builtin(position) clip_position: vec4<f32>,
};

struct WindowUniform {
    dim: vec2<f32>,
};

struct InstanceInput {
    @location(0) top_left_anchor: vec2<f32>,
    @location(1) top_left_offset: vec2<f32>,
    @location(2) bottom_right_anchor: vec2<f32>,
    @location(3) bottom_right_offset: vec2<f32>,
    @location(4) top_right_color: vec4<f32>,
    @location(5) top_left_color: vec4<f32>,
    @location(6) bottom_right_color: vec4<f32>,
    @location(7) bottom_left_color: vec4<f32>,
    @location(8) radius: f32,
    @location(9) inner_radius: f32,
    @location(10) thickness: f32,
}

@group(0) @binding(0)
var<uniform> window: WindowUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    in: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let top_left = in.top_left_anchor * window.dim + in.top_left_offset;
    let bottom_right = in.bottom_right_anchor * window.dim + in.bottom_right_offset;
    let size = bottom_right - top_left;

    var pos = top_left + vec2<f32>(
        f32(vi % u32(2)),
        f32(vi / u32(2))
    ) * size;
    pos = pos / window.dim * 2.0 - 1.0;
    out.clip_position = vec4<f32>(pos.x, -pos.y, 0.0, 1.0);

    if vi == u32(0) {
        out.color = in.top_left_color;
    } else if vi == u32(1) {
        out.color = in.top_right_color;
    } else if vi == u32(2) {
        out.color = in.bottom_left_color;
    } else if vi == u32(3) {
        out.color = in.bottom_right_color;
    }

    out.corner = size / 2.0;
    out.center = top_left + out.corner;
    out.radius = in.radius;
    out.inner_radius = in.inner_radius;
    out.thickness = in.thickness;

    return out;
}

// fragment shader

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    var color = in.color;

    let edge = 0.5;

    let dist = distance_from_rect(in.clip_position.xy, in.center, in.corner, in.radius);
    color.a *= 1.0 - smoothstep(-min(edge, in.radius), edge, dist);

    if in.thickness > 0.0 {
        let dist2 = distance_from_rect(in.clip_position.xy, in.center, in.corner - in.thickness, in.inner_radius);
        color.a *= smoothstep(-min(edge, in.inner_radius), edge, dist2);
    }

    return color;
}

fn distance_from_rect(pixel_pos: vec2<f32>, rect_center: vec2<f32>, rect_corner: vec2<f32>, radius: f32) -> f32 {
    // vec from center to pixel
    let p = pixel_pos - rect_center;
    // vec from inner rect corner to pixel
    let q = abs(p) - (rect_corner - radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) - radius;
}
