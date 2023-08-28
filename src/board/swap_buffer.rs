use std::ops::AddAssign;

use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SwapBuffer<T> {
    width: usize,
    pub r: Vec<T>,
    pub w: Vec<T>,
}

impl<T: Sync + Send + Copy + AddAssign> SwapBuffer<T> {
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

impl SwapBuffer<u64> {
    pub fn gen_delta<R: Rng>(rng: &mut R, width: usize, height: usize) -> SwapBuffer<u64> {
        let mut base = Vec::new();

        for _ in 0..(width * height) {
            if rng.gen_range(0..=10000) < 20 {
                let mut bitmask: u64 = 0;
                let bit_to_flip = rng.gen_range(0..64);
                bitmask |= 1 << bit_to_flip;

                for _ in 0..63 {
                    if rng.gen_range(0..=100) < 1 {
                        let additional_bit_to_flip = rng.gen_range(0..64);
                        bitmask |= 1 << additional_bit_to_flip;
                    } else {
                        break;
                    }
                }

                base.push(bitmask);
            } else {
                base.push(0);
            }
        }

        SwapBuffer::from_arr(base, width)
    }
}
