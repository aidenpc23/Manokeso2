use std::{
    ops::{Add, Mul, Sub},
    slice::ChunksExact,
};

pub struct SwapBuffer<T> {
    width: usize,
    read: Vec<T>,
    write: Vec<T>,
}

impl<T> SwapBuffer<T> {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.read, &mut self.write);
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.read[y * self.width + x]
    }
    pub fn rows(&self, from: usize, to: usize) -> ChunksExact<'_, T> {
        self.read[from * self.width .. to * self.width].chunks_exact(self.width)
    }
    pub fn bufs(&mut self) -> (&Vec<T>, &mut Vec<T>) {
        (&self.read, &mut self.write)
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> SwapBuffer<T> {
    pub fn from_array(width: usize, default: Vec<T>) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            read: default.clone(),
            write: default,
        }
    }

    pub fn interpolate_towards<D>(&mut self, x: usize, y: usize, value: T, delta: D)
    where
        T: Mul<D, Output = T>,
    {
        let cur = self.read[y * self.width + x];
        self.write[y * self.width + x] = cur + (value - cur) * delta;
    }

}

