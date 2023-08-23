use wgpu::{util::StagingBelt, CommandEncoder, Device, RenderPass, VertexBufferLayout};

use crate::{util::point::Point, world::BoardSlice};

#[derive(Clone, Copy)]
pub struct RenderViewInfo {
    pub pos: Point<f32>,
    pub slice: BoardSlice,
    pub dirty: bool,
}

impl RenderViewInfo {
    pub fn new() -> Self {
        Self {
            pos: Point { x: 0.0, y: 0.0 },
            slice: BoardSlice::default(),
            dirty: false,
        }
    }
}

pub trait TileData {
    type UpdateData<'a>;
    fn init(device: &Device) -> Self;
    fn descs(&self) -> Vec<VertexBufferLayout>;
    fn set_in<'a>(&'a self, render_pass: &mut RenderPass<'a>);
    fn len(&self) -> usize;
    fn update_rows<'a>(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
        row_chunks: &Self::UpdateData<'a>,
        width: usize,
        size: usize,
    );
}

#[macro_export]
macro_rules! tile_render_data {
    ( $sname:ident, $vname:ident, [$( $loc:expr => $name:ident : $type:ident ),* $(,)? ] ) => {
        pub struct $sname {
            len: usize,
            $(
                pub $name: crate::render::tile::InstanceField<$type>,
            )*
        }
        pub struct $vname<'a> {
            $(
                pub $name: &'a [$type],
            )*
        }
        impl crate::render::tile::data::TileData for $sname {
            type UpdateData<'a> = $vname<'a>;
            fn init(device: &wgpu::Device) -> Self {
                Self {
                    len: 0,
                    $(
                        $name: crate::render::tile::InstanceField::init(device, stringify!($name), $loc),
                    )*
                }
            }
            fn descs(&self) -> Vec<wgpu::VertexBufferLayout> {
                vec![$(
                    self.$name.desc(),
                )*]
            }
            fn set_in<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
                $(
                    self.$name.set_in(render_pass);
                )*
            }
            fn len(&self) -> usize {
                self.len
            }
            fn update_rows<'a>(
                &mut self,
                device: &wgpu::Device,
                encoder: &mut wgpu::CommandEncoder,
                belt: &mut wgpu::util::StagingBelt,
                data: &Self::UpdateData<'a>,
                width: usize,
                size: usize,
            ) {
                self.len = size;
                $(
                    self.$name.update_rows(
                        device,
                        encoder,
                        belt,
                        data.$name.chunks_exact(width),
                        width,
                        size,
                    );
                )*
            }
        }
    };
}
