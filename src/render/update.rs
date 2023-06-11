use wgpu::util::DeviceExt;

use crate::state::GameState;

const CHUNK_ALIGN: u32 = 5;
const CHUNK_SIZE: i32 = 2i32.pow(CHUNK_ALIGN);
const CHUNK_MASK: i32 = !(CHUNK_SIZE-1);

use super::{
    uniform::{CameraUniform, TileViewUniform},
    Renderer,
};

impl Renderer {
    pub fn update(&mut self, state: &GameState, resize: bool) {
        let camera = &state.camera;
        let size = &self.window.inner_size();
        let uniform = CameraUniform::new(camera, size);

        // get positions in the world
        // s = start, e = end
        let [cxs, cys] = self.uniforms.camera.bottom_left();
        let [cxe, cye] = self.uniforms.camera.top_right();
        let [bx, by] = [0.0, 0.0]; // TODO: get board x and y from state
        let bw = state.board.width();
        let bh = state.board.height();
        // calculate camera position relative to board position
        // also account for tile mesh (-.5 -> .5)
        let xs = (cxs - bx + 0.5) as i32;
        let ys = (cys - by + 0.5) as i32;
        let xe = (cxe - bx + 1.5) as i32;
        let ye = (cye - by + 1.5) as i32;
        // align with chunks and add an extra chunk in each direction
        let xs = (xs & CHUNK_MASK) - CHUNK_SIZE;
        let ys = (ys & CHUNK_MASK) - CHUNK_SIZE;
        let xe = (xe & CHUNK_MASK) + 2 * CHUNK_SIZE;
        let ye = (ye & CHUNK_MASK) + 2 * CHUNK_SIZE;
        // cut off values for bounds
        let xs = (xs.max(0) as usize).min(bw);
        let ys = (ys.max(0) as usize).min(bh);
        let xe = (xe.max(0) as usize).min(bw);
        let ye = (ye.max(0) as usize).min(bh);

        let old_len = self.instances.len();
        self.instances = state.board.render_attributes(xs, xe, ys, ye);
        if old_len != self.instances.len() {
            self.buffers.instance =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&self.instances),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            self.queue.write_buffer(
                &self.buffers.instance,
                0,
                bytemuck::cast_slice(&self.instances),
            );
        }

        let view = TileViewUniform::new([xs as f32, ys as f32], (xe - xs) as u32);
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
}
