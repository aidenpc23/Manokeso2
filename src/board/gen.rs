use super::swap_buffer::SwapBuffer;
use crate::util::noise::{simplex_noise, simplex_simplex_noise, NoiseNum};
use itertools::izip;
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    Rng,
};

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

impl SwapBuffer<u64> {
    pub fn gen_delta<R: Rng>(rng: &mut R, width: usize, height: usize) -> SwapBuffer<u64> {
        let mut base = Vec::new();

        for _ in 0..(width * height) {
            if rng.gen_range(0..=10000) < 20 {
                let mut bitmask: u64 = 0;
                let bit_to_flip = rng.gen_range(0..64);
                bitmask |= 1 << bit_to_flip;

                for _ in 0..63 {
                    if rng.gen_range(0..=100) < 1 {
                        let additional_bit_to_flip = rng.gen_range(0..64);
                        bitmask |= 1 << additional_bit_to_flip;
                    } else {
                        break;
                    }
                }

                base.push(bitmask);
            } else {
                base.push(0);
            }
        }

        SwapBuffer::from_arr(base, width)
    }
}
