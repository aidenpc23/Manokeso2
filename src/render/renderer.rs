use crate::{board_view::BoardView, rsc::CLEAR_COLOR, state::ClientState, util::point::Point};
use wgpu::{util::StagingBelt, CommandEncoder};
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
    pub(super) encoder: Option<CommandEncoder>,
    pub(super) tile_pipeline: TilePipeline,
    pub(super) ui_pipeline: UIPipeline,
    pub(super) staging_belt: StagingBelt,
}

impl Renderer {
    pub async fn new(event_loop: &EventLoop<()>) -> Renderer {
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let render_surface = RenderSurface::init(&window).await;
        let tile_pipeline = TilePipeline::new(&render_surface);
        let ui_pipeline = UIPipeline::new(&render_surface);
        // not exactly sure what this number should be,
        // doesn't affect performance much and depends on "normal" zoom
        let staging_belt = StagingBelt::new(4096 * 4);

        Self {
            window,
            render_surface,
            encoder: None,
            tile_pipeline,
            ui_pipeline,
            staging_belt,
        }
    }

    pub fn start_encoder(&mut self) {
        self.encoder = Some(self.render_surface.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        ));
    }

    pub fn sync(&mut self, board_view: &mut BoardView) {
        let mut encoder = self.encoder.take().expect("encoder not started");
        let writer = &mut StagingBufWriter {
            device: &self.render_surface.device,
            belt: &mut self.staging_belt,
            encoder: &mut encoder,
        };
        self.tile_pipeline.sync(board_view, writer);
        self.encoder = Some(encoder);
    }

    pub fn render(&mut self, client: &ClientState, resize: bool) {
        let size = &self.window.inner_size();
        if resize {
            self.render_surface.resize(size);
        }

        let output = self.render_surface.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.encoder.take().expect("encoder not started");

        let writer = &mut StagingBufWriter {
            device: &self.render_surface.device,
            belt: &mut self.staging_belt,
            encoder: &mut encoder,
        };
        self.tile_pipeline
            .update(writer, &client.camera, size, &client.sender);
        self.ui_pipeline.update(client, size, &self.render_surface);

        {
            let render_pass = &mut writer
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
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
            self.tile_pipeline.draw(render_pass);
            self.ui_pipeline.draw(render_pass);
        }

        self.staging_belt.finish();
        self.render_surface
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();

        self.ui_pipeline.text.atlas.trim();
    }

    pub fn pixel_to_render(&self, pos: Point<f32>) -> Point<f32> {
        let size = self.window.inner_size();
        Point {
            x: pos.x * 2.0 / size.width as f32 - 1.0,
            y: -pos.y * 2.0 / size.height as f32 + 1.0,
        }
    }

    pub fn pixel_to_world(&self, pos: Point<f32>) -> Point<f32> {
        self.tile_pipeline
            .uniforms
            .camera
            .render_to_world(self.pixel_to_render(pos))
    }
}
