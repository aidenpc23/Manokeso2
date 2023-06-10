use std::time;

use wgpu::util::DeviceExt;

use crate::state::GameState;

const CHUNK_SHIFT: u32 = 5;
const CHUNK_SIZE: usize = 2usize.pow(CHUNK_SHIFT);

use super::{
    uniform::{CameraUniform, TileViewUniform},
    Renderer,
};

impl Renderer {
    pub fn update(&mut self, state: &GameState, resize: bool) {
        let camera = &state.camera;
        let size = &self.window.inner_size();
        let uniform = CameraUniform::new(camera, size);

        let [xs, ys] = self.uniforms.camera.bottom_left();
        let [xe, ye] = self.uniforms.camera.top_right();
        let [bx, by] = [0.0, 0.0];
        let bxe = state.board.width();
        let bye = state.board.height();
        let mut xs = f32::max(xs - bx + 0.5, 0.0) as usize;
        let mut ys = f32::max(ys - by + 0.5, 0.0) as usize;
        let mut xe = f32::max(xe - bx + 1.5, xs as f32) as usize;
        let mut ye = f32::max(ye - by + 1.5, ys as f32) as usize;
        xs = usize::min((xs >> CHUNK_SHIFT) << CHUNK_SHIFT, bxe);
        ys = usize::min((ys >> CHUNK_SHIFT) << CHUNK_SHIFT, bye);
        xe = usize::min(((xe >> CHUNK_SHIFT) << CHUNK_SHIFT) + CHUNK_SIZE, bxe);
        ye = usize::min(((ye >> CHUNK_SHIFT) << CHUNK_SHIFT) + CHUNK_SIZE, bye);

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
