use std::ops::{Add, Mul, Sub};
use noise::{NoiseFn, OpenSimplex};

pub fn simplex_noise<T: NoiseNum>(
    width: usize,
    height: usize,
    range: [T; 2],
    cut: [f64; 2],
    frequency: f64,
) -> Vec<T> {
    let open_simplex = OpenSimplex::new(1234567);
    let mut result = Vec::new();

    let mut smax: f64 = f64::MIN;
    let mut smin: f64 = f64::MAX;
    for y in 0..height {
        for x in 0..width {
            let val = open_simplex.get([x as f64 * frequency, y as f64 * frequency]);
            result.push(val);
            if val > smax {
                smax = val;
            } else if val < smin {
                smin = val;
            }
        }
    }
    let srange = smax - smin;
    let cut = [cut[0], cut[1] + 1.0];
    let crange = cut[1] + cut[0];
    let mult = crange / srange;
    let rrange = (range[1] - range[0]).to_f64();
    result
        .iter()
        .map(|x| T::from_f64(((x - smin) * mult - cut[0]).clamp(0.0, 1.0) * rrange) + range[0])
        .collect()
}

pub fn simplex_simplex_noise(
    width: usize,
    height: usize,
    range: [f32; 2],
    range2: [f32; 2],
    cut: [f64; 2],
    frequency: f64,
    freqfreq: f64,
) -> Vec<f32> {
    let noise1 = simplex_noise(width, height, range, cut, frequency);
    let noise2 = simplex_noise(width, height, range2, [0.0, 0.7], freqfreq);

    noise1
        .iter()
        .zip(noise2)
        .into_iter()
        .map(|(a, b)| a * b)
        .collect()
}

pub trait NoiseNum:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Sized + Copy
{
    fn to_f64(&self) -> f64;
    fn from_f64(val: f64) -> Self;
}

impl NoiseNum for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(val: f64) -> Self {
        val as f32
    }
}

impl NoiseNum for u32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(val: f64) -> Self {
        val as u32
    }
}
