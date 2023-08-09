use std::time::Duration;

use rayon::prelude::*;

use crate::rsc::{CONNEX_NUMBER_RANGE, STABILITY_RANGE, REACTIVITY_RANGE, ENERGY_RANGE, UPS};

use super::{swap_buffer::SwapBuffer, gen::SwapBufferGen};

const BASE_KERNEL: [[f32; 3]; 3] = [[0.5, 1.0, 0.5], [1.0, 2.0, 1.0], [0.5, 1.0, 0.5]];
const ENERGY_FLOW_RATE: f32 = 1.0 / 100.0;

pub struct Board {
    pub pos: [f32; 2],
    width: usize,
    height: usize,
    pub connex_numbers: SwapBuffer<u32>,
    pub stability: SwapBuffer<f32>,
    pub reactivity: SwapBuffer<f32>,
    pub energy: SwapBuffer<f32>,
    pub alpha: SwapBuffer<f32>,
    pub beta: SwapBuffer<f32>,
    pub gamma: SwapBuffer<f32>,
    pub delta: SwapBuffer<f32>,
    pub omega: SwapBuffer<f32>,
    total_energy: f32, 
}

impl Board {
    pub fn new(pos: [f32; 2], width: usize, height: usize) -> Board {
        let mut gen = (width, height);

        let stability = gen.gen_map_base([0.6, 0.2], [0.6, 0.0], 0.058, 0.015, 0.025);
        let connex_numbers = SwapBuffer::from_arr(stability.read().iter().map(|a| (a * 20.0) as u32).collect(), width);
        let reactivity = gen.gen_map(REACTIVITY_RANGE, 0.05);
        let energy = gen.gen_map(ENERGY_RANGE, 0.01);
        let alpha = SwapBuffer::from_arr(vec![0.0; width*height], width);
        let beta = SwapBuffer::from_arr(vec![0.0; width*height], width);
        let gamma = SwapBuffer::from_arr(vec![0.0; width*height], width);
        let delta = SwapBuffer::from_arr(vec![0.0; width*height], width);
        let omega = SwapBuffer::from_arr(vec![0.0; width*height], width);

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
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        let mut s = self.stability.bufs();
        let mut e = self.energy.bufs();
        let (cr, cw) = self.connex_numbers.bufs();
        let mut r = self.reactivity.bufs();
        let (ar, aw) = self.alpha.bufs();
        let (br, bw) = self.beta.bufs();
        let (gr, gw) = self.gamma.bufs();
        let (dr, dw) = self.delta.bufs();
        let (or, ow) = self.omega.bufs();

        self.total_energy = e.1
            .par_iter_mut()
            .enumerate()
            .map(|(i, en)| {
                let x = i % self.width;
                let y = i / self.width;
                let mut sum = 0.;
                let cur = e.0[i];
                for dy in 0..=2 {
                    if y + dy >= 1 && y + dy - 1 < self.height {
                        for dx in 0..=2 {
                            if x + dx >= 1 && x + dx - 1 < self.width {
                                let i2 = (y + dy - 1) * self.width + x + dx - 1;
                                let cond = (1.0 - s.0[i]) * (1.0 - s.0[i2]);
                                let a = BASE_KERNEL[dx][dy] * cond;
                                sum += a * (e.0[i2]-cur);
                            }
                        }
                    }
                }

                let new = cur + sum * ENERGY_FLOW_RATE;
                *en = new;
                new
            }).sum();
        self.energy.swap();

        let mut e = self.energy.bufs();

        for i in 0..(self.width * self.height) {
            cw[i] = cr[i];
            s.1[i] = s.0[i];
            e.1[i] = e.0[i];
            r.1[i] = r.0[i];
            aw[i] = ar[i];
            bw[i] = br[i];
            gw[i] = gr[i];
            dw[i] = dr[i];
            ow[i] = or[i];
        }

        for i in 0..(self.width * self.height) {

            let x = i % self.width;
            let y = i / self.width;

            let g1 = cr[i] % 5;
            let g2 = (cr[i] / 5) % 5;
            let g3 = (cr[i] / 25) % 8 + 1;
            
            if e.0[i] > g3 as f32 * g2 as f32 * 5.0 {
                let j: [i32; 2] = match g1 {
                    0 => [0, 0],
                    2 => [0, 2],
                    1 => [0, -2],
                    3 => [-2, 0],
                    _ => [2, 0]
                };
                let attr = match g2 {
                    0 => &mut r,
                    1 => &mut e,
                    _ => &mut s,
                };
                let i2 = (((y as i32 + j[1]) as usize)%self.height) * self.width + (((x as i32 + j[0]) as usize)%self.width);
                let the = if g2 == 1 {g2 as f32 * 5.0} else {0.1 * attr.0[i]};
                attr.1[i2] = attr.1[i2] + g3 as f32 * the;
                e.1[i] -= g2 as f32 * 5.0;
            }

            // match cr[i] {
            //     1 => {
            //         if er[i] > 20.0 {
            //             let i2 = ((y + 2)%self.height) * self.width + x;
            //             rw[i2] = rw[i2] + 0.10 * rr[i];
            //             ew[i] -= 20.0;
            //         }
            //     }
            //     2 => {
            //         if er[i] > 20.0 {
            //             let i2 = ((self.height + y - 2)%self.height) * self.width + x;
            //             rw[i2] = rw[i2] + 0.10 * rr[i];
            //             ew[i] -= 10.0;
            //         }
            //     }
            //     3 => {
            //         if er[i] > 20.0 {
            //             let i2 = y * self.width + ((self.width + x - 2) % self.width);
            //             rw[i2] = rw[i2] + 0.10 * rr[i];
            //             ew[i] -= 20.0;
            //         }
            //     }
            //     4 => {
            //         if er[i] > 20.0 {
            //             let i2 = y * self.width + ((x + 2) % self.width);
            //             rw[i2] = rw[i2] + 0.10 * rr[i];
            //             ew[i] -= 20.0;
            //         }
            //     }
            //     5 => {
            //         if er[i] > 1.0 {
            //             let i2 = ((y + 2)%self.height) * self.width + x;
            //             ew[i2] += 1.0;
            //             ew[i] -= 1.0;
            //         }
            //     }
            //     6 => {
            //         if er[i] > 1.0 {
            //             let i2 = ((self.height + y - 2)%self.height) * self.width + x;
            //             ew[i2] += 1.0;
            //             ew[i] -= 1.0;
            //         }
            //     }
            //     7 => {
            //         if er[i] > 1.0 {
            //             let i2 = y * self.width + ((self.width + x - 2) % self.width);
            //             ew[i2] += 1.0;
            //             ew[i] -= 1.0;
            //         }
            //     }
            //     8 => {
            //         if er[i] > 1.0 {
            //             let i2 = y * self.width + ((x + 2) % self.width);
            //             ew[i2] += 1.0;
            //             ew[i] -= 1.0;
            //         }
            //     }
            //     9 => {
            //         if er[i] > 10.0 {
            //             let i2 = ((y + 2)%self.height) * self.width + x;
            //             sw[i2] = sw[i2] + 0.10 * sr[i];
            //             ew[i] -= 10.0;
            //         }
            //     }
            //     10 => {
            //         if er[i] > 10.0 {
            //             let i2 = ((self.height + y - 2)%self.height) * self.width + x;
            //             sw[i2] = sw[i2] + 0.10 * sr[i];
            //             ew[i] -= 10.0;
            //         }
            //     }
            //     11 => {
            //         if er[i] > 10.0 {
            //             let i2 = y * self.width + ((self.width + x - 2) % self.width);
            //             sw[i2] = sw[i2] + 0.10 * sr[i];
            //             ew[i] -= 10.0;
            //         }
            //     }
            //     12 => {
            //         if er[i] > 10.0 {
            //             let i2 = y * self.width + ((x + 2) % self.width);
            //             sw[i2] = sw[i2] + 0.10 * sr[i];
            //             ew[i] -= 10.0;
            //         }
            //     }
            //     _ => {
                    
            //     }
            // }
        }

        for i in 0..(self.width * self.height) {
            cw[i] = cw[i].clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
            s.1[i] = s.1[i].clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
            r.1[i] = r.1[i].clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
            e.1[i] = e.1[i].max(0.0);
        }

        self.connex_numbers.swap();
        self.stability.swap();
        self.reactivity.swap();
        self.energy.swap();
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn total_energy(&self) -> f32 {
        self.total_energy
    }

    pub fn tile_at(&self, pos: [f32; 2]) -> Option<[usize; 2]> {
        let x = pos[0] - self.pos[0];
        let y = pos[1] - self.pos[1];
        if x < 0.0 || y < 0.0 || x >= self.width as f32 || y >= self.height as f32 {
            None
        } else {
            Some([ x as usize, y as usize ])
        }
    }

    pub fn swap(&mut self, pos1: [usize; 2], pos2: [usize; 2]) {
        let pos1 = pos1[1] * self.width + pos1[0];
        let pos2 = pos2[1] * self.width + pos2[0];
        self.connex_numbers.swap_cell(pos1, pos2);
        self.stability.swap_cell(pos1, pos2);
        self.reactivity.swap_cell(pos1, pos2);
        self.energy.swap_cell(pos1, pos2);
    }
}
