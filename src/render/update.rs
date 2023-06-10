use wgpu::util::DeviceExt;

use crate::{camera::Camera, state::GameState};

use super::{buffer::Instance, Renderer, uniform::CameraUniform};

impl Renderer {
    pub fn update(&mut self, state: &GameState) {
        self.update_instances(&state);
        self.update_view(&state.camera);
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
            self.buffer.instance =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&self.instances),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            // TODO: uncomment once colors update
            // self.queue.write_buffer(
            //     &self.buffer.instance,
            //     0,
            //     bytemuck::cast_slice(&self.instances),
            // );
        }
    }

    fn update_view(&mut self, camera: &Camera) {
        let size = self.window.inner_size();
        let new_uniform = CameraUniform::new(camera, &size);
        if self.camera_uniform != new_uniform {
            self.camera_uniform = new_uniform;
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
}
