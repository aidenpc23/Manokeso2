use crate::{
    client::Camera,
    common::message::CameraView,
};
use wgpu::{util::StagingBelt, BindGroup, CommandEncoder, Device, RenderPass, RenderPipeline};
use winit::dpi::PhysicalSize;

use super::{
    data::{TileData, TileUpdateData},
    view::BoardViews, camera::RenderCamera, consts::RenderConsts,
};

pub struct TilePipeline<T: TileData> {
    pub(super) pipeline: RenderPipeline,
    pub(super) data: Vec<T>,
    pub(super) board_views: BoardViews,
    pub(super) bind_group: BindGroup,
    pub camera: RenderCamera,
    pub(super) consts: RenderConsts,
}

impl<T: TileData> TilePipeline<T> {
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);

        for (i, board) in self.data.iter().enumerate() {
            render_pass.set_bind_group(0, &self.bind_group, &[i as u32 * 256]);
            board.set_in(render_pass);
            render_pass.draw(0..4, 0..board.len() as u32);
        }
    }

    pub fn update<'a>(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
        data_updates: Option<Vec<T::UpdateData<'a>>>,
        camera: &Camera,
        window_size: &PhysicalSize<u32>,
    ) -> Option<CameraView> {
        if let Some(updates) = data_updates {
            if self.data.len() < updates.len() {
                self.data.resize_with(updates.len(), || T::init(&device));
            }
            for (i, (update, data)) in updates.iter().zip(&mut self.data).enumerate() {
                let slice = update.slice();
                data.update_rows(device, encoder, belt, &update, slice.size);
                self.board_views.update(i, slice.world_pos, slice.width);
            }
            self.board_views.write(belt, encoder, device);
        }
        self.camera.update(camera, window_size, device, encoder, belt)
    }
}
