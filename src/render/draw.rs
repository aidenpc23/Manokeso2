use wgpu::{CommandEncoder, TextureView};

use crate::rsc::CLEAR_COLOR;

use super::Renderer;

impl Renderer {
    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.draw(&mut encoder, &view);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    /// Uses the encoder to send commands to the GPU; this draws to the screen
    fn draw(&mut self, encoder: &mut CommandEncoder, view: &TextureView) {
        let render_pass = &mut encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        self.instances.connex_number.set_in(render_pass);
        self.instances.stability.set_in(render_pass);
        self.instances.reactivity.set_in(render_pass);
        self.instances.energy.set_in(render_pass);

        render_pass.draw(0..4, 0..self.instances.connex_number.len() as _);
    }
}
