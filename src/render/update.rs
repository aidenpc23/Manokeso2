use wgpu::util::DeviceExt;

use crate::{camera::Camera, state::GameState};

use super::{buffer::Instance, Renderer};

impl Renderer {
    pub fn update(&mut self, state: &GameState) {
        if self.instances.len() != state.colors.len() {
            self.instances = state
                .colors
                .iter()
                .map(|c| Instance {
                    position: [0, 0],
                    color: c.clone(),
                })
                .collect();
            self.buffer.instance = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&self.instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        } else {
            self.queue.write_buffer(
                &self.buffer.instance,
                0,
                bytemuck::cast_slice(&self.instances),
            );
        }
        self.update_view(&state.camera);
    }

    fn update_view(&mut self, camera: &Camera) {
        let size = self.window.inner_size();
        self.camera_uniform
            .update_view_proj(&camera, &[size.width, size.height]);
        self.queue.write_buffer(
            &self.buffer.camera,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}
