use itertools::izip;

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
            let mut i = 0;
            for row in izip!(
                self.connex_numbers.current()[ys..ye].iter(),
                self.conductivity.current()[ys..ye].iter(),
                self.reactivity.current()[ys..ye].iter(),
                self.energy.current()[ys..ye].iter()
            ) {
                let ds = i * width;
                let de = ds + width;
                instances.connex_number.data[ds..de].copy_from_slice(&row.0[xs..xe]);
                instances.conductivity.data[ds..de].copy_from_slice(&row.1[xs..xe]);
                instances.reactivity.data[ds..de].copy_from_slice(&row.2[xs..xe]);
                instances.energy.data[ds..de].copy_from_slice(&row.3[xs..xe]);
                i += 1;
            }
        } else {
            instances.connex_number.data.clear();
            instances.conductivity.data.clear();
            instances.reactivity.data.clear();
            instances.energy.data.clear();
            for row in izip!(
                self.connex_numbers.current()[ys..ye].iter(),
                self.conductivity.current()[ys..ye].iter(),
                self.reactivity.current()[ys..ye].iter(),
                self.energy.current()[ys..ye].iter()
            ) {
                instances.connex_number.data.extend_from_slice(&row.0[xs..xe]);
                instances.conductivity.data.extend_from_slice(&row.1[xs..xe]);
                instances.reactivity.data.extend_from_slice(&row.2[xs..xe]);
                instances.energy.data.extend_from_slice(&row.3[xs..xe]);
            }
        }
        size
    }
}
