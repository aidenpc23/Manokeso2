use crate::world::Board;
use super::Renderer;

impl Renderer {
    pub fn extract(
        &mut self,
        board: &Board,
        xs: usize,
        xe: usize,
        ys: usize,
        ye: usize,
    ) -> usize {
        let insts = &mut self.instances;
        let width = xe - xs;
        let size = width * (ye - ys);
        insts.connex_number.update_rows(board.connex_numbers.par_rows(ys, ye), size, xs, xe);
        insts.conductivity.update_rows(board.stability.par_rows(ys, ye), size, xs, xe);
        insts.reactivity.update_rows(board.reactivity.par_rows(ys, ye), size, xs, xe);
        insts.energy.update_rows(board.energy.par_rows(ys, ye), size, xs, xe);
        size
    }
}

