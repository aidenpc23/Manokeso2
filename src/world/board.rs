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
    total_energy: f32, 
}

impl Board {
    pub fn new(pos: [f32; 2], width: usize, height: usize) -> Board {
        let mut gen = (width, height);

        let stability = gen.gen_map_base([0.4, 0.2], 0.038, 0.01);
        let connex_numbers = SwapBuffer::from_arr(stability.read().iter().map(|a| (a * 20.0) as u32).collect(), width);
        let reactivity = gen.gen_map(REACTIVITY_RANGE, 0.05);
        let energy = gen.gen_map(ENERGY_RANGE, 0.01);
        //stability = SwapBuffer::from_arr(vec![0.0; width*height], width);

        let total_energy = energy.read().iter().sum();

        Board {
            pos,
            width,
            height,
            connex_numbers,
            stability,
            reactivity,
            energy,
            total_energy,
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        let (sr, sw) = self.stability.bufs();
        let (er, ew) = self.energy.bufs();
        
        self.total_energy = ew
            .par_iter_mut()
            .enumerate()
            .map(|(i, e)| {
                let x = i % self.width;
                let y = i / self.width;
                let mut sum = 0.;
                let cur = er[i];
                for dy in 0..=2 {
                    if y + dy >= 1 && y + dy - 1 < self.height {
                        for dx in 0..=2 {
                            if x + dx >= 1 && x + dx - 1 < self.width {
                                let i2 = (y + dy - 1) * self.width + x + dx - 1;
                                let cond = (1.0 - sr[i]) * (1.0 - sr[i2]);
                                let a = BASE_KERNEL[dx][dy] * cond;
                                sum += a * (er[i2]-cur);
                            }
                        }
                    }
                }

                let new = cur + sum * ENERGY_FLOW_RATE;
                *e = new;
                new
            }).sum();
        self.energy.swap();
        
        let (cr, cw) = self.connex_numbers.bufs();
        let (rr, rw) = self.reactivity.bufs();
        let (er, ew) = self.energy.bufs();

        for i in 0..(self.width * self.height) {
            cw[i] = cr[i];
            sw[i] = sr[i];
            ew[i] = er[i];
            rw[i] = rr[i];
        }

        for i in 0..(self.width * self.height) {

            let x = i % self.width;
            let y = i / self.width;

            match cr[i] {
                1 => {
                    if er[i] > 20.0 {
                        let i2 = ((y + 2)%self.height) * self.width + x;
                        rw[i2] = rw[i2] + 0.10 * rr[i];
                        ew[i] -= 20.0;
                    }
                }
                2 => {
                    if er[i] > 20.0 {
                        let i2 = ((self.height + y - 2)%self.height) * self.width + x;
                        rw[i2] = rw[i2] + 0.10 * rr[i];
                        ew[i] -= 10.0;
                    }
                }
                3 => {
                    if er[i] > 20.0 {
                        let i2 = y * self.width + ((self.width + x - 2) % self.width);
                        rw[i2] = rw[i2] + 0.10 * rr[i];
                        ew[i] -= 20.0;
                    }
                }
                4 => {
                    if er[i] > 20.0 {
                        let i2 = y * self.width + ((x + 2) % self.width);
                        rw[i2] = rw[i2] + 0.10 * rr[i];
                        ew[i] -= 20.0;
                    }
                }
                5 => {
                    if er[i] > 1.0 {
                        let i2 = ((y + 2)%self.height) * self.width + x;
                        ew[i2] += 1.0;
                        ew[i] -= 1.0;
                    }
                }
                6 => {
                    if er[i] > 1.0 {
                        let i2 = ((self.height + y - 2)%self.height) * self.width + x;
                        ew[i2] += 1.0;
                        ew[i] -= 1.0;
                    }
                }
                7 => {
                    if er[i] > 1.0 {
                        let i2 = y * self.width + ((self.width + x - 2) % self.width);
                        ew[i2] += 1.0;
                        ew[i] -= 1.0;
                    }
                }
                8 => {
                    if er[i] > 1.0 {
                        let i2 = y * self.width + ((x + 2) % self.width);
                        ew[i2] += 1.0;
                        ew[i] -= 1.0;
                    }
                }
                9 => {
                    if er[i] > 10.0 {
                        let i2 = ((y + 2)%self.height) * self.width + x;
                        sw[i2] = sw[i2] + 0.10 * sr[i];
                        ew[i] -= 10.0;
                    }
                }
                10 => {
                    if er[i] > 10.0 {
                        let i2 = ((self.height + y - 2)%self.height) * self.width + x;
                        sw[i2] = sw[i2] + 0.10 * sr[i];
                        ew[i] -= 10.0;
                    }
                }
                11 => {
                    if er[i] > 10.0 {
                        let i2 = y * self.width + ((self.width + x - 2) % self.width);
                        sw[i2] = sw[i2] + 0.10 * sr[i];
                        ew[i] -= 10.0;
                    }
                }
                12 => {
                    if er[i] > 10.0 {
                        let i2 = y * self.width + ((x + 2) % self.width);
                        sw[i2] = sw[i2] + 0.10 * sr[i];
                        ew[i] -= 10.0;
                    }
                }
                _ => {
                    
                }
            }
        }

        for i in 0..(self.width * self.height) {
            cw[i] = cw[i].clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
            sw[i] = sw[i].clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
            rw[i] = rw[i].clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
            ew[i] = ew[i].max(0.0);
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
