
// vertex shader

struct VertexOutput {
    @location(0) color: vec4<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

struct WindowUniform {
    width: u32,
    height: u32
};

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
    out.color = vec4<f32>(.5, .5, .5, 0.5);
    return out;
}

// fragment shader

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    var color = in.color;
    let mult = vec2<f32>(f32(window.width), f32(window.height));
    let center = vec2<f32>(0.5, 0.5) * mult;
    let corner = vec2<f32>(0.25, 0.25) * mult;

    let dist = distance_from_rect(in.clip_position.xy, center, corner, 100.0);

    if dist > 0.0 {
        color.a *= 1.0 - smoothstep(0.0, 0.75, dist);
    }

    return color;
}

fn distance_from_rect(pixel_pos: vec2<f32>, rect_center: vec2<f32>, rect_corner: vec2<f32>, corner_radius: f32) -> f32 {
    // vec from center to pixel
    let p = pixel_pos - rect_center;
    // vec from inner rect corner to pixel
    let q = abs(p) - (rect_corner - corner_radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) - corner_radius;
}
