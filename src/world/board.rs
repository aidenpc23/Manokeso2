use std::time::Duration;

use rand::Rng;
use rayon::{prelude::*, iter::MultiZip};

use crate::rsc::{CONNEX_NUMBER_RANGE, ENERGY_RANGE, REACTIVITY_RANGE, STABILITY_RANGE};

use super::{gen::SwapBufferGen, swap_buffer::SwapBuffer, alpha::{encode_alpha, decode_alpha, decode_beta, encode_beta}, DEFAULT_ALPHA};

pub const CARDINAL_DIRECTIONS: [(i32, i32); 5] = [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];

const BASE_KERNEL: [[f32; 3]; 3] = [[0.5, 1.0, 0.5], [1.0, 2.0, 1.0], [0.5, 1.0, 0.5]];
const GAMMA_KERNEL: [[f32; 3]; 3] = [[0.1, 1.0, 0.1], [1.0, 0.0, 1.0], [0.1, 1.0, 0.1]];
const ENERGY_FLOW_RATE: f32 = 1.0 / 100.0;
const GAMMA_FLOW_RATE: f32 = 1.0 / 8.0;

pub struct Board {
    pub pos: [f32; 2],
    width: usize,
    height: usize,
    pub connex_numbers: SwapBuffer<u32>,
    pub stability: SwapBuffer<f32>,
    pub reactivity: SwapBuffer<f32>,
    pub energy: SwapBuffer<f32>,
    pub alpha: SwapBuffer<u64>,
    pub beta: SwapBuffer<u64>,
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
        let alpha = SwapBuffer::from_arr(vec![0; width * height], width);
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
        }
    }

    pub fn update(&mut self) {
        let c = self.connex_numbers.bufs();
        let s = self.stability.bufs();
        let e = self.energy.bufs();
        let r = self.reactivity.bufs();
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
                                    let a = BASE_KERNEL[dx][dy] * cond; //* gr[i2] * gr[i];
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

        gw.par_iter_mut().enumerate().for_each(|(i, gn)| {
            let x = i % self.width;
            let y = i / self.width;
            let mut sum = 0.;
            let cur = gr[i];
            for dy in -1..=1 {
                if y as i32 + dy >= 0 && y as i32 + dy < self.height as i32 {
                    for dx in -1..=1 {
                        if x as i32 + dx >= 0 && x as i32 + dx < self.width as i32 {
                            let i2 = (y as i32 + dy) * self.width as i32 + x as i32 + dx;
                            let a = GAMMA_KERNEL[(dx + 1) as usize][(dy + 1) as usize];
                            sum += a * (gr[i2 as usize] - cur);
                        }
                    }
                }
            }

            let new = cur + sum * GAMMA_FLOW_RATE;
            *gn = new * (0.999 - 0.000001 * cur).min(1.0);
        });
        self.gamma.swap();

        aw.par_iter_mut().zip(bw).enumerate().for_each(|(i, (an, bn))| {
            let x = i % self.width;
            let y = i / self.width;
            let mut cntr = 0;
            let mut csum = 0;
            let mut ssum = 0.0;
            let mut esum = 0.0;
            let mut rsum = 0.0;
            let mut max_alpha = 0;
            let mut sb = 4;

        
            for (dx, dy) in &CARDINAL_DIRECTIONS {
                if y as i32 + *dy >= 0 && y as i32 + *dy < self.height as i32 && x as i32 + *dx >= 0 && x as i32 + *dx < self.width as i32 {
                    let i2 = ((y as i32 + *dy) * self.width as i32 + x as i32 + *dx) as usize;
                    let (bx, by) = decode_beta(br[i2]);
                    if bx + dx == 0 && by + dy == 0 {
                        let (counter, cnc, sc, ec, rc) = decode_alpha(ar[i2]);
                        cntr += counter;
                        csum += cnc;
                        ssum += sc;
                        esum += ec;
                        rsum += rc;
                        if counter > max_alpha {
                            sb = br[i2];
                            max_alpha = counter;
                        }
                    }
                }
            }
        
            if cntr == 0 {
                *an = encode_alpha(0, 0, 0.0, 0.0, 0.0);
                *bn = sb;
            } else {
                *an = encode_alpha(cntr-1, csum, ssum, esum, rsum);
                *bn = sb;
            }
        });
        self.alpha.swap();
        self.beta.swap();

        let mut e = self.energy.bufs();
        let (gr, gw) = self.gamma.bufs();
        let (ar, aw) = self.alpha.bufs();
        let (br, bw) = self.beta.bufs();

        (c.1, s.1, e.1, r.1, aw, bw, gw, dw, ow).into_par_iter().enumerate()
        .for_each(|(i, (cn, sn, en, rn, an, bn, gn, dn, on))| {
            *cn = c.0[i];
            *sn = s.0[i];
            *en = e.0[i];
            *rn = r.0[i];
            *an = ar[i];
            *bn = br[i];
            *gn = gr[i];
            *dn = dr[i];
            *on = or[i];
        });
        
        let c = self.connex_numbers.bufs();
        let s = self.stability.bufs();
        let e = self.energy.bufs();
        let r = self.reactivity.bufs();
        let (ar, aw) = self.alpha.bufs();
        let (br, bw) = self.beta.bufs();
        let (gr, gw) = self.gamma.bufs();
        let (dr, dw) = self.delta.bufs();
        let (or, ow) = self.omega.bufs();

        (c.1, s.1, e.1, r.1, aw, bw).into_par_iter().enumerate()
        .for_each(|(i, (cn, sn, en, rn, an, bn))| {
            let (counter, cnc, sc, ec, rc) = decode_alpha(ar[i]);
            if counter == 0 && ar[i] != DEFAULT_ALPHA {
                *cn = (*cn as i32 + cnc).max(0) as u32;

                let g3 = ((c.0[i] - 1) / 22) + 1;
                let gfactor = (g3-1) as f32 + (1.0 - 0.04 * (g3-1) as f32);

                *sn += sc * (10.0 / (c.0[i] as f32).powf(1.05 - 0.01 * gfactor)).min(1.0);
                *en += ec;
                *rn += rc;
                *an = encode_alpha(0, 0, 0.0, 0.0, 0.0);
                *bn = encode_beta(0, 0);
            }
        });

        let c = self.connex_numbers.bufs();
        let s = self.stability.bufs();
        let e = self.energy.bufs();
        let r = self.reactivity.bufs();

        self.alpha.swap();
        self.beta.swap();

        let (ar, aw) = self.alpha.bufs();
        let (br, bw) = self.beta.bufs();

        (c.1, s.1, e.1, r.1, aw, bw, gw).into_par_iter().enumerate()
        .for_each(|(i, (cn, sn, en, rn, an, bn, gn))| {
            let cost_mult = (c.0[i] as f32 * 0.35) + 1.0;
            let gamma_cost = cost_mult * cost_mult;

            if c.0[i] <= 20 && gr[i] < gamma_cost * 0.9 {
                *gn += (1.0002 as f32).powf(c.0[i] as f32) - 1.0;
            } else if gr[i] < (1.23 as f32).powf(c.0[i] as f32) {
                *gn += (1.02 as f32).powf(c.0[i] as f32) - 1.0;
            }

            let mut not0 = (c.0[i] > 0) as u32;
            let (g1, g2, g3) = (
                not0 * ((c.0[i] - 1) % 5),
                not0 * (((c.0[i] - 1) / 5) % 5),
                (not0 * ((c.0[i] - 1) / 22)) + 1
            );

            // not0 *= (g3 != 1) as u32;
            // let (g4, g5, g6) = (
            //     not0 * ((((c.0[i] - 1) as f32 * 3.0)) as u32 % 5),
            //     not0 * ((((c.0[i] - 1) as f32 * 5.0)) as u32 % 5),
            //     not0 * ((((c.0[i] - 1) as f32 * 7.0)) as u32 % 5),
            // );

            // ==============GAMMA ADJUSTMENTS =========================
            if *gn > gamma_cost {
                if r.0[i] > 0.0 {
                    *cn = (*cn as i32 + 1) as u32;
                } else if r.0[i] < 0.0 {
                    *cn = (*cn as i32 - 1).max(0) as u32;
                }

                let adjustments = [-0.25, -0.125, 0.0, 0.125, 0.25];
                *sn += adjustments[g2 as usize] * r.0[i];

                let step = 100.0 / 200.0;
                let sign = if g1 % 2 == 0 { -1.0 } else { 1.0 };
                *en += (step * ((g3 + 1) * (g2 + 1)) as f32 * sign * r.0[i].abs()).max(0.0);

                let r_adjustments = [-0.5, -0.25, 0.0, 0.25, 0.5];
                *rn += r_adjustments[g2 as usize] * (1.0 - s.0[i]);

                *gn -= gamma_cost;
            }
            // ============ END GAMMA ADJUSTMENTS ==========================

            // ========== CONNEX CALCULATIONS ============================
            if c.0[i] > 0 {
                let gfactor = (g3-1) as f32 + (1.0 - 0.04 * (g3-1) as f32);
                let en_move = gfactor * 0.1;

                let (ccost, cnc, scost, sc, ecost, ec, rcost, rc) = match g2 {
                    3 => (
                        (c.0[i] as f32 * gfactor).powf(2.305865),
                        if r.0[i] == 0.0 { 0 } else { g3 as i32 * r.0[i].signum() as i32 },
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.0
                    ),
                    2 => (
                        0.0,
                        0,
                        gfactor * 30.0,
                        0.1 * r.0[i],
                        0.0,
                        0.0,
                        0.0,
                        0.0
                    ),
                    1 => (0.0, 0, 0.0, 0.0, en_move, en_move, 0.0, 0.0),
                    0 => (0.0, 0, 0.0, 0.0, 0.0, 0.0, gfactor * 10.0, 0.1 * r.0[i]),
                    _ => (0.0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
                };
                
                let cost = ccost + scost + ecost + rcost;                
                

                let awave = decode_alpha(ar[i]);
                if *en >= cost {
                    *bn = g1 as u64;
                    if g1 != 4 || g2 != 1 {
                        *an = encode_alpha(
                            awave.0 + g3 as u64 + 1,
                            awave.1 + cnc,
                            awave.2 + sc,
                            awave.3 + ec,
                            awave.4 + rc
                        );
                        *en -= cost;
                    }
                }
            }

            // if g2 != 3 && g2 != 1 && e.0[i] > gfactor * g2 as f32 * 5.0 {

            //     let mult = if g2 == 2 {
            //         0.1 * (10.0 / (c.0[i2] as f32).powf(1.05 - 0.01 * gfactor)).min(1.0)
            //             * attr.0[i]
            //     } else {
            //         0.1 * attr.0[i]
            //     };

            //     if bounds[0] < attr.1[i2] && attr.1[i2] < bounds[1] {
            //         attr.1[i2] = attr.1[i2] + gfactor * mult;
            //         e.1[i] -= gfactor * g2 as f32 * 5.0;
            //     }
            // } else if g2 == 1 && e.0[i] > en_move {
            //     let i2 = (((y as i32 + dir[1]) as usize) % self.height) * self.width
            //         + (((x as i32 + dir[0]) as usize) % self.width);
            //     e.1[i2] += en_move;
            //     e.1[i] -= en_move;
            // } else if g2 == 3 {
            //     if g1 == 4 {
            //         for x2 in -(g3 as i32)..=g3 as i32 {
            //             for y2 in -(g3 as i32)..=g3 as i32 {
            //                 if x2 == 0 && y2 == 0 {
            //                     continue;
            //                 }
            //                 let i2 = (((y as i32 + y2) as usize) % self.height) * self.width
            //                     + (((x as i32 + x2) as usize) % self.width);
            //                 e.1[i] += r.1[i2].abs().min(0.01) * 7.0 * g3 as f32;
            //                 r.1[i2] = r.1[i2] - r.1[i2].signum() * r.1[i2].abs().min(0.01);
            //             }
            //         }
            //     } else {
            //         let i2 = (((y as i32 + dir[1]) as usize) % self.height) * self.width
            //             + (((x as i32 + dir[0]) as usize) % self.width);

            //         let cost = (c.0[i2] as f32 * gfactor).powf(2.305865); // COST GROWTH CONSTANT
            //         if e.0[i] > cost && c.1[i2] > 0 {
            //             c.1[i2] =
            //                 (c.1[i2] as i32 + (g3 as i32 * r.0[i].signum() as i32)) as u32;
            //             e.1[i] -= cost;
            //         }
            //     }
            // }
        });

        let c = self.connex_numbers.bufs();
        let s = self.stability.bufs();
        let e = self.energy.bufs();
        let r = self.reactivity.bufs();

        (c.1, s.1, e.1, r.1).into_par_iter()
        .for_each(|(cn, sn, en, rn)| {
            *cn = (*cn).clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
            *sn = (*sn).clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
            *rn = (*rn).clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
            *en = (*en).max(0.0);
        });

        self.connex_numbers.swap();
        self.stability.swap();
        self.reactivity.swap();
        self.energy.swap();
        self.gamma.swap();
        self.alpha.swap();
        self.beta  .swap();
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

        // if (self.connex_numbers.bufs().0[pos1] > 20 && self.stability.bufs().0[pos1] > 0.8) ||
        // (self.connex_numbers.bufs().0[pos2] > 20 && self.stability.bufs().0[pos2] > 0.8) {
        //     return;
        // }

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
