use itertools::izip;
use rayon::prelude::*;

use crate::world::Board;

use super::Instances;

impl Board {
    pub fn update_instances(
        &self,
        instances: &mut Instances,
        xs: usize,
        xe: usize,
        ys: usize,
        ye: usize,
    ) -> usize {
        let width = xe - xs;
        let size = width * (ye - ys);
        if instances.connex_number.data.len() >= size {
            instances.connex_number.update_rows(self.connex_numbers.par_rows(ys, ye), size, xs, xe);
            instances.conductivity.update_rows(self.conductivity.par_rows(ys, ye), size, xs, xe);
            instances.reactivity.update_rows(self.reactivity.par_rows(ys, ye), size, xs, xe);
            instances.energy.update_rows(self.energy.par_rows(ys, ye), size, xs, xe);

            // (
            //     instances.connex_number.data[0..size].par_chunks_exact_mut(width).zip(self.connex_numbers.par_rows(ys, ye)),
            //     instances.conductivity.data[0..size].par_chunks_exact_mut(width).zip(self.conductivity.par_rows(ys, ye)),
            //     instances.reactivity.data[0..size].par_chunks_exact_mut(width).zip(self.reactivity.par_rows(ys, ye)),
            //     instances.energy.data[0..size].par_chunks_exact_mut(width).zip(self.energy.par_rows(ys, ye)),
            // ).into_par_iter().for_each(|((cnw, cnr), (cdw, cdr), (rew, rer), (enw, enr))| {
            //     cnw.copy_from_slice(&cnr[xs..xe]);
            //     cdw.copy_from_slice(&cdr[xs..xe]);
            //     rew.copy_from_slice(&rer[xs..xe]);
            //     enw.copy_from_slice(&enr[xs..xe]);
            // });
        } else {
            let start = std::time::Instant::now();
            instances.connex_number.data.clear();
            instances.conductivity.data.clear();
            instances.reactivity.data.clear();
            instances.energy.data.clear();
            for row in izip!(
                self.connex_numbers.rows(ys, ye),
                self.conductivity.rows(ys, ye),
                self.reactivity.rows(ys, ye),
                self.energy.rows(ys, ye)
            ) {
                instances
                    .connex_number
                    .data
                    .extend_from_slice(&row.0[xs..xe]);
                instances
                    .conductivity
                    .data
                    .extend_from_slice(&row.1[xs..xe]);
                instances.reactivity.data.extend_from_slice(&row.2[xs..xe]);
                instances.energy.data.extend_from_slice(&row.3[xs..xe]);
            }
            println!("{:?}", std::time::Instant::now() - start);
        }
        size
    }
}

