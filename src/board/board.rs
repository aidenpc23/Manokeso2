use serde::{Deserialize, Serialize};

use crate::{
    board_attrs,
    rsc::{ENERGY_RANGE, REACTIVITY_RANGE},
    util::point::Point,
};

use super::{encode_alpha, gen::SwapBufferGen, swap_buffer::SwapBuffer};

board_attrs!(BoardBufs, BoardViewBufs, [
    connex_numbers: u32,
    stability: f32,
    reactivity: f32,
    energy: f32,
    alpha: u64,
    beta: u64,
    gamma: f32,
    omega: f32,
    delta: u64
]);

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub pos: Point<f32>,
    pub width: usize,
    pub height: usize,
    pub bufs: BoardBufs,
    pub total_energy: f32,
}

impl Board {
    pub fn new(pos: Point<f32>, width: usize, height: usize) -> Board {
        let mut gen = (width, height);

        let mut rng = rand::thread_rng();

        let stability = gen.gen_map_base([0.6, 0.2], [0.6, 0.0], 0.058, 0.015, 0.06);
        let connex_numbers = SwapBuffer::from_arr(
            stability.r.iter().map(|a| (a * 20.0) as u32).collect(),
            width,
        );
        let reactivity = gen.gen_map(REACTIVITY_RANGE, 0.05);
        let energy = gen.gen_map(ENERGY_RANGE, 0.01);
        let alpha = SwapBuffer::from_arr(
            vec![encode_alpha(0, 0, 0.0, 0.0, 0.0); width * height],
            width,
        );
        let beta = SwapBuffer::from_arr(vec![0; width * height], width);
        let gamma = SwapBuffer::from_arr(vec![0.0; width * height], width);
        let omega = SwapBuffer::from_arr(vec![0.0; width * height], width);
        // let delta = SwapBuffer::from_rand(&mut rand::thread_rng(), width, height, [0, 10000000000]);
        let delta = SwapBuffer::gen_delta(&mut rng, width, height);

        let total_energy = energy.r.iter().sum();

        Board {
            pos,
            width,
            height,
            bufs: BoardBufs {
                connex_numbers,
                stability,
                reactivity,
                energy,
                alpha,
                beta,
                gamma,
                omega,
                delta,
            },
            total_energy,
        }
    }

    pub fn swap(&mut self, pos1: Point<usize>, pos2: Point<usize>) {
        let pos1 = pos1.index(self.width);
        let pos2 = pos2.index(self.width);
        self.bufs.swap_cells(pos1, pos2);
    }

    pub fn player_can_swap(&self, pos1: Point<usize>, pos2: Point<usize>) -> bool {
        let pos1 = pos1.index(self.width);
        let pos2 = pos2.index(self.width);

        if (self.bufs.connex_numbers.r[pos1] > 20 && self.bufs.stability.r[pos1] > 0.8)
            || (self.bufs.connex_numbers.r[pos2] > 20 && self.bufs.stability.r[pos2] > 0.8)
        {
            false
        } else {
            true
        }
    }
}
