use ndarray::Array2;
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;

use crate::render::Instance;

pub struct Board {
    width: usize,
    height: usize,
    connex_numbers: Array2<u32>,
    stability: Array2<f32>,
    reactivity: Array2<f32>,
    energy: Array2<f32>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            connex_numbers: Array2::random((width, height), Uniform::new(0, 200)),
            stability: Array2::random((width, height), Uniform::new(0., 1.)),
            reactivity: Array2::random((width, height), Uniform::new(-1., 1.)),
            energy: Array2::random((width, height), Uniform::new(0., 150.)),
        }
    }

    pub fn render_attributes(&self, xs: usize, xe: usize, ys: usize, ye: usize) -> Vec<Instance> {
        let mut attrs = Vec::with_capacity((xe-xs) * (ye-ys));
        for y in ys..ye {
            for x in xs..xe {
                attrs.push(Instance {
                    attributes: [
                        *self.connex_numbers.get((x, y)).unwrap() as f32,
                        *self.stability.get((x, y)).unwrap(),
                        *self.reactivity.get((x, y)).unwrap(),
                        *self.energy.get((x, y)).unwrap(),
                    ]
                })
            }
        }
        attrs
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

