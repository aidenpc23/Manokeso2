use ndarray::{Array2};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;

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
            width: width,
            height: height,
            connex_numbers: Array2::random((width, height), Uniform::new(0, 200)),
            stability: Array2::random((width, height), Uniform::new(0., 1.)),
            reactivity: Array2::random((width, height), Uniform::new(-1., 1.)),
            energy: Array2::random((width, height), Uniform::new(0., 150.)),
        }
    }

    pub fn render_attributes(&self, x: usize, y: usize) -> [f32; 4] {
        self._render_attributes(x, y).expect(&format!("Failed to get attributes at {}, {}", x, y))
    }

    fn _render_attributes(&self, x: usize, y: usize) -> Option<[f32; 4]> {
        Some([
            (*self.connex_numbers.get((x, y))? as f32)/200.,
            *self.stability.get((x, y))?,
            (*self.reactivity.get((x, y))? + 1.)/2.,
            (*self.energy.get((x, y))?)/150.,
        ])
    }

    pub fn update() {

    }
}