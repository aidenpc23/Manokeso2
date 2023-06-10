use itertools::izip;
use ndarray::Array2;
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
            width,
            height,
            connex_numbers: Array2::random((width, height), Uniform::new(0, 200)),
            stability: Array2::random((width, height), Uniform::new(0., 1.)),
            reactivity: Array2::random((width, height), Uniform::new(-1., 1.)),
            energy: Array2::random((width, height), Uniform::new(0., 150.)),
        }
    }

    pub fn render_attributes(&self) -> Vec<[f32; 4]> {
        izip!(
            &self.connex_numbers,
            &self.stability,
            &self.reactivity,
            &self.energy
        )
        .map(|(c, s, r, e)| [c.clone() as f32, s.clone(), r.clone(), e.clone()])
        .collect()
    }

    pub fn update(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {}
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

