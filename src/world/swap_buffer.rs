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
    pub fn from_arr(base: Vec<T>, width: usize) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            read: base.clone(),
            write: base,
        }
    }
}
