use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Device, Queue, RenderPipeline,
};

use crate::render::{surface::RenderSurface, ui::texture::GameTexture};

use super::{
    pipeline::{UIPipeline, SHAPE_SHADER, TEXTURE_SHADER},
    text::UIText,
    uniform::WindowUniform,
    vertex::{TextureVertex, TEXTURE_VERTICES},
};

impl UIPipeline {
    pub fn new(surface: &RenderSurface) -> Self {
        let (texture_pipeline, texture_vertex_buffer, diffuse_bind_group) =
            Self::init_texture_pipeline(surface);
        let (shape_pipeline, window_buf, shape_bind_group) = Self::init_shape_pipeline(surface);

        Self {
            texture_pipeline,
            texture_vertex_buffer,
            diffuse_bind_group,
            shape_pipeline,
            shape_bind_group,
            buffers: super::pipeline::UIBuffers { window: window_buf },
            text: UIText::init(surface),
        }
    }

    fn init_shape_pipeline(surface: &RenderSurface) -> (RenderPipeline, Buffer, BindGroup) {
        let RenderSurface { device, config, .. } = surface;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(SHAPE_SHADER.into()),
        });

        let window_uniform = WindowUniform::new();
        let window_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[window_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // bind groups
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: window_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Shape Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Shape Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
        (pipeline, window_buffer, bind_group)
    }

    fn init_texture_pipeline(surface: &RenderSurface) -> (RenderPipeline, Buffer, BindGroup) {
        let RenderSurface {
            device,
            config,
            queue,
            ..
        } = surface;

        let (bind_group_layout, diffuse_bind_group) = Self::init_textures(device, queue);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Texture Shader"),
            source: wgpu::ShaderSource::Wgsl(TEXTURE_SHADER.into()),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Vertex Buffer"),
            contents: bytemuck::cast_slice(TEXTURE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Texture Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Texture Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TextureVertex::desc()],
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
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
        (pipeline, vertex_buffer, diffuse_bind_group)
    }

    fn init_textures(device: &Device, queue: &Queue) -> (BindGroupLayout, BindGroup) {
        let diffuse_bytes = include_bytes!("./textures/happy-tree.png");
        let diffuse_texture =
            GameTexture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        (texture_bind_group_layout, diffuse_bind_group)
    }
}
