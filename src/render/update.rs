use wgpu::util::DeviceExt;

use crate::{camera::Camera, state::GameState};

use super::{
    buffer::Instance,
    uniform::{CameraUniform, TileViewUniform},
    Renderer,
};

impl Renderer {
    pub fn update(&mut self, state: &GameState) {
        self.update_tile_view(&state);
        self.update_instances(&state);
        self.update_camera(&state.camera);
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

    fn update_tile_view(&mut self, state: &GameState) {
        let width = state.colors.first().map(|row| row.len()).unwrap_or(0) as u32;
        let view = TileViewUniform::new([0.0, 0.0], width);
        if self.uniforms.tile_view != view {
            println!("{:?}", view);
            self.uniforms.tile_view = view;
            self.queue.write_buffer(
                &self.buffers.tile_view,
                0,
                bytemuck::cast_slice(&[self.uniforms.tile_view]),
            )
        }
    }

    fn update_camera(&mut self, camera: &Camera) {
        let size = self.window.inner_size();
        let new_uniform = CameraUniform::new(camera, &size);
        if self.uniforms.camera != new_uniform {
            self.uniforms.camera = new_uniform;
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
