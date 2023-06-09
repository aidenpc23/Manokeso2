

// fn rgb2hsl(c: vec3<f32>) -> vec3<f32>
// {

//     // var hsl = vec3<f32>();

//     // var cmax = max(c.r, max(c.g, c.b));
//     // var cmin = min(c.r, min(c.g, c.b));
//     // var delta = cmax - cmin;

//     // if delta == 0. {
//     //     hsl.x = 0.;
//     // } else if cmax == c.r {
//     //     hsl.x = 60. * (((c.g - c.b)/delta) % 6.);
//     // } else if cmax == c.g {
//     //     hsl.x = 60. * (((c.b - c.r)/delta) + 2.);
//     // } else if cmax == c.b {
//     //     hsl.x = 60. * (((c.r - c.g)/delta) + 4.);
//     // }

//     // hsl.z = (cmax + cmin) / 2.;

//     // if delta == 0. {
//     //     hsl.y = 0.;
//     // } else {
//     //     hsl.y = delta/(1. - abs(2.*hsl.z - 1.));
//     // }

// 	return hsl;
// }

// fn hsl2rgb( hsl: vec3<f32> ) -> vec3<f32> {
//     var rgb = vec3<f32>();
//     // var C = (1. - abs(2. * hsl.z - 1.)) * hsl.y;
//     // var X = C * (1. - abs( hsl.x/60.% 2. - 1.));
//     // var m = hsl.z - C/2.;

//     // var rgb = vec3<f32>();
//     // if hsl.x < 60./360. {
//     //     rgb = vec3(C, X, 0.);
//     // } else if hsl.x < 120. {
//     //     rgb = vec3(X, C, 0.);
//     // } else if hsl.x < 180. {
//     //     rgb = vec3(0., C, X);
//     // } else if hsl.x < 240. {
//     //     rgb = vec3(0., X, C);
//     // } else if hsl.x < 300. {
//     //     rgb = vec3(X, 0., C);
//     // }  else if hsl.x < 360. {
//     //     rgb = vec3(C, 0., X);
//     // }

//     return rgb;
// }

fn hue_shift ( color: vec3<f32>, shift: f32 ) -> vec3<f32>
{
    var YIQ: mat3x3<f32> = mat3x3(
        0.299, 0.587, 0.114,
        0.596, -0.274, -0.321,
        0.211, -0.523, 0.311,
    );

    var RGB: mat3x3<f32> = mat3x3(
        1., 0.956, 0.621,
        1., -0.272, -0.647, 
        1., -1.107, 1.705,
    );

    var h = mat3x3(
        1., 0.,         0.,
        0., cos(shift), -sin(shift),
        0., sin(shift), cos(shift),
    );


    return (RGB * h * YIQ) * color;
}