// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @location(0) i: u32,
    @location(1) rgb: vec3<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

struct InstanceInput {
    @location(1) connex_number: u32,
    @location(2) stability: f32,
    @location(3) reactivity: f32,
    @location(4) energy: f32,
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
    @builtin(instance_index) i: u32,
    vertex: VertexInput,
    in: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.i = i;

    var pos = vertex.position;
    pos -= camera.pos;
    pos.x += f32(i % tile_view.width);
    pos.y += f32(i / tile_view.width);
    pos += tile_view.pos;
    pos *= camera.proj;
    out.clip_position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);

    var r = (in.reactivity+1.0) * 0.5;
    var s = in.stability;
    var e = min(in.energy * 0.0066, 1.0);
    var hsv = vec3<f32>(
        (f32(in.connex_number) * 0.005 + 0.236) % 1.0,
        0.6 + 0.4 * e,
        0.2 + 0.8 * s * e * 0.7
        );
    let hsv_reac = hue_shift_hsv(hsv, vec3<f32>(0.972, 0.0, 0.0), r * 0.5);
    // let rgb_stab = color_shift(rgb, vec3<f32>(27.0/255.0, 28.0/255.0, 41.0/255.0), 0.1 * s);
    // let rgb_en = color_shift(rgb_stab, vec3<f32>(238./255., 1.0, 0.0), 0.2 * e);
    out.rgb = hsv_to_rgb(hsv_reac);

    return out;
}

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
    let shifted_hsv = vec3<f32>(
        lerp(initial_color.x, end_color.x, step),
        lerp(initial_color.y, end_color.y, step),
        lerp(initial_color.z, end_color.z, step),
    );

    return hsv_to_rgb(shifted_hsv);
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
    let shifted_hsv = vec3<f32>(
        lerp(initial_hsv.x, end_hsv.x, step),
        initial_hsv.y,
        initial_hsv.z
    );
    
    return shifted_hsv;
}