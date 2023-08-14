use crate::{
    render::{
        tile::{CameraUniform, ConstsUniform, InstanceField, TileViewUniform},
        writer::StagingBufWriter,
    },
    state::GameState,
};
use wgpu::{BindGroup, RenderPass, RenderPipeline};
use winit::dpi::PhysicalSize;

use super::view::BoardView;

pub const SHADER: &str = concat!(include_str!("./shader.wgsl"));

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

pub struct Instances {
    pub connex_number: InstanceField<0, u32>,
    pub stability: InstanceField<1, f32>,
    pub reactivity: InstanceField<2, f32>,
    pub energy: InstanceField<3, f32>,
}

pub struct TilePipeline {
    pub(super) pipeline: RenderPipeline,
    pub(super) instances: Instances,
    pub(super) buffers: Buffers,
    pub(super) camera_bind_group: BindGroup,
    pub uniforms: Uniforms,
}

impl TilePipeline {
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        self.instances.connex_number.set_in(render_pass);
        self.instances.stability.set_in(render_pass);
        self.instances.reactivity.set_in(render_pass);
        self.instances.energy.set_in(render_pass);

        render_pass.draw(0..4, 0..self.instances.connex_number.len() as _);
    }

    pub fn update(
        &mut self,
        writer: &mut StagingBufWriter,
        state: &GameState,
        window_size: &PhysicalSize<u32>,
    ) {
        let slice = self.calc_board_slice(state);
        let BoardView {
            bx,
            by,
            xs,
            xe,
            ys,
            ye,
        } = slice;

        let view = TileViewUniform::new([bx + xs as f32, by + ys as f32], (xe - xs) as u32);
        let tile_view_changed = self.uniforms.tile_view != view;

        // don't update tile buffers if paused and board section hasn't changed
        if state.board.dirty || tile_view_changed {
            let board = &state.board;
            let width = xe - xs;
            let size = width * (ye - ys);

            let insts = &mut self.instances;
            insts.connex_number.update_rows(
                writer,
                board.connex_numbers.par_rows(ys, ye),
                size,
                xs,
                xe,
            );
            insts
                .stability
                .update_rows(writer, board.stability.par_rows(ys, ye), size, xs, xe);
            insts
                .reactivity
                .update_rows(writer, board.reactivity.par_rows(ys, ye), size, xs, xe);
            insts
                .energy
                .update_rows(writer, board.energy.par_rows(ys, ye), size, xs, xe);
        }

        if tile_view_changed {
            self.uniforms.tile_view = view;
            let slice = &[self.uniforms.tile_view];
            writer
                .mut_view::<TileViewUniform>(&self.buffers.tile_view, slice.len())
                .copy_from_slice(bytemuck::cast_slice(slice));
        }

        if self.uniforms.camera.update(&state.camera, window_size) {
            let slice = &[self.uniforms.camera];
            writer
                .mut_view::<CameraUniform>(&self.buffers.camera, slice.len())
                .copy_from_slice(bytemuck::cast_slice(slice));
        }
    }

    fn calc_board_slice(&self, state: &GameState) -> BoardView {
        // get positions in the world
        let b = state.board.pos;
        let bw = state.board.width();
        let bh = state.board.height();
        let (cw, ch) = self.uniforms.camera.world_dimensions();
        // get camera position relative to board
        let x = (state.camera.pos.x - b.x + 0.5) as i32;
        let y = (state.camera.pos.y - b.y + 0.5) as i32;
        // calculate chunk size based on max camera dimension
        let chunk_align = (cw.max(ch) as u32).max(1).ilog2();
        let chunk_size = 2i32.pow(chunk_align);
        let chunk_mask = !(chunk_size - 1);
        // align with chunks and add an extra chunk in each direction
        // s = start, e = end
        let xs = (x & chunk_mask) - 1 * chunk_size;
        let ys = (y & chunk_mask) - 1 * chunk_size;
        let xe = (x & chunk_mask) + 2 * chunk_size;
        let ye = (y & chunk_mask) + 2 * chunk_size;
        // cut off values for bounds
        let xs = (xs.max(0) as usize).min(bw);
        let ys = (ys.max(0) as usize).min(bh);
        let xe = (xe.max(0) as usize).min(bw);
        let ye = (ye.max(0) as usize).min(bh);

        BoardView {
            bx: b.x,
            by: b.y,
            xs,
            xe,
            ys,
            ye,
        }
    }
}
