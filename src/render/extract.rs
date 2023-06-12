use ndarray::{s, Axis};

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
            // let mut i = 0;
            // for rows in self.test[ys..ye].iter() {
            //     for row in rows[xs..xe].iter() {
            //         let tile = &mut instances[i];
            //         tile.connex_number = row[0] as u32;
            //         tile.stability = row[1];
            //         tile.reactivity = row[2];
            //         tile.energy = row[3];
            //         i += 1;
            //     }
            // }
            let mut i = 0;
            for row in self.test.slice(s![ys..ye, xs..xe]).axis_iter(Axis(0)) {
                instances[i..i+width].copy_from_slice(&row.as_slice().unwrap());
                i += width;
            }
            // azip!((index (y, x),
            //     &c in &self.connex_numbers.slice(s![xs..xe, ys..ye]),
            //     &s in &self.stability.slice(s![xs..xe, ys..ye]),
            //     &r in &self.reactivity.slice(s![xs..xe, ys..ye]),
            //     &e in &self.energy.slice(s![xs..xe, ys..ye])
            // ) {
            //     let i = y * width + x;
            //     let attrs = &mut instances[i].attributes;
            //     attrs[0] = c as f32;
            //     attrs[1] = s;
            //     attrs[2] = r;
            //     attrs[3] = e;
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
            for row in self.test.slice(s![ys..ye, xs..xe]).axis_iter(Axis(0)) {
                instances.extend_from_slice(&row.as_slice().unwrap());
            }
            // for rows in self.test[ys..ye].iter() {
            //     for row in rows[xs..xe].iter() {
            //         instances.push(Instance {
            //             connex_number: row[0] as u32,
            //             stability: row[1],
            //             reactivity: row[2],
            //             energy: row[3],
            //         })
            //     }
            // }
        }
        size
    }
}
