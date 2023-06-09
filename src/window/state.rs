use crate::window::uniform::CameraUniform;
use crate::Camera;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use super::{buffer::Instance, init::*};

pub struct Buffers {
    pub vertex: wgpu::Buffer,
    pub index: wgpu::Buffer,
    pub instance: wgpu::Buffer,
    pub camera: wgpu::Buffer,
}

pub struct GameWindow {
    // window & device stuff
    pub window: Window,
    pub(super) surface: wgpu::Surface,
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    // render stuff
    pub(super) render_pipeline: wgpu::RenderPipeline,
    pub(super) instances: Vec<Instance>,
    pub(super) buffer: Buffers,
    // camera
    pub(super) camera_uniform: CameraUniform,
    pub(super) camera_bind_group: wgpu::BindGroup,
}

impl GameWindow {
    pub async fn new(event_loop: &EventLoop<()>, camera: &Camera) -> GameWindow {
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let (surface, device, queue, config) = init_surface(&window).await;

        let (
            render_pipeline,
            instances,
            buffer,
            camera_uniform,
            camera_bind_group,
        ) = init_renderer(&device, &config, &camera);

        Self {
            window,
            surface,
            device,
            queue,
            render_pipeline,
            instances,
            camera_uniform,
            buffer,
            camera_bind_group,
        }
    }
}

