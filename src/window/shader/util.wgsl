fn rgb_to_hsv(color: vec3<f32>) -> vec3<f32> {
    let R = color.x;
    let G = color.y;
    let B = color.z;

    let M = max(R, max(G, B));
    let m = min(R, min(G, B));
    let V = M;
    let d = M - m;
    if d == 0.0 {
        return vec3(0.0, 0.0, V);
    }
    let S = d / M;
    var H = 1.0 / 6.0;
    if M == R {
        H *= 0.0 + (G - B) / d;
        H %= 1.0;
    } else if M == G {
        H *= 2.0 + (B - R) / d;
    } else if M == B {
        H *= 4.0 + (R - G) / d;
    }
    return vec3(H, S, V);
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

    if I == 0.0 {
        return vec3(V, K, M);
    } else if I == 1.0 {
        return vec3(N, V, M);
    } else if I == 2.0 {
        return vec3(M, V, K);
    } else if I == 3.0 {
        return vec3(M, N, V);
    } else if I == 4.0 {
        return vec3(K, M, V);
    } else if I == 5.0 {
        return vec3(V, M, N);
    }

    return vec3(0.0, 0.0, 0.0);
}