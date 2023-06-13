use crate::render::{
    buffer::{Instance, instance_descs},
    rsc::square::{INDICES, VERTICES},
    state::Buffers,
    uniform::TileViewUniform,
    Instances, Uniforms,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BufferUsages, Device, RenderPipeline, SurfaceConfiguration,
};
use winit::dpi::PhysicalSize;

use crate::{
    camera::Camera,
    render::{buffer::Vertex, uniform::CameraUniform},
};

pub const SHADER: &str = concat!(include_str!("../shader/tile.wgsl"));

pub fn init_renderer(
    device: &Device,
    config: &SurfaceConfiguration,
    camera: &Camera,
    size: &PhysicalSize<u32>,
) -> (RenderPipeline, Instances, Buffers, Uniforms, BindGroup) {
    // shaders
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });

    // buffers
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: BufferUsages::INDEX,
    });

    let instances = Instances {
        connex_number: Instance::init(device, "Connex Number"),
        conductivity: Instance::init(device, "Conductivity Number"),
        reactivity: Instance::init(device, "Reactivity Number"),
        energy: Instance::init(device, "Energy Number"),
    };

    let camera_uniform = CameraUniform::new(camera, size);
    let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let tile_view_uniform = TileViewUniform::new([0.0, 0.0], 0);
    let tile_view_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Tile View Buffer"),
        contents: bytemuck::cast_slice(&[tile_view_uniform]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    // bind groups
    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("camera_bind_group_layout"),
        });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: tile_view_buffer.as_entire_binding(),
            },
        ],
        label: Some("camera_bind_group"),
    });

    // pipeline
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&camera_bind_group_layout],
        push_constant_ranges: &[],
    });

    let mut bufs = vec![];
    bufs.push(Vertex::desc());
    bufs.extend(instance_descs());
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &bufs,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    (
        render_pipeline,
        instances,
        Buffers {
            vertex: vertex_buffer,
            index: index_buffer,
            camera: camera_buffer,
            tile_view: tile_view_buffer,
        },
        Uniforms {
            camera: camera_uniform,
            tile_view: tile_view_uniform,
        },
        camera_bind_group,
    )
}
