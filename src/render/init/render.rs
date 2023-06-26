use crate::{
    camera::Camera,
    render::{
        buffer::{
            CameraUniform, ConstsUniform, InstanceField, TileViewUniform, Vertex,
            SQUARE_VERTICES,
        },
        state::Buffers,
        Instances, Uniforms,
    },
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BufferUsages, Device, RenderPipeline, SurfaceConfiguration,
};
use winit::dpi::PhysicalSize;

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
        contents: bytemuck::cast_slice(SQUARE_VERTICES),
        usage: BufferUsages::VERTEX,
    });

    let instances = Instances {
        connex_number: InstanceField::init(device, "Connex Number"),
        stability: InstanceField::init(device, "Stability"),
        reactivity: InstanceField::init(device, "Reactivity"),
        energy: InstanceField::init(device, "Energy"),
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

    let consts_uniform = ConstsUniform::new();
    let consts_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Constants Buffer"),
        contents: bytemuck::cast_slice(&[consts_uniform]),
        usage: BufferUsages::UNIFORM,
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
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
            wgpu::BindGroupEntry {
                binding: 2,
                resource: consts_buffer.as_entire_binding(),
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
    bufs.extend([
        instances.connex_number.desc(),
        instances.stability.desc(),
        instances.reactivity.desc(),
        instances.energy.desc(),
    ]);
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
            camera: camera_buffer,
            tile_view: tile_view_buffer,
            consts: consts_buffer,
        },
        Uniforms {
            camera: camera_uniform,
            camera_next: camera_uniform,
            tile_view: tile_view_uniform,
            consts: consts_uniform,
        },
        camera_bind_group,
    )
}
