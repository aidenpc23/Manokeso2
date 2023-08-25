use std::arch::x86_64::__get_cpuid_max;

use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::rsc::{CONNEX_NUMBER_RANGE, REACTIVITY_RANGE, STABILITY_RANGE};

use super::{
    connex_ref::CONX_MAP,
    util::{decode_alpha, decode_beta, encode_alpha, encode_beta},
    Board, ZERO_ALPHA,
};

const BASE_KERNEL: [[f32; 3]; 3] = [[0.5, 1.0, 0.5], [1.0, 2.0, 1.0], [0.5, 1.0, 0.5]];
const GAMMA_KERNEL: [[f32; 3]; 3] = [[0.1, 1.0, 0.1], [1.0, 0.0, 1.0], [0.1, 1.0, 0.1]];
const OMEGA_KERNEL: [[f32; 3]; 3] = [[0.1, 1.0, 0.1], [1.0, 0.0, 1.0], [0.1, 1.0, 0.1]];
const ENERGY_FLOW_RATE: f32 = 1.0 / 100.0;
const GAMMA_FLOW_RATE: f32 = 1.0 / 8.0;
const OMEGA_FLOW_RATE: f32 = 1.0 / 200.0;
const VAR_NAME: f32 = 2.5;

pub const CARDINAL_DIRECTIONS: [(i32, i32); 5] = [(0, 2), (0, -2), (-2, 0), (2, 0), (0, 0)];

impl Board {
    pub fn update(&mut self) {
        self.spawnab_update_conx();
        self.update_omega();
        self.convolve_energy();
        self.convolve_gamma();
        self.convolve_omega();
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
                            if a.r[i2] != *ZERO_ALPHA {
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

    fn convolve_omega(&mut self) {
        let o = &mut self.omega;
        let r = &self.reactivity;
        o.w.par_iter_mut().enumerate().for_each(|(i, on)| {
            let x = i % self.width;
            let y = i / self.width;
            let mut sum = 0.;
            let cur = o.r[i];
            let y_start = (y as i32 - 1).max(0) as usize;
            let y_end = (y as i32 + 2).min(self.height as i32) as usize;
            let x_start = (x as i32 - 1).max(0) as usize;
            let x_end = (x as i32 + 2).min(self.width as i32) as usize;

            for dy in y_start..y_end {
                for dx in x_start..x_end {
                    let i2 = dy * self.width + dx;
                    let cond = r.r[i].abs() * r.r[i2 as usize].abs();
                    let kernel_value =
                        OMEGA_KERNEL[(dx - x_start) as usize][(dy - y_start) as usize];
                    let a = kernel_value * cond;
                    sum += a * (o.r[i2 as usize] - cur);
                }
            }

            let new = cur + sum * OMEGA_FLOW_RATE;
            *on = new * 0.80;
            if on.abs() < 0.0001 {
                *on = 0.0;
            }
        });
        o.swap();
    }

    fn convolve_gamma(&mut self) {
        let g = &mut self.gamma;
        let r = &self.reactivity;
        g.w.par_iter_mut().enumerate().for_each(|(i, gn)| {
            let x = i % self.width;
            let y = i / self.width;
            let mut sum = 0.;
            let cur = g.r[i];
            let y_start = (y as i32 - 1).max(0) as usize;
            let y_end = (y as i32 + 2).min(self.height as i32) as usize;
            let x_start = (x as i32 - 1).max(0) as usize;
            let x_end = (x as i32 + 2).min(self.width as i32) as usize;

            for dy in y_start..y_end {
                for dx in x_start..x_end {
                    let i2 = dy * self.width + dx;
                    let cond = r.r[i].abs() * r.r[i2 as usize].abs();
                    let kernel_value =
                        GAMMA_KERNEL[(dx - x_start) as usize][(dy - y_start) as usize];
                    let a = kernel_value * cond;
                    sum += a * (g.r[i2 as usize] - cur);
                }
            }

            let new = cur + sum * GAMMA_FLOW_RATE;
            *gn = new * (0.999 - 0.000001 * cur).min(1.0);
            if gn.abs() < 0.001 {
                *gn = 0.0;
            }
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

                    let y_start = y.saturating_sub(1);
                    let y_end = (y + 2).min(self.height);
                    let x_start = x.saturating_sub(1);
                    let x_end = (x + 2).min(self.width);

                    for dy in y_start..y_end {
                        for dx in x_start..x_end {
                            let i2 = dy * self.width + dx;
                            let cond = (1.0 - s.r[i]) * (1.0 - s.r[i2]);
                            let kernel_value = BASE_KERNEL[dx - x_start][dy - y_start];
                            let a = kernel_value * cond;
                            sum += a * (e.r[i2] - cur);
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
                let ci = c.r[i];
                let si = s.r[i];
                let ei = e.r[i];
                let ri = r.r[i];
                let gi = g.r[i];

                let cost_mult = (ci as f32 * 0.35) + 1.0;
                let gamma_cost = cost_mult * cost_mult;

                if ci <= 20 && gi < gamma_cost * 0.9 {
                    *gn = gi + ((1.0002 as f32).powf(ci as f32) - 1.0);
                } else if gi < (1.23 as f32).powf(c.r[i] as f32) {
                    *gn = gi + ((1.02 as f32).powf(c.r[i] as f32) - 1.0);
                } else {
                    *gn = gi;
                }

                if *gn > gamma_cost {
                    let csub = ci.saturating_sub(1);
                    // let eff_cycle_spd = (csub / 20).min(2);
                    let (g1, g2, g3) = ((csub % 5), ((csub / (5)) % 5), ((csub / 25) + 1));

                    let bound = 5.0;
                    let rme = (ri * ei).min(bound).max(-bound);
                    if rme > 0.0 {
                        *cn = ci + 1;
                    } else if rme < 0.0 {
                        *cn = ci.saturating_sub(1);
                    }

                    let adjustments = [-0.1, -0.05, 0.0, 0.05, 0.1];
                    *sn = si + adjustments[g2 as usize] * rme;

                    let sign = if g1 % 2 == 0 { -10.0 } else { 10.0 };
                    *en = (ei
                        + (((g3 + 1) * (g2 + 1)) as f32 * sign * r.r[i].abs())
                            / ((ei - 50.0).max(1.0)))
                    .max(0.0);

                    let r_adjustments = [-0.5, -0.25, 0.0, 0.25, 0.5];
                    *rn = ri + r_adjustments[g2 as usize] * (1.0 - s.r[i]);

                    *gn -= gamma_cost;
                } else {
                    *cn = ci;
                    *sn = si;
                    *en = ei;
                    *rn = ri;
                }
            });

        c.swap();
        s.swap();
        e.swap();
        r.swap();
        g.swap();
    }

    fn spawnab_update_conx(&mut self) {
        let c = &self.connex_numbers;
        let r = &self.reactivity;
        let s = &self.stability;
        let e = &mut self.energy;
        let a = &mut self.alpha;
        let b = &mut self.beta;
        let o = &mut self.omega;
        (&mut e.w, &mut a.w, &mut b.w, &mut o.w)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (en, an, bn, on))| {
                let ci = c.r[i];
                let si = s.r[i];
                let ei = e.r[i];
                let ri = r.r[i];
                let ai = a.r[i];
                let bi = b.r[i];
                let oi = o.r[i];
                let x = i % self.width;
                let y = i / self.width;

                let csub = ci.saturating_sub(1);
                let eff_cycle_spd = (csub / 20).min(4);
                let (g1, g2, g3) = (
                    (csub % 5),
                    ((csub / (5 - eff_cycle_spd)) % 5),
                    ((csub / 25) + 1),
                );

                let (dx, dy) = decode_beta(g1 as u64);
                let i2 = (x as i32 + dx) + (y as i32 + dy) * self.width as i32;
                let u_i2 = i2 as usize;

                let (do_conn, do_stab, do_reac) =
                    if i2 >= 0 && i2 < self.width as i32 * self.width as i32 && ri != 0.0 {
                        if ri > 0.0 {
                            (
                                (c.r[u_i2] < CONNEX_NUMBER_RANGE[1]) as i32,
                                (s.r[u_i2] < STABILITY_RANGE[1]) as u32 as f32,
                                (r.r[u_i2] < REACTIVITY_RANGE[1]) as u32 as f32,
                            )
                        } else {
                            (
                                (c.r[u_i2] > CONNEX_NUMBER_RANGE[0]) as i32,
                                (s.r[u_i2] > STABILITY_RANGE[0]) as u32 as f32,
                                (r.r[u_i2] > REACTIVITY_RANGE[0]) as u32 as f32,
                            )
                        }
                    } else {
                        (0, 0.0, 0.0)
                    };

                // if x == 274 && y == 654 {
                //     println!("{:?} : {:?}", (c.r[i], s.r[i], r.r[i]), (c.r[u_i2], s.r[u_i2], r.r[u_i2]));
                // }

                // ========== CONNEX CALCULATIONS ============================
                if ci > 0 {
                    let gfactor = (g3 - 1) as f32 + (1.0 - 0.04 * (g3 - 1) as f32);
                    let en_move = gfactor * VAR_NAME;

                    let (ccost, cnc) = if CONX_MAP[&ci].3 {
                        (
                            (ci as f32 * gfactor).powf(2.305865) * do_conn as f32,
                            if ri == 0.0 {
                                0
                            } else {
                                g3 as i32 * ri.signum() as i32 * do_conn
                            },
                        )
                    } else {
                        (0.0, 0)
                    };

                    let (scost, sc) = if CONX_MAP.get(&ci).unwrap().2 {
                        (gfactor * 10.0 * do_stab, 0.1 * ri * g3 as f32 * do_stab)
                    } else {
                        (0.0, 0.0)
                    };
                    let (ecost, ec) = if CONX_MAP.get(&ci).unwrap().1 {
                        (en_move, en_move)
                    } else {
                        (0.0, 0.0)
                    };
                    let (rcost, rc) = if CONX_MAP.get(&ci).unwrap().0 {
                        (gfactor * 10.0 * do_reac, 0.1 * ri * g3 as f32 * do_reac)
                    } else {
                        (0.0, 0.0)
                    };

                    let cost = ccost + scost + ecost + rcost;

                    let awave = decode_alpha(ai);
                    *bn = g1 as u64;
                    if ei >= cost {
                        *an = encode_alpha(
                            awave.0 + g3 as u64,
                            awave.1 + cnc,
                            awave.2 + sc,
                            awave.3 + ec,
                            awave.4 + rc,
                        );
                        *en = ei - cost;
                    } else {
                        *en = ei;
                        *an = ai;
                    }

                    if ci % 20 == 0 {
                        *on = oi + g3.pow(2) as f32 * 0.1;
                    } else {
                        *on = oi;
                    }
                } else {
                    *an = ai;
                    *bn = bi;
                    *on = oi;
                    *en = ei;
                }
            });
        e.swap();
        a.swap();
        b.swap();
        o.swap();
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
                let ci = c.r[i];
                let si = s.r[i];
                let ei = e.r[i];
                let ri = r.r[i];
                let ai = a.r[i];
                let bi = b.r[i];

                let (counter, cnc, sc, ec, rc) = decode_alpha(ai);
                if counter == 0 && ai != *ZERO_ALPHA {
                    // Apply delta values to all attributes
                    // Connex number cant fall below 0
                    *cn = (ci as i32 + cnc).max(0) as u32;

                    // Calculate group 3 and the gfactor
                    // Group three is the g3 level of categorization of connex numbers
                    // gfactor is almost equal to g3 but a little less each time so
                    // g3 = 1 while gfactor == 1 and g3 == 2 while gfactor == 1.9 and so on
                    let g3 = if ci == 0 {
                        1
                    } else {
                        ((ci - 1) / 25) + 1
                    };
                    let gfactor = (g3 - 1) as f32 + (1.0 - 0.04 * (g3 - 1) as f32);

                    // Make it such that the higher the connex number the harder to decrease stability.
                    *sn = si
                        + sc * (10.0 / (ci as f32).powf(1.05 - 0.01 * gfactor))
                            .min(1.0)
                            .max(0.025);
                    *en = ei + ec;

                    *rn = ri + rc;
                    *an = *ZERO_ALPHA;

                    *bn = encode_beta(0, 0);
                } else {
                    *cn = ci;
                    *sn = si;
                    *en = ei;
                    *rn = ri;
                    *an = ai;
                    *bn = bi;
                }
            });
        c.swap();
        s.swap();
        e.swap();
        r.swap();
        a.swap();
        b.swap();
    }

    fn update_omega(&mut self) {
        let o = &self.omega;
        let r = &mut self.reactivity;
        let e = &mut self.energy;

        (&mut r.w, &mut e.w)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (rn, en))| {
                let ei = e.r[i];
                let ri = r.r[i];
                let oi = o.r[i];
                let x = i % self.width;
                let y = i / self.width;

                let mut absorbed_reactivity = 0.0;
                let mut released_reactivity = 0.0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let i2 = (y as i32 + dy) * self.width as i32 + x as i32 + dx;
                        let u_i2 = i2 as usize;
                        if i2 >= 0 && u_i2 < self.width * self.width && (x != 0 || y != 0) {
                            let pseudo_cap = oi.min(1.0);
                            let pseudo_cap2 = o.r[u_i2].min(1.0);
                            let other = ri * pseudo_cap2;
                            released_reactivity += if ri < 0.0 {
                                other.max(ri)
                            } else {
                                other.min(ri)
                            };
                            absorbed_reactivity +=
                                (r.r[u_i2].abs() * pseudo_cap).min(r.r[u_i2].abs());
                        }
                    }
                }

                let rclamp = if released_reactivity < 0.0 {
                    released_reactivity.max(-1.0)
                } else {
                    released_reactivity.min(1.0)
                };

                *en = ei + absorbed_reactivity * 105.0;
                if released_reactivity.abs() > rn.abs() {
                    *rn = 0.0;
                } else {
                    *rn = (ri - rclamp).clamp(-1.0, 1.0);
                    if rn.abs() < 0.001 {
                        *rn = 0.0;
                    }
                }
            });

        r.swap();
        e.swap();
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
