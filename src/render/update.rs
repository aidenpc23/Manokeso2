use wgpu::util::DeviceExt;

use crate::state::GameState;

use super::{
    buffer::Instance,
    Renderer, uniform::{CameraUniform, TileViewUniform},
};

impl Renderer {
    pub fn update(&mut self, state: &GameState) {
        self.update_instances(&state);
        self.update_view(&state);
    }

    fn update_instances(&mut self, state: &GameState) {
        let old_len = self.instances.len();
        self.instances = state
            .colors
            .iter()
            .flatten()
            .map(|c| Instance { color: c.clone() })
            .collect();
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
    }

    fn update_view(&mut self, state: &GameState) {
        let camera = &state.camera;
        let size = &self.window.inner_size();
        let uniform = CameraUniform::new(camera, size);

        let width = state.colors.first().map(|row| row.len()).unwrap_or(0) as u32;
        let view = TileViewUniform::new([0.0, 0.0], width);
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
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
