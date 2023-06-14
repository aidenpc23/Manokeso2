use crate::state::GameState;

use super::{BoardView, CameraUniform, Renderer, TileViewUniform};

impl Renderer {
    pub fn update(&mut self, resize: bool) {
        let BoardView {bx, by, xs, xe, ys, ..} = self.slice;
        self.instances
            .connex_number
            .write_buf(&self.device, &self.queue);
        self.instances
            .conductivity
            .write_buf(&self.device, &self.queue);
        self.instances
            .reactivity
            .write_buf(&self.device, &self.queue);
        self.instances.energy.write_buf(&self.device, &self.queue);

        let view = TileViewUniform::new([bx + xs as f32, by + ys as f32], (xe - xs) as u32);
        if self.uniforms.tile_view != view {
            self.uniforms.tile_view = view;
            self.queue.write_buffer(
                &self.buffers.tile_view,
                0,
                bytemuck::cast_slice(&[self.uniforms.tile_view]),
            )
        }

        let size = &self.window.inner_size();
        if self.uniforms.camera_next != self.uniforms.camera {
            self.uniforms.camera = self.uniforms.camera_next;
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
