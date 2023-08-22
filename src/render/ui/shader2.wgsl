
// vertex shader

struct VertexOutput {
    @location(0) color: vec4<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

struct WindowUniform {
    width: u32,
    height: u32
};

struct InstanceInput {
    @location(0) top_left: vec2<f32>,
    @location(1) bottom_right: vec2<f32>,
    @location(2) bottom_right: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> window: WindowUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
) -> VertexOutput {
    var out: VertexOutput;
    var pos = vec2<f32>(
        f32(vi % u32(2)) - 0.5,
        f32(vi / u32(2)) - 0.5
    );
    out.clip_position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);
    out.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    return out;
}

// fragment shader

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    var color = in.color;
    let mult = vec2<f32>(f32(window.width), f32(window.height));

    let bot_left = vec2<f32>(0.25, 0.25);
    let top_right = vec2<f32>(0.75, 0.75);
    let radius = 0.0;
    let inner_radius = 0.0;
    let thickness = 10.0;

    let center = (bot_left + top_right) / 2.0 * mult;
    let corner = top_right * mult - center;

    let edge = 0.5;

    let dist = distance_from_rect(in.clip_position.xy, center, corner, radius);
    color.a *= 1.0 - smoothstep(-min(edge, radius), edge, dist);

    let dist2 = distance_from_rect(in.clip_position.xy, center, corner - thickness, inner_radius);
    color.a *= smoothstep(-min(edge, inner_radius), edge, dist2);

    return color;
}

fn distance_from_rect(pixel_pos: vec2<f32>, rect_center: vec2<f32>, rect_corner: vec2<f32>, radius: f32) -> f32 {
    // vec from center to pixel
    let p = pixel_pos - rect_center;
    // vec from inner rect corner to pixel
    let q = abs(p) - (rect_corner - radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) - radius;
}
