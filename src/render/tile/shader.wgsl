// Vertex shader

struct VertexOutput {
    @location(0) i: u32,
    @location(1) rgb: vec3<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

struct InstanceInput {
    @location(0) connex_number: u32,
    @location(1) stability: f32,
    @location(2) reactivity: f32,
    @location(3) energy: f32,
};

struct CameraUniform {
    pos: vec2<f32>,
    proj: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TileViewUniform {
    pos: vec2<f32>,
    width: u32,
};

@group(0) @binding(1)
var<uniform> tile_view: TileViewUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    @builtin(instance_index) i: u32,
    in: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.i = i;

    var pos = vec2<f32>(f32(vi % u32(2)), f32(vi / u32(2)));
    pos -= camera.pos;
    pos.x += f32(i % tile_view.width);
    pos.y += f32(i / tile_view.width);
    pos += tile_view.pos;
    pos *= camera.proj;
    out.clip_position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);

    var r = abs(in.reactivity);
    var s = in.stability;
    var e = min(in.energy * 0.011, 1.0);
    var stable = 1.0;
    if s > 0.80 && in.connex_number >= u32(10) {
        stable = 0.2;
    }
    var con0 = 1.0;
    if in.connex_number == u32(0) {
        con0 = 0.7;
    }
    var hsv = vec3<f32>(
        (f32(in.connex_number) * 0.027 + (0.236)) % 1.0,  // Should be multiplied by 0.035
        (0.6 + 0.4 * e) * con0,
        (0.1 * (1.0 - s) + 0.8 * e + 0.1) * stable * con0
        );
    
    out.rgb = hsv_to_rgb(hsv);

    out.rgb = color_shift(out.rgb, vec3<f32>(235.0/255.0, 89.0/255.0, 63.0/255.0), 0.15 * r * con0);

    return out;
}

// @vertex
// fn vs_main(
//     @builtin(vertex_index) vi: u32,
//     @builtin(instance_index) i: u32,
//     in: InstanceInput,
// ) -> VertexOutput {
//     var out: VertexOutput;
//     out.i = i;

//     var pos = vec2<f32>(f32(vi % u32(2)), f32(vi / u32(2)));
//     pos -= camera.pos;
//     pos.x += f32(i % tile_view.width);
//     pos.y += f32(i / tile_view.width);
//     pos += tile_view.pos;
//     pos *= camera.proj;
//     out.clip_position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);

//     var r = (min(max(in.reactivity, -1.0), 1.0) + 1.0) / 2.0;
//     var s = (min(max(in.stability, -1.0), 1.0) + 1.0) / 2.0;
//     out.rgb = vec3<f32>(r, s, 0.0);

//     return out;
// }

// Fragment shader

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.rgb, 1.0);
}

fn rgb_to_hsv(color: vec3<f32>) -> vec3<f32> {
    let R = color.x;
    let G = color.y;
    let B = color.z;

    let M = max(R, max(G, B));
    let m = min(R, min(G, B));
    let V = M;
    let d = M - m;

    let d_is_zero = f32(d == 0.0);
    let S = (1.0 - d_is_zero) * d / M;
    
    var H = 0.1667;
    let M_is_R = f32(M == R);
    let M_is_G = f32(M == G);
    let M_is_B = f32(M == B);

    H = M_is_R * ((G - B) / d) * H + M_is_G * (2.0 + (B - R) / d) * H + M_is_B * (4.0 + (R - G) / d) * H;
    let H_neg = f32(H < 0.0);
    H = (1.0 - H_neg) * H + H_neg * (H + 1.0);

    return vec3<f32>(H, S, V);
}

fn hsv_to_rgb(color: vec3<f32>) -> vec3<f32> {
    var H = color.x;
    let S = color.y;
    let V = color.z;

    H *= 6.0;
    let I = floor(H);
    let F = H - I;
    let M = V * (1.0 - S);
    let N = V * (1.0 - S * F);
    let K = V * (1.0 - S * (1.0 - F));

    let is0 = f32(I == 0.0);
    let is1 = f32(I == 1.0);
    let is2 = f32(I == 2.0);
    let is3 = f32(I == 3.0);
    let is4 = f32(I == 4.0);
    let is5 = f32(I == 5.0);

    let resultR = is0 * V + is1 * N + is2 * M + is3 * M + is4 * K + is5 * V;
    let resultG = is0 * K + is1 * V + is2 * V + is3 * N + is4 * M + is5 * M;
    let resultB = is0 * M + is1 * M + is2 * K + is3 * V + is4 * V + is5 * N;

    return vec3<f32>(resultR, resultG, resultB);
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a * (1.0 - t) + b * t;
}

fn color_shift(initial_color: vec3<f32>, end_color: vec3<f32>, step: f32) -> vec3<f32> {
    let shifted_color = vec3<f32>(
        lerp(initial_color.x, end_color.x, step),
        lerp(initial_color.y, end_color.y, step),
        lerp(initial_color.z, end_color.z, step),
    );

    return shifted_color;
}

fn hue_shift(initial_color: vec3<f32>, end_color: vec3<f32>, step: f32) -> vec3<f32> {
    let initial_hsv = rgb_to_hsv(initial_color);
    let end_hsv = rgb_to_hsv(end_color);

    let shifted_hsv = vec3<f32>(
        lerp(initial_hsv.x, end_hsv.x, step),
        initial_hsv.y,
        initial_hsv.z
    );

    return hsv_to_rgb(shifted_hsv);
}

fn hue_shift_hsv(initial_hsv: vec3<f32>, end_hsv: vec3<f32>, step: f32) -> vec3<f32> {
    let diff = end_hsv.x - initial_hsv.x;
    
    var adjusted_diff = diff;
    if diff > 0.5 {
        adjusted_diff -= 1.0;
    } else if diff < -0.5 {
        adjusted_diff += 1.0;
    };
    
    let shifted_hue = initial_hsv.x + adjusted_diff * step;
    
    var wrapped_hue = shifted_hue;
    
    if shifted_hue < 0.0 {
        wrapped_hue += 1.0;
    } else if shifted_hue > 1.0 {
        wrapped_hue -= 1.0;
    };
    
    return vec3<f32>(
        wrapped_hue,
        initial_hsv.y,
        initial_hsv.z
    );
}

