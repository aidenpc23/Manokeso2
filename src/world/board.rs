use crate::{
    rsc::{ENERGY_RANGE, REACTIVITY_RANGE},
    util::point::Point,
};

use super::{gen::SwapBufferGen, swap_buffer::SwapBuffer, encode_alpha};

pub struct Board {
    pub pos: Point<f32>,
    pub width: usize,
    pub height: usize,
    pub connex_numbers: SwapBuffer<u32>,
    pub stability: SwapBuffer<f32>,
    pub reactivity: SwapBuffer<f32>,
    pub energy: SwapBuffer<f32>,
    pub alpha: SwapBuffer<u64>,
    pub beta: SwapBuffer<u64>,
    pub gamma: SwapBuffer<f32>,
    pub delta: SwapBuffer<f32>,
    pub omega: SwapBuffer<f32>,
    pub dirty: bool,
    pub total_energy: f32,
}

impl Board {
    pub fn new(pos: Point<f32>, width: usize, height: usize) -> Board {
        let mut gen = (width, height);

        let stability = gen.gen_map_base([0.6, 0.2], [0.6, 0.0], 0.058, 0.015, 0.06);
        let connex_numbers = SwapBuffer::from_arr(
            stability.read().iter().map(|a| (a * 20.0) as u32).collect(),
            width,
        );
        let reactivity = gen.gen_map(REACTIVITY_RANGE, 0.05);
        let energy = gen.gen_map(ENERGY_RANGE, 0.01);
        let alpha = SwapBuffer::from_arr(vec![encode_alpha(0, 0, 0.0, 0.0, 0.0); width * height], width);
        let beta = SwapBuffer::from_arr(vec![0; width * height], width);
        let gamma = SwapBuffer::from_arr(vec![0.0; width * height], width);
        let delta = SwapBuffer::from_arr(vec![0.0; width * height], width);
        let omega = SwapBuffer::from_arr(vec![0.0; width * height], width);

        let total_energy = energy.read().iter().sum();

        Board {
            pos,
            width,
            height,
            connex_numbers,
            stability,
            reactivity,
            energy,
            alpha,
            beta,
            gamma,
            delta,
            omega,
            total_energy,
            dirty: true,
        }
    }

    pub fn player_swap(&mut self, pos1: Point<usize>, pos2: Point<usize>) {
        if self.player_can_swap(pos1, pos2) {
            self.swap(pos1, pos2);
        }
    }

    pub fn swap(&mut self, pos1: Point<usize>, pos2: Point<usize>) {
        let pos1 = pos1.index(self.width);
        let pos2 = pos2.index(self.width);

        self.connex_numbers.swap_cell(pos1, pos2);
        self.stability.swap_cell(pos1, pos2);
        self.reactivity.swap_cell(pos1, pos2);
        self.energy.swap_cell(pos1, pos2);

        self.dirty = true;
    }

    pub fn player_can_swap(&self, pos1: Point<usize>, pos2: Point<usize>) -> bool {
        let pos1 = pos1.index(self.width);
        let pos2 = pos2.index(self.width);

        if (self.connex_numbers.read()[pos1] > 20 && self.stability.read()[pos1] > 0.8)
            || (self.connex_numbers.read()[pos2] > 20 && self.stability.read()[pos2] > 0.8)
        {
            false
        } else {
            true
        }
    }
}
