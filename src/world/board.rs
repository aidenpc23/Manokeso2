use std::time::Duration;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::swap_buffer::SwapBuffer;

const BASE_KERNEL: [[f32; 3]; 3] = [[0.5, 1.0, 0.5], [1.0, 2.0, 1.0], [0.5, 1.0, 0.5]];

pub struct Board {
    pub pos: [f32; 2],
    width: usize,
    height: usize,
    pub connex_numbers: SwapBuffer<u32>,
    pub conductivity: SwapBuffer<f32>,
    pub reactivity: SwapBuffer<f32>,
    pub energy: SwapBuffer<f32>,
}

impl Board {
    pub fn new(pos: [f32; 2], width: usize, height: usize) -> Board {
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        Board {
            pos,
            width,
            height,
            connex_numbers: SwapBuffer::from_array(
                (0..height)
                    .map(|_| (0..width).map(|_| rng.gen_range(0..200)).collect())
                    .collect(),
            ),
            conductivity: SwapBuffer::from_array(
                (0..height)
                    .map(|_| (0..width).map(|_| rng.gen_range(0.0..1.0)).collect())
                    .collect(),
            ),
            reactivity: SwapBuffer::from_array(
                (0..height)
                    .map(|_| (0..width).map(|_| rng.gen_range(-1.0..1.0)).collect())
                    .collect(),
            ),
            energy: SwapBuffer::from_array(
                (0..height)
                    .map(|_| (0..width).map(|_| rng.gen_range(0.0..150.0)).collect())
                    .collect(),
            ),
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        // for x in 0..self.width {
        //     for y in 0..self.height {

        //     }
        // }

        let mut s = 0.0;
        let d = delta.as_secs_f32();

        for x in 0..self.width {
            for y in 0..self.height {
                let mut sum = 0.;
                let mut weights = 0.;
                for dx in 0..=2 {
                    for dy in 0..=2 {
                        if x + dx >= 1
                            && y + dy >= 1
                            && x + dx - 1 < self.width
                            && y + dy - 1 < self.height
                        {
                            let cond = (1. - self.conductivity.get(x + dx - 1, y + dy - 1))
                                * (if !((dx == 0) & (dy == 0)) { 1. } else { 0. });
                            sum += BASE_KERNEL[dx][dy]
                                * self.energy.get(x + dx - 1, y + dy - 1)
                                * cond;
                            weights += BASE_KERNEL[dx][dy] * cond;
                        }
                    }
                }
                let t = sum / weights;
                s += t;
                self.energy.interpolate_towards(x, y, t, d);
            }
        }
        self.energy.swap();

        // println!("{:?}", s)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
