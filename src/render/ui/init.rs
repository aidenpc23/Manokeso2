use wgpu_glyph::{ab_glyph, GlyphBrushBuilder};
use winit::dpi::PhysicalSize;

use crate::render::surface::RenderSurface;

use super::{pipeline::{UIPipeline, SHADER}, layout::UIText};

impl UIPipeline {
    pub fn new(surface: &RenderSurface, size: &PhysicalSize<u32>) -> Self {
        let RenderSurface { device, config, .. } = surface;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("UI Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Pipeline"),
            layout: Some(&render_pipeline_layout),
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
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
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

        let font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("./fonts/NotoSerif-Regular.ttf"))
                .expect("Failed to load font");
        let brush = GlyphBrushBuilder::using_font(font).build(device, config.format);
        Self { pipeline, brush, text: UIText::new() }
    }
}
