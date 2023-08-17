use crate::{
    client::Camera,
    message::CameraView,
    render::{
        tile::{CameraUniform, ConstsUniform, InstanceField, TileViewUniform},
        writer::StagingBufWriter,
    },
    sync::BoardView,
    sync::WorldInterface,
};
use wgpu::{BindGroup, RenderPass, RenderPipeline};
use winit::dpi::PhysicalSize;

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
    pub(super) tiles_dirty: bool,
}

impl TilePipeline {
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        self.instances.connex_number.set_in(render_pass);
        self.instances.stability.set_in(render_pass);
        self.instances.reactivity.set_in(render_pass);
        self.instances.energy.set_in(render_pass);

        render_pass.draw(0..4, 0..self.instances.connex_number.len() as u32);
    }

    pub fn sync(&mut self, view: &mut BoardView, writer: &mut StagingBufWriter) {
        let info = &mut view.info;
        let tile_view_changed = self
            .uniforms
            .tile_view
            .update(info.pos, info.slice.width as u32);

        // don't update tile buffers if paused and board section hasn't changed
        if info.dirty || tile_view_changed {
            let width = info.slice.width;
            let size = width * info.slice.height;

            let insts = &mut self.instances;

            insts.connex_number.update_rows(
                writer,
                view.connex_numbers.chunks_exact(width),
                width,
                size,
            );
            insts
                .stability
                .update_rows(writer, view.stability.chunks_exact(width), width, size);
            insts
                .reactivity
                .update_rows(writer, view.reactivity.chunks_exact(width), width, size);
            insts
                .energy
                .update_rows(writer, view.energy.chunks_exact(width), width, size);
            info.dirty = false;
            self.tiles_dirty = true;
        }
    }

    pub fn update(
        &mut self,
        writer: &mut StagingBufWriter,
        world: &WorldInterface,
        camera: &Camera,
        window_size: &PhysicalSize<u32>,
    ) {
        if self.tiles_dirty {
            let slice = &[self.uniforms.tile_view];
            writer
                .mut_view::<TileViewUniform>(&self.buffers.tile_view, slice.len())
                .copy_from_slice(bytemuck::cast_slice(slice));
            self.tiles_dirty = false;
        }
        if self.uniforms.camera.update(&camera, window_size) {
            let uniform = self.uniforms.camera;
            let (width, height) = uniform.world_dimensions();

            world.send(crate::message::ClientMessage::CameraUpdate(CameraView {
                pos: uniform.pos,
                width,
                height,
            }));
            let slice = &[uniform];
            writer
                .mut_view::<CameraUniform>(&self.buffers.camera, slice.len())
                .copy_from_slice(bytemuck::cast_slice(slice));
        }
    }
}
