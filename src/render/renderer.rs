use std::sync::Arc;

use crate::{client::Camera, common::message::CameraView, rsc::CLEAR_COLOR, util::point::Point};
use wgpu::{util::StagingBelt, CommandEncoder};
use winit::{
    event_loop::{ActiveEventLoop},
    window::{Fullscreen, Window, WindowAttributes},
};

use super::{
    primitive::UIPrimatives,
    shape::pipeline::ShapePipeline,
    surface::RenderSurface,
    text::pipeline::TextPipeline,
    texture::pipeline::TexturePipeline,
    tile::{data::TileData, pipeline::TilePipeline},
};

pub struct Renderer<'a, T: TileData> {
    pub window: Arc<Window>,
    pub render_surface: RenderSurface<'a>,
    encoder: CommandEncoder,
    tile_pipeline: TilePipeline<T>,
    shape_pipeline: ShapePipeline,
    texture_pipeline: TexturePipeline,
    text_pipeline: TextPipeline,
    staging_belt: StagingBelt,
}

impl<'a, T: TileData> Renderer<'a, T> {
    pub fn new(event_loop: &ActiveEventLoop, tile_shader: &str, fullscreen: bool) -> Self {
        let window = Arc::new(event_loop
            .create_window(WindowAttributes::default()/*.with_visible(false)*/)
            .unwrap());

        if fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)))
        }

        let render_surface = pollster::block_on(RenderSurface::new(window.clone()));
        // not exactly sure what this number should be,
        // doesn't affect performance much and depends on "normal" zoom
        let staging_belt = StagingBelt::new(4096 * 4);

        Self {
            window,
            encoder: Self::create_encoder(&render_surface.device),
            tile_pipeline: TilePipeline::new(&render_surface, tile_shader),
            shape_pipeline: ShapePipeline::new(&render_surface),
            text_pipeline: TextPipeline::new(&render_surface),
            texture_pipeline: TexturePipeline::new(&render_surface),
            staging_belt,
            render_surface,
        }
    }

    fn create_encoder(device: &wgpu::Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        })
    }

    pub fn update(
        &mut self,
        data: Option<T::UpdateData<'_>>,
        camera: &Camera,
        resized: bool,
    ) -> Option<CameraView> {
        let size = &self.window.inner_size();
        if resized {
            self.render_surface.resize(size);
        }

        let camera_view = self.tile_pipeline.update(
            &self.render_surface.device,
            &mut self.encoder,
            &mut self.staging_belt,
            data,
            camera,
            size,
        );

        camera_view
    }

    pub fn update_ui(&mut self, ui: &UIPrimatives, resized: bool) {
        self.text_pipeline.update(&self.render_surface, &ui.text);
        self.shape_pipeline
            .update(&self.render_surface, &ui.rounded_rects, resized);
    }

    pub fn draw(&mut self) {
        let output = self.render_surface.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = std::mem::replace(&mut self.encoder, Self::create_encoder(&self.render_surface.device));
        {
            let render_pass = &mut encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
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

    pub fn render_to_pixel(&self, pos: Point<f32>) -> Point<f32> {
        let size = self.window.inner_size();
        Point {
            x: (pos.x + 1.0) / 2.0 * size.width as f32,
            y: (-pos.y + 1.0) / 2.0 * size.height as f32,
        }
    }

    pub fn world_to_pixel(&self, pos: Point<f32>) -> Point<f32> {
        self.render_to_pixel(self.tile_pipeline.uniforms.camera.world_to_render(pos))
    }
}
