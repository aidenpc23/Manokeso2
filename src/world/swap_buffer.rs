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
    pub fn from_rand<R>(rng: &mut R, width: usize, height: usize, range: [T; 2]) -> SwapBuffer<T>
    where
        T: SampleUniform,
        R: Rng,
    {
        let range = Uniform::new_inclusive(range[0], range[1]);
        let arr: Vec<T> = (0..height * width).map(|_| rng.sample(&range)).collect();
        SwapBuffer {
            width,
            read: arr.clone(),
            write: arr,
        }
    }
}

pub trait SwapBufferGen {
    fn rand_swap_buf<T : SampleUniform + Copy>(&mut self, range: [T; 2]) -> SwapBuffer<T>;
}

impl<R: Rng> SwapBufferGen for (&mut R, usize, usize) {
    fn rand_swap_buf<T : SampleUniform + Copy>(&mut self, range: [T; 2]) -> SwapBuffer<T> {
        SwapBuffer::from_rand(self.0, self.1, self.2, range)
    }
}

