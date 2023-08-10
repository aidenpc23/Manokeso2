use crate::{camera::Camera, rsc::CLEAR_COLOR, state::GameState};
use wgpu::util::StagingBelt;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use super::{
    surface::RenderSurface, tile::pipeline::TilePipeline, ui::pipeline::UIPipeline,
    writer::StagingBufWriter,
};

pub struct Renderer {
    pub window: Window,
    pub(super) render_surface: RenderSurface,
    pub(super) tile_pipeline: TilePipeline,
    pub(super) ui_pipeline: UIPipeline,
    pub(super) staging_belt: StagingBelt,
}

impl Renderer {
    pub async fn new(event_loop: &EventLoop<()>, camera: &Camera) -> Renderer {
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let size = window.inner_size();
        let render_surface = RenderSurface::init(&window).await;
        let tile_pipeline = TilePipeline::new(&render_surface, &camera, &size);
        let ui_pipeline = UIPipeline::new(&render_surface);
        // not exactly sure what this number should be,
        // doesn't affect performance much and depends on "normal" zoom
        let staging_belt = StagingBelt::new(4096 * 4);

        Self {
            window,
            render_surface,
            tile_pipeline,
            ui_pipeline,
            staging_belt,
        }
    }

    pub fn render(&mut self, state: &GameState, resize: bool) {
        if resize {
            self.render_surface.resize(&self.window.inner_size());
        }

        let output = self.render_surface.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.render_surface
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        self.tile_pipeline.update(&mut StagingBufWriter {
            device: &self.render_surface.device,
            belt: &mut self.staging_belt,
            encoder: &mut encoder,
        }, state, &self.window.inner_size());

        {
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
            self.ui_pipeline.draw(render_pass);
            self.tile_pipeline.draw(render_pass);
        }

        self.staging_belt.finish();
        self.render_surface
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();
    }

    pub fn pixel_to_render(&self, pos: [f32; 2]) -> [f32; 2] {
        let size = self.window.inner_size();
        return [
            pos[0] * 2.0 / size.width as f32 - 1.0,
            -pos[1] * 2.0 / size.height as f32 + 1.0,
        ];
    }

    pub fn pixel_to_tile(&self, pos: [f32; 2]) -> [f32; 2] {
        self.tile_pipeline
            .uniforms
            .camera
            .render_to_tile(self.pixel_to_render(pos))
    }
}
