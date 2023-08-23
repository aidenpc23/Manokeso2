use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::rsc::{CONNEX_NUMBER_RANGE, REACTIVITY_RANGE, STABILITY_RANGE};

use super::{
    alpha::{decode_alpha, decode_beta, encode_alpha, encode_beta},
    Board
};

const BASE_KERNEL: [[f32; 3]; 3] = [[0.5, 1.0, 0.5], [1.0, 2.0, 1.0], [0.5, 1.0, 0.5]];
const GAMMA_KERNEL: [[f32; 3]; 3] = [[0.1, 1.0, 0.1], [1.0, 0.0, 1.0], [0.1, 1.0, 0.1]];
const ENERGY_FLOW_RATE: f32 = 1.0 / 100.0;
const GAMMA_FLOW_RATE: f32 = 1.0 / 8.0;
const VAR_NAME: f32 = 2.5;

pub const CARDINAL_DIRECTIONS: [(i32, i32); 5] = [(0, 2), (0, -2), (-2, 0), (2, 0), (0, 0)];

impl Board {
    pub fn update(&mut self) {
        self.spawn_alpha_beta();
        self.convolve_energy();
        self.convolve_gamma();
        self.update_alpha_beta();
        self.update_gamma();
        self.apply_alpha_beta();
        self.apply_bounds();

        self.dirty = true;
    }

    fn update_alpha_beta(&mut self) {
        let a = &mut self.alpha;
        let b = &mut self.beta;
        self.total_energy += a
            .w
            .par_iter_mut()
            .zip(&mut b.w)
            .enumerate()
            .map(|(i, (an, bn))| {
                let x = i % self.width;
                let y = i / self.width;
                let mut cntr = 0;
                let mut csum = 0;
                let mut ssum = 0.0;
                let mut esum = 0.0;
                let mut rsum = 0.0;
                let mut max_counter = 0;
                let mut sb = 4;

                for (dx, dy) in &CARDINAL_DIRECTIONS {
                    if y as i32 + *dy >= 0
                        && y as i32 + *dy < self.height as i32
                        && x as i32 + *dx >= 0
                        && x as i32 + *dx < self.width as i32
                    {
                        let i2 = ((y as i32 + *dy) * self.width as i32 + x as i32 + *dx) as usize;
                        let (bx, by) = decode_beta(b.r[i2]);
                        if bx + dx == 0 && by + dy == 0 {
                            let (counter, cnc, sc, ec, rc) = decode_alpha(a.r[i2]);
                            if a.r[i2] != encode_alpha(0, 0, 0.0, 0.0, 0.0) {
                                cntr = if cntr != 0 {
                                    cntr.min(counter)
                                } else {
                                    counter
                                };
                            }
                            csum += cnc;
                            ssum += sc;
                            esum += ec;
                            rsum += rc;
                            if counter > max_counter {
                                sb = b.r[i2];
                                max_counter = counter;
                            }
                        }
                    }
                }

                if cntr > 0 {
                    *an = encode_alpha(cntr - 1, csum, ssum, esum, rsum);
                } else {
                    *an = encode_alpha(0, csum, ssum, esum, rsum);
                }
                *bn = sb;
                esum
            })
            .sum::<f32>();

        a.swap();
        b.swap();
    }

    fn convolve_gamma(&mut self) {
        let g = &mut self.gamma;
        g.w.par_iter_mut().enumerate().for_each(|(i, gn)| {
            let x = i % self.width;
            let y = i / self.width;
            let mut sum = 0.;
            let cur = g.r[i];
            for dy in -1..=1 {
                if y as i32 + dy >= 0 && y as i32 + dy < self.height as i32 {
                    for dx in -1..=1 {
                        if x as i32 + dx >= 0 && x as i32 + dx < self.width as i32 {
                            let i2 = (y as i32 + dy) * self.width as i32 + x as i32 + dx;
                            let a = GAMMA_KERNEL[(dx + 1) as usize][(dy + 1) as usize];
                            sum += a * (g.r[i2 as usize] - cur);
                        }
                    }
                }
            }

            let new = cur + sum * GAMMA_FLOW_RATE;
            *gn = new * (0.999 - 0.000001 * cur).min(1.0);
        });
        g.swap();
    }

    fn convolve_energy(&mut self) {
        let e = &mut self.energy;
        let s = &mut self.stability;
        self.total_energy =
            e.w.par_iter_mut()
                .enumerate()
                .map(|(i, en)| {
                    let x = i % self.width;
                    let y = i / self.width;
                    let mut sum = 0.;
                    let cur = e.r[i];
                    for dy in 0..=2 {
                        if y + dy >= 1 && y + dy - 1 < self.height {
                            for dx in 0..=2 {
                                if x + dx >= 1 && x + dx - 1 < self.width {
                                    let i2 = (y + dy - 1) * self.width + x + dx - 1;
                                    let cond = (1.0 - s.r[i]) * (1.0 - s.r[i2]);
                                    let a = BASE_KERNEL[dx][dy] * cond; //* gr[i2] * gr[i];
                                    sum += a * (e.r[i2] - cur);
                                }
                            }
                        }
                    }

                    let new = cur + sum * ENERGY_FLOW_RATE;
                    *en = new;
                    new
                })
                .sum();
        e.swap();
    }

    fn update_gamma(&mut self) {
        let c = &mut self.connex_numbers;
        let s = &mut self.stability;
        let e = &mut self.energy;
        let r = &mut self.reactivity;
        let g = &mut self.gamma;

        (&mut c.w, &mut s.w, &mut e.w, &mut r.w, &mut g.w)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (cn, sn, en, rn, gn))| {
                let cost_mult = (c.r[i] as f32 * 0.35) + 1.0;
                let gamma_cost = cost_mult * cost_mult;

                if c.r[i] <= 20 && g.r[i] < gamma_cost * 0.9 {
                    *gn = g.r[i] + (1.0002 as f32).powf(c.r[i] as f32) - 1.0;
                } else if g.r[i] < (1.23 as f32).powf(c.r[i] as f32) {
                    *gn = g.r[i] + (1.02 as f32).powf(c.r[i] as f32) - 1.0;
                } else {
                    *gn = g.r[i];
                }

                let temp = if c.r[i] == 0 { 0 } else { c.r[i] - 1 };
                let (g1, g2, g3) = ((temp % 5), ((temp / 5) % 5), ((temp / 25) + 1));
                if *gn > gamma_cost {
                    if r.r[i] > 0.0 {
                        *cn = c.r[i] + 1;
                    } else if r.r[i] < 0.0 {
                        *cn = c.r[i].saturating_sub(1);
                    }

                    let adjustments = [-0.25, -0.125, 0.0, 0.125, 0.25];
                    *sn = s.r[i] + adjustments[g2 as usize] * r.r[i];

                    let step = 100.0 / 200.0;
                    let sign = if g1 % 2 == 0 { -1.0 } else { 1.0 };
                    *en = e.r[i]
                        + (step * ((g3 + 1) * (g2 + 1)) as f32 * sign * r.r[i].abs()).max(0.0);

                    let r_adjustments = [-0.5, -0.25, 0.0, 0.25, 0.5];
                    *rn = r.r[i] + r_adjustments[g2 as usize] * (1.0 - s.r[i]);

                    *gn -= gamma_cost;
                } else {
                    *cn = c.r[i];
                    *sn = s.r[i];
                    *en = e.r[i];
                    *rn = r.r[i];
                }
            });

        c.swap();
        s.swap();
        e.swap();
        r.swap();
        g.swap();
    }

    fn spawn_alpha_beta(&mut self) {
        let c = &self.connex_numbers;
        let r = &self.reactivity;
        let e = &mut self.energy;
        let a = &mut self.alpha;
        let b = &mut self.beta;
        (&mut e.w, &mut a.w, &mut b.w)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (en, an, bn))| {
                let temp = c.r[i].saturating_sub(1);
                let (g1, g2, g3) = ((temp % 5), ((temp / 5) % 5), ((temp / 25) + 1));
                // ========== CONNEX CALCULATIONS ============================
                if c.r[i] > 0 {
                    let gfactor = (g3 - 1) as f32 + (1.0 - 0.04 * (g3 - 1) as f32);
                    let en_move = gfactor * VAR_NAME;

                    let (ccost, cnc, scost, sc, ecost, ec, rcost, rc) = match g2 {
                        3 => (
                            (c.r[i] as f32 * gfactor).powf(2.305865),
                            if r.r[i] == 0.0 {
                                0
                            } else {
                                g3 as i32 * r.r[i].signum() as i32
                            },
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                        ),
                        2 => (0.0, 0, gfactor * 15.0, 0.1 * r.r[i], 0.0, 0.0, 0.0, 0.0),
                        1 => (0.0, 0, 0.0, 0.0, en_move, en_move, 0.0, 0.0),
                        0 => (0.0, 0, 0.0, 0.0, 0.0, 0.0, gfactor * 20.0, 0.1 * r.r[i]),
                        _ => (0.0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
                    };

                    let cost = ccost + scost + ecost + rcost;

                    let awave = decode_alpha(a.r[i]);
                    *bn = g1 as u64;
                    if e.r[i] >= cost && (g1 != 4 || g2 != 1) {
                        *an = encode_alpha(
                            awave.0 + g3 as u64,
                            awave.1 + cnc,
                            awave.2 + sc,
                            awave.3 + ec,
                            awave.4 + rc,
                        );
                        *en = e.r[i] - cost;
                    } else {
                        *an = a.r[i];
                    }
                } else {
                    *an = a.r[i];
                    *bn = b.r[i];
                }
            });
        e.swap();
        a.swap();
        b.swap();
    }

    fn apply_alpha_beta(&mut self) {
        let c = &mut self.connex_numbers;
        let s = &mut self.stability;
        let e = &mut self.energy;
        let r = &mut self.reactivity;
        let a = &mut self.alpha;
        let b = &mut self.beta;

        (&mut c.w, &mut s.w, &mut e.w, &mut r.w, &mut a.w, &mut b.w)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (cn, sn, en, rn, an, bn))| {
                let (counter, cnc, sc, ec, rc) = decode_alpha(a.r[i]);
                if counter == 0 && a.r[i] != encode_alpha(0, 0, 0.0, 0.0, 0.0) {
                    // Apply delta values to all attributes
                    // Connex number cant fall below 0
                    *cn = (c.r[i] as i32 + cnc).max(0) as u32;

                    // Calculate group 3 and the gfactor
                    // Group three is the g3 level of categorization of connex numbers
                    // gfactor is almost equal to g3 but a little less each time so
                    // g3 = 1 while gfactor == 1 and g3 == 2 while gfactor == 1.9 and so on
                    let g3 = if c.r[i] == 0 {
                        1
                    } else {
                        ((c.r[i] - 1) / 25) + 1
                    };
                    let gfactor = (g3 - 1) as f32 + (1.0 - 0.04 * (g3 - 1) as f32);

                    // Make it such that the higher the connex number the harder to decrease stability.
                    *sn = s.r[i]
                        + sc * (10.0 / (c.r[i] as f32).powf(1.05 - 0.01 * gfactor))
                            .min(1.0)
                            .max(0.025);
                    *en = e.r[i] + ec;

                    *rn = r.r[i] + rc;
                    *an = encode_alpha(0, 0, 0.0, 0.0, 0.0);

                    *bn = encode_beta(0, 0);
                } else {
                    *cn = c.r[i];
                    *sn = s.r[i];
                    *en = e.r[i];
                    *rn = r.r[i];
                    *an = a.r[i];
                    *bn = b.r[i];
                }
            });
        c.swap();
        s.swap();
        e.swap();
        r.swap();
        a.swap();
        b.swap();
    }

    fn apply_bounds(&mut self) {
        let c = &mut self.connex_numbers;
        let s = &mut self.stability;
        let e = &mut self.energy;
        let r = &mut self.reactivity;

        (&mut c.w, &mut s.w, &mut e.w, &mut r.w)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (cn, sn, en, rn))| {
                *cn = (c.r[i]).clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
                *sn = (s.r[i]).clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
                *rn = (r.r[i]).clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
                *en = (e.r[i]).max(0.0);
            });

        c.swap();
        s.swap();
        e.swap();
        r.swap();
    }
}
