use crate::camera::Camera;
use wgpu::SurfaceConfiguration;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use super::{init::*, CameraUniform, Instance, TileViewUniform};

pub struct Buffers {
    pub vertex: wgpu::Buffer,
    pub index: wgpu::Buffer,
    pub camera: wgpu::Buffer,
    pub tile_view: wgpu::Buffer,
}

pub struct Uniforms {
    pub camera: CameraUniform,
    pub tile_view: TileViewUniform,
}

pub struct Instances {
    pub connex_number: Instance<u32>,
    pub conductivity: Instance<f32>,
    pub reactivity: Instance<f32>,
    pub energy: Instance<f32>,
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
            instances,
            uniforms,
            buffers,
            camera_bind_group,
        }
    }
}
