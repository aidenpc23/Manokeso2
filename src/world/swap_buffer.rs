use rayon::prelude::*;

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
    pub fn read(&self) -> &Vec<T> {
        &self.r
    }
    pub fn swap_cell(&mut self, pos1: usize, pos2: usize) {
        self.r.swap(pos1, pos2);
    }
    pub fn get(&self, pos: usize) -> &T {
        &self.r[pos]
    }
    pub fn set(&mut self, pos: usize, val: T) {
        self.r[pos] = val;
    }
}

impl<T : Copy> SwapBuffer<T> {
    pub fn from_arr(base: Vec<T>, width: usize) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            r: base.clone(),
            w: base,
        }
    }
}
