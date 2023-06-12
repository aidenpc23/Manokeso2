use itertools::izip;
use ndarray::{azip, s, Axis};

use crate::world::Board;

use super::Instance;

impl Board {
    pub fn update_instances(
        &self,
        instances: &mut Vec<Instance>,
        xs: usize,
        xe: usize,
        ys: usize,
        ye: usize,
    ) -> usize {
        let width = xe - xs;
        let size = width * (ye - ys);
        if instances.len() >= size {
            let mut i = 0;
            for rows in izip!(
                self.connex_numbers.current()[ys..ye].iter(),
                self.conductivity.current()[ys..ye].iter(),
                self.reactivity.current()[ys..ye].iter(),
                self.energy.current()[ys..ye].iter()
            ) {
                for row in izip!(
                    &rows.0[xs..xe],
                    &rows.1[xs..xe],
                    &rows.2[xs..xe],
                    &rows.3[xs..xe]
                ) {
                    let tile = &mut instances[i];
                    tile.connex_number = *row.0 as u32;
                    tile.stability = *row.1;
                    tile.reactivity = *row.2;
                    tile.energy = *row.3;
                    i += 1;
                }
            }

            // azip!((index (y, x),
            //     &c in &self.connex_numbers.current().slice(s![xs..xe, ys..ye]),
            //     &s in &self.stability.current().slice(s![xs..xe, ys..ye]),
            //     &r in &self.reactivity.current().slice(s![xs..xe, ys..ye]),
            //     &e in &self.energy.current().slice(s![xs..xe, ys..ye])
            // ) {
            //     let i = y * width + x;
            //     let instance: &mut Instance = &mut instances[i];
            //     instance.connex_number = c;
            //     instance.stability = s;
            //     instance.reactivity = r;
            //     instance.energy = e;
            // });
            // for y in ys..ye {
            //     for x in xs..xe {
            //         let i = (y - ys) * width + (x - xs);
            //         let attrs = &mut instances[i].attributes;
            //         let test = self.test[x][y];
            //         attrs[0] = *self.connex_numbers.get((x, y)).unwrap() as f32;
            //         attrs[1] = *self.stability.get((x, y)).unwrap();
            //         attrs[2] = *self.reactivity.get((x, y)).unwrap();
            //         attrs[3] = *self.energy.get((x, y)).unwrap();
            //     }
            // }
        } else {
            instances.clear();
            for rows in izip!(
                self.connex_numbers.current()[ys..ye].iter(),
                self.conductivity.current()[ys..ye].iter(),
                self.reactivity.current()[ys..ye].iter(),
                self.energy.current()[ys..ye].iter()
            ) {
                for row in izip!(
                    &rows.0[xs..xe],
                    &rows.1[xs..xe],
                    &rows.2[xs..xe],
                    &rows.3[xs..xe]
                ) {
                    instances.push(Instance {
                        connex_number: *row.0 as u32,
                        stability: *row.1,
                        reactivity: *row.2,
                        energy: *row.3,
                    });
                }
            }
        }
        size
    }
}
