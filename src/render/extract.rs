use super::{buffer::CameraUniform, BoardView, Renderer};
use crate::state::GameState;

impl Renderer {
    pub fn extract(&mut self, state: &GameState) {
        let slice = self.calc_board_slice(state);
        let BoardView { xs, xe, ys, ye, .. } = slice;

        let board = &state.board;
        let width = xe - xs;
        let size = width * (ye - ys);

        let insts = &mut self.instances;
        insts
            .connex_number
            .update_rows(board.connex_numbers.par_rows(ys, ye), size, xs, xe);
        insts
            .stability
            .update_rows(board.stability.par_rows(ys, ye), size, xs, xe);
        insts
            .reactivity
            .update_rows(board.reactivity.par_rows(ys, ye), size, xs, xe);
        insts
            .energy
            .update_rows(board.energy.par_rows(ys, ye), size, xs, xe);

        self.slice = slice;
        self.uniforms.camera_next = CameraUniform::new(&state.camera, &self.window.inner_size());
    }

    fn calc_board_slice(&self, state: &GameState) -> BoardView {
        // get positions in the world
        let [bx, by] = state.board.pos;
        let bw = state.board.width();
        let bh = state.board.height();
        let [cw, ch] = self.uniforms.camera.world_dimensions();
        // get camera position relative to board
        let x = (state.camera.pos[0] - bx + 0.5) as i32;
        let y = (state.camera.pos[1] - by + 0.5) as i32;
        // calculate chunk size based on max camera dimension
        let chunk_align = (cw.max(ch) as u32).max(1).ilog2();
        let chunk_size = 2i32.pow(chunk_align);
        let chunk_mask = !(chunk_size - 1);
        // align with chunks and add an extra chunk in each direction
        // s = start, e = end
        let xs = (x & chunk_mask) - 1 * chunk_size;
        let ys = (y & chunk_mask) - 1 * chunk_size;
        let xe = (x & chunk_mask) + 2 * chunk_size;
        let ye = (y & chunk_mask) + 2 * chunk_size;
        // cut off values for bounds
        let xs = (xs.max(0) as usize).min(bw);
        let ys = (ys.max(0) as usize).min(bh);
        let xe = (xe.max(0) as usize).min(bw);
        let ye = (ye.max(0) as usize).min(bh);

        BoardView {
            bx: state.board.pos[0],
            by: state.board.pos[1],
            xs,
            xe,
            ys,
            ye,
        }
    }
}
