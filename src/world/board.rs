use rayon::prelude::*;

use crate::rsc::{CONNEX_NUMBER_RANGE, ENERGY_RANGE, REACTIVITY_RANGE, STABILITY_RANGE};

use super::{gen::SwapBufferGen, swap_buffer::SwapBuffer};

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

        let stability = gen.gen_map_base([0.6, 0.2], [0.6, 0.0], 0.058, 0.015, 0.06);
        let connex_numbers = SwapBuffer::from_arr(
            stability.read().iter().map(|a| (a * 20.0) as u32).collect(),
            width,
        );
        let reactivity = gen.gen_map(REACTIVITY_RANGE, 0.05);
        let energy = gen.gen_map(ENERGY_RANGE, 0.01);
        let alpha = gen.gen_map_base([0.6, 0.2], [0.6, 0.0], 0.058, 0.015, 0.025);
        let beta = gen.gen_map_base([0.6, 0.2], [0.6, 0.0], 0.058, 0.015, 0.025);
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
        }
    }

    pub fn update(&mut self) {
        let mut s = self.stability.bufs();
        let e = self.energy.bufs();
        let c = self.connex_numbers.bufs();
        let mut r = self.reactivity.bufs();
        let (ar, aw) = self.alpha.bufs();
        let (br, bw) = self.beta.bufs();
        let (gr, gw) = self.gamma.bufs();
        let (dr, dw) = self.delta.bufs();
        let (or, ow) = self.omega.bufs();

        self.total_energy =
            e.1.par_iter_mut()
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
                                    sum += a * (e.0[i2] - cur);
                                }
                            }
                        }
                    }

                    let new = cur + sum * ENERGY_FLOW_RATE;
                    *en = new;
                    new
                })
                .sum();
        self.energy.swap();

        let mut e = self.energy.bufs();

        for i in 0..(self.width * self.height) {
            c.1[i] = c.0[i];
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

            // if ar[i].abs() > br[i].abs() {

            // } else {

            // }

            if i > 0 {
                let g1 = (c.0[i] - 1) % 5;
                let g2 = ((c.0[i] - 1) / 5) % 5;
                let g3 = ((c.0[i] - 1) / 25) % 8 + 1;
                let dir: [i32; 2] = match g1 {
                    0 => [0, 2],
                    1 => [0, -2],
                    2 => [-2, 0],
                    3 => [2, 0],
                    _ => [0, 0],
                };
                let gfactor = g3 as f32 + (1.0 - 0.04 * g3 as f32);

                if g2 != 3 && e.0[i] > gfactor * g2 as f32 * 5.0 {
                    let attr = match g2 {
                        0 => &mut r,
                        1 => &mut e,
                        _ => &mut s,
                    };
                    let bounds = match g2 {
                        0 => REACTIVITY_RANGE,
                        1 => [ENERGY_RANGE[0], f32::MAX],
                        _ => STABILITY_RANGE,
                    };
                    let i2 = (((y as i32 + dir[1]) as usize) % self.height) * self.width
                        + (((x as i32 + dir[0]) as usize) % self.width);

                    let mult = if g2 == 1 {
                        g2 as f32 * 5.0
                    } else if g2 == 2 {
                        0.1 * (10.0 / (c.0[i2] as f32).powf(1.05 - 0.01 * gfactor)).min(1.0)
                            * attr.0[i]
                    } else {
                        0.1 * attr.0[i]
                    };

                    if bounds[0] < attr.1[i2] && attr.1[i2] < bounds[1] {
                        attr.1[i2] = attr.1[i2] + gfactor * mult;
                        e.1[i] -= gfactor * g2 as f32 * 5.0;
                    }
                } else if g2 == 3 {
                    if g1 == 4 {
                        for x2 in -(g3 as i32)..=g3 as i32 {
                            for y2 in -(g3 as i32)..=g3 as i32 {
                                if x2 == 0 && y2 == 0 {
                                    continue;
                                }
                                let i2 = (((y as i32 + y2) as usize) % self.height) * self.width
                                    + (((x as i32 + x2) as usize) % self.width);
                                e.1[i] += r.1[i2].abs().min(0.01) * 7.0 * g3 as f32;
                                r.1[i2] = r.1[i2] - r.1[i2].signum() * r.1[i2].abs().min(0.01);
                            }
                        }
                    } else {
                        let i2 = (((y as i32 + dir[1]) as usize) % self.height) * self.width
                            + (((x as i32 + dir[0]) as usize) % self.width);

                        let cost = (c.0[i2] as f32 * gfactor).powf(2.305865); // COST GROWTH CONSTANT
                        if e.0[i] > cost && c.1[i2] > 0 {
                            c.1[i2] =
                                (c.1[i2] as i32 + (g3 as i32 * r.0[i].signum() as i32)) as u32;
                            e.1[i] -= cost;
                        }
                    }
                }
            }
        }

        for i in 0..(self.width * self.height) {
            c.1[i] = c.1[i].clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
            s.1[i] = s.1[i].clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
            r.1[i] = r.1[i].clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
            e.1[i] = e.1[i].max(0.0);

            // s.1[i] = ar[i];
            // r.1[i] = br[i];
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
            Some([x as usize, y as usize])
        }
    }

    pub fn player_swap(&mut self, pos1: [usize; 2], pos2: [usize; 2]) {
        let pos1 = pos1[1] * self.width + pos1[0];
        let pos2 = pos2[1] * self.width + pos2[0];

        if (self.connex_numbers.bufs().0[pos1] > 20 && self.stability.bufs().0[pos1] > 0.8)
            || (self.connex_numbers.bufs().0[pos2] > 20 && self.stability.bufs().0[pos2] > 0.8)
        {
            return;
        }

        self.connex_numbers.swap_cell(pos1, pos2);
        self.stability.swap_cell(pos1, pos2);
        self.reactivity.swap_cell(pos1, pos2);
        self.energy.swap_cell(pos1, pos2);
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
