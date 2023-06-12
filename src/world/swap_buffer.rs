use std::ops::{Add, Sub, Mul};

pub struct SwapBuffer<T: Add> {
    buffers: [Vec<Vec<T>>; 2],
    curr_buff: usize,
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> SwapBuffer<T> {

    pub fn from_array(default: Vec<Vec<T>>) -> SwapBuffer<T> {
        SwapBuffer {
            buffers: [default.clone(), default],
            curr_buff: 0,
        }
    }

    pub fn current(&self) -> &Vec<Vec<T>> {
        &self.buffers[self.curr_buff]
    }

    pub fn interpolate_towards<D>(&mut self, x: usize, y: usize, value: T, delta: D) where T : Mul<D,Output = T> {
        self.add(x, y, (value - self.get(x, y)) * delta);
    }

    pub fn add(&mut self, x: usize, y: usize, value: T) {
        self.set(x, y, self.get(x, y) + value);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.buffers[1-self.curr_buff][x][y] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.buffers[self.curr_buff][x][y]
    }
    
    pub fn swap(&mut self) {
        self.curr_buff = 1 - self.curr_buff;
    }
}