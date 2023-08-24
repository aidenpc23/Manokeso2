use crate::{client::Camera, message::CameraView, rsc::CLEAR_COLOR, util::point::Point};
use wgpu::{util::StagingBelt, CommandEncoder};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use super::{
    shape::pipeline::ShapePipeline,
    surface::RenderSurface,
    text::pipeline::TextPipeline,
    texture::pipeline::TexturePipeline,
    tile::{
        data::{RenderViewInfo, TileData},
        pipeline::TilePipeline,
    },
    ui::RenderableUI,
};

pub struct Renderer<T: TileData> {
    pub window: Window,
    pub render_surface: RenderSurface,
    encoder: Option<CommandEncoder>,
    tile_pipeline: TilePipeline<T>,
    shape_pipeline: ShapePipeline,
    texture_pipeline: TexturePipeline,
    text_pipeline: TextPipeline,
    staging_belt: StagingBelt,
}

impl<T: TileData> Renderer<T> {
    pub async fn new(event_loop: &EventLoop<()>, tile_shader: &str) -> Self {
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let render_surface = RenderSurface::new(&window).await;
        // not exactly sure what this number should be,
        // doesn't affect performance much and depends on "normal" zoom
        let staging_belt = StagingBelt::new(4096 * 4);

        Self {
            window,
            encoder: None,
            tile_pipeline: TilePipeline::new(&render_surface, tile_shader),
            shape_pipeline: ShapePipeline::new(&render_surface),
            text_pipeline: TextPipeline::new(&render_surface),
            texture_pipeline: TexturePipeline::new(&render_surface),
            staging_belt,
            render_surface,
        }
    }

    pub fn start_encoder(&mut self) {
        self.encoder = Some(self.render_surface.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        ));
    }

    pub fn sync<'a>(&mut self, info: &RenderViewInfo, data: &T::UpdateData<'a>) {
        let mut encoder = self.encoder.take().expect("encoder not started");
        self.tile_pipeline.sync(
            &self.render_surface.device,
            &mut encoder,
            &mut self.staging_belt,
            info,
            data,
        );
        self.encoder = Some(encoder);
    }

    pub fn update(
        &mut self,
        camera: &Camera,
        ui: &RenderableUI,
        resized: bool,
    ) -> Option<CameraView> {
        let size = &self.window.inner_size();
        if resized {
            self.render_surface.resize(size);
        }

        let mut encoder = self.encoder.take().expect("encoder not started");
        let camera_view = self.tile_pipeline.update(
            &self.render_surface.device,
            &mut encoder,
            &mut self.staging_belt,
            camera,
            size,
        );
        self.encoder = Some(encoder);

        self.text_pipeline.update(&self.render_surface, &ui.text);
        self.shape_pipeline.update(&self.render_surface, &ui.rounded_rects, resized);

        camera_view
    }

    pub fn draw(&mut self) {
        let output = self.render_surface.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.encoder.take().expect("encoder not started");
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
            self.tile_pipeline.draw(render_pass);
            self.shape_pipeline.draw(render_pass);
            // self.texture_pipeline.draw(render_pass);
            self.text_pipeline.draw(render_pass);
        }

        self.staging_belt.finish();
        self.render_surface
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();

        self.text_pipeline.atlas.trim();
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
