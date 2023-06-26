use crate::camera::Camera;
use wgpu::SurfaceConfiguration;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use super::{buffer::ConstsUniform, init::*, CameraUniform, InstanceField, TileViewUniform};

pub struct Buffers {
    pub vertex: wgpu::Buffer,

    pub camera: wgpu::Buffer,
    pub tile_view: wgpu::Buffer,
    pub consts: wgpu::Buffer,
}

pub struct Uniforms {
    pub camera_next: CameraUniform,
    pub camera: CameraUniform,
    pub tile_view: TileViewUniform,
    pub consts: ConstsUniform,
}

pub struct Instances {
    pub connex_number: InstanceField<1, u32>,
    pub stability: InstanceField<2, f32>,
    pub reactivity: InstanceField<3, f32>,
    pub energy: InstanceField<4, f32>,
}

pub struct BoardView {
    pub bx: f32,
    pub by: f32,
    pub xs: usize,
    pub xe: usize,
    pub ys: usize,
    pub ye: usize,
}

impl Default for BoardView {
    fn default() -> Self {
        return Self {
            bx: 0.0,
            by: 0.0,
            xs: 0,
            xe: 0,
            ys: 0,
            ye: 0,
        };
    }
}

pub struct Renderer {
    // window & device stuff
    pub window: Window,
    pub(super) surface: wgpu::Surface,
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    pub(super) config: SurfaceConfiguration,
    // render stuff
    pub(super) render_pipeline: wgpu::RenderPipeline,
    pub(super) slice: BoardView,
    pub(super) instances: Instances,
    pub(super) buffers: Buffers,
    pub(super) uniforms: Uniforms,
    pub(super) camera_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub async fn new(event_loop: &EventLoop<()>, camera: &Camera) -> Renderer {
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let size = window.inner_size();

        let (surface, device, queue, config) = init_surface(&window).await;

        let (render_pipeline, instances, buffers, uniforms, camera_bind_group) =
            init_renderer(&device, &config, &camera, &size);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            slice: Default::default(),
            instances,
            uniforms,
            buffers,
            camera_bind_group,
        }
    }

    pub fn pixel_to_render(&self, pos: [f32; 2]) -> [f32; 2] {
        let size = self.window.inner_size();
        return [
            pos[0] * 2.0 / size.width as f32 - 1.0,
            -pos[1] * 2.0 / size.height as f32 + 1.0,
        ];
    }

    pub fn pixel_to_world(&self, pos: [f32; 2]) -> [f32; 2] {
        self.uniforms.camera.render_to_world(self.pixel_to_render(pos))
    }
}
