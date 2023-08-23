use std::num::NonZeroU64;

use crate::{
    client::Camera,
    message::CameraView,
    render::tile::{CameraUniform, ConstsUniform, TileViewUniform},
};
use wgpu::{util::StagingBelt, BindGroup, CommandEncoder, Device, RenderPass, RenderPipeline};
use winit::dpi::PhysicalSize;

use super::data::{RenderViewInfo, TileData};

pub struct Buffers {
    pub camera: wgpu::Buffer,
    pub tile_view: wgpu::Buffer,
    pub consts: wgpu::Buffer,
}

pub struct Uniforms {
    pub camera: CameraUniform,
    pub tile_view: TileViewUniform,
    pub consts: ConstsUniform,
}

pub struct TilePipeline<T: TileData> {
    pub(super) pipeline: RenderPipeline,
    pub(super) data: T,
    pub(super) buffers: Buffers,
    pub(super) bind_group: BindGroup,
    pub uniforms: Uniforms,
    pub(super) tiles_dirty: bool,
}

impl<T: TileData> TilePipeline<T> {
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, &self.bind_group, &[]);

        self.data.set_in(render_pass);

        render_pass.draw(0..4, 0..self.data.len() as u32);
    }

    pub fn sync<'a>(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
        info: &RenderViewInfo,
        data: &T::UpdateData<'a>,
    ) {
        let tile_view_changed = self
            .uniforms
            .tile_view
            .update(info.pos, info.slice.width as u32);

        // don't update tile buffers if paused and board section hasn't changed
        if info.dirty || tile_view_changed {
            let width = info.slice.width;
            let size = width * info.slice.height;

            self.data
                .update_rows(device, encoder, belt, data, width, size);

            self.tiles_dirty = true;
        }
    }

    pub fn update(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
        camera: &Camera,
        window_size: &PhysicalSize<u32>,
    ) -> Option<CameraView> {
        if self.tiles_dirty {
            let slice = &[self.uniforms.tile_view];
            let mut view = belt.write_buffer(
                encoder,
                &self.buffers.tile_view,
                0,
                unsafe {
                    NonZeroU64::new_unchecked(
                        (slice.len() * std::mem::size_of::<TileViewUniform>()) as u64,
                    )
                },
                device,
            );
            view.copy_from_slice(bytemuck::cast_slice(slice));
            self.tiles_dirty = false;
        }
        let mut camera_view = None;
        if self.uniforms.camera.update(&camera, window_size) {
            let uniform = self.uniforms.camera;
            let (width, height) = uniform.world_dimensions();

            camera_view = Some(CameraView {
                pos: uniform.pos,
                width,
                height,
            });
            let slice = &[uniform];
            let mut view = belt.write_buffer(
                encoder,
                &self.buffers.camera,
                0,
                unsafe {
                    NonZeroU64::new_unchecked(
                        (slice.len() * std::mem::size_of::<CameraUniform>()) as u64,
                    )
                },
                device,
            );
            view.copy_from_slice(bytemuck::cast_slice(slice));
        }
        camera_view
    }
}
