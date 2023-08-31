use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SwapBuffer<T> {
    width: usize,
    pub r: Vec<T>,
    pub w: Vec<T>,
}

impl<T: Sync + Send + Copy> SwapBuffer<T> {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.r, &mut self.w);
    }
    pub fn par_rows(&self, from: usize, to: usize) -> rayon::slice::ChunksExact<'_, T> {
        self.r[from * self.width..to * self.width].par_chunks_exact(self.width)
    }
}

impl<T: Copy> SwapBuffer<T> {
    pub fn from_arr(base: Vec<T>, width: usize) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            r: base.clone(),
            w: base,
        }
    }
}

