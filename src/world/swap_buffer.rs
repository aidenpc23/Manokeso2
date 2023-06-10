use ndarray::{Array2, iter::Indices};

pub struct SwapBuffer<T> {
    buffers: [Array2<T>; 2],
    curr_buff: usize,
}

impl<T: Copy> SwapBuffer<T> {
    pub fn from_elem(width: usize, height: usize, default: T) -> SwapBuffer<T> {
        SwapBuffer {
            buffers: [Array2::from_elem((width, height), default),
                Array2::from_elem((width, height), default)],
            curr_buff: 0,
        }
    }

    pub fn from_array(default: Array2<T>) -> SwapBuffer<T> {
        SwapBuffer {
            buffers: [default.clone(), default],
            curr_buff: 0,
        }
    }

    pub fn set(&mut self, indices: (usize, usize), elem: T) {
        self.buffers[self.curr_buff][indices] = elem;
    }

    pub fn get(&self, indices: (usize, usize)) -> T {
        self.buffers[self.curr_buff].get(indices)
            .expect(&format!("Could not get element at ({}, {})", indices.0, indices.1)).to_owned()
    }

    pub fn swap(&mut self) {
        self.curr_buff = 1 - self.curr_buff;
    }
}