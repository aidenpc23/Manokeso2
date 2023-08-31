use std::num::NonZeroU64;

use crate::{
    client::Camera,
    common::message::CameraView,
    render::tile::{CameraUniform, ConstsUniform, BoardViewUniform},
};
use wgpu::{util::StagingBelt, BindGroup, CommandEncoder, Device, RenderPass, RenderPipeline};
use winit::dpi::PhysicalSize;

use super::data::{TileData, TileUpdateData};

pub struct Buffers {
    pub camera: wgpu::Buffer,
    pub tile_view: wgpu::Buffer,
    pub consts: wgpu::Buffer,
}

pub struct Uniforms {
    pub camera: CameraUniform,
    pub tile_view: BoardViewUniform,
    pub consts: ConstsUniform,
}

pub struct TilePipeline<T: TileData> {
    pub(super) pipeline: RenderPipeline,
    pub(super) data: T,
    pub(super) buffers: Buffers,
    pub(super) bind_group: BindGroup,
    pub uniforms: Uniforms,
}

impl<T: TileData> TilePipeline<T> {
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, &self.bind_group, &[]);

        self.data.set_in(render_pass);

        render_pass.draw(0..4, 0..self.data.len() as u32);
    }

    pub fn update<'a>(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
        data: Option<T::UpdateData<'a>>,
        camera: &Camera,
        window_size: &PhysicalSize<u32>,
    ) -> Option<CameraView> {
        if let Some(data) = data {
            let slice = data.slice();
            self.data
                .update_rows(device, encoder, belt, &data, slice.size);
            if self
                .uniforms
                .tile_view
                .update(slice.world_pos, slice.width as u32)
            {
                let slice = &[self.uniforms.tile_view];
                let mut view = belt.write_buffer(
                    encoder,
                    &self.buffers.tile_view,
                    0,
                    unsafe {
                        NonZeroU64::new_unchecked(
                            (slice.len() * std::mem::size_of::<BoardViewUniform>()) as u64,
                        )
                    },
                    device,
                );
                view.copy_from_slice(bytemuck::cast_slice(slice));
            }
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
