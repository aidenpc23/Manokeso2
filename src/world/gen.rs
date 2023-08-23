use std::ops::{Add, Mul, Sub};

use itertools::izip;
use noise::{NoiseFn, OpenSimplex};
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    Rng,
};

use super::swap_buffer::SwapBuffer;

impl<T: Copy> SwapBuffer<T> {
    #[allow(dead_code)]
    pub fn from_rand<R>(rng: &mut R, width: usize, height: usize, range: [T; 2]) -> SwapBuffer<T>
    where
        T: SampleUniform,
        R: Rng,
    {
        let range = Uniform::new_inclusive(range[0], range[1]);
        let arr: Vec<T> = (0..height * width).map(|_| rng.sample(&range)).collect();
        SwapBuffer::from_arr(arr, width)
    }
}

pub trait SwapBufferGen {
    fn gen_map<T: NoiseNum>(&mut self, range: [T; 2], frequency: f64) -> SwapBuffer<T>;
    fn gen_map_cut<T: NoiseNum>(
        &mut self,
        range: [T; 2],
        cut: [f64; 2],
        frequency: f64,
    ) -> SwapBuffer<T>;
    fn gen_map_base(
        &mut self,
        cut1: [f64; 2],
        cut: [f64; 2],
        freq1: f64,
        freq2: f64,
        freqfreq: f64,
    ) -> SwapBuffer<f32>;
}

impl SwapBufferGen for (usize, usize) {
    fn gen_map<T: NoiseNum>(&mut self, range: [T; 2], frequency: f64) -> SwapBuffer<T> {
        SwapBuffer::from_arr(
            simplex_noise(self.0, self.1, range, [0.0, 0.0], frequency),
            self.0,
        )
    }
    fn gen_map_cut<T: NoiseNum>(
        &mut self,
        range: [T; 2],
        cut: [f64; 2],
        frequency: f64,
    ) -> SwapBuffer<T> {
        SwapBuffer::from_arr(simplex_noise(self.0, self.1, range, cut, frequency), self.0)
    }
    fn gen_map_base(
        &mut self,
        cut1: [f64; 2],
        cut2: [f64; 2],
        freq1: f64,
        freq2: f64,
        freqfreq: f64,
    ) -> SwapBuffer<f32> {
        SwapBuffer::from_arr(
            izip!(
                simplex_simplex_noise(
                    self.0,
                    self.1,
                    [0.0, 1.0],
                    [0.1, 1.0],
                    cut1,
                    freq1,
                    freqfreq
                ),
                simplex_noise(self.0, self.1, [0., 0.5], cut2, freq2),
                simplex_simplex_noise(
                    self.0,
                    self.1,
                    [0., 1.25],
                    [0.0, 1.0],
                    [3.0, 0.0],
                    0.0093,
                    0.008
                ),
                simplex_noise(self.0, self.1, [0., 2.5], [5.0, 0.0], 0.002),
                simplex_noise(self.0, self.1, [0.0, 1.0], [1.0, 5.0], 0.006)
            )
            .map(|(a, b, c, d, e)| (a.max(b).max(c) * e).max(d))
            .collect(),
            self.0,
        )
    }
}

pub fn simplex_noise<T: NoiseNum>(
    width: usize,
    height: usize,
    range: [T; 2],
    cut: [f64; 2],
    frequency: f64,
) -> Vec<T> {
    let open_simplex = OpenSimplex::new(rand::random());
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
