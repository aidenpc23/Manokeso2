use noise::{utils::{PlaneMapBuilder, NoiseMapBuilder}, OpenSimplex};
use rand::{
    distributions::{
        uniform::SampleUniform,
        Uniform,
    },
    Rng,
};
use rayon::prelude::*;

pub struct SwapBuffer<T> {
    width: usize,
    read: Vec<T>,
    write: Vec<T>,
}

impl<T: Sync> SwapBuffer<T> {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.read, &mut self.write);
    }
    pub fn par_rows(&self, from: usize, to: usize) -> rayon::slice::ChunksExact<'_, T> {
        self.read[from * self.width..to * self.width].par_chunks_exact(self.width)
    }
    pub fn rows(&self, from: usize, to: usize) -> std::slice::ChunksExact<'_, T> {
        self.read[from * self.width..to * self.width].chunks_exact(self.width)
    }
    pub fn bufs(&mut self) -> (&Vec<T>, &mut Vec<T>) {
        (&self.read, &mut self.write)
    }
    pub fn read(&self) -> &Vec<T> {
        &self.read
    }
}

impl<T : Copy> SwapBuffer<T> {
    pub fn from_rand<R>(rng: &mut R, width: usize, height: usize, min: T, max: T) -> SwapBuffer<T>
    where
        T: SampleUniform,
        R: Rng,
    {
        let range = Uniform::new_inclusive(min, max);
        let arr: Vec<T> = (0..height * width).map(|_| rng.sample(&range)).collect();
        SwapBuffer {
            width,
            read: arr.clone(),
            write: arr,
        }
    }

    pub fn from(base: Vec<T>, width: usize) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            read: base.clone(),
            write: base,
        }
    }
}

pub trait SwapBufferGen<T> {
    fn gen_map(&mut self, min: T, max: T, frequency: f32) -> SwapBuffer<T>;
}

impl SwapBufferGen<f32> for (usize, usize) {
    fn gen_map(&mut self, min: f32, max: f32, frequency: f32) -> SwapBuffer<f32> {
        SwapBuffer::from(simplex_noise(self.0, self.1, min, max, frequency), self.0)
    }
}

impl SwapBufferGen<u32> for (usize, usize) {
    fn gen_map(&mut self, min: u32, max: u32, frequency: f32) -> SwapBuffer<u32> {
        SwapBuffer::from(simplex_noise_u32(self.0, self.1, min, max, frequency), self.0)
    }
}

pub trait SwapBufferRandGen {
    fn rand_swap_buf<T : SampleUniform + Copy>(&mut self, min: T, max: T) -> SwapBuffer<T>;
}

impl<R: Rng> SwapBufferRandGen for (&mut R, usize, usize) {
    fn rand_swap_buf<T : SampleUniform + Copy>(&mut self, min: T, max: T) -> SwapBuffer<T> {
        SwapBuffer::from_rand(self.0, self.1, self.2, min, max)
    }
}

pub fn simplex_noise(width: usize, height: usize, min: f32, max: f32, frequency: f32) -> Vec<f32> {
    let open_simplex = OpenSimplex::new(375);
    
    let noise_map = PlaneMapBuilder::<_, 2>::new(&open_simplex)
        .set_size(width, height)
        .set_x_bounds(min.into(), max.into())
        .set_y_bounds(min.into(), max.into())
        .build();
    
    let mut result = Vec::<f32>::new();

    for y in 0..height {
        for x in 0..width {
            let val = noise_map.get_value((x as f32 * frequency) as usize, (y as f32 * frequency) as usize);
            result.push(val as f32);
        }
    }

    result
}


pub fn simplex_noise_u32(width: usize, height: usize, min: u32, max: u32, frequency: f32) -> Vec<u32> {
    let open_simplex = OpenSimplex::new(375);

    let noise_map = PlaneMapBuilder::<_, 2>::new(&open_simplex)
        .set_size(width, height)
        .set_x_bounds(min.into(), max.into())
        .set_y_bounds(min.into(), max.into())
        .build();
    
    let mut result = Vec::<u32>::new();

    for y in 0..height {
        for x in 0..width {
            let val = noise_map.get_value((x as f32 * frequency) as usize, (y as f32 * frequency) as usize);
            result.push(val as u32);
        }
    }

    result
}
