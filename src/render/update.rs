use wgpu::util::DeviceExt;

use crate::state::GameState;

use super::{
    uniform::{CameraUniform, TileViewUniform},
    Renderer,
};

impl Renderer {
    pub fn update(&mut self, state: &GameState, resize: bool) {
        let camera = &state.camera;
        let size = &self.window.inner_size();
        let uniform = CameraUniform::new(camera, size);

        let (xs, xe, ys, ye) = self.calc_board_slice(state);
        // let start = time::Instant::now();
        let len = state
            .board
            .update_instances(&mut self.instances, xs, xe, ys, ye);
        // let taken = time::Instant::now() - start;
        // println!("{:?}", taken);
        self.instances.connex_number.write(&self.device, &self.queue, len);
        self.instances.conductivity.write(&self.device, &self.queue, len);
        self.instances.reactivity.write(&self.device, &self.queue, len);
        self.instances.energy.write(&self.device, &self.queue, len);

        let [bx, by] = state.board.pos;
        let view = TileViewUniform::new([bx + xs as f32, by + ys as f32], (xe - xs) as u32);
        if self.uniforms.tile_view != view {
            self.uniforms.tile_view = view;
            self.queue.write_buffer(
                &self.buffers.tile_view,
                0,
                bytemuck::cast_slice(&[self.uniforms.tile_view]),
            )
        }

        if self.uniforms.camera != uniform {
            self.uniforms.camera = uniform;
            self.queue.write_buffer(
                &self.buffers.camera,
                0,
                bytemuck::cast_slice(&[self.uniforms.camera]),
            );
            if resize {
                self.config.width = size.width;
                self.config.height = size.height;
                self.surface.configure(&self.device, &self.config);
            }
        }
    }

    fn calc_board_slice(&self, state: &GameState) -> (usize, usize, usize, usize) {
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
        (xs, xe, ys, ye)
    }
}
