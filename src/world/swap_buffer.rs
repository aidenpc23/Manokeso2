use std::{
    ops::{Add, Mul, Sub},
    slice::ChunksExact,
};

pub struct SwapBuffer<T: Add> {
    width: usize,
    cur: Vec<T>,
    other: Vec<T>,
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> SwapBuffer<T> {
    pub fn from_array(width: usize, default: Vec<T>) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            cur: default.clone(),
            other: default,
        }
    }

    pub fn interpolate_towards<D>(&mut self, x: usize, y: usize, value: T, delta: D)
    where
        T: Mul<D, Output = T>,
    {
        let cur = self.cur[y * self.width + x];
        self.other[y * self.width + x] = cur + (value - cur) * delta;
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.cur[y * self.width + x]
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.cur, &mut self.other);
    }

    pub fn rows(&self, from: usize, to: usize) -> ChunksExact<'_, T> {
        self.cur[from * self.width .. to * self.width].chunks_exact(self.width)
    }
}

