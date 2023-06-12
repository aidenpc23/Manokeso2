use ndarray::Array2;
use ndarray_rand::RandomExt;
use rand::{distributions::Uniform, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::render::Instance;

pub struct Board {
    pub pos: [f32; 2],
    width: usize,
    height: usize,
    pub test: Array2<Instance>,
    // pub connex_numbers: Array2<u32>,
    // pub stability: Array2<f32>,
    // pub reactivity: Array2<f32>,
    // pub energy: Array2<f32>,
}

impl Board {
    pub fn new(pos: [f32; 2], width: usize, height: usize) -> Board {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        Board {
            test: Array2::from_shape_fn((height, width), |(x, y)| Instance {
                connex_number: rng.gen_range(0..200),
                stability: rng.gen_range(0.0..1.0),
                reactivity: rng.gen_range(-1.0..1.0),
                energy: rng.gen_range(0.0..150.0),
            }),
            pos,
            width,
            height,
            // connex_numbers: Array2::random((width, height), Uniform::new(0, 200)),
            // stability: Array2::random((width, height), Uniform::new(0., 1.)),
            // reactivity: Array2::random((width, height), Uniform::new(-1., 1.)),
            // energy: Array2::random((width, height), Uniform::new(0., 150.)),
        }
    }

    pub fn update(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                todo!();
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
