use rayon::prelude::*;

pub struct SwapBuffer<T> {
    width: usize,
    read: Vec<T>,
    write: Vec<T>,
}

impl<T: Sync + Send + Copy> SwapBuffer<T> {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.read, &mut self.write);
    }
    pub fn par_rows(&self, from: usize, to: usize) -> rayon::slice::ChunksExact<'_, T> {
        self.read[from * self.width..to * self.width].par_chunks_exact(self.width)
    }
    pub fn bufs(&mut self) -> (&Vec<T>, &mut Vec<T>) {
        (&self.read, &mut self.write)
    }
    pub fn read(&self) -> &Vec<T> {
        &self.read
    }
    pub fn swap_cell(&mut self, pos1: usize, pos2: usize) {
        self.read.swap(pos1, pos2);
    }
    pub fn god_get(&self, pos: usize) -> &T {
        &self.read[pos]
    }
    pub fn god_set(&mut self, pos: usize, val: T) {
        self.read[pos] = val;
    }
}

impl<T : Copy> SwapBuffer<T> {
    pub fn from_arr(base: Vec<T>, width: usize) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            read: base.clone(),
            write: base,
        }
    }
}
